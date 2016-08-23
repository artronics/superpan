#![allow(dead_code)]

use std::rc::Rc;
use std::result;
use device::Device;
use pib::{PibAttribute, Pib};

pub mod app;
pub mod mac;
mod phy;
mod pib;
pub mod device;

pub struct IEEE8021504 {
    pub mlme: Mlme,
}

impl IEEE8021504 {
    pub fn new(device: Rc<Device>) -> IEEE8021504 {
        let mlme = Mlme::new(device);
        IEEE8021504 {
            mlme: mlme,
        }
    }
}

pub type MlmeConfirm<T> = result::Result<T, Status>;

pub enum Status {
    Success,
    //    LIMIT_REACHED,
    //    NO_BEACON,
    //    SCAN_IN_PROGRESS,
    //    COUNTER_ERROR,
    //    FRAME_TOO_LONG,
    //    UNAVAILABLE_KEY,
    //    UNSUPPORTED_SECURITY,
    //    INVALID_PARAMETER
}

pub struct Mlme {
    device: Rc<Device>,
    pib: Pib,
}

impl Mlme {
    fn new(device: Rc<Device>) -> Mlme {
        Mlme {
            device: device,
            pib: Pib::new_with_defaults(),
        }
    }
    pub fn set_req(&self, set_req: PibAttribute) -> MlmeConfirm<PibAttribute> {
        Ok(set_req)
    }

    pub fn reset_request(&self, reset_req: ResetReq) -> ResetCnf {
        ResetCnf { status: Status::Success, }
    }
}

pub struct ResetReq {
    pub set_default_pib: bool,
}

pub struct ResetCnf {
    status: Status,
}

pub enum ScanType {
    Ed,
    Active,
    Passive,
    Orphan,
}

pub struct ScanReq {
    scan_type: ScanType,
    channels: Vec<usize>,
    duration: usize,
    channel_page: usize,
}

pub struct ScanRes {
    status: Status,
    scan_type: ScanType,
    channel_page: usize,
    energy_detected_list: Vec<u8>,
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    #[test]
    fn test_pass() {}
}
