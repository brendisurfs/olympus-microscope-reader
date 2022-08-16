use std::fmt::Error;

use rusb::DeviceDescriptor;

pub struct UsbReader;

impl UsbReader {
    pub fn new() -> Self {
        Self
    }

    pub fn get_serial_num(&self) -> Result<String, String> {
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
