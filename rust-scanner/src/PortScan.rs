use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::process::exit;
use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;

use super::{PortRange,ScanType,DEFAULT_SCAN_TYPE,DEFAULT_PORT_RANGE,DEFAULT_TARGET};

#[derive(Debug)]
pub struct PortScan {
    scan_type: ScanType,
    range: PortRange,
    target: IpAddr,
    output_path: String,
    result: Vec<u16>
}

impl Serialize for PortScan{
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

impl PortScan {
    // Setters
    pub fn set_port(&mut self, port_range_str: &str){self.range = str_to_port_range(port_range_str);}
    pub fn set_protocol(&mut self, scan_type_str: &str){self.scan_type = str_to_scan_type(scan_type_str);}
    pub fn set_target(&mut self, target: &str){self.target = str_to_ip_addr(target);}
    pub fn set_scan_type(&mut self, scan_type_str: &str){self.scan_type = str_to_scan_type(scan_type_str);}
    pub fn set_output_path(&mut self, output_path: &str){self.output_path = check_output_path(output_path);}
    // Field manipulations
    pub fn add_port_to_result(&mut self, port: u16){self.result.push(port);}
    // Check if file already exists
    fn create_and_write_file(&self){
        match File::create(&self.output_path){
            Ok(mut file) => self.write_scan_result_in_file(&mut file),
            Err(_) => exit_with_error("Error creating output file")
        }
    }
    fn write_scan_result_in_file(&self, file: &mut File){
        match file.write_all(scan_to_json(&self).as_bytes()){
            Ok(_) => println!("Scan result written in {}", self.output_path),
            Err(_) => exit_with_error("Error writing scan result in file")
        }
    }


    // start scan
    pub fn start(&mut self){
        println!("Starting {:?} scan on {:?}", self.scan_type, self.target);
        match self.range {
            PortRange::Single(port) => {
                println!("Scanning port {}", port);
                launch_scan_on_port(self, port);
            },
            PortRange::Range(start,end) => {
                println!("Scanning ports {}-{}", start, end);
                for i in start..=end {
                    launch_scan_on_port(self, i);
                }
            }
        }
        if self.output_path.len() > 0 {
            self.create_and_write_file();
        }
    }

    pub fn create_scan() -> PortScan{
        PortScan {
            range: DEFAULT_PORT_RANGE,
            scan_type: DEFAULT_SCAN_TYPE,
            target: DEFAULT_TARGET,
            output_path: String::new(),
            result: Vec::new()
        }
    }
}

fn str_to_ip_addr(arg: &str) -> IpAddr{
    let ip_obj = arg.parse::<IpAddr>();
    match ip_obj{
        Ok(ip) => ip,
        Err(_) => panic!("Invalid IP address")
    }
}

fn launch_scan_on_port(scan: &mut PortScan, port: u16){
    match scan.scan_type {
        ScanType::Syn => syn_scan_on_port(scan,port),
        ScanType::Connect => connect_scan_on_port(scan,port)
    }
}

fn syn_scan_on_port(_scan: &mut PortScan, _port: u16){
    println!("SYN scan not implemented yet.");
    exit(0);
}

fn connect_scan_on_port(scan: &mut PortScan, port: u16){
    //println!("Connect scan on port {} on {}", port, target);
    match TcpStream::connect(scan.target.to_string()+":"+port.to_string().as_str()){
        Ok(_) => {
            println!("Port {} is open", port);
            scan.add_port_to_result(port);
        },
        Err(_) => ()
    }
}