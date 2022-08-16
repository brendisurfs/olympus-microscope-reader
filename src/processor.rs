use std::time::Duration;

use rusb::DeviceHandle;
use rusb::RequestType;

/// Processor - does most of the transforming.
/// THis will be mostly reading in [u8] streams and outputing to somewhere else,
/// probably to rtsp.
pub struct Processor;

impl Processor {
    // read the byte stream from the camera, convert to pixel data in this impl.
    pub fn read_byte_stream() -> Vec<u8> {
        let device_list = rusb::devices().expect("could not read devices");

        // FIND THE DEVICE FROM NAME HERE

        // then make a buffer

        let _ = device_list.iter().map(|device| {
            let mut reader_buffer = [0; 512];
            let read_request_type = rusb::request_type(
                rusb::Direction::In,
                RequestType::Standard,
                rusb::Recipient::Endpoint,
            );
            let reader_timeout = Duration::from_secs(1);
            //     // this is where we will read into a buf
            // device.open().expect("fail to open").read_control(
            //     read_request_type,
            //     request,
            //     value,
            //     0,
            //     &mut reader_buffer,
            //     reader_timeout,
            // );
        });
        vec![1, 2, 3]
    }

    fn transform_byte_stream(byte_stream: Vec<u8>) {
        println!("not implemented");
    }
}
