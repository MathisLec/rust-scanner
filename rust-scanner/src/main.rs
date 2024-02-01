use std::env;
use std::process::exit;

use libscanner::PortScan::PortScan;
/// print the help
fn print_help(){
    println!("rust-scanner [OPTIONS] <TARGET>");
    println!("\t-h, --help\t\tPrints help information");
    println!("\t-p, --port\t\tPort range to scan. Default: 1-1024");
    println!("\t\t\t\tValid port ranges:");
    println!("\t\t\t\t\tSingle port: 80");
    println!("\t\t\t\t\tPort range: 1-1024");
    println!("\t-s, --scan\t\tScan type. Default: connect");
    println!("\t\t\t\tValid scan types:");
    println!("\t\t\t\t\tsyn");
    println!("\t\t\t\t\tconnect");
    println!("\t-o, --output\t\tPath of the output file.");
}

/// iterate over the arguments and detect if defined parameters are set and change the PortScan field if so
fn parse_args(scan: &mut PortScan){
    // Get args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No target specified");
        exit(1);
    }
    // Iterate over args
    for i in 1..args.len(){
        let current_arg = match args.get(i){
            Some(arg) => arg.as_str(),
            None => {
                println!("No argument specified");
                exit(1);
            }
        };

        if current_arg.starts_with("-"){
            match current_arg {
                "-h" | "--help" => {
                    print_help();
                    exit(0);
                },
                "-p" | "--port" => {
                    match args.get(i+1){
                        Some(port) => scan.set_port(port),
                        None => {
                            println!("No port specified");
                            exit(1);
                        }
                    }
                },
                "-s" | "--scan" => {
                    match args.get(i+1){
                        Some(scan_type) => scan.set_scan_type(scan_type),
                        None => {
                            println!("No scan type specified");
                            exit(1);
                        }
                    }
                },
                "-o" | "--output" => {
                    match args.get(i+1){
                        Some(output) => scan.set_output_path(output),
                        None => {
                            println!("No output file specified");
                            exit(1);
                        }
                    }
                },
                _ => println!("Unknown argument :: {}",current_arg)
            }
        }
    }
    // The last argument should be the target
    let last_arg = args.get(args.len()-1);
    match last_arg {
        Some(arg) => {
            scan.set_target(arg.as_str());
        },
        None => {
            println!("No target specified");
            exit(1);
        }
    }
}

fn main() {
    let mut scan = PortScan::create_scan();
    parse_args(&mut scan);
    scan.start();
}
