use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::parser::parser::InstrParser;
use crate::{get_reg, expect_token};

pub struct DataSwapParser;

impl DataSwapParser {
    /// Return the binary representation of the "Single Data Swap" instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // If true, swap byte quantity, otherwise swap word quantity
        let is_byte = tokens[0].value.ends_with("b") as u32;

        let rd = get_reg!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);

        let rm = get_reg!(tokens, 3);
        expect_token!(tokens, 4, TokenType::Comma);

        expect_token!(tokens, 5, TokenType::OpenBracket);
        let rn = get_reg!(tokens, 6);
        expect_token!(tokens, 7, TokenType::CloseBracket);

        return ((((((cond << 5 | 0b00010) << 1 | is_byte) << 2 | 0b00)
            << 4 | rn) << 4 | rd) << 8 | 0b00001001) << 4 | rm;
    }
}
