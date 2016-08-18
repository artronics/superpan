use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};


use phy::frame::{Frame};

pub trait Stack {
    fn start() -> ();
}

pub struct IEEE8021504 {
    tx_frame_ch: Sender<Frame>,
    rx_frame_ch: Receiver<Frame>,
}

impl IEEE8021504 {
    pub fn new() -> IEEE8021504 {
        let (tx, rx) = channel();
        IEEE8021504 {
            tx_frame_ch: tx,
            rx_frame_ch: rx,
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;

    #[test]
    fn test_pass() {
        assert!(true)
    }
}
