use std::result;

const SFD: u8 = 0xA7;


#[derive(Debug)]
pub struct MalformedFrame {
    msg: String,
}

pub struct Frame {
    content: Vec<u8>,
    frame_ctrl: u16,
}

impl Frame {
    pub fn new(content: Vec<u8>) -> result::Result<Frame, MalformedFrame> {
        let vl = validate_frame(&content);
        if let Some(malformed_frame) = vl {
            Err(malformed_frame)
        } else {
            // Frame Control field contains of two octeds at index 6 and 7
            let b6 = content[6] as u16;
            let b7 = content[7] as u16;
            let frame_ctrl: u16 = b7 << 8 | b6;

            Ok(Frame {
                content: content,
                frame_ctrl: frame_ctrl,
            })
        }
    }
}

#[derive(PartialEq)]
pub enum FrameType {
    Beacon,
    Data,
    Ack,
    Command,
    Reserved,
}

impl FrameCtrl for Frame {
    fn frame_ctrl(&self) -> u16 {
        self.frame_ctrl
    }
}

//TODO: Ali : haji implement fuctions whose returns `unimplemented!()`
// see ack_requested, also use eval_bool_inx to get bool value for specific bit index
// for each func write a test. use test_frame_ctrl_is_ack_requested as reference
pub trait FrameCtrl {
    //frame_ctrl returns 16-bit (2 octeds) corrensponding "frame control" field
    //see specification->5.2.1.1
    fn frame_ctrl(&self) -> u16;

    // bit 0,1,2 represents Frame Type.
    fn frame_type(&self) -> FrameType {
        match self.frame_ctrl() & 0x0007 {
            0 => FrameType::Beacon,
            1 => FrameType::Data,
            2 => FrameType::Ack,
            3 => FrameType::Command,
            _ => FrameType::Reserved,
        }
    }
    fn security_enabled(&self) -> bool {
        unimplemented!();
    }
    fn frame_pending(&self) -> bool {
        unimplemented!();
    }
    fn ack_requested(&self) -> bool {
        eval_bool_inx(self.frame_ctrl(), 5)
    }
    fn pan_id_compression(&self) -> bool {
        unimplemented!();
    }
    //    fn dst_address_mode(&self) -> address::AddressMode {
    //        unimplemented!();
    //    }
    //    fn src_address_mode(&self) -> address::AddressMode {
    //        unimplemented!();
    //    }
}

fn eval_bool_inx(value: u16, inx: isize) -> bool {
    if value >> inx & 1 == 1 { true } else { false }
}

fn validate_frame(content: &Vec<u8>) -> Option<MalformedFrame> {
    // Minimum length for a frame belongs to ack frame with 11 bytes
    if content.len() < 11 {
        return Some(MalformedFrame { msg: "a Frame must have at least 11 bytes.".to_string() });
    }
    // index 5 is frame length,
    // The Frame Length field specifies the total number of octets contained in the PSDU (i.e., PHY payload).
    match content[5] {
        l @ 0...4 | l @ 6...8 => {
            let s: String = format!("PHR error, the length {} is reserved.", l);
            Some(MalformedFrame { msg: s })
        }

        _ => None,
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;

    #[test]
    fn test_validate_frame() {
        // a correct ack frame
        let mut b = vec![0, 0, 0, 0, super::SFD, 5, 0xea, 0xf2, 123, 0xff, 0xff];
        // Frame::new(b).unwrap(); //this should not throw exp
        // use 3 as PHR which is reserved
        b = vec![0, 0, 0, 0, super::SFD, 3, 0xea, 0xf2, 123, 0xff, 0xff];
        let msg: &str = "PHR error, the length 3 is reserved.";
        Frame::new(b).map_err(|mf| assert!(mf.msg == msg));
    }

    #[test]
    fn test_frame_it_should_get_frame_ctrl() {
        let b = vec![0, 0, 0, 0, super::SFD, 5, 0xea, 0xf2, 123, 0xff, 0xff];
        let f = Frame::new(b).unwrap();
        assert!(f.frame_ctrl == 0xf2ea);
    }

    #[test]
    fn test_frame_ctrl_frame_type() {
        let mut f = FrameCtrlImpl { v: 0x0000 };
        assert!(f.frame_type() == FrameType::Beacon);
        f = FrameCtrlImpl { v: 0x0002 };
        assert!(f.frame_type() == FrameType::Ack);
    }

    #[test]
    fn test_frame_ctrl_ack_requested() {
        let mut f = FrameCtrlImpl { v: 0x0020 };
        assert!(f.ack_requested() == true);
        f = FrameCtrlImpl { v: !0x0020 };
        assert!(f.ack_requested() == false);
    }

    struct FrameCtrlImpl {
        v: u16,
    }

    impl FrameCtrl for FrameCtrlImpl {
        fn frame_ctrl(&self) -> u16 {
            self.v
        }
    }
}
