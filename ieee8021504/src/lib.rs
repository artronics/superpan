#![allow(dead_code)]

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::rc::Rc;

use device::Device;

pub mod app;
pub mod mac;
mod phy;
pub mod device;

pub struct IEEE8021504 {
    device: Rc<Device>,
}

impl IEEE8021504 {
    pub fn new(device: Rc<Device>) -> IEEE8021504 {
        IEEE8021504 {
            device: device,
        }
    }
}

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

pub trait Requester {
    fn request(&self) -> ResetCnf;
}

pub trait Confirmer {}

pub struct MLME {
    device: Rc<Device>,
}

impl MLME {
    pub fn request<R>(&self, request: R) -> ResetCnf
        where R: Requester
    {
        request.request()
    }
}

pub struct ResetReq {
    set_default_pib: bool,
}

impl Requester for ResetReq {
    fn request(&self) -> ResetCnf {
        println!("receive reset req");
        ResetCnf { status: Status::SUCCESS }
    }
}

pub struct ResetCnf {
    status: Status,
}

impl Confirmer for ResetCnf {
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
