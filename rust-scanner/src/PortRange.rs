use serde::Serialize;

const PORT_DELIMITER: char = '-';

#[derive(Debug, PartialEq, Serialize)]
pub enum PortRange {
    Single(u16),
    Range(u16, u16)
}

pub fn str_to_port_range(arg: &str) -> PortRange {
    let parts: Vec<&str> = arg.split(PORT_DELIMITER).collect();
    match parts[..] {
        [start, end] => check_port_range(start, end),
        [start] => PortRange::Single(start.parse::<u16>().unwrap()),
        _ => panic!("Invalid port range")
    }
}

pub fn check_port_range(start: &str, end: &str) -> PortRange {
    let start = start.parse::<u16>().unwrap();
    let end = end.parse::<u16>().unwrap();
    if start > end {
        panic!("Invalid port range");
    }
    PortRange::Range(start, end)
}