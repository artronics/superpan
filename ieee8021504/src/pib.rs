use super::MlmeConfirm;

pub struct Pib {
    mac_extended_address: Option<u64>,
    //    mac_associated_pan_coord: PIBAttribute,
}

pub enum PibAttribute {
    MacExtendedAddress(Option<u64>),
    //    MacAssociatedPANCoord,
}

impl Pib {
    pub fn new_with_defaults() -> Pib {
        Pib {
            mac_extended_address: None,
            //            mac_associated_pan_coord: false,
        }
    }

    pub fn set(&mut self, attr: PibAttribute) -> MlmeConfirm<PibAttribute> {
        match attr {
            PibAttribute::MacExtendedAddress(x) => {
                self.mac_extended_address = x;
                Ok(attr)
            }
        }
    }
}


