use crate::{compute_hash_with_options, HashError, HashResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

const MANIFEST_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    pub version: String,
    pub algorithm: String,
    pub generated_at: u64,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub root: Option<String>,
    pub entries: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestEntry {
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub modified: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestFormat {
    Json,
    Csv,
    Plain,
}

#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub matched: usize,
    pub mismatched: Vec<Mismatch>,
    pub missing: Vec<ManifestEntry>,
    pub extra: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Mismatch {
    pub entry: ManifestEntry,
    pub actual: String,
}

impl VerificationReport {
    pub fn has_failures(&self) -> bool {
        !self.mismatched.is_empty() || !self.missing.is_empty() || !self.extra.is_empty()
    }

    pub fn total_entries(&self) -> usize {
        self.matched + self.mismatched.len() + self.missing.len()
    }
}

pub fn detect_format_from_extension(path: &Path) -> Option<ManifestFormat> {
    match path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
    {
        Some(ext) if ext == "json" || ext == "manifest.json" => Some(ManifestFormat::Json),
        Some(ext) if ext == "csv" => Some(ManifestFormat::Csv),
        Some(ext) if ext == "txt" || ext == "mf" => Some(ManifestFormat::Plain),
        _ => None,
    }
}

pub fn generate_manifest(root: &Path, algorithm: &str, recursive: bool) -> HashResult<Manifest> {
    generate_manifest_with_options(root, algorithm, recursive, None, None)
}

pub fn generate_manifest_with_options(
    root: &Path,
    algorithm: &str,
    recursive: bool,
    cancel: Option<&AtomicBool>,
    mut on_progress: Option<&mut dyn FnMut(usize, usize)>,
) -> HashResult<Manifest> {
    if !root.exists() {
        return Err(HashError::PathNotFound(root.display().to_string()));
    }
    if !root.is_dir() {
        return Err(HashError::NotADirectory(root.display().to_string()));
    }
    if cancel.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
        return Err(HashError::Cancelled);
    }

    let files = collect_files(root, recursive)?;
    let total_files = files.len();
    let mut entries = Vec::with_capacity(total_files);
    for (idx, file) in files.into_iter().enumerate() {
        if cancel.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
            return Err(HashError::Cancelled);
        }
        let rel = file
            .strip_prefix(root)
            .map_err(|_| HashError::Canonicalize {
                path: file.display().to_string(),
                source: std::io::Error::other("path prefix error"),
            })?;
        let hash = compute_hash_with_options(&file, algorithm, cancel, None)?;
        let metadata = fs::metadata(&file)?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|ts| ts.duration_since(UNIX_EPOCH).ok())
            .map(|dur| dur.as_secs());
        let path_str = to_manifest_path(rel);
        entries.push(ManifestEntry {
            path: path_str,
            hash,
            size: metadata.len(),
            modified,
        });
        if let Some(cb) = on_progress.as_deref_mut() {
            cb(idx + 1, total_files);
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));

    let generated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(Manifest {
        version: MANIFEST_VERSION.to_string(),
        algorithm: algorithm.to_ascii_lowercase(),
        generated_at,
        recursive,
        root: Some(root.to_string_lossy().to_string()),
        entries,
    })
}

pub fn write_manifest<W: Write>(
    manifest: &Manifest,
    format: ManifestFormat,
    writer: W,
) -> HashResult<()> {
    match format {
        ManifestFormat::Json => write_manifest_json(manifest, writer),
        ManifestFormat::Csv => write_manifest_csv(manifest, writer),
        ManifestFormat::Plain => write_manifest_plain(manifest, writer),
    }
}

pub fn read_manifest<R: Read>(reader: R, format: ManifestFormat) -> HashResult<Manifest> {
    match format {
        ManifestFormat::Json => read_manifest_json(reader),
        ManifestFormat::Csv => read_manifest_csv(reader),
        ManifestFormat::Plain => read_manifest_plain(reader),
    }
}

pub fn verify_manifest(manifest: &Manifest, root: &Path) -> HashResult<VerificationReport> {
    verify_manifest_with_options(manifest, root, None, None)
}

