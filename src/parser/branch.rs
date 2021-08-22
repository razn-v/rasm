use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::label::Label;
use crate::parser::parser::InstrParser;
use crate::get_label;

pub struct BranchParser;

impl BranchParser {
    /// Return the binary representation of the "Branch and Branch with Link"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token], labels: &Vec<Label>) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;
        // If true, branch with link
        let link = opcode.to_string().ends_with("l") as u32;

        let label = get_label!(tokens, 1, labels);
        // Calculate offset and take the lower 24 bits
        let offset = label.offset(&tokens[0], 24);

        return ((cond << 3 | 0b101) << 1 | link) << 24 | offset;
    }
}
