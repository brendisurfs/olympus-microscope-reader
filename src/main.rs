mod processor;
mod usb_reader;

use std::env;

use rusb::open_device_with_vid_pid;
use rusb::Device;
use rusb::DeviceHandle;
use rusb::GlobalContext;
use rusb::UsbContext;
use usb_reader::UsbReader;

use rusb::Context;
use rusb::Result;

const VID: u16 = 0x046d;
const PID: u16 = 0xc539;
fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "0");
    let mut ctx = Context::new()?;
    // TODO: this does not read the usb, even though lsusb reads it fine.
    //
    let (mut device, mut handle) =
        UsbReader::open_device(VID, PID).expect("could not open usb device, will not run");

    UsbReader::get_device_info(&mut handle)?;
    Ok(())
}
