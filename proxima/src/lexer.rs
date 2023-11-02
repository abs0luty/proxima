use std::str::Chars;

use crate::{
    interner::{IdentifierId, PathId, StringId, DUMMY_IDENTIFIER_ID, DUMMY_STRING_ID},
    location::{CharLocation, SpanLocation},
    stable_likely::unlikely,
    token::{Error, Keyword, Punctuator, RawToken, Token},
};

struct Lexer<'s> {
    /// Path of the file being scanned.
    path: PathId,

    /// Content of the file being scanned.
    source: &'s str,

    /// Current character.
    ///
    /// **NOTE**: Can easily be stored as `Option<char>` without worrying about additional discriminant
    /// space, because `None` is represented as `1114112u32` (not all `u32`s are
    /// valid `char`s). See https://godbolt.org/z/5nG9Pjoxh.
    ///
    /// ```
    /// assert!(std::mem::size_of::<char>() == std::mem::size_of::<Option<char>>());
    /// ```
    current: Option<char>,

    /// Next character.
    ///
    /// **NOTE**: Can be stored as `Option<char>` without worrying about additional discriminant
    /// space. See [`Lexer::current`] for more details.
    next: Option<char>,

    /// Iterator through source text characters (unicode codepoints).
    chars: Chars<'s>,

    /// Offset of the current character in the source text.
    location: CharLocation,

    /// Last processed identifier.
    processed_identifier: IdentifierId,

    /// Last processed string.
    processed_string: StringId,

    /// Last processed number.
    processed_number: f64,
}

impl<'s> Lexer<'s> {
    #[inline]
    #[must_use]
    pub fn new(path: PathId, source: &'s str) -> Self {
        let eof_offset = source.len();
        let mut chars = source.chars();

        let current = chars.next();
        let next = chars.next();

        Self {
            path,
            chars: source.chars(),
            source,
            location: CharLocation::new(1, 0, 0),
            current,
            next,
            processed_identifier: DUMMY_IDENTIFIER_ID,
            processed_string: DUMMY_STRING_ID,
            processed_number: 0.0,
        }
    }

    const fn is_eof(&self) -> bool {
        self.current.is_none()
    }

    fn advance(&mut self) {
        self.location.set_offset(
            self.location.offset()
                + match self.current {
                    Some(c) => c.len_utf8(),
                    None => 0,
                },
        );

        if self.current == Some('\n') {
            self.location.set_line(self.location.line() + 1);
            self.location.set_column(0);
        } else {
            self.location.set_column(self.location.column() + 1);
        }

        self.current = self.next;
        self.next = self.chars.next();
    }

    fn advance_twice(&mut self) {
        self.advance();
        self.advance();
    }

    const fn current_byte_location(&self) -> SpanLocation {
        SpanLocation::new(self.location, self.location.next_byte_location())
    }

    fn advance_with(&mut self, raw: impl Into<RawToken>) -> Option<Token> {
        let token = Token::new(raw.into(), self.current_byte_location());
        self.advance();

        Some(token)
    }

    fn advance_twice_with(&mut self, raw: impl Into<RawToken>) -> Option<Token> {
        let token = Token::new(raw.into(), self.current_byte_location());
        self.advance_twice();

        Some(token)
    }

    fn advance_while(
        &mut self,
        start_location: CharLocation,
        mut f: impl FnMut(Option<char>, Option<char>) -> bool,
    ) -> &'s str {
        while f(self.current, self.next) && !self.is_eof() {
            self.advance();
        }