pub fn verify_manifest_with_options(
    manifest: &Manifest,
    root: &Path,
    cancel: Option<&AtomicBool>,
    mut on_progress: Option<&mut dyn FnMut(usize, usize)>,
) -> HashResult<VerificationReport> {
    if !root.exists() {
        return Err(HashError::PathNotFound(root.display().to_string()));
    }
    if !root.is_dir() {
        return Err(HashError::NotADirectory(root.display().to_string()));
    }
    if cancel.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
        return Err(HashError::Cancelled);
    }

    let recursive = manifest.recursive;
    let files = collect_files(root, recursive)?;

    let mut actual_map: BTreeMap<String, PathBuf> = BTreeMap::new();
    for file in files {
        let rel = file
            .strip_prefix(root)
            .map_err(|_| HashError::Canonicalize {
                path: file.display().to_string(),
                source: std::io::Error::other("path prefix error"),
            })?;
        let key = to_manifest_path(rel);
        actual_map.insert(key, file);
    }

    let mut matched = 0usize;
    let mut mismatched = Vec::new();
    let mut missing = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let total_entries = manifest.entries.len();

    for (idx, entry) in manifest.entries.iter().enumerate() {
        if cancel.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
            return Err(HashError::Cancelled);
        }
        let normalized_path = normalize_manifest_rel_path(&entry.path)?;
        let mut normalized_entry = entry.clone();
        normalized_entry.path = normalized_path.clone();

        if let Some(path) = actual_map.get(&normalized_path) {
            let computed = compute_hash_with_options(path, &manifest.algorithm, cancel, None)?;
            if computed == entry.hash {
                matched += 1;
            } else {
                mismatched.push(Mismatch {
                    entry: normalized_entry,
                    actual: computed,
                });
            }
            seen.insert(normalized_path);
        } else {
            missing.push(normalized_entry);
        }
        if let Some(cb) = on_progress.as_deref_mut() {
            cb(idx + 1, total_entries);
        }
    }

    let extra: Vec<String> = actual_map
        .keys()
        .filter(|path| !seen.contains(*path))
        .cloned()
        .collect();

    Ok(VerificationReport {
        matched,
        mismatched,
        missing,
        extra,
    })
}

fn collect_files(root: &Path, recursive: bool) -> HashResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    if recursive {
        for entry in WalkDir::new(root).follow_links(false) {
            let entry = entry.map_err(|err| {
                if let Some(io) = err.io_error() {
                    HashError::Io(std::io::Error::new(io.kind(), io.to_string()))
                } else {
                    HashError::ManifestParse(err.to_string())
                }
            })?;
            if entry.file_type().is_file() {
                files.push(entry.into_path());
            }
        }
    } else {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn to_manifest_path(path: &Path) -> String {
    let count = path.components().count();
    let mut vec = Vec::with_capacity(count);
    for component in path.components() {
        vec.push(component.as_os_str().to_string_lossy());
    }
    vec.join("/")
}

fn from_manifest_path(path: &str) -> PathBuf {
    PathBuf::from_iter(path.split('/'))
}

pub fn relative_path_string(path: &Path) -> String {
    to_manifest_path(path)
}

fn write_manifest_json<W: Write>(manifest: &Manifest, writer: W) -> HashResult<()> {
    serde_json::to_writer_pretty(writer, manifest)
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))
}

fn read_manifest_json<R: Read>(reader: R) -> HashResult<Manifest> {
    let mut manifest: Manifest =
        serde_json::from_reader(reader).map_err(|err| HashError::ManifestParse(err.to_string()))?;
    ensure_manifest_supported(&mut manifest)?;
    Ok(manifest)
}

fn write_manifest_csv<W: Write>(manifest: &Manifest, mut writer: W) -> HashResult<()> {
    writeln!(&mut writer, "# manifest_version={}", manifest.version)?;
    writeln!(&mut writer, "# algorithm={}", manifest.algorithm)?;
    writeln!(&mut writer, "# recursive={}", manifest.recursive)?;
    if let Some(root) = &manifest.root {
        writeln!(&mut writer, "# root={root}")?;
    }

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(writer);
    csv_writer
        .write_record(["path", "hash", "size", "modified"])
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
    for entry in &manifest.entries {
        csv_writer
            .write_record([
                entry.path.as_str(),
                entry.hash.as_str(),
                entry.size.to_string().as_str(),
                entry
                    .modified
                    .map(|v| v.to_string())
                    .unwrap_or_default()
                    .as_str(),
            ])
            .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
    }
    csv_writer
        .flush()
        .map_err(|err| HashError::ManifestSerialize(err.to_string()))?;
    Ok(())
}

