mod processor;
mod usb_reader;

use std::env;

use usb_reader::UsbReader;

use rusb::Context;
use rusb::Result;

const VID: u16 = 0x046d;
const PID: u16 = 0xc539;
fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let mut ctx = Context::new()?;
    // TODO: this does not read the usb, even though lsusb reads it fine.
    let (mut device, mut handle) = UsbReader::open_device(&mut ctx, VID, PID)
        .expect("could not open usb device, will not run");
    println!("{:?}", device);

    println!("{:?}", handle);

    UsbReader::get_device_info(&mut handle)?;
    Ok(())
}
