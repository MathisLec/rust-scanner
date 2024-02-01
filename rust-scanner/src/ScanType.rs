use serde::Serialize;
/// The scan type could be a SYN scan (not implemented yet) or a standard Connect scan
#[derive(Debug, Serialize, PartialEq)]
pub enum ScanType {
    Syn,
    Connect
}
/// convert a string slice into a scan type
pub fn str_to_scan_type(arg: &str) -> ScanType {
    let bind = arg.to_lowercase();
    let arg= bind.as_str();
    match arg {
        "syn" => ScanType::Syn,
        "connect" => ScanType::Connect,
        _ => panic!("Invalid scan type")
    }
}