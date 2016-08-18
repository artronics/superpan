extern crate ieee8021504;

mod sim;

use ieee8021504::phy::frame::{Frame, FrameCtrl};

fn main() {
    let f = Frame::new(vec!(0,0,0,0,0x7A,5,6,0,123,100,100)).unwrap();

    println!("Hello, world! {:?}", f.ack_requested());
}
