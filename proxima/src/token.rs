use crate::location::SpanLocation;
use paste::paste;
use std::fmt::Display;

macro_rules! keywords {
    ($($kw:ident),*) => {
        paste! {
            #[derive(Clone, Copy)]
            enum Keyword {
                $([<$kw:camel>]),*
            }

            impl Keyword {
                fn from_str(s: &str) -> Option<Self> {
                    match s {
                        $(stringify!($kw) => Some(Keyword::[<$kw:camel>])),*,
                        _ => None
                    }
                }
            }

            impl Display for Keyword {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(Keyword::[<$kw:camel>] => write!(f, "{}", stringify!($kw))),*
                    }
                }
            }
        }
    };
}

keywords!(
    struct, throw, foreach, enum, print, println, if, else, while, for, break, continue, func,
    return, using, switch, case, include, class, new
);

#[derive(Clone, Copy)]
enum Punctuator {
    Arrow,              // ->
    Eq,                 // =
    DoubleEq,           // ==
    Bang,               // !
    BangEq,             // !=
    LessEq,             // <=
    Less,               // <
    Greater,            // >
    GreaterEq,          // >=
    PlusEq,             // +=
    MinusEq,            // -=
    StarEq,             // *=
    SlashEq,            // /=
    PercentEq,          // %=
    AtEq,               // @=
    AmpersandEq,        // &=
    CaretEq,            // ^=
    BarEq,              // |=
    DoubleColonEq,      // ::=
    LeftShiftEq,        // <<=
    RightShiftEq,       // >>=
    TripleRightShiftEq, // >>>=
    DoublePlus,         // ++
    DoubleMinus,        // --
    DoubleLess,         // <<
    DoubleGreater,      // >>
    TripleGreater,      // >>>
    DoubleDot,          // ..
    DoubleStar,         // **
    QuestionColon,      // ?:
    DoubleQuestion,     // ??
    Tilde,              // ~
    Caret,              // ^
    DoubleCaret,        // ^^
    Bar,                // |
    DoubleBar,          // ||
    Ampersand,          // &
    DoubleAmpersand,    // &&
    Question,           // ?
    Colon,              // :
    DoubleColon,        // ::
    OpenParent,         // (
    CloseParent,        // )
    OpenBracket,        // [
    CloseBracket,       // ]
    OpenBrace,          // {
    CloseBrace,         // }
    Comma,              // ,
    Dot,                // .
    Number,
    HexNumber,
    New,
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    At,      // @
    Eof,
}

#[derive(Clone, Copy)]
enum RawToken {
    Punctuator(Punctuator),
    Keyword(Keyword),
    Number,
    Word,
    Text,
    EndOfFile,
}

struct Token {
    raw: RawToken,
    location: SpanLocation,
}

impl Token {
    #[inline]
    #[must_use]
    pub fn new(raw: RawToken, location: SpanLocation) -> Self {
        Self { raw, location }
    }

    #[inline]
    #[must_use]
    pub const fn raw(&self) -> RawToken {
        self.raw
    }

    #[inline]
    #[must_use]
    pub const fn location(&self) -> SpanLocation {
        self.location
    }
}
