use bytes::BytesMut;
use std::io;

/// Trait of helper objects to write out messages as bytes, for use with
/// `FramedWrite`.
pub trait Encoder<Item>
where
    Item: ?Sized,
{
    /// The type of encoding errors.
    ///
    /// `FramedWrite` requires `Encoder`s errors to implement `From<io::Error>`
    /// in the interest letting it return `Error`s directly.
    type Error: From<io::Error>;

    /// Encodes a frame into the buffer provided.
    ///
    /// This method will encode `item` into the byte buffer provided by `dst`.
    /// The `dst` provided is an internal buffer of the `Framed` instance and
    /// will be written out when possible.
    fn encode(&mut self, item: &Item, dst: &mut BytesMut) -> Result<(), Self::Error>;
}
