use crate::opcode::OpCode;
use crate::token::{Token, TokenType};
use crate::parser::parser::InstrParser;
use crate::{expect_token, get_token, get_reg, get_shift_type};

pub struct DataProcParser;

impl DataProcParser {
    /// Return the binary representation of the "Data Processing" instruction
    pub fn parse(opcode: OpCode, tokens: &[Token]) -> u32 {
        // Condition code
        let cond = InstrParser::parse_cond(opcode, &tokens[0]);

        // True if operand 2 is an immediate value, false if it's a register
        let is_imm = tokens.last().unwrap().token_type == TokenType::Number
            && tokens.get(tokens.len() - 2).unwrap().token_type
            == TokenType::Comma;

        // Decimal representation of the opcode
        let opcode_n = match opcode {
            OpCode::AND => 0,
            OpCode::EOR => 1,
            OpCode::SUB => 2,
            OpCode::RSB => 3,
            OpCode::ADD => 4,
            OpCode::ADC => 5,
            OpCode::SBC => 6,
            OpCode::RSC => 7,
            OpCode::TST => 8,
            OpCode::TEQ => 9,
            OpCode::CMP => 10,
            OpCode::CMN => 11,
            OpCode::ORR => 12,
            OpCode::MOV => 13,
            OpCode::BIC => 14,
            OpCode::MVN => 15,
            _ => unreachable!(),
        };

        // If true, condition codes wil be altered
        // This is implied for CMP, CMN, TEQ and TST
        let set_cond = match opcode {
            OpCode::CMP | OpCode::CMN | OpCode::TEQ | OpCode::TST => true,
            _ => {
                tokens[0].value
                    // Remove cond to avoid any false positive
                    .replace(&cond.to_string().to_lowercase(), "")
                    .ends_with("s")
            }
        } as u32;

        // Parse the registers
        let mut rd = 0u32;
        let mut rn = 0u32;
        let mut idx = 2;

        match opcode {
            OpCode::MOV | OpCode::MVN => {
                rd = get_reg!(tokens, 1);
                expect_token!(tokens, 2, TokenType::Comma);
            }
            OpCode::CMP | OpCode::CMN | OpCode::TEQ | OpCode::TST => {
                // These opcodes use Rn as their first register whereas the
                // others use Rd
                rn = get_reg!(tokens, 1);
                expect_token!(tokens, 2, TokenType::Comma);
            },
            _ => {
                // The opcodes left use Rd (first register) and Rn (second
                // register)
                rd = get_reg!(tokens, 1);
                expect_token!(tokens, 2, TokenType::Comma);

                rn = get_reg!(tokens, 3);
                expect_token!(tokens, 4, TokenType::Comma);
                idx = 4;
            },
        }

        // Parse operand 2 which can either be Rm{,<shift>} or <#expression>
        // where <shift> is either <shiftname> <register> or <shiftname>
        // #expression
        let op2;
        let rm;
        let token = get_token!(tokens, idx + 1, [TokenType::Keyword,
            TokenType::Number]);
        if token.token_type == TokenType::Number {
            // <#expression>
            let imm = InstrParser::parse_imm(token)
                .expect("Invalid immediate!");
            op2 = imm.rotate << 8 | imm.value;
        } else {
            // Rm{,<shift>}
            if tokens.get(idx + 2).is_none() {
                // Rm
                rm = get_reg!(tokens, idx + 1);
                op2 = 0u32 << 4 | rm;
            } else {
                // Rm,<shift>
                rm = get_reg!(tokens, idx + 1);
                expect_token!(tokens, idx + 2, TokenType::Comma);

                let shift_type = get_shift_type!(tokens, idx + 3);
                let shift_value = get_token!(tokens, idx + 4,
                    [TokenType::Keyword, TokenType::Number]);

                // Parse the shift value
                let shift;
                if shift_value.token_type == TokenType::Number {
                    // <shiftname> <expression>
                    let shift_value = InstrParser::parse_imm(shift_value)
                        .expect("Invalid shift value!").value;
                    shift = (shift_value << 2 | shift_type) << 1 | 0;
                } else {
                     // <shiftname> <register>
                    let shift_value = get_reg!(tokens, idx + 4);
                    shift = ((shift_value << 1 | 0) << 2 | shift_type) << 1 | 1;
                }

                op2 = shift << 4 | rm;
            }
        }

        // Convert boolean to u32
        let cond = cond as u32;

        return ((((((cond << 2 | 0b00) << 1 | (is_imm as u32)) << 4 | opcode_n)
            << 1 | set_cond) << 4 | rn) << 4 | rd) << 12 | op2;
    }
}
