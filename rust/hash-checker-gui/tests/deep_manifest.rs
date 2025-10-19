use hash_checker::{
    generate_manifest, read_manifest, verify_manifest, write_manifest, Manifest, ManifestFormat,
    VerificationReport,
};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_file(path: &Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, data).expect("write file");
}

fn build_sample_manifest(root: &Path) -> Manifest {
    write_file(&root.join("alpha/file-a.bin"), b"initial-aaaaaaaaaaaa");
    write_file(
        &root.join("alpha/file-b.txt"),
        b"The quick brown fox jumps over the lazy dog.",
    );
    write_file(&root.join("bravo.log"), b"log-line-1\nlog-line-2\n");
    write_file(
        &root.join("charlie/delta/nested.json"),
        br#"{"message":"nested"}"#,
    );
    write_file(&root.join("emoji-\u{1F600}.dat"), b":)");

    generate_manifest(root, "sha256", true).expect("generate manifest")
}

fn disturb_tree(root: &Path) {
    // mutate an existing entry
    write_file(&root.join("alpha/file-a.bin"), b"mutated-content");
    // remove one entry
    fs::remove_file(root.join("charlie/delta/nested.json")).expect("remove nested file");
    // add a new extra file
    write_file(&root.join("rogue.tmp"), b"unexpected");
}

fn assert_verification(report: &VerificationReport) {
    assert_eq!(report.matched, 3);
    assert_eq!(report.mismatched.len(), 1);
    assert_eq!(report.missing.len(), 1);
    assert_eq!(report.extra.len(), 1);
}

#[test]
fn deep_tree_manifest_detects_mismatch_missing_and_extra() {
    let temp = tempdir().expect("temp dir");
    let root = temp.path().join("workspace");
    fs::create_dir_all(&root).expect("create root dir");

    let manifest = build_sample_manifest(&root);
    assert_eq!(manifest.entries.len(), 5, "expected five recorded entries");

    disturb_tree(&root);

    let report = verify_manifest(&manifest, &root).expect("verify manifest");
    assert_verification(&report);
}

#[test]
fn manifest_round_trip_write_and_verify() {
    let temp = tempdir().expect("temp dir");
    let root = temp.path().join("workspace");
    fs::create_dir_all(&root).expect("create root dir");

    let manifest = build_sample_manifest(&root);

    let report_path = temp.path().join("reports/manifest.json");
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).expect("create report dir");
    }
    let mut buffer = Vec::new();
    write_manifest(&manifest, ManifestFormat::Json, &mut buffer).expect("serialize manifest");
    fs::write(&report_path, &buffer).expect("persist manifest file");

    let read_manifest = read_manifest(buffer.as_slice(), ManifestFormat::Json).expect("read");
    assert_eq!(read_manifest.entries.len(), manifest.entries.len());

    disturb_tree(&root);
    let report = verify_manifest(&read_manifest, &root).expect("verify");
    assert_verification(&report);
}
