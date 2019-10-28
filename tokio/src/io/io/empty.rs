use crate::io::{AsyncBufRead, AsyncRead};

use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

// An async reader which is always at EOF.
///
/// This struct is generally created by calling [`empty`]. Please see
/// the documentation of [`empty()`][`empty`] for more details.
///
/// This is an asynchronous version of [`std::io::empty`][std].
///
/// [`empty`]: fn.empty.html
/// [std]: https://doc.rust-lang.org/std/io/struct.Empty.html
pub struct Empty {
    _p: (),
}

/// Creates a new empty async reader.
///
/// All reads from the returned reader will return `Poll::Ready(Ok(0))`.
///
/// This is an asynchronous version of [`std::io::empty`][std].
///
/// # Examples
///
/// A slightly sad example of not reading anything into a buffer:
///
/// ```rust
/// # use tokio::io::{self, AsyncReadExt};
/// # async fn dox() {
/// let mut buffer = String::new();
/// io::empty().read_to_string(&mut buffer).await.unwrap();
/// assert!(buffer.is_empty());
/// # }
/// ```
///
/// [std]: https://doc.rust-lang.org/std/io/fn.empty.html
pub fn empty() -> Empty {
    Empty { _p: () }
}

impl AsyncRead for Empty {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }
}

impl AsyncBufRead for Empty {
    #[inline]
    fn poll_read_into_buf(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    #[inline]
    fn get_buf(self: Pin<&mut Self>) -> &[u8] {
        &[]
    }

    #[inline]
    fn consume(self: Pin<&mut Self>, _: usize) {}
}

impl fmt::Debug for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty { .. }")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Empty>();
    }
}
