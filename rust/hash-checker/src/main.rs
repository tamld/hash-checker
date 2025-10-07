use std::path::PathBuf;

use clap::Parser;
use hash_checker::{supported_algorithms, verify_hash};

#[derive(Parser, Debug)]
#[command(name = "hash-checker", version, about = "Verify file integrity using cryptographic hashes.")]
struct Cli {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(value_name = "EXPECTED_HASH")]
    expected_hash: Option<String>,

    #[arg(short = 'a', long = "algorithm", value_name = "ALGORITHM")]
    algorithm: Option<String>,

    #[arg(long = "list-algorithms", help = "List supported algorithms and exit")]
    list_algorithms: bool,

    #[arg(long = "gui", help = "Launch the graphical interface (not yet available in Rust MVP)")]
    gui: bool,

    #[arg(long = "no-cli", help = "Force GUI mode even if CLI args are provided")]
    no_cli: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.list_algorithms {
        println!("Available algorithms:");
        for algo in supported_algorithms() {
            println!("- {}", algo);
        }
        return;
    }

    if cli.gui || cli.no_cli {
        eprintln!("GUI mode is not yet implemented in the Rust MVP.");
        std::process::exit(5);
    }

    let file = match cli.file {
        Some(path) => path,
        None => {
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    let expected = match cli.expected_hash {
        Some(hash) => hash,
        None => {
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    match verify_hash(&file, &expected, cli.algorithm.as_deref()) {
        Ok((true, _computed)) => {
            println!("Hashes match ✅");
            std::process::exit(0);
        }
        Ok((false, computed)) => {
            eprintln!("Hashes do not match ❌");
            eprintln!("Computed: {}", computed);
            std::process::exit(3);
        }
        Err(err) => {
            eprintln!("Verification failed: {}", err);
            std::process::exit(1);
        }
    }
}
