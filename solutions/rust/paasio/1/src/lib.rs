use std::io::{Read, Result, Write};

pub struct RWStats<T>(T, usize, usize);
pub type ReadStats<T> = RWStats<T>;
pub type WriteStats<T> = RWStats<T>;

impl<T> RWStats<T> {
    pub fn new(wrapped: T) -> RWStats<T> { Self(wrapped, 0, 0) }
    pub fn get_ref(&self) -> &T { &self.0 }
    pub fn bytes_through(&self) -> usize { self.1 }
    pub fn reads(&self) -> usize { self.2 }     // for both R&W, but its ok
    pub fn writes(&self) -> usize { self.2 }    // for both R&W, but its ok
}

impl<R: Read> Read for RWStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf).map(|n| { self.1 += n; self.2 += 1; n })
    }
}

impl<W: Write> Write for RWStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf).map(|n| { self.1 += n; self.2 += 1; n })
    }

    fn flush(&mut self) -> Result<()> { self.0.flush() }
}