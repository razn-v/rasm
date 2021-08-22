use std::collections::HashSet;

use crate::opcode::OpCode;
use crate::register::Register;
use crate::psrf::PSRF;
use crate::token::{Token, TokenType};
use crate::label::Label;
use crate::parser::parser::InstrParser;
use crate::{
    expect_token, get_token, get_reg, get_number, get_shift_type, get_label
};

pub struct PsrTransferParser;

impl PsrTransferParser {
    /// Return the binary representation of the "PSR Transfer" instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;
        // Program status register
        let psr;
        // PSR format
        let psrf;

        if opcode == OpCode::MRS {
            let rd = get_reg!(tokens, 1);
            expect_token!(tokens, 2, TokenType::Comma);

            psr = get_reg!(tokens, 3);
            if psr == Register::CPSR as u32 || psr == Register::SPSR as u32 {
                let ps = (psr == Register::SPSR as u32) as u32;
                return ((((cond << 5 | 0b00010) << 1 | ps) << 6 | 0b001111)
                    << 4 | rd) << 12 | 0b000000000000;
            }

            panic!("Expected CPSR or SPSR.");
        }

        // Try to parse a register
        if let Some(reg) = InstrParser::parse_reg(&tokens[1]) {
            if reg == Register::CPSR || reg == Register::SPSR {
                expect_token!(tokens, 2, TokenType::Comma);

                psr = (reg == Register::SPSR) as u32;
                let rm = get_reg!(tokens, 3) as u32;

                return ((((cond << 5 | 0b00010) << 1 | psr)
                    << 10 | 0b1010011111) << 8 | 0b00000000) << 4 | rm;
            } else {
                panic!("Expected CPSR or SPSR.");
            }
        } else {
            // If the parsing didn't success, we know it's a flag
            psrf = InstrParser::parse_psrf(&tokens[1])
                .expect("Expected CPSR_flg or SPSR_flg.") as u32;
        }

        expect_token!(tokens, 2, TokenType::Comma);

        // The next token is either a register or an immediate
        let source_op;
        let mut is_imm = false;

        let token = get_token!(tokens, 3, [TokenType::Keyword,
            TokenType::Number]);
        if token.token_type == TokenType::Number {
            let imm = get_number!(tokens, 3);
            source_op = imm.rotate << 8 | imm.value;
            is_imm = true;
        } else {
            let rm = get_reg!(tokens, 3);
            source_op = 0b00000000 << 4 | rm;
        }

        let pd = (psrf == PSRF::SPSR as u32) as u32;

        return (((((cond << 2 | 0b00) << 1 | (is_imm as u32)) << 2 | 0b10)
            << 1 | pd) << 10 | 0b1010001111) << 12 | source_op;
    }
}

pub struct DataTransferParser;

impl DataTransferParser {
    /// Return the binary representation of the "Single Data Transfer"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token], labels: &Vec<Label>) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // Remove all characters except the letters 'B' and 'T'
        let clean_opcode = tokens[0].value
            .replace(&cond.to_string().to_lowercase(), "")
            .replace(&opcode.to_string().to_lowercase(), "");

        // Check if the transfer is halfword and signed
        let hw_sgd = match clean_opcode.as_ref() {
            "h" | "sb" | "sh" => true,
            _ => false,
        };

        // True if the offset is an immediate value, false if it's a register
        let mut is_reg = false as u32;
        // If true, add offset before transfer, otherwise add offset after
        // transfer
        let mut pre = false as u32;
        // If true, add offset to base, otherwise substract offset from base
        let mut up = true as u32;
        // If true, byte transfer, otherwise word transfer
        let byte_trans = (!hw_sgd && clean_opcode.contains("b")) as u32;
        // Writes back the base register if true
        let mut write = tokens.last().unwrap().value.eq("!") as u32;
        // If true, load from memory, otherwise store to memory
        let load = (opcode == OpCode::LDR) as u32;
        // If true, write will be set to true in a post-indexed instruction
        // (case 3 below)
        let t = !hw_sgd && clean_opcode.contains("t");

        // First register
        let rd = get_reg!(tokens, 1);
        expect_token!(tokens, 2, TokenType::Comma);

