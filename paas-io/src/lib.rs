use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    total_reads: usize,
    total_bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            data: wrapped,
            total_reads: 0,
            total_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn reads(&self) -> usize {
        self.total_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let Ok(bytes) = self.data.read(buf) else { return self.data.read(buf) };

        self.total_reads += 1;
        self.total_bytes += bytes;

        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    to_write_to: W,
    total_writes: usize,
    total_bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            to_write_to: wrapped,
            total_writes: 0,
            total_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.to_write_to
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn writes(&self) -> usize {
        self.total_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let Ok(bytes) = self.to_write_to.write(buf) else { return self.to_write_to.write(buf) };
        self.total_writes += 1;
        self.total_bytes += bytes;

        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.to_write_to.flush()
    }
}