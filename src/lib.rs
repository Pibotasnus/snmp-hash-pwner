//! # Library
//!
//! Implements basic utilities to calculate SNMP authentication hashes

pub use config::Config;
pub use context::Context;

use indicatif::ProgressBar;
use md5;
use std::io::{self, stdout, BufRead, BufReader, Write};
use std::{fs, process};

mod config;
mod context;

/// Calculate
fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16))
        .collect()
}

fn xor_hex_strings(hex1: &str, hex2: &str) -> Result<String, &'static str> {
    if hex1.len() != hex2.len() {
        return Err("Hex strings must be of equal length");
    }

    let bytes1 = hex::decode(hex1).unwrap_or_default();
    let bytes2 = hex::decode(hex2).unwrap_or_default();

    let xor_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
    Ok(xor_bytes.iter().map(|b| format!("{:02x}", b)).collect())
}

fn calculate_md5(context: &Context, password: &mut String) -> Result<String, &'static str> {
    let pwd_len = match password.len() {
        0 => return Err("Empty password"),
        // 1 => return Err("Too low password"),
        length => length,
    };
    let mut count = 0;
    let mut pwd_index = 0;
    let mut pwd_buff = String::from("");
    while count < 1048576 {
        for _ in 0..64 {
            pwd_buff.push_str(
                password
                    .chars()
                    .nth(pwd_index % pwd_len)
                    .unwrap_or_else(|| ' ')
                    .to_string()
                    .as_str(),
            );
            pwd_index += 1;
        }
        count += 64;
    }
    let mut key = md5::compute(pwd_buff.as_bytes());
    let str_key = [
        md5::Digest(*key).as_slice(),
        hex::decode(&context.engine_id).unwrap_or_default().as_ref(),
        md5::Digest(*key).as_slice(),
    ]
    .concat();
    key = md5::compute(str_key);
    let mut entend_key = format!("{:x}", key);
    entend_key.push_str("00".repeat(48).as_str());
    let ipad = "36".repeat(64);
    let key_1 = match xor_hex_strings(&entend_key, &ipad) {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    let opad = "5c".repeat(64);
    let key_2 = match xor_hex_strings(&entend_key, &opad) {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    let mut input_str = format!("{}{}", key_1, context.message);
    let mut input_hash = md5::compute(hex::decode(&input_str).unwrap_or_default());
    input_str = format!("{}{:x}", key_2, input_hash);
    input_hash = md5::compute(hex::decode(&input_str).unwrap_or_default());
    input_str = format!("{:x}", input_hash);
    Ok(input_str.as_str()[0..24].to_string())
}

pub fn snmp_pwner(config: Config) -> Result<String, io::Error> {
    let raw_fc = match fs::read_to_string(&config.target_file) {
        Ok(fc) => fc,
        Err(e) => return Err(e),
    };
    let context = Context::new(raw_fc).unwrap_or_else(|err| {
        eprintln!("Error while creating context {}", err);
        process::exit(1)
    });
    let pwd_dict_file = match fs::File::open(&config.dict_file) {
        Ok(fc) => fc,
        Err(e) => return Err(e),
    };
    let temp_dict_file = pwd_dict_file.try_clone().unwrap();
    let reader = BufReader::new(temp_dict_file);
    let mut count = 0;
    let mut stdout = stdout();
    let bar = ProgressBar::new_spinner();
    for password in reader.lines() {
        bar.set_message(format!("\rTried {} passwords so far ...", count));
        // or
        // stdout.write(format!("\rProcessing {}%...", i).as_bytes()).unwrap();
        stdout.flush().unwrap();
        count += 1;
        let mut password_clone = password?.clone();
        let result = match calculate_md5(&context, &mut password_clone) {
            Ok(result) => result,
            Err(err) => {
                if err == "Empty password" {
                    continue;
                }
                eprintln!("Error while calculating hash {}", err);
                process::exit(1)
            }
        };
        if result == context.hash {
            bar.finish();
            return Ok(password_clone);
        }
    }
    bar.finish();
    Ok("[-] No password found in dict file".to_string())
}
