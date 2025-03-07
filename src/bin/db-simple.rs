use anyhow::{Context, Result};
use clap::{command, Parser, Subcommand};
use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

const FILE: &str = "data/database-simple.txt";

/// A simple append based database
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
}

/// Write key value pair to the database
fn set(key: &str, value: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(FILE)
        .with_context(|| format!("Cannot open file {}", FILE))?;

    writeln!(file, "{},{}", key, value)
        .with_context(|| format!("Failed to write ({}, {}) to database", key, value))?;

    Ok(())
}

/// Get value for the given key from database
fn get(key: &str) -> Option<String> {
    let file = BufReader::new(
        File::open(FILE)
            .with_context(|| format!("Failed to read file {}.", FILE))
            .unwrap(),
    );

    let mut lines: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    lines.reverse();

    let re = Regex::new(&format!(r"{},(.*)", key)).unwrap();

    for line in lines.iter() {
        match re.captures(line) {
            Some(cap) => return Some(cap[1].to_string()),
            None => (),
        }
    }
    None
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Get { key } => match get(key) {
            Some(v) => println!(r"{}: {}", key, v),
            None => print!("Key '{}' not found", key),
        },
        Commands::Set { key, value } => set(key, value)?,
    };

    Ok(())
}
