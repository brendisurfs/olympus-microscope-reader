use std::fmt::Error;

use rusb::DeviceDescriptor;

pub struct UsbReader<'a> {
    device_name: &'a str,
}

impl<'a> UsbReader<'a> {
    pub fn new(device_name: &'a str) -> Self {
        Self { device_name }
    }

    pub fn get_serial_num(&self) -> Result<String, String> {
        let device_list = match rusb::devices() {
            Ok(devices) => devices,
            Err(why) => panic!("could not find devices: {why}"),
        };
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
