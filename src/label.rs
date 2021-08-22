use crate::token::Token;

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub line: usize,
}

impl From<&Token> for Label {
    fn from(token: &Token) -> Self {
        Self {
            // Remove the trailing ':' char
            name: token.value[..token.value.len() - 1].to_string(),
            line: token.line,
        }
    }
}

impl Label {
    /// Calculate the offset of `n_bits` bits between the label and an
    /// instruction
    pub fn offset(&self, instr: &Token, n_bits: usize) -> u32 {
        match n_bits {
            24 => {
                // Label before instruction
                if instr.line > self.line {
                    return (self.line - instr.line) as u32 & 0xffffff;
                }
                // Label after instruction
                return (self.line - (instr.line + 2)) as u32 & 0xffffff;
            },
            8 => {
                return ((self.line - instr.line) as i32).abs() as u32 & 0xff;
            },
            12 => {
                // Label before instruction
                if instr.line > self.line {
                    return ((self.line - instr.line * 4) as i32).abs()
                        as u32 & 0xfff;
                }
                // Label after instruction
                return (((self.line - instr.line - 2) * 4) as i32).abs()
                    as u32 & 0xfff;
            },
            _ => unreachable!(),
        };
    }
 }
