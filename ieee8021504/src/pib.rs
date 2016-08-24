use mac::{ResetReq, ResetCnf};
use mac::MlmeConfirm;

#[derive(Copy, Clone)]
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
                Ok(PibAttribute::MacExtendedAddress(self.mac_extended_address))
            }
        }
    }
    pub fn get(&self, attr: PibAttribute) -> MlmeConfirm<PibAttribute> {
        match attr {
            PibAttribute::MacExtendedAddress(_) => {
                Ok(PibAttribute::MacExtendedAddress(self.mac_extended_address))
            }
        }
    }
    //TODO find a way to call all attribute set
    //it is not possible to simply change pib with new one. for all attributes
    //we need to call set. should i call them one by one?!
    pub fn reset(&self, reset_req: ResetReq) -> MlmeConfirm<ResetCnf> {
        unimplemented!();
        Ok(ResetCnf {})
    }
}


