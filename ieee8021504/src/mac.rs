use std::rc::Rc;
use std::cell::RefCell;
use std::result;

use device::Device;
use pib::{Pib, PibAttribute};

pub struct Mac {
    device: Rc<Device>,
    pib: Pib,
}

impl Mac {
    pub fn new(device: Rc<Device>) -> Mac {
        Mac {
            device: device,
            pib: Pib::new_with_defaults(),
        }
    }
}

impl Mlme for Mac {
    fn mlme_get(&self, get_req: PibAttribute) -> MlmeConfirm<PibAttribute> {
        self.pib.get(get_req)
    }
    fn mlme_set(&mut self, set_req: PibAttribute) -> MlmeConfirm<PibAttribute> {
        self.pib.set(set_req)
    }
    fn mlme_reset(&mut self, reset_req: ResetReq) -> MlmeConfirm<ResetCnf> {
        unimplemented!()
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

pub trait Mlme {
    fn mlme_get(&self, get_req: PibAttribute) -> MlmeConfirm<PibAttribute>;
    fn mlme_set(&mut self, set_req: PibAttribute) -> MlmeConfirm<PibAttribute>;
    fn mlme_reset(&mut self, reset_req: ResetReq) -> MlmeConfirm<ResetCnf>;
}

pub trait Mcps {}


pub struct ResetReq {
    pub set_default_pib: bool,
}

pub struct ResetCnf {}

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

pub struct ScanCnf {
    status: Status,
    scan_type: ScanType,
    channel_page: usize,
    energy_detected_list: Vec<u8>,
}

/// Mac constants
pub const A_BASE_SLOT_DURATION: usize = 60;
