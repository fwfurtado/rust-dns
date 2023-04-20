use std::fmt;

const NULL_BYTE: u8 = 0x00;
const MAX_JUMP_COUNT: usize = 10;
const JUMP_BYTE: u8 = 0xC0;

#[derive(Debug)]
pub enum BufferedReaderError {
    EmptyBuffer,
    MaxJumpExceeded,
}

impl fmt::Display for BufferedReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BufferedReaderError::EmptyBuffer => write!(f, "buffer is empty"),
            BufferedReaderError::MaxJumpExceeded => write!(f, "max jump exceeded"),
        }
    }
}

impl std::error::Error for BufferedReaderError {}



pub struct BufferedReader<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> BufferedReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            offset: 0,
        }
    }

    pub fn read_qname(&mut self) -> Result<String, BufferedReaderError> {
        let mut qname = String::new();
        let mut offset = self.offset;

        let mut jumped = false;
        let mut jump_count = 0;

        loop {
            if jump_count > MAX_JUMP_COUNT {
                return Err(BufferedReaderError::MaxJumpExceeded);
            }

            let first_byte = self.buffer[offset];

            if first_byte == JUMP_BYTE {

                if !jumped {
                    self.offset = offset + 2;
                }

                let second_byte = self.buffer[offset + 1];
                let new_offset = ((first_byte as u16) ^ (JUMP_BYTE as u16)) << 8 | second_byte as u16;

                offset = new_offset as usize;

                jumped = true;
                jump_count += 1;
            } else {
                offset += 1;

                if first_byte == NULL_BYTE {
                    break;
                }

                if !qname.is_empty() {
                    qname.push('.');
                }

                let length = first_byte as usize;

                let bytes = &self.buffer[offset..offset + length];

                let read_string = &String::from_utf8_lossy(bytes).to_lowercase();

                offset += length;
                qname.push_str(&read_string);
            }
        }

        if !jumped {
            self.offset = offset;
        }

        Ok(qname)
    }
}


impl Reader for BufferedReader<'_> {
    fn read_u8(&mut self) -> Result<u8, BufferedReaderError> {
        if self.buffer.is_empty() {
            return Err(BufferedReaderError::EmptyBuffer);
        }

        let byte = self.buffer[self.offset];
        self.offset += 1;
        Ok(byte)
    }
}

pub trait Reader {
    fn read_u8(&mut self) -> Result<u8, BufferedReaderError>;

    fn read_u16(&mut self) -> Result<u16, BufferedReaderError> {
        let bytes = [self.read_u8()?, self.read_u8()?];
        Ok(u16::from_be_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, BufferedReaderError> {
        let bytes = [
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
        ];
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, BufferedReaderError> {
        let bytes = [
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
        ];
        Ok(u64::from_be_bytes(bytes))
    }

}


pub trait BitReader {
    fn read_bool(&mut self, bits: u8) -> bool;
    fn read_u8(&mut self, bits: u8) -> u8;
}

impl BitReader for u8 {
    fn read_bool(&mut self, bits: u8) -> bool {
        let byte = *self;
        let bit = byte & (1 << bits);
        bit != 0
    }

    fn read_u8(&mut self, bits: u8) -> u8 {
        let byte = *self;
        let bit = byte & (1 << bits);
        bit
    }
}
