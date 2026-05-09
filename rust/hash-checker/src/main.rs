use std::fs::File;
use std::io::{self, Read, Write};
use std::io::{stderr, IsTerminal};
use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};
use hash_checker::{
    detect_format_from_extension, generate_manifest, read_manifest, relative_path_string,
    resolve_root, run_batch, supported_algorithms, verify_hash, verify_manifest, write_manifest,
    BatchInput, BatchReport, BatchStatus, HashError, HashResult, ManifestFormat,
    VerificationReport,
};
use serde::Deserialize;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "hash-checker",
    version,
    about = "Verify file integrity using cryptographic hashes."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

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

#[derive(Subcommand, Debug)]
enum Commands {
    Manifest(ManifestCommand),
    Batch(BatchArgs),
}

#[derive(Args, Debug)]
struct ManifestCommand {
    #[command(subcommand)]
    action: ManifestAction,
}

#[derive(Subcommand, Debug)]
enum ManifestAction {
    Export(ManifestExportArgs),
    Verify(ManifestVerifyArgs),
}

#[derive(Args, Debug)]
struct BatchArgs {
    #[arg(
        long = "input",
        value_name = "PATH",
        help = "Input file containing batch entries (JSON or CSV). Use '-' or omit to read from stdin."
    )]
    input: Option<PathBuf>,

    #[arg(
        long = "input-format",
        value_enum,
        default_value_t = BatchFormatArg::Json,
        help = "Format of the input definition."
    )]
    input_format: BatchFormatArg,

    #[arg(
        long = "output",
        value_name = "PATH",
        help = "Write report to file instead of stdout."
    )]
    output: Option<PathBuf>,

    #[arg(
        long = "output-format",
        value_enum,
        default_value_t = BatchFormatArg::Json,
        help = "Format of the generated report."
    )]
    output_format: BatchFormatArg,
}

#[derive(Args, Debug)]
struct ManifestExportArgs {
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

    #[arg(short = 'f', long = "format", default_value = "json")]
    format: ManifestFormatArg,

