use std::sync::mpsc::Receiver;

use IEEE8021504;
use device::DevInterrupt;
use device::DevInterrupt::{ScanRes};
use pib::PibAttribute;
use super::{Status, ResetReq};

pub struct App {
    pub ieee8021504: IEEE8021504,
    pub interrupt_ch: Receiver<DevInterrupt>,
}

impl App {
    pub fn start(&self) {
        self.init();
        loop {
            let i = self.interrupt_ch.recv().unwrap();
            match i {
                ScanRes => {
                    println!("yohoooo!!")
                }
            }
        }
    }
    fn init(&self) {
        //first reset mlme
        self.ieee8021504.mlme.reset_request(ResetReq { set_default_pib: true });
    }
}