fn read_manifest_csv<R: Read>(reader: R) -> HashResult<Manifest> {
    let mut content = String::new();
    let mut buf_reader = std::io::BufReader::new(reader);
    buf_reader.read_to_string(&mut content)?;

    let mut version = MANIFEST_VERSION.to_string();
    let mut algorithm = None;
    let mut recursive = false;
    let mut root = None;
    let mut data_lines = String::new();

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix('#') {
            let parts: Vec<&str> = stripped.trim().splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0].trim() {
                    "manifest_version" => version = parts[1].trim().to_string(),
                    "algorithm" => algorithm = Some(parts[1].trim().to_ascii_lowercase()),
                    "recursive" => {
                        recursive = parts[1].trim().eq_ignore_ascii_case("true");
                    }
                    "root" => root = Some(parts[1].trim().to_string()),
                    _ => {}
                }
            }
        } else {
            data_lines.push_str(line);
            data_lines.push('\n');
        }
    }

    let algo = algorithm.ok_or_else(|| {
        HashError::ManifestParse("missing algorithm metadata in CSV manifest".to_string())
    })?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data_lines.as_bytes());

    let mut entries = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|err| HashError::ManifestParse(err.to_string()))?;
        let path = record
            .get(0)
            .ok_or_else(|| HashError::ManifestParse("missing path column".to_string()))?
            .to_string();
        let hash = record
            .get(1)
            .ok_or_else(|| HashError::ManifestParse("missing hash column".to_string()))?
            .to_string();
        let size = record
            .get(2)
            .unwrap_or("0")
            .parse::<u64>()
            .map_err(|_| HashError::ManifestParse("invalid size value".to_string()))?;
        let modified = record.get(3).and_then(|value| {
            if value.trim().is_empty() {
                None
            } else {
                value.trim().parse::<u64>().ok()
            }
        });
        entries.push(ManifestEntry {
            path,
            hash,
            size,
            modified,
        });
    }

    let mut manifest = Manifest {
        version,
        algorithm: algo,
        generated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        recursive,
        root,
        entries,
    };
    ensure_manifest_supported(&mut manifest)?;
    Ok(manifest)
}

fn write_manifest_plain<W: Write>(manifest: &Manifest, mut writer: W) -> HashResult<()> {
    writeln!(&mut writer, "# manifest_version={}", manifest.version)?;
    writeln!(&mut writer, "# algorithm={}", manifest.algorithm)?;
    writeln!(&mut writer, "# recursive={}", manifest.recursive)?;
    if let Some(root) = &manifest.root {
        writeln!(&mut writer, "# root={root}")?;
    }
    for entry in &manifest.entries {
        writeln!(
            &mut writer,
            "{}  {}  {}  {}",
            entry.hash,
            entry.path,
            entry.size,
            entry
                .modified
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string())
        )?;
    }
    Ok(())
}

