mod processor;
mod usb_reader;

use usb_reader::UsbReader;

use rusb::Context;
use rusb::Result;

const REQUEST: u8 = 0x0A;
const VID: u16 = 0x046d;
const PID: u16 = 0xc539;
fn main() -> Result<()> {
    let mut ctx = Context::new().unwrap();
    let (mut device, mut handle) = UsbReader::open_device(&mut ctx, VID, PID)
        .expect("could not open usb device, will not run");

    println!("{:?}", handle);
    Ok(())
}
