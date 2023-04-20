#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    Query,
    IQuery,
    Status,
    Unknown(u8),
}


impl From<u8> for Opcode {
    fn from(index: u8) -> Self {
        match index {
            0 => Self::Query,
            1 => Self::IQuery,
            2 => Self::Status,
            _ => Self::Unknown(index),
        }
    }
}


impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Self::Query => 0,
            Self::IQuery => 1,
            Self::Status => 2,
            Self::Unknown(index) => index,
        }
    }
}