//! `ota package` — build and inspect `.otapkg` files.

use std::str::FromStr;

use anyhow::{Context, Result};
use ota_core::Version;
use ota_package::{Package, PackageBuilder};

use crate::cli::{BuildArgs, InspectArgs, PackageArgs, PackageCommand};
use crate::io;

/// Dispatch an `ota package` invocation.
///
/// # Errors
///
/// Propagates any error raised by the selected operation.
pub fn run(args: PackageArgs) -> Result<()> {
    match args.command {
        PackageCommand::Build(build_args) => build(&build_args),
        PackageCommand::Inspect(inspect_args) => inspect(&inspect_args),
    }
}

/// Assemble an unsigned package from a payload file.
fn build(args: &BuildArgs) -> Result<()> {
    let version = Version::from_str(&args.version)
        .with_context(|| format!("invalid version {:?}", args.version))?;
    let payload = io::read(&args.input)?;

    let bytes = PackageBuilder::new(
        args.model.as_str(),
        version,
        args.target.as_str(),
        args.quantization.into(),
    )
    .with_chunk_size(args.chunk_size)
    .with_payload(payload)
    .build_unsigned()
    .context("failed to build package")?;

    io::write(&args.output, &bytes)?;
    println!(
        "wrote {} ({} bytes, unsigned)",
        args.output.display(),
        bytes.len()
    );
    Ok(())
}

/// Parse a package, verify its payload and print its manifest.
fn inspect(args: &InspectArgs) -> Result<()> {
    let bytes = io::read(&args.input)?;
    let package = Package::parse(&bytes)
        .with_context(|| format!("{} is not a valid .otapkg", args.input.display()))?;
    package
        .verify_payload()
        .context("payload verification failed")?;

    let manifest = package.manifest();
    println!("file:           {}", args.input.display());
    println!("model:          {}", manifest.model_name);
    println!("version:        {}", manifest.model_version);
    println!("target:         {}", manifest.target);
    println!("quantization:   {}", manifest.quantization.as_str());
    println!("payload_len:    {} bytes", manifest.payload_len);
    println!("chunk_size:     {} bytes", manifest.chunk_size);
    println!("chunk_count:    {}", manifest.chunk_count);
    println!(
        "payload_sha256: {}",
        ota_crypto::hex_encode(&manifest.payload_sha256)
    );
    match package.signature() {
        Some(signature) => println!(
            "signature:      {} ({} bytes)",
            signature.algorithm.as_str(),
            signature.bytes.len()
        ),
        None => println!("signature:      none (unsigned)"),
    }
    println!("payload:        ok");
    Ok(())
}
