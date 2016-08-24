#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use device::Device;
use mac::{Mac, Mlme, Mcps};

pub mod app;
pub mod mac;
mod phy;
mod pib;
mod packet;
pub mod device;

pub struct IEEE8021504 {
    pub mac: Mac,
}

impl IEEE8021504 {
    pub fn new(device: Rc<Device>) -> IEEE8021504 {
        IEEE8021504 {
            mac: Mac::new(device),
        }
    }
}


#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    #[test]
    fn test_pass() {}
}
