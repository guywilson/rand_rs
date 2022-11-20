use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut size = String::new();
    let mut output_file = String::new();

    if args.len() > 1 {
        for i in 0..args.len() {
            if args[i].eq(&"-o") {
                output_file.replace_range(.., &args[i + 1]);
//              println!("Got output file: {output_file}");
            }
            else if args[i].eq(&"-s") {
                size.replace_range(.., &args[i + 1]);
                println!("Requested size: {size}");
            }
        }
    }
    else {
        print_usage();
        exit(-1);
    }

    let mut byte_count: u32 = 0;
    let mut num_bytes: u32 = parse_value(size);
    let remainder = num_bytes % 64;

    if remainder > 0 {
        num_bytes += 64 - remainder;
    }

    println!("Writing {} random bytes to {}", num_bytes, output_file);

    let mut f = File::open("/dev/random").expect("Could not open random device");
    let mut o = File::create(output_file).expect("Could not open file for writing");

    let mut buf = [0; 64];

    while byte_count < num_bytes {
        f.read_exact(&mut buf).expect("Failed to read from random device");
        o.write(&buf).expect("Failed to write to output file");

        byte_count += 64;
    }

    Ok(())
}

fn parse_value(value: String) -> u32 {
    let value: u32 = value.trim().parse().expect("Please type a number!");

    return value;
}

fn print_usage() {
    println!("Usage:");
    println!("    rand <options>");
    println!("        -o <output file>");
    println!("        -s <size>");
    println!("");
}
