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
    pub device: Rc<Device>,
}
