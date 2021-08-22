use std::str::FromStr;

use crate::opcode::OpCode;
use crate::cond::Cond;
use crate::token::{Token, TokenType};
use crate::register::{Register, CoRegister};
use crate::immediate::Immediate;
use crate::shift::Shift;
use crate::psrf::PSRF;
use crate::cpn::CPN;

/// Check if the token at index `idx` is of type `type`
#[macro_export]
macro_rules! expect_token {
    ($tokens:ident, $idx:expr, $type:path) => {
        let token = $tokens.get($idx)
            .expect(&format!("Expected a {:?}.", $type));
        if token.token_type != $type {
            panic!("Expected a {:?}, got a {:?}", $type, token.token_type);
        }
    }
}

/// Return the token at index `idx` if its type is one of `types`, otherwise
/// panic
#[macro_export]
macro_rules! get_token {
    ($tokens:ident, $idx:expr, $types:expr) => {
        match $tokens.get($idx) {
            Some(token) => {
                if $types.iter().any(|ty| ty == &token.token_type) {
                    token
                } else {
                    panic!("Expected one of {:?}, got a {:?}", $types, token.token_type);
                }
            },
            None => panic!("Expected one of {:?}.", $types),
        }
    }
}

/// Parse a number at the specificed index
#[macro_export]
macro_rules! get_number {
    ($tokens:ident, $idx:expr) => {
        match $tokens.get($idx) {
            Some(x) if x.token_type == TokenType::Number => {
                InstrParser::parse_imm(x).expect("Invalid immediate!")
            },
            _ => panic!("Expected an immediate."),
        }
    }
}

/// Parse a register at the specified index
#[macro_export]
macro_rules! get_reg {
    ($tokens:ident, $idx:expr) => {
        InstrParser::parse_reg($tokens.get($idx)
            .expect("Expected a register."))
            .expect("Invalid register!") as u32;
    }
}

/// Parse a shift type at the specified index
#[macro_export]
macro_rules! get_shift_type {
    ($tokens:ident, $idx:expr) => {
        InstrParser::parse_shift($tokens.get($idx)
            .expect("Expected a shift type."))
            .expect("Invalid shift type!") as u32;
    }
}

/// Parse a coprocessor number at the specified index
#[macro_export]
macro_rules! get_cpn {
    ($tokens:ident, $idx:expr) => {
        InstrParser::parse_cpn($tokens.get($idx)
            .expect("Expected a coprocessor number."))
            .expect("Invalid coprocessor number!") as u32;
    }
}

/// Parse a coprocessor register at the specified index
#[macro_export]
macro_rules! get_creg {
    ($tokens:ident, $idx:expr) => {
        InstrParser::parse_creg($tokens.get($idx)
            .expect("Expected a coprocessor register."))
            .expect("Invalid coprocessor register!") as u32;
    }
}

/// Return a label in `labels` with the name of the token at `idx`
#[macro_export]
macro_rules! get_label {
    ($tokens:ident, $idx:expr, $labels:ident) => {
        match $tokens.get($idx) {
            Some(token) => {
                if token.token_type != TokenType::Keyword {
                    panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                        token.token_type);
                } else {
                    $labels.iter().find(|label| {
                        label.name == token.value
                    }).expect(&format!("No label with name {} found.",
                        token.value))
                }
            },
            None => panic!("Expected a label."),
        }
    }
}

pub struct InstrParser;

impl InstrParser {
    /// Parse an instruction opcode
    pub fn parse_opcode(token: &Token) -> Option<OpCode> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        // Read the token char by char until we found a valid opcode
        for i in 0..token.value.len() {
            if i + 2 <= token.value.len() {
                // Prevent cases where the opcode is parsed too early
                // Example: Opcode BL being parsed as B
                if let Ok(opc) = OpCode::from_str(&token.value[0..i + 2]) {
                    // Opcode found
                    return Some(opc);
                }
            }

            if let Ok(opc) = OpCode::from_str(&token.value[0..i + 1]) {
                // Opcode found
                return Some(opc);
            }
        }

        None
    }

    /// Parse a condition code
    pub fn parse_cond(opcode: OpCode, token: &Token) -> Cond {
        // Remove opcode
        let string = token.value.replace(
            &opcode.to_string().to_lowercase(), "");

        // Read the string char by char until we find a valid two-character
        // condition mnemonic
        for i in 0..string.len() {
            if let Ok(cond) = Cond::from_str(&string[0..i + 1]) {
                // Cond found
                return cond;
            }
        }

        // Return AL (Always) if no cond was found
        Cond::AL
    }

    /// Parse a register
    pub fn parse_reg(token: &Token) -> Option<Register> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        match Register::from_str(&token.value) {
            Ok(reg) => Some(reg),
            Err(_) => None,
        }
    }

    /// Parse an immediate
    pub fn parse_imm(token: &Token) -> Option<Immediate> {
        let s = token.value.replace("#", "");

        let imm: Option<u32> = match s.contains("0x") {
            // The immediate is in hex format
            true => u32::from_str_radix(&s.replace("0x", ""), 16).ok(),
            false => s.parse::<u32>().ok(),
        };

        if imm.is_some() {
            let imm: u32 = imm.unwrap();

            // Encode the immediate
            let mut encoded = 0u32;
            for i in 0..16 {
                let m = imm.rotate_left(i * 2);
                if m < 256 {
                    encoded = (i << 8) | m;
                    break;
                }
            }

            return Some(Immediate {
                value: encoded & 0xff,
                rotate: (encoded >> 8) & 0xf
            });
        }

        None
    }

    /// Parse a shift type
    pub fn parse_shift(token: &Token) -> Option<Shift> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        match Shift::from_str(&token.value) {
            Ok(reg) => Some(reg),
            Err(_) => None,
        }
    }

    /// Parse a PSR format
    pub fn parse_psrf(token: &Token) -> Option<PSRF> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        match PSRF::from_str(&token.value) {
            Ok(flag) => Some(flag),
            Err(_) => None,
        }
    }

    /// Parse a coprocessor number
    pub fn parse_cpn(token: &Token) -> Option<CPN> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        match CPN::from_str(&token.value) {
            Ok(cpn) => Some(cpn),
            Err(_) => None,
        }
    }

    /// Parse a coprocessor register
    pub fn parse_creg(token: &Token) -> Option<CoRegister> {
        if token.token_type != TokenType::Keyword {
            panic!("Expected a {:?}, got a {:?}", TokenType::Keyword,
                token.token_type);
        }

        match CoRegister::from_str(&token.value) {
            Ok(creg) => Some(creg),
            Err(_) => None,
        }
    }
}
