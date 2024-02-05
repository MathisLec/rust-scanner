use std::fs;
use std::process::exit;
use crate::port_scan::PortScan;

/// exit the programme after printing the specified string slice msg
pub fn exit_with_error(msg: &str) {
    println!("{}", msg);
    exit(1);
}
/// Check if the output file from path output_path exists and return a Result::Ok if it doesn't or a Result::Err if it does
pub fn check_output_path(output_path: &str) -> Result<String, &'static str> {
    // Check if the file exists yet
    match fs::metadata(output_path) {
        Ok(_) => Err("Output file already exists"),
        Err(_) => Ok(output_path.to_string())
    }
}
/// Serialize the PortScan structure as a json through a custom implementation or return a Result::Err if serialization fails
pub fn scan_to_json(scan: &PortScan) -> Result<String, &'static str> {
    match serde_json::to_string_pretty(scan) {
        Ok(json) => Ok(json),
        Err(_) => {
            Err("Error serializing scan result")
        }
    }
}