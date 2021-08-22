use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::parser::parser::InstrParser;
use crate::{get_reg, expect_token};

pub struct MulLongParser;

impl MulLongParser {
    /// Return the binary representation of the
    /// "Multiply Long and Multiply-Accumulate Long" instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]);

        let is_signed = (opcode == OpCode::SMULL || opcode == OpCode::SMLAL)
            as u32;
        let accumulate = (opcode == OpCode::UMLAL || opcode == OpCode::SMLAL)
            as u32;
        // If true, condition codes wil be altered
        let set_cond = tokens[0].value
            // Remove cond to avoid any fake positive
            .replace(&cond.to_string().to_lowercase(), "")
            .ends_with("s") as u32;

        // We expect 4 registers
        let rdlo = get_reg!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);
        let rdhi = get_reg!(tokens, 3);
        expect_token!(tokens, 4, TokenType::Comma);
        let rm = get_reg!(tokens, 5);
        expect_token!(tokens, 6, TokenType::Comma);
        let rs = get_reg!(tokens, 7);

        // Convert boolean to u32
        let cond = cond as u32;

        return ((((((((cond << 5 | 0b0001) << 1 | is_signed) << 1 | accumulate)
            << 1 | set_cond) << 4 | rdhi) << 4 | rdlo) << 4 | rs)
            << 4 | 0b1001) << 4 | rm;
    }
}
