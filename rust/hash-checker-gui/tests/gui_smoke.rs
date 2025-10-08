use assert_cmd::Command;

#[test]
fn gui_smoke_test_runs() {
    let mut cmd = Command::cargo_bin("hash-checker-gui").expect("binary available");
    cmd.arg("--smoke-test")
        .env("RUST_BACKTRACE", "1")
        .assert()
        .success();
}
