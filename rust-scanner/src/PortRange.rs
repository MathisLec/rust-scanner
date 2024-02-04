use serde::Serialize;
/// Delimiter used to separate the 2 parts of a range port specification
const PORT_DELIMITER: char = '-';
/// The port range could be a single port or multiple (Range) ports to scan
#[derive(Debug, PartialEq, Serialize)]
pub enum PortRange {
    Single(u16),
    Range(u16, u16)
}
/// convert a string slice into a PortRange structure
pub fn str_to_port_range(arg: &str) -> Result<PortRange, &'static str> {
    let parts: Vec<&str> = arg.split(PORT_DELIMITER).collect();
    match parts[..] {
        [start, end] => check_port_range(start, end),
        [start] => {
            match start.parse::<u16>() {
                Ok(start) => Ok(PortRange::Single(start)),
                Err(_) => Err("Invalid port range")
            }
        },
        _ => panic!("Invalid port range")
    }
}
/// make some checks on the specified range before creating an associated object
pub fn check_port_range(start: &str, end: &str) -> Result<PortRange, &'static str> {
    let start = start.parse::<u16>();
    let end = end.parse::<u16>();
    match (start, end) {
        (Ok(s), Ok(e)) => {
            if s > e || s < 0 || e > 65535 {
                Err("Invalid port range")
            } else {
                Ok(PortRange::Range(s, e))
            }
        },
        _ => Err("Invalid port range")
    }
}