fn read_manifest_plain<R: Read>(reader: R) -> HashResult<Manifest> {
    let mut version = MANIFEST_VERSION.to_string();
    let mut algorithm = None;
    let mut recursive = false;
    let mut root = None;
    let mut entries = Vec::new();

    let buf_reader = std::io::BufReader::new(reader);
    for line in buf_reader.lines() {
        let line = line?;
        if let Some(stripped) = line.strip_prefix('#') {
            let parts: Vec<&str> = stripped.trim().splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0].trim() {
                    "manifest_version" => version = parts[1].trim().to_string(),
                    "algorithm" => algorithm = Some(parts[1].trim().to_ascii_lowercase()),
                    "recursive" => {
                        recursive = parts[1].trim().eq_ignore_ascii_case("true");
                    }
                    "root" => root = Some(parts[1].trim().to_string()),
                    _ => {}
                }
            }
            continue;
        }

        let mut segments = line.split("  ");
        let hash = segments
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                HashError::ManifestParse("invalid hash column in plain manifest".to_string())
            })?
            .to_string();
        let path = segments
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                HashError::ManifestParse("invalid path column in plain manifest".to_string())
            })?
            .to_string();
        let size = if let Some(value) = segments.next() {
            if value.trim().is_empty() {
                0
            } else {
                value
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| HashError::ManifestParse("invalid size column".to_string()))?
            }
        } else {
            0
        };
        let modified =
            if let Some(value) = segments.next() {
                let value = value.trim();
                if value.is_empty() || value == "-" {
                    None
                } else {
                    Some(value.parse::<u64>().map_err(|_| {
                        HashError::ManifestParse("invalid modified column".to_string())
                    })?)
                }
            } else {
                None
            };

        if segments.next().is_some() {
            return Err(HashError::ManifestParse(
                "invalid line in plain manifest".to_string(),
            ));
        }

        entries.push(ManifestEntry {
            path,
            hash,
            size,
            modified,
        });
    }

    let algo = algorithm.ok_or_else(|| {
        HashError::ManifestParse("missing algorithm metadata in plain manifest".to_string())
    })?;

    let mut manifest = Manifest {
        version,
        algorithm: algo,
        generated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        recursive,
        root,
        entries,
    };
    ensure_manifest_supported(&mut manifest)?;
    Ok(manifest)
}

fn normalize_manifest_rel_path(raw: &str) -> HashResult<String> {
    let normalized = raw.trim().replace('\\', "/");
    if normalized.is_empty() {
        return Err(HashError::ManifestParse(
            "manifest entry path cannot be empty".to_string(),
        ));
    }
    if normalized.starts_with('/') || normalized.contains(':') {
        return Err(HashError::ManifestParse(format!(
            "manifest entry path '{raw}' must be relative"
        )));
    }
    let mut segments = Vec::new();
    for seg in normalized.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            return Err(HashError::ManifestParse(format!(
                "path traversal '..' is not allowed in manifest entry '{raw}'"
            )));
        }
        segments.push(seg);
    }
    if segments.is_empty() {
        return Err(HashError::ManifestParse(format!(
            "invalid manifest entry path '{raw}'"
        )));
    }
    Ok(segments.join("/"))
}

fn ensure_manifest_supported(manifest: &mut Manifest) -> HashResult<()> {
    if manifest.version != MANIFEST_VERSION {
        return Err(HashError::ManifestVersion {
            expected: MANIFEST_VERSION,
            found: manifest.version.clone(),
        });
    }
    for entry in &mut manifest.entries {
        entry.path = normalize_manifest_rel_path(&entry.path)?;
    }
    Ok(())
}

