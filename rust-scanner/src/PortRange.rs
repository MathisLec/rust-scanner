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
pub fn str_to_port_range(arg: &str) -> PortRange {
    let parts: Vec<&str> = arg.split(PORT_DELIMITER).collect();
    match parts[..] {
        [start, end] => check_port_range(start, end),
        [start] => PortRange::Single(start.parse::<u16>().unwrap()),
        _ => panic!("Invalid port range")
    }
}
/// make some checks on the specified range before creating an associated object
pub fn check_port_range(start: &str, end: &str) -> PortRange {
    let start = start.parse::<u16>().unwrap();
    let end = end.parse::<u16>().unwrap();
    if start > end {
        panic!("Invalid port range");
    }
    PortRange::Range(start, end)
}