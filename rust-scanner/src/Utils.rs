use std::fs;
use std::process::exit;
use crate::PortScan::PortScan;

/// exit the programme after printing the specified string slice
pub fn exit_with_error(msg: &str) {
    println!("{}", msg);
    exit(1);
}
/// make some checks on the string slice
pub fn check_output_path(output_path: &str) -> Result<String, &'static str> {
    // Check if the file exists yet
    if let Err(_) = fs::metadata(output_path) {
        return Ok(output_path.to_string());
    }
    Err("Output file already exists")
}
/// convert the current scan structure into the serialized form in JSON
pub fn scan_to_json(scan: &PortScan) -> Result<String, &'static str> {
    match serde_json::to_string_pretty(scan) {
        Ok(json) => Ok(json),
        Err(_) => {
            Err("Error serializing scan result")
        }
    }
}