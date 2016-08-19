use super::Signal;

//Simulator intracts with this trait
pub trait Device {
    fn tick(&mut self) -> Option<Signal>;
}

//PHY layer in stack (ieee8021502) intracts with this trait
pub trait PhyDevice {
    fn current_channel() -> u32;
}

pub struct PanDevice {
    pub sim_tick: u64,
    pub tx: Vec<Signal>,
    pub rx: Vec<Signal>,

    rx_is_on: bool,
    tx_is_on: bool,
}

impl Device for PanDevice {
    fn tick(&mut self) -> Option<Signal> {
        if self.tx_is_on && !self.tx.is_empty() {
            Some(self.tx.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use super::super::{Signal};
    use std::collections;

    #[test]
    fn test_pass_sim() {
        let mut d = create_dev();
        assert!(d.tick()==Some(Signal(0)));
        assert!(d.tx.len()==1)
    }

    fn create_dev() -> PanDevice {
        PanDevice {
            sim_tick: 10000,
            tx: vec!(Signal(0),Signal(1)),
            rx: Vec::new(),
            tx_is_on: true,
            rx_is_on: false,
        }
    }
}
