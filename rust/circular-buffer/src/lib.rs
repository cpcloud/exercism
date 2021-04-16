use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

#[derive(Clone)]
pub struct CircularBuffer<T> {
    buf: Vec<Option<T>>,
    capacity: usize,
    len: usize,
    writer_pos: usize,
    reader_pos: usize,
}

impl<T: fmt::Debug> fmt::Debug for CircularBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.buf)
    }
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            capacity,
            len: 0,
            writer_pos: 0,
            reader_pos: 0,
        }
    }

    pub fn reader_pos(&self) -> usize {
        self.reader_pos
    }

    pub fn writer_pos(&self) -> usize {
        self.writer_pos
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    fn advance_writer(&mut self) {
        self.len += 1;
        self.writer_pos = (self.writer_pos + 1) % self.capacity;
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        self.buf[self.writer_pos].replace(element);
        self.advance_writer();

        Ok(())
    }

    fn advance_reader(&mut self) {
        self.len -= 1;
        self.reader_pos = (self.reader_pos + 1) % self.capacity;
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let result = self.buf[self.reader_pos].take().ok_or(Error::EmptyBuffer)?;
        self.advance_reader();
        Ok(result)
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.buf.fill(None);
    }

    pub fn overwrite(&mut self, element: T) {
        if !self.is_full() {
            self.write(element).expect("the buffer not to be full");
        } else {
            self.buf[self.reader_pos].replace(element);
            self.reader_pos = (self.reader_pos + 1) % self.capacity;
        }
    }
}
