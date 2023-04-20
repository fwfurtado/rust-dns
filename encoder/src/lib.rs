use std::fmt;

const NULL_BYTE: u8 = 0x00;

#[derive(Debug)]
pub enum BufferedWriterError {
    Full,
}

impl fmt::Display for BufferedWriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BufferedWriterError::Full => write!(f, "buffer is full"),
        }
    }
}

impl std::error::Error for BufferedWriterError {}


pub struct BufferedWriter<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}

impl BufferedWriter<'_> {
    pub fn new(buffer: &mut [u8]) -> BufferedWriter {
        BufferedWriter {
            buffer,
            offset: 0,
        }
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<(), BufferedWriterError> {
        for label in qname.split('.') {
            let length = label.len() as u8;
            self.write_u8(length)?;

            for byte in label.as_bytes() {
                self.write_u8(*byte)?;
            }
        }

        self.write_u8(NULL_BYTE)?;

        Ok(())
    }
}

const MAX_BUFFER_SIZE: usize = 512;

impl Writer for BufferedWriter<'_> {
    fn write_u8(&mut self, byte: u8) -> Result<(), BufferedWriterError> {
        if self.offset >= MAX_BUFFER_SIZE {
            return Err(BufferedWriterError::Full);
        }

        self.buffer[self.offset] = byte;
        self.offset += 1;

        Ok(())
    }
}


pub trait Writer {
    fn write_u8(&mut self, byte: u8) -> Result<(), BufferedWriterError>;

    fn write_u16(&mut self, bytes: u16) -> Result<(), BufferedWriterError> {
        let to_be_write =bytes.to_be_bytes();
        self.write_u8(to_be_write[0])?;
        self.write_u8(to_be_write[1])?;
        Ok(())
    }

    fn write_u32(&mut self, bytes: u32) -> Result<(), BufferedWriterError> {
        let to_be_write =bytes.to_be_bytes();
        self.write_u8(to_be_write[0])?;
        self.write_u8(to_be_write[1])?;
        self.write_u8(to_be_write[2])?;
        self.write_u8(to_be_write[3])?;
        Ok(())
    }

}
