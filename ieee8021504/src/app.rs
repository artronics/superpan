use std::sync::mpsc::Receiver;

use IEEE8021504;
use device::DevInterrupt;
use device::DevInterrupt::{ScanRes};
use pib::PibAttribute;
use mac::{Mlme, Status, ResetReq};

pub struct App {
    pub ieee8021504: IEEE8021504,
    pub interrupt_ch: Receiver<DevInterrupt>,
}

impl App {
    pub fn start(&mut self) {
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
    fn init(&mut self) {
        //first reset mlme
        self.ieee8021504.mac.mlme_reset(ResetReq { set_default_pib: true });
    }
}

