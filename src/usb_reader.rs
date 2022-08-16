use std::fmt::Error;

use rusb::DeviceDescriptor;
use rusb::DeviceHandle;

pub struct UsbReader<'a> {
    device_name: &'a str,
}

impl<'a> UsbReader<'a> {
    // find_device - finds the specific device from the device list.
    // not implemented yet.
    pub fn find_device(device_name: &'a str) -> Self {
        println!("not implemented yet");
        Self { device_name }
    }

    // get_serial_num - gets the serial number from the device selected.
    pub fn get_serial_num(&self) -> Result<String, String> {
        let device_list = match rusb::devices() {
            Ok(devices) => devices,
            Err(why) => panic!("could not find devices: {why}"),
        };

        let devices_opened: Vec<DeviceHandle<_>> = device_list
            .iter()
            .map(|x| x.open().expect("could not open device"))
            .collect();
        println!("{:?}", devices_opened);
        Ok("not implemented".to_string())
    }

    /// read_camera_name - reads the cameras name, using the device descriptor.
    pub fn read_camera_name(&self, descriptor: DeviceDescriptor) -> Result<String, String> {
        Ok("not implemented".to_string())
    }

    // get the device descriptor from the USB device, to be passed to self.
    pub fn get_device_descriptor(&self) -> Result<(), Error> {
        Ok(())
    }
}
