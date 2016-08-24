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
    mac: Rc<RefCell<Mac>>
}

pub struct Mcps {
    mac: Rc<RefCell<Mac>>
}

impl Mlme {
    pub fn new(mac: Rc<RefCell<Mac>>) -> Mlme {
        Mlme {
            mac: mac
        }
    }
    pub fn set(&mut self, set_req: PibAttribute) -> MlmeConfirm<PibAttribute> {
        self.mac.borrow_mut().pib.set(set_req)
    }
    pub fn get(&self, get_req: PibAttribute) -> MlmeConfirm<PibAttribute> {
        self.mac.borrow().pib.get(get_req)
    }
    pub fn reset(&mut self, reset_req: ResetReq) -> MlmeConfirm<ResetCnf> {
        self.mac.borrow_mut().pib.reset(reset_req)
    }
    //    pub fn scan(&self, scan_req: ScanReq) -> MlmeConfirm<ScanCnf> {}
}

impl Mcps {
    pub fn new(mac: Rc<RefCell<Mac>>) -> Mcps {
        Mcps {
            mac: mac,
        }
    }
}


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
