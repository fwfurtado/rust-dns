#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QueryClass {
    Unknown(u16),
    IN,
    CS,
    CH,
    HS,
}

impl From<u16> for QueryClass {
    fn from(index: u16) -> Self {
        match index {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => Self::Unknown(index),
        }
    }
}

impl Into<u16> for QueryClass {
    fn into(self) -> u16 {
        match self {
            Self::IN => 1,
            Self::CS => 2,
            Self::CH => 3,
            Self::HS => 4,
            Self::Unknown(index) => index,
        }
    }
}