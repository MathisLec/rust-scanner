use serde::Serialize;

#[derive(Debug,Serialize)]
pub enum ScanType {
    Syn,
    Connect
}

pub fn str_to_scan_type(arg: &str) -> ScanType{
    match arg {
        "syn" => ScanType::Syn,
        "connect" => ScanType::Connect,
        _ => panic!("Invalid scan type")
    }
}