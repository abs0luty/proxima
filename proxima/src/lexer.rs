use std::str::Chars;

use crate::{
    interner::{IdentifierId, PathId, StringId, DUMMY_IDENTIFIER_ID, DUMMY_STRING_ID},
    location::{CharLocation, SpanLocation},
    stable_likely::unlikely,
    token::{Error, Keyword, RawToken, Token},
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

    fn advance_with(&mut self, raw: impl Into<RawToken>) -> Token {
        let token = Token::new(raw.into(), self.current_byte_location());
        self.advance();

        token
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
            _ => {
                if self.current.is_id_start() {
                    return Some(self.next_identifier_or_keyword());
                }

                Some(self.advance_with(Error::UnexpectedChar))
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
