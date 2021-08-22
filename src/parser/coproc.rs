use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::label::Label;
use crate::parser::parser::InstrParser;
use crate::{
    get_cpn, expect_token, get_number, get_creg, get_reg, get_token, get_label
};

pub struct CpOpsParser;

impl CpOpsParser {
    /// Return the binary representation of the "Coprocessor Data Operations"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // Coprocessor number
        let cpn = get_cpn!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);

        // Coprocessor operation code
        let cpopc = get_number!(tokens, 3).value;
        expect_token!(tokens, 4, TokenType::Comma);

        // Coprocessor registers
        let crd = get_creg!(tokens, 5);
        expect_token!(tokens, 6, TokenType::Comma);
        let crn = get_creg!(tokens, 7);
        expect_token!(tokens, 8, TokenType::Comma);
        let crm = get_creg!(tokens, 9);

        // Optional field
        // Coprocessor information
        let mut cp = 0u32;
        if tokens.get(10).is_some() {
            expect_token!(tokens, 10, TokenType::Comma);
            cp = get_number!(tokens, 11).value;

            // Larger than 3 bits
            if cp > 7 {
                panic!("Immediate value out of range.");
            }
        }

        return (((((((cond << 4 | 0b1110) << 4 | cpopc) << 4 | crn) << 4 | crd)
            << 4 | cpn) << 3 | cp) << 1 | 0b0) << 4 | crm;
    }
}

pub struct CpTransfersParser;

impl CpTransfersParser {
    /// Return the binary representation of the "Coprocessor Data Transfers"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token], labels: &Vec<Label>) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // If true, add offset before transfer, otherwise add offset after
        // transfer
        let mut pre = false as u32;
         // If true, add offset to base, otherwise substract offset from base
        let mut up = true as u32;
        // If true, perform long transfer, otherwise perform short transfer
        let trans_len = tokens.get(0).unwrap().value
            // Remove cond to avoid any false positive
            .replace(&cond.to_string(), "")
            .ends_with("l") as u32;
        let write = tokens.last().unwrap().value.eq("!") as u32;
        // If true, load from memory, otherwise store to memory
        let load = (opcode == OpCode::LDC) as u32;

        // Coprocessor number
        let cpn = get_cpn!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);

        // Coprocessor register
        let crd = get_creg!(tokens, 3);
        expect_token!(tokens, 4, TokenType::Comma);

        // Now we parse an <address> which can either be:
        //    1 - <expression>
        //
        //  2.1 - [Rn]
        //    2 - [Rn,<#expression>]{!}
        //
        //    3 - [Rn],<#expression>
        let mut offset = 0u32;
        let rn;

        let token = get_token!(tokens, 5, [TokenType::Keyword,
            TokenType::OpenBracket]);
        if token.token_type == TokenType::Keyword {
            // Case 1
            let label = get_label!(tokens, 5, labels);
            // Calculate the offset and take the lower 8 bits
            offset = label.offset(&tokens[0], 8);
            pre = true as u32;
            up = false as u32;
            rn = 15;
        } else {
            // Case 2.1, 2.2 and 3
            rn = get_reg!(tokens, 6);

            // Check if token 8 exists, if not, token 7 must be a close bracket
            if !tokens.get(8).is_some() {
                // Case 2.1
                // Offset is zero
                expect_token!(tokens, 7, TokenType::CloseBracket);
            } else {
                // Case 2.2 and 3
                let mut idx = 8;
                let token = get_token!(tokens, 7, [TokenType::Comma,
                    TokenType::CloseBracket]);
                if token.token_type == TokenType::CloseBracket {
                    // Case 3
                    expect_token!(tokens, 8, TokenType::Comma);
                    idx = 9;
                } else {
                    pre = true as u32;
                }

                let imm = get_number!(tokens, idx).value;
                if imm % 4 != 0 {
                    panic!("Coprocessor offset out of range.");
                }
                offset = imm / 4;
            }
        }

        return (((((((((cond << 3 | 0b110) << 1 | pre) << 1 | up)
            << 1 | trans_len) << 1 | write) << 1 | load) << 4 | rn)
            << 4 | crd) << 4 | cpn) << 8 | offset;
    }
}

pub struct CpRegTransParser;

impl CpRegTransParser {
    /// Return the binary representation of the "Coprocessor Register Transfers"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // If true, load from memory, otherwise store to memory
        let load = (opcode == OpCode::MRC) as u32;

        // Coprocessor number
        let cpn = get_cpn!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);

        // Coprocessor operation code
        let cpopc = get_number!(tokens, 3).value;
        expect_token!(tokens, 4, TokenType::Comma);

        let rd = get_reg!(tokens, 5);
        expect_token!(tokens, 6, TokenType::Comma);

        // Coprocessor registers
        let crn = get_creg!(tokens, 7);
        expect_token!(tokens, 8, TokenType::Comma);
        let crm = get_creg!(tokens, 9);

        // Optional field
        // Coprocessor information
        let mut cp = 0u32;
        if tokens.get(10).is_some() {
            expect_token!(tokens, 10, TokenType::Comma);
            cp = get_number!(tokens, 11).value;

            // Larger than 3 bits
            if cp > 7 {
                panic!("Immediate value out of range.");
            }
        }

        return ((((((((cond << 4 | 0b1110) << 3 | cpopc) << 1 | load)
            << 4 | crn) << 4 | rd) << 4 | cpn) << 3 | cp) << 1 | 0b1)
            << 4 | crm;
    }
}
