use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::parser::parser::InstrParser;
use crate::{get_reg, expect_token};

pub struct MulParser;

impl MulParser {
    /// Return the binary representation of the
    /// "Multiply and Multiply-Accumulate" instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]);

        let accumulate = (opcode == OpCode::MLA) as u32;
        // If true, condition codes wil be altered
        let set_cond = tokens[0].value
            // Remove cond to avoid any fake positive
            .replace(&cond.to_string().to_lowercase(), "")
            .ends_with("s") as u32;

        // We expect 3 registers for MUL and 4 for MLA
        let rd = get_reg!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);
        let rm = get_reg!(tokens, 3);
        expect_token!(tokens, 4, TokenType::Comma);
        let rs = get_reg!(tokens, 5);

        // The 4th register is only used by MLA
        let mut rn = 0b0000;
        if opcode == OpCode::MLA {
            // Make sure we have a comma after Rs
            expect_token!(tokens, 6, TokenType::Comma);
            rn = get_reg!(tokens, 7);
        }

        // Convert boolean to u32
        let cond = cond as u32;

        return (((((((cond << 6 | 0b000000) << 1 | accumulate) << 1 | set_cond)
            << 4 | rd) << 4 | rn) << 4 | rs) << 4 | 0b1001) << 4 | rm;
    }
}
