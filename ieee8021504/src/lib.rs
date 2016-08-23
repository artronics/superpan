#![allow(dead_code)]

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

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
    SUCCESS,
    LIMIT_REACHED,
    NO_BEACON,
    SCAN_IN_PROGRESS,
    COUNTER_ERROR,
    FRAME_TOO_LONG,
    UNAVAILABLE_KEY,
    UNSUPPORTED_SECURITY,
    INVALID_PARAMETER
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
    pub fn set_req(&self, setReq: PibAttribute) -> MlmeConfirm<PibAttribute> {
        Ok(setReq)
    }

    pub fn reset_request(&self, resetReq: ResetReq) -> ResetCnf {
        ResetCnf { status: Status::SUCCESS, }
    }
}

//pub struct SetReq<T> {
//    pib_attribute: PIBAttribute,
//    pib_attribute_value: T,
//}
//
//pub struct SetCnf {
//    status: Status,
//    pib_attribute: PIBAttribute,
//}

pub struct ResetReq {
    pub set_default_pib: bool,
}

pub struct ResetCnf {
    status: Status,
}

pub enum ScanType {
    ED,
    ACTIVE,
    PASSIVE,
    ORPHAN,
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