pub fn resolve_root<'a>(
    manifest: &'a Manifest,
    explicit: Option<&'a Path>,
    manifest_path: &'a Path,
) -> PathBuf {
    if let Some(path) = explicit {
        return path.to_path_buf();
    }
    if let Some(root) = &manifest.root {
        let candidate = PathBuf::from(root);
        if candidate.exists() && candidate.is_dir() {
            return candidate;
        }
    }
    manifest_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn apply_entry_path(root: &Path, relative: &str) -> PathBuf {
    let rel = from_manifest_path(relative);
    root.join(rel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_files() -> (TempDir, PathBuf, PathBuf) {
        let dir = TempDir::new().unwrap();
        let file_a = dir.path().join("a.txt");
        let file_b = dir.path().join("nested").join("b.txt");
        fs::create_dir_all(file_b.parent().unwrap()).unwrap();
        fs::write(&file_a, b"hello world").unwrap();
        fs::write(&file_b, b"goodbye world").unwrap();
        (dir, file_a, file_b)
    }

    #[test]
    fn generate_and_verify_manifest_recursive() {
        let (dir, _, _) = setup_files();
        let manifest = generate_manifest(dir.path(), "sha256", true).expect("manifest");
        assert_eq!(manifest.entries.len(), 2);
        let report = verify_manifest(&manifest, dir.path()).expect("verify");
        assert!(!report.has_failures());
        assert_eq!(report.matched, 2);
    }

    #[test]
    fn detect_extra_and_missing_files() {
        let (dir, file_a, file_b) = setup_files();
        let manifest = generate_manifest(dir.path(), "sha256", true).expect("manifest");
        // remove one file and alter the other
        fs::remove_file(file_b).unwrap();
        fs::write(file_a, b"mutated").unwrap();
        let report = verify_manifest(&manifest, dir.path()).expect("report");
        assert!(report.has_failures());
        assert_eq!(report.missing.len(), 1);
        assert_eq!(report.mismatched.len(), 1);
    }

    #[test]
    fn json_round_trip() {
        let (dir, _, _) = setup_files();
        let manifest = generate_manifest(dir.path(), "sha256", true).expect("manifest");
        let mut buffer = Vec::new();
        write_manifest(&manifest, ManifestFormat::Json, &mut buffer).expect("write");
        let parsed = read_manifest(buffer.as_slice(), ManifestFormat::Json).expect("read");
        assert_eq!(manifest.entries.len(), parsed.entries.len());
        assert_eq!(manifest.version, parsed.version);
    }

    #[test]
    fn manifest_normalizes_backslashes_and_rejects_traversal() {
        let (dir, _, _) = setup_files();
        let mut manifest = generate_manifest(dir.path(), "sha256", true).expect("manifest");
        // Simulate Windows backslash path in manifest entry
        for entry in &mut manifest.entries {
            entry.path = entry.path.replace('/', "\\");
        }
        let mut buffer = Vec::new();
        write_manifest(&manifest, ManifestFormat::Json, &mut buffer).expect("write");
        let parsed = read_manifest(buffer.as_slice(), ManifestFormat::Json).expect("read");
        assert!(parsed.entries.iter().all(|e| !e.path.contains('\\')));
        let report = verify_manifest(&parsed, dir.path()).expect("verify normalized");
        assert!(!report.has_failures());
        assert_eq!(report.matched, 2);

        // Reject parent directory traversal in untrusted manifest
        let mut malicious = parsed;
        malicious.entries[0].path = "../secret.txt".to_string();
        let mut bad_buf = Vec::new();
        write_manifest(&malicious, ManifestFormat::Json, &mut bad_buf).expect("write malicious");
        let err = read_manifest(bad_buf.as_slice(), ManifestFormat::Json)
            .expect_err("expected path traversal rejection");
        assert!(matches!(err, HashError::ManifestParse(_)));
    }

    #[test]
    fn apply_entry_path_resolves_correctly() {
        let root = Path::new("/base/dir");
        assert_eq!(apply_entry_path(root, "file.txt"), root.join("file.txt"));
        assert_eq!(
            apply_entry_path(root, "subdir/file.txt"),
            root.join("subdir").join("file.txt")
        );
        assert_eq!(
            apply_entry_path(root, "a/b/c/d.txt"),
            root.join("a").join("b").join("c").join("d.txt")
        );
    }

    #[test]
    fn test_detect_format_from_extension() {
        assert_eq!(
            detect_format_from_extension(Path::new("file.json")),
            Some(ManifestFormat::Json)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("file.JSON")),
            Some(ManifestFormat::Json)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("file.manifest.json")),
            Some(ManifestFormat::Json)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("data.csv")),
            Some(ManifestFormat::Csv)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("DATA.CSV")),
            Some(ManifestFormat::Csv)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("hashes.txt")),
            Some(ManifestFormat::Plain)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("hashes.TXT")),
            Some(ManifestFormat::Plain)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("manifest.mf")),
            Some(ManifestFormat::Plain)
        );
        assert_eq!(
            detect_format_from_extension(Path::new("MANIFEST.MF")),
            Some(ManifestFormat::Plain)
        );
        assert_eq!(detect_format_from_extension(Path::new("file.xml")), None);
        assert_eq!(detect_format_from_extension(Path::new("file.bin")), None);
        assert_eq!(detect_format_from_extension(Path::new("file")), None);
        assert_eq!(detect_format_from_extension(Path::new("")), None);
        assert_eq!(detect_format_from_extension(Path::new(".hidden")), None);
    }
}
