use crate::opcode::OpCode;
use crate::token::Token;
use crate::parser::parser::InstrParser;
use crate::register::Register;
use crate::get_reg;

pub struct BrXchgParser;

impl BrXchgParser {
    /// Return the binary representation of the "Branch and Exchange"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // Register
        let rn = get_reg!(tokens, 1);
        if rn == Register::CPSR as u32 {
            panic!("Invalid register!");
        }

        return (cond << 24 | 0b0001_0010_1111_1111_1111_0001) << 4 | rn;
    }
}
