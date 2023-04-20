#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResultCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    BadVersion,
    BadKey,
    BadTime,
    BadMode,
    BadName,
    BadAlg,
    BadTrunc,
    BadCookie,
    Other(u8),
}


impl From<u8> for ResultCode{
    fn from(index: u8) -> Self {
        match index {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImplemented,
            5 => Self::Refused,
            6 => Self::YXDomain,
            7 => Self::YXRRSet,
            8 => Self::NXRRSet,
            9 => Self::NotAuth,
            10 => Self::NotZone,
            16 => Self::BadVersion,
            17 => Self::BadKey,
            18 => Self::BadTime,
            19 => Self::BadMode,
            20 => Self::BadName,
            21 => Self::BadAlg,
            22 => Self::BadTrunc,
            23 => Self::BadCookie,
            _ => Self::Other(index),
        }
    }
}

impl Into<u8> for ResultCode {
    fn into(self) -> u8 {
        match self {
            Self::NoError => 0,
            Self::FormatError => 1,
            Self::ServerFailure => 2,
            Self::NameError => 3,
            Self::NotImplemented => 4,
            Self::Refused => 5,
            Self::YXDomain => 6,
            Self::YXRRSet => 7,
            Self::NXRRSet => 8,
            Self::NotAuth => 9,
            Self::NotZone => 10,
            Self::BadVersion => 16,
            Self::BadKey => 17,
            Self::BadTime => 18,
            Self::BadMode => 19,
            Self::BadName => 20,
            Self::BadAlg => 21,
            Self::BadTrunc => 22,
            Self::BadCookie => 23,
            Self::Other(index) => index,
        }
    }
}