use std::result::Result;

const SHR: u8 = 0xA7;
const aMaxPHYPacketSize: u8 = 127;

pub struct Tick(u64);
pub struct Signal {
    data: u8,
    tick: Tick,
}
pub struct Packet {
    bytes: Vec<u8>,
}
pub struct MalformedPacket {
    bytes: Vec<u8>,
}
pub struct Device {
    rx_packet: Packet,
}

enum PacketState {
    Preamble,
    SHR,
    PHR,
    PSDU,
}
pub struct PacketGenerator {
    rx_buff: Vec<u8>,
    cur_pck_state: PacketState,
}
impl PacketGenerator {
    fn new() -> PacketGenerator {
        PacketGenerator {
            rx_buff: Vec::with_capacity(32),
            cur_pck_state: PacketState::Preamble,
        }
    }
    pub fn reset(&mut self) {
        self.rx_buff.clear();
        self.cur_pck_state = PacketState::Preamble;
    }
    pub fn gen_packet(&mut self, data: u8) -> Result<Option<Packet>, MalformedPacket> {
        self.rx_buff.push(data);
        match self.cur_pck_state {
            PacketState::Preamble => {
                if data == 0 {
                    // a preamble is always zero
                    match self.rx_buff.len() {
                        0...3 => Ok(None),
                        4 => {
                            self.cur_pck_state = PacketState::SHR;
                            Ok(None)
                        }
                        _ => Ok(None), //Never happen
                    }

                } else {
                    self.send_err()
                }
            }
            PacketState::SHR => {
                if data == SHR {
                    self.cur_pck_state = PacketState::PHR;
                    Ok(None)
                } else {
                    self.send_err()
                }
            }
            PacketState::PHR => {
                match data {
                    0...4 | 6...8 => self.send_err(),//reserved size
                    x if x > aMaxPHYPacketSize => self.send_err(),
                    _ => {
                        self.cur_pck_state = PacketState::PSDU;
                        Ok(None)
                    }
                }
            }
            PacketState::PSDU => {
                // PSDU has a size of PHR (index->5) which means we should substract 6 elements from
                // rx_buff
                if self.rx_buff.len() - 6 == self.rx_buff[5] as usize {
                    let bytes = self.rx_buff.clone();
                    self.reset();
                    Ok(Some(Packet { bytes: bytes })) //Here we have a good frame
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn send_err(&mut self) -> Result<Option<Packet>, MalformedPacket> {
        let rx = self.rx_buff.clone();
        self.reset();
        Err(MalformedPacket { bytes: rx })
    }
}



#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use super::SHR;

    #[test]
    fn test_preamble() {
        let mut gen = PacketGenerator::new();
        assert_good_seq(&mut gen);
        // gen should reset itself after it's done with prev packet
        assert_good_seq(&mut gen);
        // after each err, gen should reset itself so be ready for next good seq
        gen.gen_packet(12);//some errors
        gen.gen_packet(43);
        assert_good_seq(&mut gen);
    }

    fn assert_good_seq(gen: &mut PacketGenerator) {
        let mut pck = vec![0, 0, 0, 0, SHR, 5, 0, 0, 123, 0]; //this packet is not complete. it needs one last ele
        for x in pck {
            let p = gen.gen_packet(x.clone());
            assert!(p.is_ok()); //there is no err in pck
            p.map(|r| assert!(r.is_none())); //pck is not complete yet
        }
        // Now complete the packet
        let p = gen.gen_packet(0);
        assert!(p.is_ok()); //there is no err in pck
        p.map(|r| assert!(r.is_some())); //there must be a packet as output

    }
}
