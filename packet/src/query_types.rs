#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QueryType {
    Unknown(u16),
    A,
}

impl From<u16> for QueryType {
    fn from(index: u16) -> Self {
        match index {
            1 => Self::A,
            _ => Self::Unknown(index),
        }
    }
}

impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            Self::A => 1,
            Self::Unknown(index) => index,
        }
    }
}