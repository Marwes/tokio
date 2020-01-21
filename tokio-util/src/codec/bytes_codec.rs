use crate::codec::decoder::Decoder;
use crate::codec::encoder::Encoder;

use bytes::{BufMut, BytesMut};
use std::io;

/// A simple `Codec` implementation that just ships bytes around.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BytesCodec(());

impl BytesCodec {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> BytesCodec {
        BytesCodec(())
    }
}

impl Decoder for BytesCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
        if !buf.is_empty() {
            let len = buf.len();
            Ok(Some(buf.split_to(len)))
        } else {
            Ok(None)
        }
    }
}

impl<I> Encoder<I> for BytesCodec
where
    I: AsRef<[u8]>,
{
    type Error = io::Error;

    fn encode(&mut self, data: I, buf: &mut BytesMut) -> Result<(), io::Error> {
        let data = data.as_ref();
        buf.reserve(data.len());
        buf.put(data);
        Ok(())
    }
}
