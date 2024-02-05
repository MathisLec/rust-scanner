use serde::Serialize;
/// The scan type could be a SYN scan (not implemented yet) or a standard Connect scan
#[derive(Debug, Serialize, PartialEq)]
pub enum ScanType {
    Syn,
    Connect
}
/// Return the associated ScanType structure from the specified string slice. ScanType could be Syn or Connect. Return Result::Err if the scan type is not recognized.
pub fn str_to_scan_type(arg: &str) -> Result<ScanType, &'static str> {
    let bind = arg.to_lowercase();
    let arg= bind.as_str();
    match arg {
        "syn" => Ok(ScanType::Syn),
        "connect" => Ok(ScanType::Connect),
        _ => Err("Wrong scan type specified")
    }
}