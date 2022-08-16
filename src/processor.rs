/// Processor - does most of the transforming.
/// THis will be mostly reading in [u8] streams and outputing to somewhere else,
/// probably to rtsp.
pub struct Processor;

impl Processor {
    // read the byte stream from the camera, convert to pixel data in this impl.
    pub fn read_byte_stream() -> Vec<u8> {
        println!("not implemented");
        vec![1, 2, 3]
    }

    fn transform_byte_stream(byte_stream: Vec<u8>) {
        println!("not implemented");
    }
}
