//! Scale / stress test for the `.otapkg` parser (issue #18).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Instant;

use ota_core::Version;
use ota_package::{Package, PackageBuilder, Quantization};

#[test]
fn parses_a_package_with_ten_thousand_chunks() {
    // ~1 MB payload split into 10_000 chunks.
    let payload = vec![0x37; 1_000_000];
    let bytes = PackageBuilder::new(
        "scale",
        Version::new(1, 0, 0),
        "generic",
        Quantization::Int8,
    )
    .with_chunk_size(100)
    .with_payload(payload.clone())
    .build_unsigned()
    .expect("build");

    let start = Instant::now();
    let package = Package::parse(&bytes).expect("parse");
    let parse_elapsed = start.elapsed();

    assert_eq!(package.manifest().chunk_count, 10_000);
    assert_eq!(package.chunks().len(), 10_000);
    assert!(package.verify_payload().is_ok());

    // Sanity: should complete well within a few seconds on a dev machine.
    assert!(
        parse_elapsed.as_secs() < 10,
        "parse took too long: {parse_elapsed:?}"
    );
}