        // Now we parse an <address> which can either be:
        //    1 - <expression>
        //
        //  2.1 - [Rn]
        //    2 - [Rn,<#expression>]{!}
        //    3 - [Rn,{+/-}Rm{,<shift>}]{!}
        //
        //  3.1 - [Rn],<#expression>
        //    2 - [Rn],{+/-}Rm{,<shift>}
        //
        // Note: There is no shift if the transfer is halfword and signed.

        // Second register, used as a base for the offset
        let rn;
        let mut rm = 0u32;
        let mut offset = 0u32;

        let token = get_token!(tokens, 3, [TokenType::OpenBracket,
            TokenType::Keyword]);
        if token.token_type == TokenType::OpenBracket {
            // Case 2.* and 3.*
            // The next token after the bracket is always a register
            rn = get_reg!(tokens, 4);

            // Check if there is no more token after token 5
            if tokens.get(6).is_none() {
                // Case 2.1
                // Offset is zero
                expect_token!(tokens, 5, TokenType::CloseBracket);
                pre = true as u32;
            } else {
                // Case 2.2, 2.3 and 3.*
                let mut idx = 5;

                // We expect either a comma or a close bracket
                let token = get_token!(tokens, idx, [TokenType::Comma,
                    TokenType::CloseBracket]);
                if token.token_type == TokenType::CloseBracket {
                    // Post-indexed instruction
                    expect_token!(tokens, idx + 1, TokenType::Comma);
                    idx = 6;
                    // Set W bit if letter 't' is present
                    write = t as u32;
                } else {
                    pre = true as u32;
                }

                let token = get_token!(tokens, idx + 1,
                    [TokenType::Keyword, TokenType::Number, TokenType::Plus,
                    TokenType::Minus]);
                if token.token_type == TokenType::Number {
                    // Case 2.2 and 3.1
                    offset = get_number!(tokens, idx + 1).value;
                } else {
                    // Case 2.3 and 3.2
                    let token = get_token!(tokens, idx + 1,
                        [TokenType::Plus, TokenType::Minus,
                        TokenType::Keyword]);
                    if token.token_type != TokenType::Keyword {
                        // Set to sustract offset from base if false
                        up = (token.token_type == TokenType::Plus) as u32;
                        idx += 1;
                    }

                    rm = get_reg!(tokens, idx + 1);
                    let mut shift = 0u32;
                    // Check if the last token exists, we add write to idx
                    // in case we have an exclamation point at the end
                    if tokens.get(idx + 3 + write as usize).is_some()
                        && !hw_sgd {
                        // The next token must be a comma if the previous token
                        // exists
                        expect_token!(tokens, idx + 2, TokenType::Comma);

                        let shift_type = get_shift_type!(tokens, idx + 3);
                        // Can't be a register
                        let shift_value = get_number!(tokens, idx + 4)
                            .value;
                        shift = (shift_value << 2 | shift_type) << 1 | 0;
                    }

                    is_reg = true as u32;
                    offset = shift << 4 | rm;
                }

                if idx == 5 {
                    // The instruction must end with a close bracket
                    expect_token!(tokens, tokens.len() - 1 - write as
                        usize, TokenType::CloseBracket);
                }
            }
        } else {
            // Case 1
            let label = get_label!(tokens, 3, labels);
            // Calculate the offset and take the lower 12 bits
            offset = label.offset(&tokens[0], 12);
            pre = true as u32;
            up = false as u32;
            rn = 15;
        }

        if hw_sgd {
            let sh = match clean_opcode.as_ref() {
                // Unsigned halfwords
                "h"  => 0b01,
                // Signed byte
                "sb" => 0b10,
                // Signed halfwords
                "sh" => 0b11,
                _    => unreachable!(),
            };

            if is_reg == true as u32 {
                return (((((((((((cond << 3 | 0b000) << 1 | pre) << 1 | up)
                    << 1 | 0b0) << 1 | write) << 1 | load) << 4 | rn)
                    << 4 | rd) << 5 | 0b00001) << 2 | sh) << 1 | 0b1) << 4 | rm;
            }

            let offset_hi = offset >> 4;
            let offset_lo = offset & 0xf;

            return ((((((((((((cond << 3 | 0b000) << 1 | pre) << 1 | up)
                << 1 | 0b1) << 1 | write) << 1 | load) << 4 | rn)
                << 4 | rd) << 4 | offset_hi) << 1 | 0b1) << 2 | sh) << 1 | 0b1)
                << 4 | offset_lo;
        }