    #[arg(short = 'a', long = "algorithm", default_value = "sha256")]
    algorithm: String,

    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct ManifestVerifyArgs {
    #[arg(value_name = "MANIFEST")]
    manifest: PathBuf,

    #[arg(short = 'f', long = "format")]
    format: Option<ManifestFormatArg>,

    #[arg(long = "root", value_name = "DIRECTORY")]
    root: Option<PathBuf>,

    #[arg(long = "report-limit", default_value_t = 10)]
    report_limit: usize,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LogFormat {
    None,
    Text,
    Json,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ManifestFormatArg {
    Json,
    Csv,
    Txt,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum BatchFormatArg {
    Json,
    Csv,
}

impl ManifestFormatArg {
    fn to_manifest_format(self) -> ManifestFormat {
        match self {
            ManifestFormatArg::Json => ManifestFormat::Json,
            ManifestFormatArg::Csv => ManifestFormat::Csv,
            ManifestFormatArg::Txt => ManifestFormat::Plain,
        }
    }
}

fn main() {
    let Cli {
        command,
        file,
        expected_hash,
        algorithm,
        list_algorithms,
        gui,
        no_cli,
        log_format,
    } = Cli::parse();

    init_logging(log_format);

    if let Some(command) = command {
        match handle_command(command) {
            Ok(code) => std::process::exit(code),
            Err(err) => {
                error!(error = %err, "command_failed");
                eprintln!("Manifest command failed: {err}");
                std::process::exit(1);
            }
        }
    }

    if list_algorithms {
        info!("Listing supported algorithms");
        println!("Available algorithms:");
        for algo in supported_algorithms() {
            println!("- {algo}");
        }
        return;
    }

    if gui || no_cli {
        error!("GUI mode requested but not available in the Rust MVP");
        eprintln!("GUI mode is not yet implemented in the Rust MVP.");
        std::process::exit(5);
    }

    let file = match file {
        Some(path) => path,
        None => {
            error!(code = "missing_file_argument", "No file path provided");
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    let expected = match expected_hash {
        Some(hash) => hash,
        None => {
            error!(code = "missing_expected_hash", "No expected hash provided");
            eprintln!("Error: provide both <FILE> and <EXPECTED_HASH> in CLI mode.");
            std::process::exit(2);
        }
    };

    info!(file = %file.display(), "Starting verification");
    match verify_hash(&file, &expected, algorithm.as_deref()) {
        Ok((true, _computed)) => {
            info!(file = %file.display(), "Hashes match");
            println!("Hashes match ✅");
            std::process::exit(0);
        }
        Ok((false, computed)) => {
            warn!(file = %file.display(), computed = %computed, "Hashes do not match");
            eprintln!("Hashes do not match ❌");
            eprintln!("Computed: {computed}");
            std::process::exit(3);
        }
        Err(err) => {
            error!(file = %file.display(), error = %err, "Verification failed");
            eprintln!("Verification failed: {err}");
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

fn handle_command(command: Commands) -> HashResult<i32> {
    match command {
        Commands::Manifest(manifest) => handle_manifest_command(manifest),
        Commands::Batch(args) => handle_batch_command(args),
    }
}

fn handle_manifest_command(manifest: ManifestCommand) -> HashResult<i32> {
    match manifest.action {
        ManifestAction::Export(args) => {
            handle_manifest_export(args)?;
            Ok(0)
        }
        ManifestAction::Verify(args) => handle_manifest_verify(args),
    }
}

fn handle_batch_command(args: BatchArgs) -> HashResult<i32> {
    let inputs = read_batch_inputs(&args)?;
    if inputs.is_empty() {
        return Err(HashError::ManifestParse(
            "batch input is empty; provide at least one entry".to_string(),
        ));
    }

    let report = run_batch(&inputs);
    write_batch_report(&report, &args)?;
    eprintln!(
        "Batch summary: matched={}, mismatched={}, missing={}, errored={}",
        report.summary.matched,
        report.summary.mismatched,
        report.summary.missing,
        report.summary.errored
    );

    Ok(report.exit_code())
}

fn handle_manifest_export(args: ManifestExportArgs) -> HashResult<()> {
    let manifest = generate_manifest(&args.directory, &args.algorithm, args.recursive)?;
    let format = args.format.to_manifest_format();

    if let Some(path) = args.output {
        let file = File::create(&path)?;
        write_manifest(&manifest, format, file)?;
        eprintln!(
            "Manifest written to {} ({} entries, algorithm={})",
            path.display(),
            manifest.entries.len(),
            manifest.algorithm
        );
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        write_manifest(&manifest, format, &mut handle)?;
        handle.flush()?;
        eprintln!(
            "Manifest generated ({} entries, algorithm={})",
            manifest.entries.len(),
            manifest.algorithm
        );
    }

    Ok(())
}

fn handle_manifest_verify(args: ManifestVerifyArgs) -> HashResult<i32> {
    let format = match args.format {
        Some(explicit) => explicit.to_manifest_format(),
        None => detect_format_from_extension(&args.manifest).ok_or_else(|| {
            hash_checker::HashError::UnsupportedManifestFormat(args.manifest.display().to_string())
        })?,
    };

    let file = File::open(&args.manifest)?;
    let manifest = read_manifest(file, format)?;
    let root = resolve_root(&manifest, args.root.as_deref(), &args.manifest);
    let mut report = verify_manifest(&manifest, &root)?;

    if let Some(relative) = manifest_relative_to_root(&args.manifest, &root) {
        report.extra.retain(|entry| entry != &relative);
    }

    print_report(&report, args.report_limit);

    if report.mismatched.is_empty() && report.missing.is_empty() && report.extra.is_empty() {
        println!(
            "All {} entries matched for {}.",
            report.total_entries(),
            root.display()
        );
        return Ok(0);
    }

    if !report.mismatched.is_empty() || !report.missing.is_empty() {
        return Ok(3);
    }

    Ok(4)
}

fn print_report(report: &VerificationReport, limit: usize) {
    println!("Matched files: {}", report.matched);

    if !report.mismatched.is_empty() {
        println!("Mismatched files ({}):", report.mismatched.len());
        for mismatch in report.mismatched.iter().take(limit) {
            println!(
                "  - {} (expected {}, actual {})",
                mismatch.entry.path, mismatch.entry.hash, mismatch.actual
            );
        }
        if report.mismatched.len() > limit {
            println!(
                "  … and {} more mismatched entries.",
                report.mismatched.len() - limit
            );
        }
    }

    if !report.missing.is_empty() {
        println!("Missing files ({}):", report.missing.len());
        for entry in report.missing.iter().take(limit) {
            println!("  - {}", entry.path);
        }
        if report.missing.len() > limit {
            println!(
                "  … and {} more missing entries.",
                report.missing.len() - limit
            );
        }
    }

    if !report.extra.is_empty() {
        println!("Extra files on disk ({}):", report.extra.len());
        for path in report.extra.iter().take(limit) {
            println!("  - {path}");
        }
        if report.extra.len() > limit {
            println!("  … and {} more extra entries.", report.extra.len() - limit);
        }
    }
}

fn manifest_relative_to_root(manifest_path: &Path, root: &Path) -> Option<String> {
    if let Ok(rel) = manifest_path.strip_prefix(root) {
        return Some(relative_path_string(rel));
    }

    let manifest_canonical = manifest_path.canonicalize().ok()?;
    let root_canonical = root.canonicalize().ok()?;
    manifest_canonical
        .strip_prefix(root_canonical)
        .ok()
        .map(relative_path_string)
}

fn read_batch_inputs(args: &BatchArgs) -> HashResult<Vec<BatchInput>> {
    let read_from_stdin = args
        .input
        .as_ref()
        .map(|path| path == Path::new("-"))
        .unwrap_or(args.input.is_none());

    if read_from_stdin {
        let stdin = io::stdin();
        let handle = stdin.lock();
        read_batch_inputs_from_reader(handle, args.input_format)
    } else {
        let path = args.input.as_ref().expect("input path present");
        let file = File::open(path)?;
        read_batch_inputs_from_reader(file, args.input_format)
    }
}

fn read_batch_inputs_from_reader<R: Read>(
    reader: R,
    format: BatchFormatArg,
) -> HashResult<Vec<BatchInput>> {
    match format {
        BatchFormatArg::Json => {
            serde_json::from_reader(reader).map_err(|err| HashError::ManifestParse(err.to_string()))
        }
        BatchFormatArg::Csv => {
            let mut rdr = csv::ReaderBuilder::new()
                .trim(csv::Trim::All)
                .from_reader(reader);

            let mut inputs = Vec::new();
            for row in rdr.deserialize::<CsvBatchRow>() {
                let record: CsvBatchRow =
                    row.map_err(|err| HashError::ManifestParse(err.to_string()))?;
                inputs.push(BatchInput {
                    path: PathBuf::from(record.path),
                    expected: record.expected,
                    algorithm: record.algorithm,
                });
            }
            Ok(inputs)
        }
    }
}

#[derive(Debug, Deserialize)]
struct CsvBatchRow {
    path: String,
    expected: String,
    #[serde(default)]
    algorithm: Option<String>,
}

fn write_batch_report(report: &BatchReport, args: &BatchArgs) -> HashResult<()> {
    let mut write_target: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    match args.output_format {
        BatchFormatArg::Json => {
            serde_json::to_writer_pretty(&mut write_target, report)
                .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
            if args.output.is_none() {
                write_target.write_all(b"\n")?;
            }
        }
        BatchFormatArg::Csv => {
            write_batch_report_csv(report, &mut *write_target)?;
        }
    }
    Ok(())
}

fn write_batch_report_csv(report: &BatchReport, writer: &mut dyn Write) -> HashResult<()> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(&mut *writer);

    wtr.write_record(["path", "status", "expected", "actual", "algorithm", "error"])
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;

    for entry in &report.entries {
        let status = match entry.status {
            BatchStatus::Match => "match",
            BatchStatus::Mismatch => "mismatch",
            BatchStatus::Missing => "missing",
            BatchStatus::Error => "error",
        };
        wtr.write_record([
            entry.path.as_str(),
            status,
            entry.expected.as_str(),
            entry.actual.as_deref().unwrap_or(""),
            entry.algorithm.as_deref().unwrap_or(""),
            entry.error.as_deref().unwrap_or(""),
        ])
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
    }

    wtr.flush()
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
    drop(wtr);
    writer.flush()?;
    Ok(())
}
