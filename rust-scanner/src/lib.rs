pub mod PortScan;
mod PortRange;
mod ScanType;
mod Utils;

/**
* Tests Section
*/
#[cfg(test)]
mod tests {
    use super::*;

    use PortRange::str_to_port_range;

    use PortScan::str_to_ip_addr;

    #[test]
    fn test_str_to_port_range(){
        assert_eq!(str_to_port_range("1"),PortRange::PortRange::Single(1));
        assert_eq!(str_to_port_range("1-1024"),PortRange::PortRange::Range(1,1024));
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