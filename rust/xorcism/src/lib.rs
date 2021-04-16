use std::borrow::Borrow;

#[cfg(feature = "io")]
use std::io::{Read, Write};

#[cfg(feature = "io")]
use xorcism_io::{XorcismReader, XorcismWriter};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: std::iter::Cycle<std::slice::Iter<'a, u8>>,
}

#[cfg(feature = "io")]
pub mod xorcism_io {
    use super::Xorcism;
    use std::io::{self, Read, Write};

    pub struct XorcismReader<'a, R> {
        xorcism: Xorcism<'a>,
        reader: R,
    }

    impl<'a, R> XorcismReader<'a, R> {
        pub fn new(xorcism: Xorcism<'a>, reader: R) -> Self {
            Self { xorcism, reader }
        }
    }

    pub struct XorcismWriter<'a, W> {
        xorcism: Xorcism<'a>,
        writer: W,
    }

    impl<'a, W> XorcismWriter<'a, W> {
        pub fn new(xorcism: Xorcism<'a>, writer: W) -> Self {
            Self { xorcism, writer }
        }
    }

    impl<'a, R> Read for XorcismReader<'a, R>
    where
        R: Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.reader.read(buf).map(|n| {
                self.xorcism.munge_in_place(buf);
                n
            })
        }
    }

    impl<'a, W> Write for XorcismWriter<'a, W>
    where
        W: Write,
    {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let writer = &mut self.writer;
            self.xorcism
                .munge(buf)
                .try_fold(0_usize, move |total, byte| {
                    Ok(total + writer.write(&[byte])?)
                })
        }

        fn flush(&mut self) -> io::Result<()> {
            self.writer.flush()
        }
    }
}

pub trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<K>(key: &'a K) -> Self
    where
        K: AsRef<[u8]> + ?Sized + 'a,
    {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .zip(&mut self.key)
            .for_each(move |(byte, &k)| {
                *byte ^= k;
            })
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'s, D, I>(&'s mut self, data: D) -> impl Iterator<Item = u8> + Captures<'a> + 's
    where
        D: IntoIterator<Item = I>,
        D::IntoIter: 's,
        I: Borrow<u8>,
    {
        data.into_iter()
            .zip(&mut self.key)
            .map(move |(byte, k)| byte.borrow() ^ k)
    }

    #[cfg(feature = "io")]
    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        XorcismReader::new(self, reader)
    }

    #[cfg(feature = "io")]
    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        XorcismWriter::new(self, writer)
    }
}
