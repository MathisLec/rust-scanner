use std::env;
use std::process::exit;

use libscanner::create_scan;
use libscanner::PortScan;


fn print_help(){
    println!("rust-scanner [OPTIONS] <TARGET>");
    println!("\t-h, --help\t\tPrints help information");
    println!("\t-p, --port\t\tPort range to scan. Default: 1-1024");
    println!("\t\t\t\tValid port ranges:");
    println!("\t\t\t\t\tSingle port: 80");
    println!("\t\t\t\t\tPort range: 1-1024");
}


fn parse_args(scan: &mut PortScan){
    // Get args
    let args: Vec<String> = env::args().collect();
    // Iterate over args
    for i in 0..args.len(){
        let current_arg = args.get(i).unwrap().as_str();
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
                _ => println!("Unknown argument :: {}",current_arg)
            }
        }
    }
    // The last argument should be the target
    scan.set_target(args.get(args.len()-1).unwrap().as_str());
}

fn main() {
    let mut scan = create_scan();
    parse_args(&mut scan);
    scan.start();
}
