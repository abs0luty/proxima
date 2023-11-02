use crate::location::SpanLocation;
use derive_more::Display;
use paste::paste;
use std::fmt::Display;

macro_rules! keywords {
    ($($kw:ident),*) => {
        paste! {
            #[derive(Clone, Copy)]
            pub enum Keyword {
                $([<$kw:camel>]),*
            }

            impl Keyword {
                pub fn from(s: &str) -> Option<Self> {
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
pub enum Punctuator {
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

/// Represents error that scanning process can fail with.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
pub enum Error {
    #[display(fmt = "digit doesn't correspond to base")]
    DigitDoesNotCorrespondToBase,
    #[display(fmt = "empty character literal")]
    EmptyCharacterLiteral,
    #[display(fmt = "empty escape sequence")]
    EmptyEscapeSequence,
    #[display(fmt = "empty wrapped identifier literal")]
    EmptyWrappedIdentifier,
    #[display(fmt = "expected `}}` in byte escape sequence")]
    ExpectedCloseBracketInByteEscapeSequence,
    #[display(fmt = "expected `}}` in Unicode escape sequence")]
    ExpectedCloseBracketInUnicodeEscapeSequence,
    #[display(fmt = "expected digit in byte escape sequence")]
    ExpectedDigitInByteEscapeSequence,
    #[display(fmt = "expected digit in Unicode escape sequence")]
    ExpectedDigitInUnicodeEscapeSequence,
    #[display(fmt = "expected `{{` in byte escape sequence")]
    ExpectedOpenBracketInByteEscapeSequence,
    #[display(fmt = "expected `{{` in Unicode escape sequence")]
    ExpectedOpenBracketInUnicodeEscapeSequence,
    #[display(fmt = "exponent has no digits")]
    ExponentHasNoDigits,
    #[display(fmt = "exponent requires decimal mantissa")]
    ExponentRequiresDecimalMantissa,
    #[display(fmt = "number contains no digits")]
    NumberContainsNoDigits,
    #[display(fmt = "invalid byte escape sequence")]
    InvalidByteEscapeSequence,
    #[display(fmt = "invalid digit")]
    InvalidDigit,
    #[display(fmt = "invalid radix point")]
    InvalidRadixPoint,
    #[display(fmt = "invalid Unicode escape sequence")]
    InvalidUnicodeEscapeSequence,
    #[display(fmt = "more than one character in character literal")]
    MoreThanOneCharInCharLiteral,
    #[display(fmt = "number cannot be parsed")]
    NumberParseError,
    #[display(fmt = "underscore must separate successive digits")]
    UnderscoreMustSeparateSuccessiveDigits,
    #[display(fmt = "unexpected character")]
    UnexpectedChar,
    #[display(fmt = "unknown escape sequence")]
    UnknownEscapeSequence,
    #[display(fmt = "untermined character literal")]
    UnterminatedCharLiteral,
    #[display(fmt = "unterminated string literal")]
    UnterminatedStringLiteral,
    #[display(fmt = "unterminated wrapped identifier")]
    UnterminatedWrappedIdentifier,
}

#[derive(Clone, Copy)]
pub enum RawToken {
    Punctuator(Punctuator),
    Keyword(Keyword),
    Error(Error),
    Identifier,
    Number,
    Text,
}

impl From<Keyword> for RawToken {
    fn from(kw: Keyword) -> Self {
        RawToken::Keyword(kw)
    }
}

impl From<Punctuator> for RawToken {
    fn from(p: Punctuator) -> Self {
        RawToken::Punctuator(p)
    }
}

impl From<Error> for RawToken {
    fn from(e: Error) -> Self {
        RawToken::Error(e)
    }
}

pub struct Token {
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
