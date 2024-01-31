use std::fs;
use std::process::exit;
use crate::PortScan::PortScan;

pub fn exit_with_error(msg: &str) {
    println!("{}", msg);
    exit(1);
}

pub fn check_output_path(output_path: &str) -> String {
    if let Err(_) = fs::metadata(output_path) {
        return output_path.to_string();
    }
    exit_with_error("Output file already exists");
    String::new()
}

pub fn scan_to_json(scan: &PortScan) -> String {
    match serde_json::to_string_pretty(scan) {
        Ok(json) => json,
        Err(_) => {
            exit_with_error("Error serializing scan result");
            String::new()
        }
    }
}