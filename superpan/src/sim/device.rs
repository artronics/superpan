use super::Signal;


// Simulator intracts with this trait
pub trait Device {
    /// Before calling init device is off. calling init sould not be considered as
    /// first tick.
    fn init(&mut self);

    /// each tick is a symbol time in which, a device can transmit
    /// its data to the simulator
    fn tick(&mut self) -> Option<Signal>;

    /// simulator calls this method to send a Signal to Device
    /// during one tick, simulator might call this method zero or many times
    /// a device should keep all the receiving signals however it should consume
    /// one of them. In other word if a device gets more than one signal during each
    /// tick, it means more than one device tried to send a Signal hence interference
    fn receive(&mut self, Signal);
}

// PHY layer in stack (ieee8021502) intracts with this trait
pub trait PhyDevice {
    fn current_channel() -> u32;
}

/// `TranceiverState` defines the behavior of transeiver during each simulator tick.
/// `Transmitting` state has the highest priority. A device always transits to
/// `Transmitting` state as soon as it finds `tx` not empty and consecuently the device will
/// ignore all receiving data.
pub enum TransceiverState {
    Off,
    Receiving,
    Transmitting,
}

pub enum ChannelState {
    Busy,
    Free,
}

pub struct PanDevice {
    pub sim_tick: u64,
    pub tx: Vec<Signal>,
    pub rx: Vec<Signal>,

    trans_state: TransceiverState,
}

impl PanDevice {
    fn new() -> PanDevice {
        let tx = Vec::with_capacity(10);
        let rx = Vec::with_capacity(10);
        PanDevice {
            sim_tick: 0,
            tx: tx,
            rx: rx,
            trans_state: TransceiverState::Off,
        }
    }
}

impl Device for PanDevice {
    fn init(&mut self) {
        self.trans_state = TransceiverState::Off;
    }
    fn tick(&mut self) -> Option<Signal> {
        match self.trans_state {
            TransceiverState::Off => None,
            TransceiverState::Transmitting => {
                // if tx is empty we should do a state transition
                self.tx.pop().map(|x| {
                    if self.tx.is_empty() {
                        self.trans_state = TransceiverState::Receiving;
                    }
                    x
                })
            }
            //            TransceiverState::Receiving => {
            //                if self.rx.len() == 1 {
            //                    //TODO write the receive logic
            //                    unimplemented!();
            //                    self.rx.clear();
            //                    None
            //                } else if self.rx.len() > 1 {
            //                    panic!("Interference is not allowed!");
            //                }
            //                None
            //            },
            _ => None,
        }
    }

    fn receive(&mut self, sig: Signal) {
        self.rx.push(sig);
    }
}

/// `demodulator` resolves a list of received signals to one optional `Signal`.
/// This is not OQPSK or any other low level demodulator, however all received
/// signals during **one** tick must be passed to this function to resolve interference
/// or be filtered by `current_channel` value
fn demodulator(rx: &Vec<Signal>) -> Option<Signal> {
    let s: Option<Signal>;
    if rx.len() == 1 {
        s = Some(rx[0]);
        return s;
    }
    None
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use super::super::Signal;

    #[test]
    fn test_pass_sim() {
        let mut d = PanDevice::new();
        d.trans_state = TransceiverState::Transmitting;
        d.tx.push(Signal(10));
        let t = d.tick();
        assert!(t == Some(Signal(10)));
        assert!(d.trans_state as usize == TransceiverState::Receiving as usize);
    }
}
