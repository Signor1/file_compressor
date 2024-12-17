extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

// method for compressing
fn compress(source: &str, target: &str) {
    let mut input = BufReader::new(File::open(source).expect("Failed to open source file"));

    let output = File::create(target).expect("Failed to create target file");

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!(
        "Original size: {:?} bytes",
        input.get_ref().metadata().unwrap().len()
    );

    println!(
        "Compressed size: {:?} bytes",
        output.metadata().unwrap().len()
    );

    println!("Compression completed in {:?}", start.elapsed());
}

// method for decompressing
fn decompress(source: &str, target: &str) {
    let input = File::open(source).expect("Failed to open source file");

    let mut decoder = GzDecoder::new(input);

    let mut output = File::create(target).expect("Failed to create target file");

    let start = Instant::now();

    copy(&mut decoder, &mut output).unwrap();

    println!(
        "Decompressed file size: {:?} bytes",
        output.metadata().unwrap().len()
    );

    println!("Decompression completed in {:?}", start.elapsed());
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage:\n  Compress:   <mode: compress> <source> <target>\n  Decompress: <mode: decompress> <source> <target>"
        );
        return;
    }

    let mode = &args[1];
    let source = &args[2];
    let target = &args[3];

    match mode.as_str() {
        "compress" => {
            println!("Compressing {} to {}...", source, target);
            compress(source, target);
        }
        "decompress" => {
            println!("Decompressing {} to {}...", source, target);
            decompress(source, target);
        }
        _ => {
            eprintln!("Invalid mode. Use 'compress' or 'decompress'.");
        }
    }
}
