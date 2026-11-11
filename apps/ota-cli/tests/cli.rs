//! End-to-end tests for the `ota` binary.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Path to the compiled `ota` binary under test.
const BIN: &str = env!("CARGO_BIN_EXE_ota");

/// A scratch directory that is removed when the guard is dropped.
struct TempDir(PathBuf);

impl TempDir {
    /// Create a unique scratch directory for `label`.
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!("ota-cli-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    /// A path inside the scratch directory.
    fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run the binary with `args` and return its output.
fn ota(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("run ota binary")
}

/// Run the binary, asserting it succeeded, and return stdout.
fn ota_ok(args: &[&str]) -> String {
    let output = ota(args);
    assert!(
        output.status.success(),
        "expected success for {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    String::from_utf8(output.stdout).expect("utf-8 stdout")
}

/// Build an unsigned package for `payload` and return its path.
fn build_package(dir: &TempDir, payload: &Path) -> PathBuf {
    let output = dir.join("model.otapkg");
    ota_ok(&[
        "package",
        "build",
        "--model",
        "vision-v3",
        "--version",
        "1.2.0",
        "--target",
        "stm32h7",
        "--quantization",
        "int8",
        "--input",
        payload.to_str().unwrap(),
        "--output",
        output.to_str().unwrap(),
    ]);
    output
}

#[test]
fn builds_and_inspects_a_package() {
    let dir = TempDir::new("build");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0xab; 4096]).unwrap();

    let package = build_package(&dir, &payload);
    let report = ota_ok(&["package", "inspect", package.to_str().unwrap()]);

    assert!(report.contains("model:          vision-v3"), "{report}");
    assert!(report.contains("version:        1.2.0"), "{report}");
    assert!(report.contains("target:         stm32h7"), "{report}");
    assert!(report.contains("payload_len:    4096 bytes"), "{report}");
    assert!(
        report.contains("signature:      none (unsigned)"),
        "{report}"
    );
    assert!(report.contains("payload:        ok"), "{report}");
}

#[test]
fn signs_and_verifies_a_package() {
    let dir = TempDir::new("sign");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0x5a; 2048]).unwrap();
    let unsigned = build_package(&dir, &payload);

    let private = dir.join("signing.key");
    ota_ok(&["sign", "keygen", "--output", private.to_str().unwrap()]);
    let public = dir.join("signing.key.pub");
    assert!(private.exists());
    assert!(public.exists());

    let signed = dir.join("model.signed.otapkg");
    ota_ok(&[
        "sign",
        "package",
        "--key",
        private.to_str().unwrap(),
        "--input",
        unsigned.to_str().unwrap(),
        "--output",
        signed.to_str().unwrap(),
    ]);

    let report = ota_ok(&["package", "inspect", signed.to_str().unwrap()]);
    assert!(
        report.contains("signature:      ed25519 (64 bytes)"),
        "{report}"
    );

    let verified = ota_ok(&[
        "sign",
        "verify",
        "--key",
        public.to_str().unwrap(),
        signed.to_str().unwrap(),
    ]);
    assert!(verified.contains("signature ok"), "{verified}");
}

#[test]
fn rejects_a_signature_from_another_key() {
    let dir = TempDir::new("wrongkey");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0x11; 1024]).unwrap();
    let unsigned = build_package(&dir, &payload);

    let private = dir.join("signing.key");
    ota_ok(&["sign", "keygen", "--output", private.to_str().unwrap()]);
    let signed = dir.join("model.signed.otapkg");
    ota_ok(&[
        "sign",
        "package",
        "--key",
        private.to_str().unwrap(),
        "--input",
        unsigned.to_str().unwrap(),
        "--output",
        signed.to_str().unwrap(),
    ]);

    let other = dir.join("other.key");
    ota_ok(&["sign", "keygen", "--output", other.to_str().unwrap()]);
    let other_public = dir.join("other.key.pub");

    let output = ota(&[
        "sign",
        "verify",
        "--key",
        other_public.to_str().unwrap(),
        signed.to_str().unwrap(),
    ]);
    assert!(!output.status.success(), "verification should have failed");
}

#[test]
fn rejects_an_unsigned_package() {
    let dir = TempDir::new("unsigned");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0x00; 512]).unwrap();
    let unsigned = build_package(&dir, &payload);

    let private = dir.join("signing.key");
    ota_ok(&["sign", "keygen", "--output", private.to_str().unwrap()]);
    let public = dir.join("signing.key.pub");

    let output = ota(&[
        "sign",
        "verify",
        "--key",
        public.to_str().unwrap(),
        unsigned.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "unsigned package should not verify"
    );
}

#[test]
fn rejects_a_tampered_payload() {
    let dir = TempDir::new("tamper");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0x77; 4096]).unwrap();
    let package = build_package(&dir, &payload);

    let mut bytes = std::fs::read(&package).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    std::fs::write(&package, &bytes).unwrap();

    let output = ota(&["package", "inspect", package.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "tampered payload should not verify"
    );
}

#[test]
fn rejects_an_invalid_version() {
    let dir = TempDir::new("badversion");
    let payload = dir.join("model.bin");
    std::fs::write(&payload, vec![0x00; 16]).unwrap();
    let output_path = dir.join("model.otapkg");

    let output = ota(&[
        "package",
        "build",
        "--model",
        "m",
        "--version",
        "not-a-version",
        "--target",
        "t",
        "--input",
        payload.to_str().unwrap(),
        "--output",
        output_path.to_str().unwrap(),
    ]);
    assert!(!output.status.success(), "invalid version should fail");
}
