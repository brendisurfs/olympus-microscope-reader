use std::fmt::Error;
use std::io::BufRead;

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

        let device_serials: Vec<String> = device_list
            .iter()
            .map(|x| x.open().expect("could not open device"))
            .map(|device| {
                let internal_device = device
                    .device()
                    .device_descriptor()
                    .expect("could not read device descriptor");
                match device.read_serial_number_string_ascii(&internal_device) {
                    Ok(serial) => serial,
                    Err(_) => "NO SERIAL".to_string(),
                }
            })
            .collect();

        let device_names: Vec<String> = device_list
            .iter()
            .map(|device| {
                let device_descriptor = device
                    .device_descriptor()
                    .expect("could not get device descriptor")
                    .product_string_index()
                    .expect("could not get device name");

                let device_name = device
                    .open()
                    .unwrap()
                    .read_string_descriptor_ascii(device_descriptor)
                    .expect("could not read string device descriptor");

                device_name.trim().to_string()
            })
            .collect();

        println!("device names: {:?}", device_names);
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
