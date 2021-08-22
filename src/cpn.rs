use std::str::FromStr;

/// List of available coprocessor numbers
pub enum CPN {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

impl FromStr for CPN {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p0"  => Ok(Self::P0),
            "p1"  => Ok(Self::P1),
            "p2"  => Ok(Self::P2),
            "p3"  => Ok(Self::P3),
            "p4"  => Ok(Self::P4),
            "p5"  => Ok(Self::P5),
            "p6"  => Ok(Self::P6),
            "p7"  => Ok(Self::P7),
            "p8"  => Ok(Self::P8),
            "p9"  => Ok(Self::P9),
            "p10" => Ok(Self::P10),
            "p11" => Ok(Self::P11),
            "p12" => Ok(Self::P12),
            "p13" => Ok(Self::P13),
            "p14" => Ok(Self::P14),
            "p15" => Ok(Self::P15),
            _     => Err(()),
        }
    }
}
