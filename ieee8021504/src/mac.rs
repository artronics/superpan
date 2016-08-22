struct PIB {
    macExtendedAddress: u64,
    macAssociatedPANCoord: bool,
}

impl PIB {
    pub fn new_with_default(ext_address: u64) -> PIB {
        PIB {
            macExtendedAddress: ext_address,
            macAssociatedPANCoord: false,
        }
    }
}

pub mod MLME {}

/// Mac constants
pub mod MAC {
    pub const aBaseSlotDuration: usize = 60;
}