use std::time::Duration;

use rusb::Device;
use rusb::DeviceHandle;
use rusb::GlobalContext;
use rusb::Result;
use rusb::UsbContext;

pub struct UsbReader;

impl UsbReader {
    //
    pub fn open_device(
        vid: u16,
        pid: u16,
    ) -> Option<(Device<GlobalContext>, DeviceHandle<GlobalContext>)> {
        let device_ctx = match rusb::open_device_with_vid_pid(vid, pid) {
            Some(info) => info,
            None => panic!("could not get device info from vid and pid"),
        };
        let device = device_ctx.device();
        match device.open() {
            Ok(handle) => Some((device, handle)),
            Err(why) => {
                println!("{why}");
                None
            }
        }
    }

    /// get_device_info - retrieves information from the usb device we are targeting
    /// (the microscope)
    // NOTE: this should return a result of a struct for all the data we need.
    #[allow(clippy::or_fun_call)]
    pub fn get_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
        handle.reset()?;

        let timeout = Duration::from_secs(1);
        let langs = handle.read_languages(timeout)?;
        let device_desc = handle.device().device_descriptor()?;

        println!("config: {}", handle.active_configuration()?);

        // check if something is in languages
        if !langs.is_empty() {
            let language = langs[0];

            let mfg_info = handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Manufacturer Not Found".to_string());

            let product_info = handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Product Info Not Found".to_string());

            let serial_num = handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Serial Num Not Found".to_string());

            println!("language: {language:?}\n mfg info: {mfg_info}\n product info: {product_info}\n serial num: {serial_num}");
        }

        Ok(())
    }
}
