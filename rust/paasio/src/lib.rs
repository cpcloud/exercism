use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_read: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            bytes_read: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.bytes_read += bytes_read;
        self.num_reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_written: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            bytes_written: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.writer.write(buf)?;
        self.bytes_written += bytes_written;
        self.num_writes += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
