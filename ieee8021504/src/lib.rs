#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

use device::Device;
use mac::{Mac, Mlme, Mcps};

pub mod app;
pub mod mac;
mod phy;
mod pib;
pub mod device;

pub struct IEEE8021504 {
    pub mlme: Mlme,
    pub mcps: Mcps,
}

impl IEEE8021504 {
    pub fn new(device: Rc<Device>) -> IEEE8021504 {
        let mlme_mac = Rc::new(RefCell::new(Mac::new(device)));
        let mcps_mac = mlme_mac.clone();
        IEEE8021504 {
            mlme: Mlme::new(mlme_mac),
            mcps: Mcps::new(mcps_mac),
        }
    }
}


#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    #[test]
    fn test_pass() {}
}
