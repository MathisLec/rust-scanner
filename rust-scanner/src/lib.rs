use std::net::IpAddr;
use std::net::Ipv4Addr;

const PORT_DELIMITER: char = '-';
const DEFAULT_PORT_RANGE: PortRange = PortRange::Range(1,1024); // Well-Known Ports
const DEFAULT_SCAN_TYPE: ScanType= ScanType::Syn;
const DEFAULT_TARGET: IpAddr = IpAddr::V4(Ipv4Addr::new(0,0,0,0));

/**
* Port scan structure
*/
pub struct PortScan {
    range: PortRange,
    scan_type: ScanType,
    target: IpAddr
}

impl PortScan {
    // Setters
    pub fn set_port(&mut self, port_range_str: &str){self.range = str_to_port_range(port_range_str);}
    pub fn set_protocol(&mut self, scan_type_str: &str){self.scan_type = str_to_scan_type(scan_type_str);}
    pub fn set_target(&mut self, target: &str){self.target = str_to_ip_addr(target);}
    // start scan

    pub fn start(&self){
        println!("Starting {:?} scan on {:?}", self.scan_type, self.target);
        match self.range {
            PortRange::Single(port) => {
                println!("Scanning port {}", port);
                launch_scan_on_port(&self.scan_type, &self.target, port);
            },
            PortRange::Range(start,end) => {
                for i in start..=end {
                    println!("Scanning ports {}-{}", start, end);
                    launch_scan_on_port(&self.scan_type, &self.target, i);
                }
            }
        }
    }
}
#[derive(Debug,PartialEq)]
enum PortRange {
    Single(u16),
    Range(u16,u16)
}
#[derive(Debug)]
enum ScanType {
    Syn,
    Connect
}

/**
* Structure Functions
*/

fn syn_scan_on_port(target: &IpAddr, port: u16){
    //println!("SYN scan on port {} on {}", port, target);
    //TODO: Implement SYN scan
}

fn connect_scan_on_port(target: &IpAddr, port: u16){
    //println!("Connect scan on port {} on {}", port, target);
    // TODO: Implement connect scan
}

fn launch_scan_on_port(scan_type: &ScanType, target: &IpAddr, port: u16){
    match scan_type {
        ScanType::Syn => syn_scan_on_port(target,port),
        ScanType::Connect => connect_scan_on_port(target,port)
    }
}

fn check_port_range(start: &str, end: &str) -> PortRange{
    let start = start.parse::<u16>().unwrap();
    let end = end.parse::<u16>().unwrap();
    if start > end {
        panic!("Invalid port range");
    }
    PortRange::Range(start,end)
}
fn str_to_port_range(arg: &str) -> PortRange{
    let parts: Vec<&str> = arg.split(PORT_DELIMITER).collect();
    match parts[..]{
        [start, end] => check_port_range(start,end),
        [start] => PortRange::Single(start.parse::<u16>().unwrap()),
        _ => panic!("Invalid port range")
    }
}

fn str_to_ip_addr(arg: &str) -> IpAddr{
    let ip_obj = arg.parse::<IpAddr>();
    match ip_obj{
        Ok(ip) => ip,
        Err(_) => panic!("Invalid IP address")
    }
}

fn str_to_scan_type(arg: &str) -> ScanType{
    match arg {
        "syn" => ScanType::Syn,
        "connect" => ScanType::Connect,
        _ => panic!("Invalid scan type")
    }
}


pub fn create_scan() -> PortScan{
    PortScan {
        range: DEFAULT_PORT_RANGE,
        scan_type: DEFAULT_SCAN_TYPE,
        target: DEFAULT_TARGET
    }
}

/**
* Tests Section
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_port_range(){
        assert_eq!(str_to_port_range("1"),PortRange::Single(1));
        assert_eq!(str_to_port_range("1-1024"),PortRange::Range(1,1024));
    }
    #[test]
    #[should_panic]
    fn test_str_to_port_range_invalid_format(){
        str_to_port_range("1-1024-1024");
    }

    #[test]
    #[should_panic]
    fn test_str_to_port_range_invalid_range(){
        str_to_port_range("1024-1");
    }

    #[test]
    #[should_panic]
    fn test_str_to_ip_addr(){
        str_to_ip_addr("256.256.256.256");
    }
}