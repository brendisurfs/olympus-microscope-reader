mod processor;
mod usb_reader;

use processor::Processor;
use usb_reader::UsbReader;
fn main() {
    let new_reader = UsbReader::new("Olympus");
    let serial_num = new_reader.get_serial_num();

    println!("Hello, world!");
}
