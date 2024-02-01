## Rust scanner project

### Introduction

This project is a minimal implementation of a port scanner in Rust language.

### Build

In order to build the project in **Debug Mode**, you must place into the `rust-scanner` directory (which contains `Cargo.toml`), and run the following command:

```bash
cargo build
```
It will produce the executable into `target/debug/rust-scanner`

You possibly want to build the project as a **release** build. To do so, run the following command instead:
```bash
cargo build --release
```
It will produce the executable into `target/release/rust-scanner`

### Start the scan
**Disclaimer**

If you have any error when executing the CLI, please refer to the help option:
```bash
rust-scanner --help # or -h
```

## From Cargo
You can run the executable without using the executable produced into the build stage by enter the following command:
```bash
cargo run -- <TARGET_IP>
```

## From the executable
The simple way to execute the generated executable is the following command:
```bash
rust-scanner <TARGET_IP>
```

## Options
```
rust-scanner [OPTIONS] <TARGET>
        -h, --help              Prints help information
        -p, --port              Port range to scan. Default: 1-1024
                                Valid port ranges:
                                        Single port: 80
                                        Port range: 1-1024
        -s, --scan              Scan type. Default: connect
                                Valid scan types:
                                        syn
                                        connect
```