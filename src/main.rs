//! # SNMPHASHPWNER
//!
//! `snpmhashpwner` is a simple Cli application crack a password from an snpm
//! authentication exchange. This works only for MD5 based SNMP Auth

use clap::Parser;
use snmphashpwner::{snmp_pwner, Config};
use std::process;

fn main() {
    let config: Config = Config::parse();
    println!("[!] Cracking hash in file: {} ...", config.target_file);
    println!(
        "\nThe password is: {}",
        snmp_pwner(config).unwrap_or_else(|err| {
            eprintln!("[!] An error has occured: {}", err);
            process::exit(1)
        })
    );
}
