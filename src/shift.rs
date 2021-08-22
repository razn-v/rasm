use std::str::FromStr;

#[derive(Debug)]
/// List of shifts applied to immediates and registers
pub enum Shift {
    /// Logical left
    ASL,
    /// Logical right
    LSR,
    /// Arithmetic right
    ASR,
    /// Rotate right
    ROR,
}

impl FromStr for Shift {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // LSL is a synonym for ASL
            "asl" | "lsl" => Ok(Self::ASL),
            "lsr"         => Ok(Self::LSR),
            "asr"         => Ok(Self::ASR),
            "ror"         => Ok(Self::ROR),
            _             => Err(()),
        }
    }
}
