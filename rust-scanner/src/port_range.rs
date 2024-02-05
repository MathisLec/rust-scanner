use serde::Serialize;
/// Delimiter used to separate the 2 parts of a range port specification
const PORT_DELIMITER: char = '-';
/// The port range could be a single port or multiple (Range) ports to scan
#[derive(Debug, PartialEq, Serialize)]
pub enum PortRange {
    Single(u16),
    Range(u16, u16)
}
/// Return the associated PortRange structure from the specified string slice. PortRange could be Single or Range. Return Result::Err if the port range is not recognized.
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
        _ => Err("Invalid port range")
    }
}
/// Make parse checks on the specified range before creating an associated object. Return a Result::Ok if the range is valid or a Result::Err if it's not.
pub fn check_port_range(start: &str, end: &str) -> Result<PortRange, &'static str> {
    // parse is limited to u16, so can't be negative neither greater than 65535 (compiler warning when we verify it)
    let start = start.parse::<u16>();
    let end = end.parse::<u16>();
    match (start, end) {
        (Ok(s), Ok(e)) => {
            if s > e {
                Err("Invalid port range")
            } else {
                Ok(PortRange::Range(s, e))
            }
        },
        _ => Err("Invalid port range")
    }
}