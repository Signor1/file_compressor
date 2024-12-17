extern crate flate2;

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
fn decompress(source: &str, target: &str) {}

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
}