        &self.source[start_location.offset()..self.location.offset()]
    }

    fn skip_whitespaces(&mut self) {
        self.advance_while(self.location, |current, _| current.is_whitespace());
    }

    fn location_from(&self, start_location: CharLocation) -> SpanLocation {
        SpanLocation::new(start_location, self.location)
    }

    fn next_identifier_or_keyword(&mut self) -> Token {
        let start_location = self.location;
        let identifier_candidate =
            self.advance_while(start_location, |current, _| current.is_id_continue());

        match Keyword::from(identifier_candidate) {
            Some(kw) => Token::new(RawToken::Keyword(kw), self.location_from(start_location)),
            None => Token::new(RawToken::Identifier, self.location_from(start_location)),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();

        if unlikely(self.is_eof()) {
            return None;
        }

        match (self.current, self.next) {
            (Some(':'), _) => self.advance_with(Punctuator::Colon),
            (Some('@'), _) => self.advance_with(Punctuator::At),
            (Some('+'), Some('+')) => self.advance_twice_with(Punctuator::DoublePlus),
            (Some('+'), Some('=')) => self.advance_twice_with(Punctuator::PlusEq),
            (Some('+'), _) => self.advance_with(Punctuator::Plus),
            (Some('-'), Some('>')) => self.advance_twice_with(Punctuator::Arrow),
            (Some('-'), Some('-')) => self.advance_twice_with(Punctuator::DoubleMinus),
            (Some('-'), Some('=')) => self.advance_twice_with(Punctuator::MinusEq),
            (Some('-'), _) => self.advance_with(Punctuator::Minus),
            (Some('*'), Some('*')) => self.advance_twice_with(Punctuator::DoubleAsterisk),
            (Some('*'), Some('=')) => self.advance_twice_with(Punctuator::AsteriskEq),
            (Some('*'), _) => self.advance_with(Punctuator::Asterisk),
            (Some('/'), Some('=')) => self.advance_twice_with(Punctuator::SlashEq),
            (Some('/'), _) => self.advance_with(Punctuator::Slash),
            (Some('!'), Some('=')) => self.advance_twice_with(Punctuator::BangEq),
            (Some('!'), _) => self.advance_with(Punctuator::Bang),
            (Some('>'), Some('>')) => self.advance_twice_with(Punctuator::RightShift),
            (Some('>'), Some('=')) => self.advance_twice_with(Punctuator::GreaterEq),
            (Some('>'), _) => self.advance_with(Punctuator::Greater),
            (Some('<'), Some('<')) => self.advance_twice_with(Punctuator::LeftShift),
            (Some('<'), Some('=')) => self.advance_twice_with(Punctuator::LessEq),
            (Some('<'), _) => self.advance_with(Punctuator::Less),
            (Some('='), Some('=')) => self.advance_twice_with(Punctuator::DoubleEq),
            (Some('='), _) => self.advance_with(Punctuator::Eq),
            (Some('|'), Some('=')) => self.advance_twice_with(Punctuator::BarEq),
            (Some('|'), Some('|')) => self.advance_twice_with(Punctuator::DoubleBar),
            (Some('|'), _) => self.advance_with(Punctuator::Bar),
            (Some('?'), Some('?')) => self.advance_twice_with(Punctuator::DoubleQuestion),
            (Some('?'), Some(':')) => self.advance_twice_with(Punctuator::QuestionColon),
            (Some('?'), _) => self.advance_with(Punctuator::Question),
            (Some('&'), Some('&')) => self.advance_twice_with(Punctuator::DoubleAmpersand),
            (Some('&'), _) => self.advance_with(Punctuator::Ampersand),
            (Some('^'), Some('=')) => self.advance_twice_with(Punctuator::CaretEq),
            (Some('^'), _) => self.advance_with(Punctuator::Caret),
            (Some('~'), _) => self.advance_with(Punctuator::Tilde),
            (Some('('), _) => self.advance_with(Punctuator::OpenParent),
            (Some(')'), _) => self.advance_with(Punctuator::CloseParent),
            (Some('['), _) => self.advance_with(Punctuator::OpenBracket),
            (Some(']'), _) => self.advance_with(Punctuator::CloseBracket),
            (Some('{'), _) => self.advance_with(Punctuator::OpenBrace),
            (Some('}'), _) => self.advance_with(Punctuator::CloseBrace),
            (Some(','), _) => self.advance_with(Punctuator::Comma),
            (Some(';'), _) => self.advance_with(Punctuator::Semicolon),
            (Some('%'), Some('=')) => self.advance_with(Punctuator::PercentEq),
            (Some('%'), _) => self.advance_with(Punctuator::Percent),
            (Some('.'), Some('.')) => self.advance_twice_with(Punctuator::DoubleDot),
            _ => {
                if self.current.is_id_start() {
                    return Some(self.next_identifier_or_keyword());
                }

                self.advance_with(Error::UnexpectedChar)
            }
        }
    }
}

trait CharExt {
    fn is_whitespace(&self) -> bool;
    fn is_id_start(&self) -> bool;
    fn is_id_continue(&self) -> bool;
}

impl CharExt for Option<char> {
    fn is_whitespace(&self) -> bool {
        // Note that it is ok to hard-code the values, because
        // the set is stable and doesn't change with different
        // Unicode versions.
        matches!(
            self,
            Some('\u{0009}')   // \t
            | Some('\u{000A}') // \n
            | Some('\u{000B}') // vertical tab
            | Some('\u{000C}') // form feed
            | Some('\u{000D}') // \r
            | Some('\u{0020}') // space

            // NEXT LINE from latin1
            | Some('\u{0085}')

            // Bidi markers
            | Some('\u{200E}') // LEFT-TO-RIGHT MARK
            | Some('\u{200F}') // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | Some('\u{2028}') // LINE SEPARATOR
            | Some('\u{2029}') // PARAGRAPH SEPARATOR
        )
    }

    fn is_id_start(&self) -> bool {
        matches!(self, Some(c) if unicode_xid::UnicodeXID::is_xid_start(*c) || *c == '_')
    }

    fn is_id_continue(&self) -> bool {
        matches!(self, Some(c) if unicode_xid::UnicodeXID::is_xid_continue(*c))
    }
}
