pub mod port_scan;
mod port_range;
mod scan_type;
mod utils;

/**
* Tests Section
*/
#[cfg(test)]
mod tests {
    use super::*;

    use port_range::str_to_port_range;

    use utils::check_output_path;

    use scan_type::str_to_scan_type;

    // PortRange tests
    #[test]
    fn test_str_to_port_range_ok(){
        let single_port = str_to_port_range("1024");
        assert!(single_port.is_ok());
        let range_port = str_to_port_range("1-1024");
        assert!(range_port.is_ok());
    }
    #[test]
    fn test_str_to_port_range_err(){
        let single_port = str_to_port_range("-1024");
        assert!(single_port.is_err());
        let single_port = str_to_port_range("50000000000000000");
        assert!(single_port.is_err());
        let range_port = str_to_port_range("1-1024-1024");
        assert!(range_port.is_err());
        let range_port = str_to_port_range("1024-1");
        assert!(range_port.is_err());
    }

    #[test]
    fn test_str_to_port_range_struct(){
        match str_to_port_range("1024"){
            Ok(port_range::PortRange::Single(port)) => {
                assert_eq!(port, 1024);
            },
            _ => assert!(false)
        }
        match str_to_port_range("1024-2048") {
            Ok(port_range::PortRange::Range(start, end)) => {
                assert_eq!(start, 1024);
                assert_eq!(end, 2048);
            },
            _ => assert!(false)
        }
    }
    // ScanType tests
    #[test]
    fn test_str_to_scan_type_ok(){
        let syn_scan = str_to_scan_type("syn");
        assert!(syn_scan.is_ok());
        let connect_scan = str_to_scan_type("connect");
        assert!(connect_scan.is_ok());
        let syn_scan = str_to_scan_type("SYN");
        assert!(syn_scan.is_ok());
        let connect_scan = str_to_scan_type("CONNECT");
        assert!(connect_scan.is_ok());
        let syn_scan = str_to_scan_type("SyN");
        assert!(syn_scan.is_ok());
        let connect_scan = str_to_scan_type("CoNnEcT");
        assert!(connect_scan.is_ok());
    }
    #[test]
    fn test_str_to_scan_type_err(){
        let syn_scan = str_to_scan_type("synn");
        assert!(syn_scan.is_err());
        let connect_scan = str_to_scan_type("connectt");
        assert!(connect_scan.is_err());
        let random_scan = str_to_scan_type("gfpoeqirhgpoqeihprgoqeporghoi");
        assert!(random_scan.is_err());
    }
    #[test]
    fn test_str_to_scan_type_struct(){
        match str_to_scan_type("syn"){
            Ok(scan_type::ScanType::Syn) => {},
            _ => assert!(false)
        }
        match str_to_scan_type("connect"){
            Ok(scan_type::ScanType::Connect) => {},
            _ => assert!(false)
        }
    }
    // Utils tests
    #[test]
    fn test_check_output_path_ok() {
        // The random named file should not exist in the current directory, so can be created
        let path = check_output_path("peorighnpomqeirhgnqùoperhnùqpàierhngù");
        assert!(path.is_ok());
    }
    #[test]
    fn test_check_output_path_err() {
        // The file lib.rs should exist in the current directory
        let path = check_output_path("src/lib.rs");
        assert!(path.is_err());
    }

    #[test]
    fn test_check_output_path_struct() {
        let str_path = "abcdefgh";
        match check_output_path(str_path) {
            Ok(path) => {
                assert_eq!(path, str_path);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_scan_to_json_ok() {
        let scan = port_scan::PortScan::create_scan();
        let json = utils::scan_to_json(&scan);
        assert!(json.is_ok());
    }

    // pub PortScan tests
    #[test]
    fn test_str_to_ip_addr_ok() {
        let ip = port_scan::str_to_ip_addr("127.0.0.1");
        assert!(ip.is_ok());
        let ip = port_scan::str_to_ip_addr("::1");
        assert!(ip.is_ok());
    }

    #[test]
    fn test_str_to_ip_addr_err() {
        let ip = port_scan::str_to_ip_addr("159.526.215.215");
        assert!(ip.is_err());
        let ip = port_scan::str_to_ip_addr("azerty");
        assert!(ip.is_err());
    }

}