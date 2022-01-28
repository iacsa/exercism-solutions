#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    capacity: usize,
    buffer: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer::<T> {
            capacity: capacity,
            buffer: (0..capacity).map(|_| None).collect(),
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(v) = self.buffer[self.read_index % self.capacity].take() {
            self.read_index += 1;
            Ok(v)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            Ok(self.overwrite(element))
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            // compensate for the overwritten element, as it can no longer be read
            self.read_index += 1;
        }
        self.buffer[self.write_index % self.capacity] = Some(element);
        self.write_index += 1;
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|element| *element = None);
        self.read_index = self.write_index;
    }

    pub fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }

    pub fn is_full(&self) -> bool {
        self.read_index + self.capacity == self.write_index
    }
}
