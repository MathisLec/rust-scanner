pub mod PortScan;
pub mod PortRange;
pub mod ScanType;
pub mod Utils;

use std::net::{IpAddr, Ipv4Addr};
use PortScan::PortScan;
use PortRange::PortRange;
use ScanType::ScanType;

const PORT_DELIMITER: char = '-';
const DEFAULT_PORT_RANGE: PortRange = PortRange::Range(1, 1024);
// Well-Known Ports
const DEFAULT_SCAN_TYPE: ScanType = ScanType::Connect;
const DEFAULT_TARGET: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

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