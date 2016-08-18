use std::result;

const SFD: u8 = 0xA7;

// type Result<'a> = result::Result<Frame, MalformedFrame<'a>>;

#[derive(Debug)]
pub struct MalformedFrame<'a> {
    msg: &'a str,
}
pub struct Frame {
    content: Vec<u8>,
    frame_ctrl: u16,
}

impl Frame {
    pub fn new<'a>(content: Vec<u8>) -> result::Result<Frame, MalformedFrame<'a>> {
        let vl = validate_frame(&content);
        match vl {
            Some(malformed_frame) => Err(malformed_frame),
            None => {
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
}

fn validate_frame<'a>(content: &Vec<u8>) -> Option<MalformedFrame<'a>> {
    // Minimum length for a frame belongs to ack frame with 11 bytes
    if content.len() < 11 {
        return Some(MalformedFrame { msg: "a Frame must have at least 11 bytes." });
    }
    // index 5 is frame length,
    // The Frame Length field specifies the total number of octets contained in the PSDU (i.e., PHY payload).
    match content[5] {
        l @ 0...4 | l @ 6...8 => {
            let a: &String = &format!("PHR error, the length {} is reserved.", l);
            // let s: &'a str = &a[..];
            let s: &'a str = &format!("PHR error, the length {} is reserved.", l);
            // let s: &'a str = format!("PHR error, the length {} is reserved.", l);
            // let msg: &'a str = &s[..];
            // let s: &'a str = "heel";
            Some(MalformedFrame { msg: s })
        }

        _ => None,

    }
}

impl FrameCtrl for Frame {
    fn frame_ctrl(&self) -> u16 {
        self.frame_ctrl
    }
}

pub trait FrameCtrl {
    fn frame_ctrl(&self) -> u16;
    fn is_ack_requested(&self) -> bool {
        eval_bool_inx(self.frame_ctrl(), 5)
    }
    fn is_frame_pending(&self) -> bool {
        unimplemented!();
    }
}
fn eval_bool_inx(value: u16, inx: isize) -> bool {
    if value >> inx & 1 == 1 { true } else { false }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_frame() {
        // a correct ack frame
        let mut b = vec![0, 0, 0, 0, super::SFD, 5, 0xea, 0xf2, 123, 0xff, 0xff];
        let mut f = Frame::new(b).unwrap(); //this should not throw exp
        //use 3 as PHR which is reserved
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
    fn test_frame_ctrl_is_ack_requested() {
        let mut f = FrameCtrlImpl { v: 0x0020 };
        assert!(f.is_ack_requested() == true);
        f = FrameCtrlImpl { v: !0x0020 };
        assert!(f.is_ack_requested() == false);
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
