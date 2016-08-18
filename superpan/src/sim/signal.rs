trait SignalData {
    type T;
    fn data(&self) -> Option<Self::T>;
}

trait SignalTick {
    fn tick(&self) -> Tick;
}

#[derive(Copy, Clone)]
enum Tick {
    Tx,
    Rx,
}

struct ByteTick {
    data: Option<u8>,
    tick: Tick,
}

impl SignalData for ByteTick {
    type T = u8;
    fn data(&self) -> Option<u8> {
        self.data
    }
}

impl SignalTick for ByteTick {
    fn tick(&self) -> Tick {
        self.tick.clone()
    }
}


#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::{ByteTick, Tick};

    #[test]
    fn test_pass_sim() {
        let t = ByteTick { data: Some(8), tick: Tick::Rx };
        assert!(true)
    }
}
