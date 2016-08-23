use std::sync::mpsc::Receiver;
use std::thread;

use IEEE8021504;
use device::DevInterrupt;
use device::DevInterrupt::{ScanRes};

pub struct App {
    pub ieee8021504: IEEE8021504,
    pub interrupt_ch: Receiver<DevInterrupt>,
}

impl App {
    pub fn start(&self) {
        loop {
            let i = self.interrupt_ch.recv().unwrap();
            match i {
                ScanRes => {
                    println!("yohoooo!!")
                }
            }
        }
    }
}