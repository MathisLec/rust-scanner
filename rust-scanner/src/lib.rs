use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use xenet::util::packet_builder::tcp::TcpPacketBuilder;
use xenet::util::packet_builder::tcp::DEFAULT_SRC_PORT;
use xenet::packet::tcp::TcpFlags;
use xenet::socket::{AsyncSocket, IpVersion, SocketOption, SocketType};
use crate::pcap::PacketCaptureOptions;
use default_net;
use xenet::packet::ip::IpNextLevelProtocol;

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

async fn syn_scan_on_port(target: &IpAddr, port: u16){
    //println!("SYN scan on port {} on {}", port, target);
    // Source IP address in SocketAddr

    let interface = default_net::get_default_interface().unwrap();
    let src_ip = interface.ipv4[0].addr;
    let src = SocketAddr::new(IpAddr::V4(src_ip),DEFAULT_SRC_PORT);

    let dst = SocketAddr::new(*target,port);

    let packet_bytes = TcpPacketBuilder::new(src,dst).flags(TcpFlags::SYN).build();

    // Preparation du socket
    let socket_option = SocketOption {
        ip_version: IpVersion::V4,
        socket_type: SocketType::Raw,
        protocol: Some(IpNextLevelProtocol::Tcp),
        timeout: None,
        ttl: None,
        non_blocking: true,
    };
    let socket = AsyncSocket::new(socket_option).unwrap();

    // Preparation du listener
    /*
    PacketCaptureOptions

    let mut capture_options: PacketCaptureOptions = PacketCaptureOptions {
        interface_index: scan_setting.if_index,
        interface_name: scan_setting.if_name.clone(),
        src_ips: HashSet::new(),
        dst_ips: HashSet::new(),
        src_ports: HashSet::new(),
        dst_ports: HashSet::new(),
        ether_types: HashSet::new(),
        ip_protocols: HashSet::new(),
        duration: scan_setting.timeout,
        read_timeout: scan_setting.wait_time,
        promiscuous: false,
        store: true,
        store_limit: u32::MAX,
        receive_undefined: false,
        tunnel: scan_setting.tunnel,
        loopback: scan_setting.loopback,
    };
    for target in scan_setting.targets.clone() {
        capture_options.src_ips.insert(target.ip_addr);
        capture_options.src_ports.extend(target.get_ports());
    }
    match scan_setting.scan_type {
        ScanType::TcpSynScan => {
            capture_options
                .ip_protocols
                .insert(IpNextLevelProtocol::Tcp);
        }
        ScanType::TcpConnectScan => {
            capture_options
                .ip_protocols
                .insert(IpNextLevelProtocol::Tcp);
        }
        _ => {}
    }
    let listener: Listner = Listner::new(capture_options);

    match socket.send_to(&packet_bytes, dst).await {
        Ok(_) => {}
        Err(_) => {}
    }
    */
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