//! Golden-fixture round-trip tests for the `.otapkg` v1 format.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::PathBuf;

use ota_package::Package;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join("v1")
        .join(name)
}

#[test]
fn minimal_fixture_parses() {
    let bytes = fs::read(fixture("minimal.otapkg")).expect("read minimal");
    let package = Package::parse(&bytes).expect("parse minimal");
    assert_eq!(package.manifest().model_name, "minimal");
    assert_eq!(package.manifest().model_version.to_string(), "0.1.0");
    assert_eq!(package.manifest().payload_len, 0);
    assert_eq!(package.manifest().chunk_count, 0);
    assert!(!package.is_signed());
    assert!(package.verify_payload().is_ok());
    assert!(package.payload().is_empty());
}

#[test]
fn small_fixture_parses_and_verifies() {
    let bytes = fs::read(fixture("small.otapkg")).expect("read small");
    let package = Package::parse(&bytes).expect("parse small");
    assert_eq!(package.manifest().model_name, "small");
    assert_eq!(package.manifest().model_version.to_string(), "1.0.0");
    assert_eq!(package.manifest().payload_len, 1024);
    assert_eq!(package.manifest().chunk_count, 2);
    assert!(!package.is_signed());
    assert!(package.verify_payload().is_ok());
    let payload = package.payload();
    let expected = fs::read(fixture("small.bin.payload")).expect("read payload");
    assert_eq!(payload, expected);
}

#[test]
fn signed_fixture_parses_and_is_signed() {
    let bytes = fs::read(fixture("signed.otapkg")).expect("read signed");
    let package = Package::parse(&bytes).expect("parse signed");
    assert_eq!(package.manifest().model_name, "small");
    assert!(package.is_signed());
    assert!(package.verify_payload().is_ok());
}
