use crate::token::{Token, TokenType};
use std::ops::Range;

/// This let us track our position on the input and the position of the
/// currently lexed token
struct Cursor {
    /// Position of the current char being lexed
    pos: usize,
    /// Position (start) of the current token being lexed
    token_pos: usize,
    /// Line number, increased at each newline token found
    line: usize,
}

pub struct Lexer<'a> {
    cursor: Cursor,
    input: &'a String,
    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            cursor: Cursor { pos: 0, token_pos: 0, line: 0 },
            input,
            tokens: Vec::new(),
        }
    }

    /// Transform the input into a list of tokens. At the end of each 'get'
    /// function, the cursor must point to the last character of the token.
    pub fn lex(&mut self) {
        // While we have char left to lex
        while let Some(chr) = self.peek(0) {
            self.cursor.token_pos = self.cursor.pos;

            if chr.is_ascii_alphabetic() || chr == '_' {
                self.get_keyword_token();
            } else if chr == '#' || chr.is_ascii_digit() {
                self.get_number_token();
            } else if chr == '!' || chr == '+' || chr == '-' || chr == '['
                    || chr == ']' || chr == '{' || chr == '}' || chr == ','
                    || chr == '^' || chr == '\n' {
                self.get_symbol_token();
            } else if chr == ' ' {
                // Do nothing
            } else {
                panic!("Invalid char '{}' found while lexing.", chr);
            }

            self.step();
        }
    }

    /// Return the next nth character of the input
    fn peek(&self, n: usize) -> Option<char> {
        if self.cursor.pos + n >= self.input.len() {
            return None;
        }
        Some(self.input.as_bytes()[self.cursor.pos + n] as char)
    }

    /// Return the input in the range `range` plus the current position of the
    /// cursor
    fn peek_range(&self, range: Range<usize>) -> Option<&str> {
        let mut range = range;
        range.start += self.cursor.pos;
        range.end += self.cursor.pos;

        if range.end < self.input.len() {
            return Some(&self.input[range]);
        }
        None
    }

    fn step(&mut self) {
        self.cursor.pos += 1;
    }

    fn push_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            value: self.input[self.cursor.token_pos..self.cursor.pos + 1]
                .to_string(),
            line: self.cursor.line,
        });
    }

    fn get_keyword_token(&mut self) {
        // Step through the entire keyword
        while let Some(chr) = self.peek(1) {
            if chr.is_ascii_alphabetic() || chr.is_ascii_digit() || chr == '_' {
                self.step();
            } else if chr == ':' {
                // If the keyword contains ':', we assume its a label
                self.step();
                self.push_token(TokenType::Label);
                return;
            } else {
                // End of the token
                break;
            }
        }
        self.push_token(TokenType::Keyword);
    }

    fn get_number_token(&mut self) {
        // Skip '#'
        if self.peek(0).unwrap() == '#' {
            self.step();
        }

        let mut is_hex = false;
        // If the number is in hex format
        if let Some(s) = self.peek_range(0..2) {
            if s == "0x" {
                is_hex = true;
                // Skip "0"
                self.step();
                // Skip "x"
                self.step();
            }
        }

        while let Some(chr) = self.peek(1) {
            // Accept hex digit if the number is in hex format
            let cond = match is_hex {
                true => chr.is_ascii_hexdigit(),
                false => chr.is_ascii_digit(),
            };

            if cond {
                self.step();
            } else {
                // End of the token
                break;
            }
        }
        self.push_token(TokenType::Number);
    }

    fn get_symbol_token(&mut self) {
        let ty = match self.peek(0).unwrap() {
            '!'  => TokenType::Exclamation,
            '+'  => TokenType::Plus,
            '-'  => TokenType::Minus,
            '['  => TokenType::OpenBracket,
            ']'  => TokenType::CloseBracket,
            '{'  => TokenType::OpenCurlyBrace,
            '}'  => TokenType::CloseCurlyBrace,
            ','  => TokenType::Comma,
            '^'  => TokenType::Caret,
            // Ignore useless endlines
            '\n' if self.cursor.pos != 0 => {
                self.cursor.line += 1;
                TokenType::Endline
            }
            _    => return,
        };
        self.push_token(ty);
    }
}
