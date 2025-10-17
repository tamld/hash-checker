use std::io::{stderr, IsTerminal};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use hash_checker::{supported_algorithms, verify_hash};
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "hash-checker",
    version,
    about = "Verify file integrity using cryptographic hashes."
)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(value_name = "EXPECTED_HASH")]
    expected_hash: Option<String>,

    #[arg(short = 'a', long = "algorithm", value_name = "ALGORITHM")]
    algorithm: Option<String>,

    #[arg(long = "list-algorithms", help = "List supported algorithms and exit")]
    list_algorithms: bool,

    #[arg(
        long = "gui",
        help = "Launch the graphical interface (not yet available in Rust MVP)"
    )]
    gui: bool,

    #[arg(long = "no-cli", help = "Force GUI mode even if CLI args are provided")]
    no_cli: bool,

    #[arg(
        long = "log-format",
        value_enum,
        value_name = "FORMAT",
        default_value_t = LogFormat::None,
        help = "Structured log output: none (default), text, or json"
    )]
    log_format: LogFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LogFormat {
    None,
    Text,
    Json,
}

fn main() {
    let cli = Cli::parse();

    init_logging(cli.log_format);

    if cli.list_algorithms {
        info!("Listing supported algorithms");
        println!("Available algorithms:");
        for algo in supported_algorithms() {
            println!("- {}", algo);
        }
        return;
    }

    if cli.gui || cli.no_cli {
        error!("GUI mode requested but not available in the Rust MVP");
        eprintln!("GUI mode is not yet implemented in the Rust MVP.");
        std::process::exit(5);
    }

    let file = match cli.file {
        Some(path) => path,
        None => {
            error!(code = "missing_file_argument", "No file path provided");
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    let expected = match cli.expected_hash {
        Some(hash) => hash,
        None => {
            error!(code = "missing_expected_hash", "No expected hash provided");
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    info!(file = %file.display(), "Starting verification");
    match verify_hash(&file, &expected, cli.algorithm.as_deref()) {
        Ok((true, _computed)) => {
            info!(file = %file.display(), "Hashes match");
            println!("Hashes match ✅");
            std::process::exit(0);
        }
        Ok((false, computed)) => {
            warn!(file = %file.display(), computed = %computed, "Hashes do not match");
            eprintln!("Hashes do not match ❌");
            eprintln!("Computed: {}", computed);
            std::process::exit(3);
        }
        Err(err) => {
            error!(file = %file.display(), error = %err, "Verification failed");
            eprintln!("Verification failed: {}", err);
            std::process::exit(1);
        }
    }
}

fn init_logging(format: LogFormat) {
    match format {
        LogFormat::None => {}
        LogFormat::Text => {
            let _ = tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_ansi(stderr().is_terminal())
                .with_level(true)
                .with_target(false)
                .with_max_level(tracing::Level::INFO)
                .try_init();
        }
        LogFormat::Json => {
            let _ = tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .json()
                .with_current_span(false)
                .with_target(false)
                .with_max_level(tracing::Level::INFO)
                .try_init();
        }
    }
}
