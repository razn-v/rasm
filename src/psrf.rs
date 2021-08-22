use std::str::FromStr;

/// PSR formats
pub enum PSRF {
    CPSR,
    SPSR,
}

impl FromStr for PSRF {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpsr_flg" => Ok(Self::CPSR),
            "spsr_flg" => Ok(Self::SPSR),
            _          => Err(()),
        }
    }
}