        return (((((((((cond << 2 | 0b01) << 1 | is_reg) << 1 | pre) << 1 | up)
            << 1 | byte_trans) << 1 | write) << 1 | load) << 4 | rn)
            << 4 | rd) << 12 | offset;
    }
}


pub struct BlockTransferParser;

impl BlockTransferParser {
    /// Return the binary representation of the "Block Data Transfer"
    /// instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]) as u32;

        // Remove condition code
        let clean_opcode = tokens[0].value
            .replace(&cond.to_string().to_lowercase(), "");

        // If true, add offset before transfer, otherwise add offset after
        // transfer
        let pre: u32;
        // If true, add offset to base, otherwise substract offset from base
        let up: u32;
        // If true, load PSR or force user mode, otherwise do not load PSR or
        // force user mode
        let force = tokens.last().unwrap().value.eq("^") as u32;
        // Writes back the base register if true
        let mut write = false as u32;
        // If true, load from memory, otherwise store to memory
        let load: u32;

        match clean_opcode.as_ref() {
            "ldmed" | "ldmib" => {
                load  = true as u32;
                pre   = true as u32;
                up    = true as u32;
            },
            "ldmfd" | "ldmia" => {
                load  = true as u32;
                pre   = false as u32;
                up    = true as u32;
            },
            "ldmea" | "ldmdb" => {
                load  = true as u32;
                pre   = true as u32;
                up    = false as u32;
            },
            "ldmfa" | "ldmda" => {
                load  = true as u32;
                pre   = false as u32;
                up    = false as u32;
            },
            "stmfa" | "stmib" => {
                load  = false as u32;
                pre   = true as u32;
                up    = true as u32;
            },
            "stmea" | "stmia" => {
                load  = false as u32;
                pre   = false as u32;
                up    = true as u32;
            },
            "stmfd" | "stmdb" => {
                load  = false as u32;
                pre   = true as u32;
                up    = false as u32;
            },
            "stmed" | "stmda" => {
                load  = false as u32;
                pre   = false as u32;
                up    = false as u32;
            },
            _ => panic!("Invalid opcode."),
        };

        // First register
        let rn = get_reg!(tokens, 1);
        // List of registers
        let mut rlist = HashSet::<u32>::new();

        let mut idx = 2;
        let token = get_token!(tokens, 2, [TokenType::Exclamation,
            TokenType::Comma]);
        if token.token_type == TokenType::Exclamation {
            write = true as u32;
            idx = 3;
        }

        expect_token!(tokens, idx, TokenType::Comma);
        expect_token!(tokens, idx + 1, TokenType::OpenCurlyBrace);

        // Index of the close brace, marking the end of rlist
        let curly_idx = tokens.iter()
            .position(|token| token.token_type == TokenType::CloseCurlyBrace)
            .expect("Close brace expected at the end of instruction.");

        // Start at the first register after the open brace
        let mut i = idx + 2;
        // Loop until we hit the close brace
        while i < curly_idx {
            let reg1 = get_reg!(tokens, i);

            // If there is no registers left
            if i + 1 == curly_idx {
                rlist.insert(reg1);
                break;
            }

            // The next token is either a comma or a minus (for register range)
            let next_token = get_token!(tokens, i + 1,
                [TokenType::Comma, TokenType::Minus]);
            if next_token.token_type == TokenType::Minus {
                // Get the second register marking the end of the register range
                let reg2 = get_reg!(tokens, i + 2);
                // Push every register in the range
                for n in reg1..reg2 + 1 {
                    rlist.insert(n);
                }

                // If there is no registers left
                if i + 3 == curly_idx {
                    break;
                }

                expect_token!(tokens, i + 3, TokenType::Comma);
                i += 4;
            } else {
                rlist.insert(reg1);
                i += 2;
            }
        }

        // Binary representation of rlist
        let mut rlist_bin = 0u32;
        for reg in rlist {
            rlist_bin |= 2u32.pow(reg);
        }

        return (((((((cond << 3 | 0b100) << 1 | pre) << 1 | up) << 1 | force)
            << 1 | write) << 1 | load) << 4 | rn) << 16 | rlist_bin;
    }
}
