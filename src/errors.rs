use std::{net::AddrParseError, num::ParseIntError};

#[derive(PartialEq, Eq, Debug)]
pub enum NetworkParseError {
    AddrParseError(AddrParseError),
    ParseIntError(ParseIntError),
    CidrParseError,
    NetworkLengthError,
}
