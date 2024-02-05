use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;

use crate::port_range::PortRange;
use crate::port_range::str_to_port_range;

use crate::scan_type::ScanType;
use crate::scan_type::str_to_scan_type;

use crate::utils::{scan_to_json, check_output_path, exit_with_error};
/// The default port range when creating the PortScan structure (Well-Known Ports)
const DEFAULT_PORT_RANGE: PortRange = PortRange::Range(1, 1024);
/// The default scan type when creating the PortScan structure
const DEFAULT_SCAN_TYPE: ScanType = ScanType::Connect;
/// The default ip target when creating the PortScan structure
const DEFAULT_TARGET: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));


#[derive(Debug)]
pub struct PortScan {
    scan_type: ScanType,
    range: PortRange,
    target: IpAddr,
    output_path: Option<String>,
    result: Vec<u16>
}
/// Implement the Serialize trait in order to transform the structure into json object
impl Serialize for PortScan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PortScan", 4)?;
        state.serialize_field("Scan-Type", &self.scan_type)?;
        state.serialize_field("Scan-Port", &self.range)?;
        state.serialize_field("Scan-Target", &self.target)?;
        state.serialize_field("Scan-Opened-Ports", &self.result)?;
        state.end()
    }
}
/// Structure methods
impl PortScan {
    // Setters (will try to parse the string slice and exit the program if it fails)
    pub fn set_port(&mut self, port_range_str: &str) {
        match str_to_port_range(port_range_str){
            Ok(port_range) => self.range = port_range,
            Err(msg) => exit_with_error(msg)
        }
    }
    pub fn set_target(&mut self, target: &str) {
        match str_to_ip_addr(target){
            Ok(ip) => self.target = ip,
            Err(msg) => exit_with_error(msg)
        }
    }
    pub fn set_scan_type(&mut self, scan_type_str: &str) {
        match str_to_scan_type(scan_type_str){
            Ok(scan_type) => self.scan_type = scan_type,
            Err(msg) => exit_with_error(msg)
        }
    }
    pub fn set_output_path(&mut self, output_path: &str) {
        match check_output_path(output_path) {
            Ok(output_path) => self.output_path = Some(output_path),
            Err(msg) => exit_with_error(msg)
        }
    }
    // Field manipulations
    pub fn add_port_to_result(&mut self, port: u16) { self.result.push(port); }
    // Output generation
    fn create_and_write_file(&self) {
        match &self.output_path {
            Some(output_path) => {
                match File::create(output_path) {
                    Ok(mut file) => self.write_scan_result_in_file(&mut file),
                    Err(_) => exit_with_error("Error creating output file")
                }
            },
            None => exit_with_error("Error creating output file :: output_path field is None")
        }
    }
    fn write_scan_result_in_file(&self, file: &mut File) {
        match scan_to_json(&self) {
            Ok(json) => {
                match file.write_all(json.as_bytes()) {
                    Ok(_) => {
                        match &self.output_path {
                            Some(path) => println!("Scan result written in {}", path),
                            None => exit_with_error("Error writing scan result in file :: output_path field is None")
                        }
                    },
                    Err(_) => exit_with_error("Error writing scan result in file")
                }
            },
            Err(msg) => exit_with_error(msg)
        }
    }


    // start scan
    pub fn start(&mut self) {
        println!("Starting {:?} scan on {:?}", self.scan_type, self.target);
        match self.range {
            PortRange::Single(port) => {
                println!("Scanning port {}", port);
                launch_scan_on_port(self, port);
            },
            PortRange::Range(start, end) => {
                println!("Scanning ports {}-{}", start, end);
                for i in start..=end {
                    launch_scan_on_port(self, i);
                }
            }
        }
        match self.output_path{
            Some(_) => self.create_and_write_file(),
            None => ()
        }
    }
    // The "default constructor"
    pub fn create_scan() -> PortScan {
        PortScan {
            range: DEFAULT_PORT_RANGE,
            scan_type: DEFAULT_SCAN_TYPE,
            target: DEFAULT_TARGET,
            output_path: None,
            result: Vec::new()
        }
    }
}
/// Return the associated IpAddr structure from the specified string slice. Return Result::Ok if the IP address is valid or a Result::Err if it's not.
pub fn str_to_ip_addr(arg: &str) -> Result<IpAddr, &'static str> {
    let ip_obj = arg.parse::<IpAddr>();
    match ip_obj {
        Ok(ip) => Ok(ip),
        Err(_) => Err("Invalid IP address")
    }
}
/// Launch the correct scan function from the ScanType field
fn launch_scan_on_port(scan: &mut PortScan, port: u16) {
    match scan.scan_type {
        ScanType::Syn => syn_scan_on_port(scan, port),
        ScanType::Connect => connect_scan_on_port(scan, port)
    }
}
/// Perform SYN scan on the specified port (Not implemented yet)
fn syn_scan_on_port(_scan: &mut PortScan, _port: u16) {
    todo!("SYN scan not implemented yet.");
}
/// Perform Connect scan on the specified port. If the connection is successful, add the port to the result field.
fn connect_scan_on_port(scan: &mut PortScan, port: u16) {
    //println!("Connect scan on port {} on {}", port, target);
    match TcpStream::connect(scan.target.to_string() + ":" + port.to_string().as_str()) {
        Ok(_) => {
            println!("Port {} is open", port);
            scan.add_port_to_result(port);
        },
        Err(_) => ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_port() {
        let mut scan = PortScan::create_scan();
        scan.set_port("1-1024");
        assert_eq!(scan.range, PortRange::Range(1, 1024));
        scan.set_port("1024");
        assert_eq!(scan.range, PortRange::Single(1024));
    }
    #[test]
    fn test_set_target() {
        let mut scan = PortScan::create_scan();
        scan.set_target("127.0.0.1");
        assert_eq!(scan.target, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
    #[test]
    fn test_set_scan_type() {
        let mut scan = PortScan::create_scan();
        scan.set_scan_type("syn");
        assert_eq!(scan.scan_type, ScanType::Syn);
        scan.set_scan_type("connect");
        assert_eq!(scan.scan_type, ScanType::Connect);
    }
    #[test]
    fn test_set_output_path() {
        let mut scan = PortScan::create_scan();
        scan.set_output_path("output.json");
        match scan.output_path {
            Some(path) => assert_eq!(path, "output.json"),
            None => assert!(false)
        }
    }
    #[test]
    fn test_add_port_to_result() {
        let mut scan = PortScan::create_scan();
        scan.add_port_to_result(80);
        assert_eq!(scan.result, vec![80]);
    }
}