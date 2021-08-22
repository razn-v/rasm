use std::str::FromStr;
use std::fmt;

/// Conditional code found in some instructions
#[derive(Debug)]
pub enum Cond {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
}

impl FromStr for Cond {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(Self::EQ),
            "ne" => Ok(Self::NE),
            "cs" => Ok(Self::CS),
            "cc" => Ok(Self::CC),
            "mi" => Ok(Self::MI),
            "pl" => Ok(Self::PL),
            "vs" => Ok(Self::VS),
            "vc" => Ok(Self::VC),
            "hi" => Ok(Self::HI),
            "ls" => Ok(Self::LS),
            "ge" => Ok(Self::GE),
            "lt" => Ok(Self::LT),
            "gt" => Ok(Self::GT),
            "le" => Ok(Self::LE),
            _    => Err(()),
        }
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
