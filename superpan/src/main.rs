extern crate ieee8021504;

use std::rc::Rc;
use std::sync::mpsc::channel;

mod sim;

use ieee8021504::device::Device;
use ieee8021504::IEEE8021504;
use ieee8021504::app::App;

fn main() {
    let (tx, rx) = channel();

    let dev = Rc::new(Device::new(tx));
    let ieee = IEEE8021504::new(dev.clone());
    let mut app = App { ieee8021504: ieee, interrupt_ch: rx };
    dev.as_ref().send();
    app.start();

    println!("Hello, world! ");
}
