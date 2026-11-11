//! `ota model` — compare models and check target fit.

use anyhow::{Context, Result};
use ota_package::Package;

use crate::cli::{DiffArgs, FitArgs, ModelArgs, ModelCommand};
use crate::io;

/// Dispatch an `ota model` invocation.
///
/// # Errors
///
/// Propagates any error raised by the selected operation.
pub fn run(args: ModelArgs) -> Result<()> {
    match args.command {
        ModelCommand::Diff(diff_args) => diff(&diff_args),
        ModelCommand::Fit(fit_args) => fit(&fit_args),
    }
}

/// Compare two packages' manifests and payload hashes.
fn diff(args: &DiffArgs) -> Result<()> {
    let left_bytes = io::read(&args.left)?;
    let right_bytes = io::read(&args.right)?;
    let left = Package::parse(&left_bytes)
        .with_context(|| format!("{} is not a valid .otapkg", args.left.display()))?;
    let right = Package::parse(&right_bytes)
        .with_context(|| format!("{} is not a valid .otapkg", args.right.display()))?;

    let lm = left.manifest();
    let rm = right.manifest();

    println!("field            left                          right");
    println!("model_name       {:<30} {}", lm.model_name, rm.model_name);
    println!(
        "model_version    {:<30} {}",
        lm.model_version, rm.model_version
    );
    println!("target           {:<30} {}", lm.target, rm.target);
    println!(
        "quantization     {:<30} {}",
        lm.quantization.as_str(),
        rm.quantization.as_str()
    );
    println!("payload_len      {:<30} {}", lm.payload_len, rm.payload_len);
    println!("chunk_count      {:<30} {}", lm.chunk_count, rm.chunk_count);
    println!(
        "payload_sha256   {:<30} {}",
        hex_short(&lm.payload_sha256),
        hex_short(&rm.payload_sha256)
    );

    if lm == rm {
        println!("\nmanifests are identical");
    } else {
        println!("\nmanifests differ");
    }
    Ok(())
}

/// Check whether a package fits a target profile.
fn fit(args: &FitArgs) -> Result<()> {
    let bytes = io::read(&args.input)?;
    let package = Package::parse(&bytes)
        .with_context(|| format!("{} is not a valid .otapkg", args.input.display()))?;
    let manifest = package.manifest();

    let package_size = bytes.len() as u64;
    let payload_size = manifest.payload_len;
    let arena = args.arena_bytes;

    let mut ok = true;
    if package_size > args.max_package_bytes {
        ok = false;
        println!(
            "FAIL: package size {} exceeds max_package_bytes {}",
            package_size, args.max_package_bytes
        );
    } else {
        println!(
            "OK:   package size {} <= {}",
            package_size, args.max_package_bytes
        );
    }
    if payload_size + arena > args.flash_bytes {
        ok = false;
        println!(
            "FAIL: payload {} + arena {} exceeds flash_bytes {}",
            payload_size, arena, args.flash_bytes
        );
    } else {
        println!(
            "OK:   payload {} + arena {} <= flash {}",
            payload_size, arena, args.flash_bytes
        );
    }
    if arena > args.ram_bytes {
        ok = false;
        println!("FAIL: arena {} exceeds ram_bytes {}", arena, args.ram_bytes);
    } else {
        println!("OK:   arena {} <= ram_bytes {}", arena, args.ram_bytes);
    }

    if ok {
        println!("\nFIT: package fits the target profile");
    } else {
        println!("\nNO FIT: package does not fit the target profile");
        // Return an error so the exit code is non-zero.
        anyhow::bail!("package does not fit the target profile");
    }
    Ok(())
}

fn hex_short(bytes: &[u8; 32]) -> String {
    ota_crypto::hex_encode(bytes)[..12].to_string()
}
