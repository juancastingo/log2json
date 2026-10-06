use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process::exit;

use log2json::cli::CliArgs;
use log2json::{process_stream, ParserOptions};

fn main() {
    let args = CliArgs::parse();

    let custom_pattern = if let Some(ref pat_str) = args.pattern {
        match Regex::new(pat_str) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Invalid regex pattern '{}': {}", pat_str, e);
                exit(2);
            }
        }
    } else {
        None
    };

    let options = ParserOptions {
        omit_raw: args.omit_raw,
        custom_pattern,
    };

    // Prepare reader
    let reader: Box<dyn io::BufRead> = match args.input.as_deref() {
        None | Some("-") => Box::new(io::stdin().lock()),
        Some(path) => match File::open(path) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("Failed to open input file '{}': {}", path, e);
                exit(2);
            }
        },
    };

    // Prepare writer
    let writer: Box<dyn io::Write> = match args.output.as_deref() {
        None | Some("-") => Box::new(io::stdout().lock()),
        Some(path) => match File::create(path) {
            Ok(f) => Box::new(BufWriter::new(f)),
            Err(e) => {
                eprintln!("Failed to create output file '{}': {}", path, e);
                exit(2);
            }
        },
    };

    if let Err(e) = process_stream(reader, writer, &options, args.pretty) {
        eprintln!("Error streaming log transformation: {}", e);
        exit(1);
    }
}
