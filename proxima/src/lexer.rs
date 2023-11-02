use std::str::Chars;

use crate::interner::{IdentifierId, PathId, StringId, DUMMY_IDENTIFIER_ID, DUMMY_STRING_ID};

struct Lexer<'a> {
    path: PathId,
    source: &'a str,
    current: Option<char>,
    next: Option<char>,
    chars: Chars<'a>,
    offset: usize,
    processed_identifier: IdentifierId,
    processed_string: StringId,
    processed_number: f64,
}

impl<'a> Lexer<'a> {
    pub fn new(path: PathId, source: &'a str) -> Self {
        let eof_offset = source.len();
        let mut chars = source.chars();

        let current = chars.next();
        let next = chars.next();

        Self {
            path,
            chars: source.chars(),
            source,
            offset: 0,
            current,
            next,
            processed_identifier: DUMMY_IDENTIFIER_ID,
            processed_string: DUMMY_STRING_ID,
            processed_number: 0.0,
        }
    }
}
