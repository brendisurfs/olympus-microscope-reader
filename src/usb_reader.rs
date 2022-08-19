use std::time::Duration;

use rusb::Device;
use rusb::DeviceHandle;
use rusb::Result;
use rusb::UsbContext;

pub struct UsbReader;

impl UsbReader {
    //
    pub fn open_device<T: UsbContext>(
        ctx: &mut T,
        vid: u16,
        pid: u16,
    ) -> Option<(Device<T>, DeviceHandle<T>)> {
        let devices = match ctx.devices() {
            Ok(d) => d,
            Err(_) => return None,
        };

        for device in devices.iter() {
            let desc = match device.device_descriptor() {
                Ok(descriptor_value) => descriptor_value,
                Err(_) => continue,
            };

            // match up the device with the passed in pid and vendor id
            if desc.vendor_id() == vid && desc.product_id() == pid {
                match device.open() {
                    Ok(handle) => Some((device, handle)),
                    Err(_) => continue,
                };
            };
        }
        None
    }

    /// get_device_info - retrieves information from the usb device we are targeting
    /// (the microscope)
    // NOTE: this should return a result of a struct for all the data we need.
    pub fn get_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
        let device_desc = handle.device().device_descriptor()?;
        let timeout = Duration::from_secs(1);
        let langs = handle.read_languages(timeout)?;

        println!("config: {}", handle.active_configuration()?);

        // check if something is in languages
        if !langs.is_empty() {
            let language = langs[0];
            println!("Language: {language:?}");

            let mfg_info = handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Manufacturer Not Found".to_string());
            println!("{}", mfg_info);

            let product_info = handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Product Info Not Found".to_string());

            let serial_num = handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Serial Num Not Found".to_string());
        }

        Ok(())
    }
}
