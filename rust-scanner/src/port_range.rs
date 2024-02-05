use std::num::ParseIntError;
use serde::Serialize;
/// Delimiter used to separate the 2 parts of a range port specification
const PORT_DELIMITER: char = '-';
/// The port range could be a single port or multiple (Range) ports to scan
#[derive(Debug, PartialEq, Serialize)]
pub enum PortRange {
    Single(u16),
    Range(u16, u16)
}

pub enum PortRangeError {
    InvalidPortRange(&'static str),
    InvalidPortNumber(&'static str)
}

impl From<ParseIntError> for PortRangeError{
    fn from(_: ParseIntError) -> PortRangeError {
        PortRangeError::InvalidPortNumber("Invalid port number")
    }
}

/// Return the associated PortRange structure from the specified string slice. PortRange could be Single or Range. Return Result::Err if the port range is not recognized.
pub fn str_to_port_range(arg: &str) -> Result<PortRange, &'static str> {
    let parts: Vec<&str> = arg.split(PORT_DELIMITER).collect();
    match parts[..] {
        [start, end] => {
            match check_port_range(start, end){
                Ok(range) => Ok(range),
                Err(_) => Err("Error while parsing port range")
            }
        },
        [start] => {
            match start.parse::<u16>() {
                Ok(start) => Ok(PortRange::Single(start)),
                Err(_) => Err("Invalid port range")
            }
        },
        _ => Err("Invalid port range")
    }
}
/// Make parse checks on the specified range before creating an associated object. Return a Result::Ok if the range is valid or a Result::Err if it's not.
pub fn check_port_range(start: &str, end: &str) -> Result<PortRange, PortRangeError> {
    // parse is limited to u16, so can't be negative neither greater than 65535 (compiler warning when we verify it)
    let start = start.parse::<u16>()?;
    let end = end.parse::<u16>()?;
    if start < end {
        Ok(PortRange::Range(start, end))
    }
    else {
        Err(PortRangeError::InvalidPortRange("Invalid port range"))
    }
}