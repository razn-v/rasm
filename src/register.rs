use std::str::FromStr;

/// List of available registers
#[derive(PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    CPSR,
    SPSR,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r0"                => Ok(Self::R0),
            "r1"                => Ok(Self::R1),
            "r2"                => Ok(Self::R2),
            "r3"                => Ok(Self::R3),
            "r4"                => Ok(Self::R4),
            "r5"                => Ok(Self::R5),
            "r6"                => Ok(Self::R6),
            "r7"                => Ok(Self::R7),
            "r8"                => Ok(Self::R8),
            "r9"                => Ok(Self::R9),
            "r10"               => Ok(Self::R10),
            "r11" | "fp"        => Ok(Self::R11),
            "r12"               => Ok(Self::R12),
            "r13" | "sp"        => Ok(Self::R13),
            "r14" | "lr"        => Ok(Self::R14),
            "r15" | "pc"        => Ok(Self::R15),
            "cpsr" | "cpsr_all" => Ok(Self::CPSR),
            "spsr" | "spsr_all" => Ok(Self::SPSR),
            _                   => Err(()),
        }
    }
}

/// List of available coprocessor registers
pub enum CoRegister {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
}

impl FromStr for CoRegister {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c0"  => Ok(Self::C0),
            "c1"  => Ok(Self::C1),
            "c2"  => Ok(Self::C2),
            "c3"  => Ok(Self::C3),
            "c4"  => Ok(Self::C4),
            "c5"  => Ok(Self::C5),
            "c6"  => Ok(Self::C6),
            "c7"  => Ok(Self::C7),
            "c8"  => Ok(Self::C8),
            "c9"  => Ok(Self::C9),
            "c10" => Ok(Self::C10),
            "c11" => Ok(Self::C11),
            "c12" => Ok(Self::C12),
            "c13" => Ok(Self::C13),
            "c14" => Ok(Self::C14),
            "c15" => Ok(Self::C15),
            _     => Err(()),
        }
    }
}
