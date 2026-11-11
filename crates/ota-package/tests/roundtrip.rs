//! End-to-end tests for the `.otapkg` container.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ota_core::Version;
use ota_package::{
    append_signature, Error, Package, PackageBuilder, Quantization, SignatureAlgorithm,
};

fn payload_of(len: u32) -> Vec<u8> {
    (0..len).map(|i| u8::try_from(i % 251).unwrap()).collect()
}

fn build(payload: &[u8], chunk_size: u32) -> Vec<u8> {
    PackageBuilder::new(
        "vision-v3",
        Version::new(1, 2, 3),
        "stm32h7",
        Quantization::Int8,
    )
    .with_chunk_size(chunk_size)
    .with_payload(payload.to_vec())
    .build_unsigned()
    .expect("build")
}

#[test]
fn round_trips_manifest_and_payload() {
    let payload = payload_of(1000);
    let bytes = build(&payload, 4096);
    let package = Package::parse(&bytes).expect("parse");

    assert_eq!(package.manifest().model_name, "vision-v3");
    assert_eq!(package.manifest().model_version, Version::new(1, 2, 3));
    assert_eq!(package.manifest().target, "stm32h7");
    assert_eq!(package.manifest().quantization, Quantization::Int8);
    assert_eq!(package.manifest().payload_len, 1000);
    assert_eq!(package.manifest().chunk_count, 1);
    assert!(!package.is_signed());
    assert_eq!(package.payload(), payload);
    package.verify_payload().expect("payload ok");
}

#[test]
fn splits_payload_into_chunks() {
    let payload = payload_of(1000);
    let bytes = build(&payload, 256);
    let package = Package::parse(&bytes).expect("parse");

    assert_eq!(package.manifest().chunk_size, 256);
    assert_eq!(package.manifest().chunk_count, 4);
    assert_eq!(package.chunks().len(), 4);
    assert_eq!(package.chunks()[0].data.len(), 256);
    assert_eq!(package.chunks()[3].data.len(), 232);
    for (position, chunk) in package.chunks().iter().enumerate() {
        assert_eq!(chunk.index, u32::try_from(position).unwrap());
        chunk.verify().expect("chunk crc");
    }
    assert_eq!(package.payload(), payload);
}

#[test]
fn handles_empty_payload() {
    let bytes = build(&[], 1024);
    let package = Package::parse(&bytes).expect("parse");
    assert_eq!(package.manifest().chunk_count, 0);
    assert!(package.chunks().is_empty());
    package.verify_payload().expect("empty payload ok");
}

#[test]
fn preserves_prerelease_version() {
    let bytes = PackageBuilder::new(
        "m",
        Version::with_pre(0, 4, 0, "rc.1"),
        "t",
        Quantization::Fp16,
    )
    .with_payload(b"x".to_vec())
    .build_unsigned()
    .expect("build");
    let package = Package::parse(&bytes).expect("parse");
    assert_eq!(package.manifest().model_version.to_string(), "0.4.0-rc.1");
}

#[test]
fn rejects_bad_magic() {
    let mut bytes = build(b"hello", 1024);
    bytes[0] ^= 0xFF;
    assert_eq!(Package::parse(&bytes).unwrap_err(), Error::BadMagic);
}

#[test]
fn rejects_truncated_input() {
    let bytes = build(b"hello", 1024);
    assert_eq!(
        Package::parse(&bytes[..bytes.len() - 1]).unwrap_err(),
        Error::Truncated
    );
}

#[test]
fn rejects_tampered_header() {
    let mut bytes = build(b"hello", 1024);
    bytes[16] ^= 0x01;
    assert!(matches!(
        Package::parse(&bytes).unwrap_err(),
        Error::HeaderCrcMismatch { .. }
    ));
}

#[test]
fn detects_payload_corruption() {
    let bytes = build(b"hello world", 1024);
    let mut corrupted = bytes;
    let last = corrupted.len() - 1;
    corrupted[last] ^= 0xFF;
    let package = Package::parse(&corrupted).expect("framing still valid");
    assert!(matches!(
        package.verify_payload().unwrap_err(),
        Error::ChunkCrcMismatch { .. } | Error::PayloadHashMismatch
    ));
}

#[test]
fn appending_signature_keeps_digest_stable() {
    let unsigned = build(b"model weights", 1024);
    let digest_before = Package::parse(&unsigned).expect("parse").signing_digest();

    let signature = [0xAB_u8; 64];
    let signed =
        append_signature(&unsigned, SignatureAlgorithm::Ed25519, &signature).expect("sign");

    let package = Package::parse(&signed).expect("parse signed");
    assert!(package.is_signed());
    let sig = package.signature().expect("signature present");
    assert_eq!(sig.algorithm, SignatureAlgorithm::Ed25519);
    assert_eq!(sig.bytes, &signature);
    assert_eq!(package.signing_digest(), digest_before);
    assert_eq!(package.signed_region(), &unsigned[..]);
    package.verify_payload().expect("payload ok");
}

#[test]
fn refuses_double_signature() {
    let unsigned = build(b"model", 1024);
    let signed =
        append_signature(&unsigned, SignatureAlgorithm::Ed25519, &[0_u8; 64]).expect("sign");
    assert_eq!(
        append_signature(&signed, SignatureAlgorithm::Ed25519, &[0_u8; 64]).unwrap_err(),
        Error::AlreadySigned
    );
}

#[test]
fn quantization_codes_round_trip() {
    for quantization in [
        Quantization::None,
        Quantization::Int8,
        Quantization::Int4,
        Quantization::Fp16,
        Quantization::Fp32,
        Quantization::Mixed,
    ] {
        assert_eq!(
            Quantization::from_code(quantization.code()).unwrap(),
            quantization
        );
    }
    assert!(Quantization::from_code(999).is_err());
}
