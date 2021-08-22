use std::str::FromStr;
use std::fmt;

/// List of available opcodes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    ADC,
    ADD,
    AND,
    B,
    BIC,
    BL,
    BX,
    CDP,
    CMN,
    CMP,
    EOR,
    LDC,
    LDM,
    LDR,
    MCR,
    MLA,
    MOV,
    MRC,
    MRS,
    MSR,
    MUL,
    MVN,
    ORR,
    RSB,
    RSC,
    SBC,
    STC,
    STM,
    STR,
    SUB,
    SWI,
    SWP,
    TEQ,
    TST,
    // Derivations of MUL
    UMULL,
    UMLAL,
    SMULL,
    SMLAL,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adc"   => Ok(Self::ADC),
            "add"   => Ok(Self::ADD),
            "and"   => Ok(Self::AND),
            "b"     => Ok(Self::B),
            "bic"   => Ok(Self::BIC),
            "bl"    => Ok(Self::BL),
            "bx"    => Ok(Self::BX),
            "cdp"   => Ok(Self::CDP),
            "cmn"   => Ok(Self::CMN),
            "cmp"   => Ok(Self::CMP),
            "eor"   => Ok(Self::EOR),
            "ldc"   => Ok(Self::LDC),
            "ldm"   => Ok(Self::LDM),
            "ldr"   => Ok(Self::LDR),
            "mcr"   => Ok(Self::MCR),
            "mla"   => Ok(Self::MLA),
            "mov"   => Ok(Self::MOV),
            "mrc"   => Ok(Self::MRC),
            "mrs"   => Ok(Self::MRS),
            "msr"   => Ok(Self::MSR),
            "mul"   => Ok(Self::MUL),
            "mvn"   => Ok(Self::MVN),
            "orr"   => Ok(Self::ORR),
            "rsb"   => Ok(Self::RSB),
            "rsc"   => Ok(Self::RSC),
            "sbc"   => Ok(Self::SBC),
            "stc"   => Ok(Self::STC),
            "stm"   => Ok(Self::STM),
            "str"   => Ok(Self::STR),
            "sub"   => Ok(Self::SUB),
            "swi"   => Ok(Self::SWI),
            "swp"   => Ok(Self::SWP),
            "teq"   => Ok(Self::TEQ),
            "tst"   => Ok(Self::TST),
            "umull" => Ok(Self::UMULL),
            "umlal" => Ok(Self::UMLAL),
            "smull" => Ok(Self::SMULL),
            "smlal" => Ok(Self::SMLAL),
            _       => Err(()),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
