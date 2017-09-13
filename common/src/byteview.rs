use std::io;
use std::path::Path;
use std::borrow::Cow;
use std::ops::Deref;

use memmap::{Mmap, Protection};
use owning_ref::OwningHandle;

use errors::Result;

/// Gives access to bytes loaded from somewhere.
///
/// This type can be used to uniformly access bytes that were created
/// either from mmapping in a path, a vector or a borrowed slice.
pub enum ByteView<'a> {
    Buf(Cow<'a, [u8]>),
    Mmap(Mmap),
}

impl<'a> ByteView<'a> {
    /// Constructs a byte view from a Cow.
    pub fn from_cow(cow: Cow<'a, [u8]>) -> ByteView<'a> {
        ByteView::Buf(cow)
    }

    /// Constructs an object file from a byte slice.
    pub fn from_slice(buffer: &'a [u8]) -> ByteView<'a> {
        ByteView::from_cow(Cow::Borrowed(buffer))
    }

    /// Constructs an object file from a vector.
    pub fn from_vec(buffer: Vec<u8>) -> ByteView<'static> {
        ByteView::from_cow(Cow::Owned(buffer))
    }

    /// Constructs an object file from a file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ByteView<'static>> {
        let mmap = Mmap::open_path(path, Protection::Read)?;
        Ok(ByteView::Mmap(mmap))
    }

    /// A safe way to get a subslice from the byteview.
    pub fn get_data(&self, start: usize, len: usize) -> Result<&[u8]> {
        let buffer = self.buffer();
        let end = start.wrapping_add(len);
        if end < start || end > buffer.len() {
            Err(
                io::Error::new(io::ErrorKind::UnexpectedEof, "out of range").into(),
            )
        } else {
            Ok(&buffer[start..end])
        }
    }

    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        match *self {
            ByteView::Buf(ref buf) => buf,
            ByteView::Mmap(ref mmap) => unsafe { mmap.as_slice() },
        }
    }
}

impl<'a> Deref for ByteView<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.buffer()
    }
}

impl<'a> AsRef<[u8]> for ByteView<'a> {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
    }
}

pub struct ByteViewBacking<'a, T> {
    inner: OwningHandle<Box<ByteView<'a>>, Box<(&'a [u8], T)>>,
}

impl<'a, T> ByteViewBacking<'a, T> {
    pub fn new<F: FnOnce(&'a [u8]) -> Result<T>>(
        byteview: ByteView<'a>, f: F) -> Result<ByteViewBacking<'a, T>>
    {
        Ok(ByteViewBacking {
            inner: OwningHandle::try_new(Box::new(byteview), |bv| -> Result<_> {
                let bytes: &[u8] = unsafe { &*bv };
                Ok(Box::new((bytes, f(bytes)?)))
            })?
        })
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.inner.0
    }

    pub fn get_value(&self) -> &T {
        &self.inner.1
    }
}
