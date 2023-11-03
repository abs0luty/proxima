use crate::{
    interner::PathId,
    lexer::Lexer,
    location::{HasLocation, Location},
    token::{LexError, RawToken, Token},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Lex(LexError),
    UnexpectedToken { expected: RawToken, found: Token },
}

pub struct Parser {
    path: PathId,
    tokens: Vec<Token>,
    current_token_idx: usize,
}

impl Parser {
    #[inline]
    #[must_use]
    pub const fn new_from_tokens(path: PathId, tokens: Vec<Token>) -> Self {
        Self {
            path,
            tokens,
            current_token_idx: 0,
        }
    }

    #[inline]
    #[must_use]
    pub fn new(path: PathId, source: &str) -> Self {
        Self::new_from_tokens(path, Lexer::new(path, source).collect())
    }

    fn consume(&mut self, raw: RawToken) -> Result<Token, Error> {
        let current = self.current();
        if raw != current.raw() {
            return Err(Error::UnexpectedToken {
                expected: raw,
                found: current,
            });
        }

        self.current_token_idx += 1;
        Ok(current)
    }

    fn current(&self) -> Token {
        self.get(0)
    }

    fn next(&self) -> Token {
        self.get(1)
    }

    fn get(&self, offset: usize) -> Token {
        self.tokens
            .get(self.current_token_idx + offset)
            .copied()
            .unwrap_or(Token::new(
                RawToken::EndOfFile,
                self.tokens
                    .last()
                    .map(|token| token.location())
                    .unwrap_or(Location::of_first_byte()),
            ))
    }
}
