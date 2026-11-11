//! `ota sign` — manage keys, sign packages and verify signatures.

use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use ota_crypto::{
    decode_private_key, decode_public_key, encode_private_key, encode_public_key, Signature,
    SigningKey, SEED_LEN,
};
use ota_package::{append_signature, Package, SignatureAlgorithm};

use crate::cli::{KeygenArgs, SignArgs, SignCommand, SignPackageArgs, VerifyArgs};
use crate::io;

/// Dispatch an `ota sign` invocation.
///
/// # Errors
///
/// Propagates any error raised by the selected operation.
pub fn run(args: SignArgs) -> Result<()> {
    match args.command {
        SignCommand::Keygen(keygen_args) => keygen(&keygen_args),
        SignCommand::Package(package_args) => package(&package_args),
        SignCommand::Verify(verify_args) => verify(&verify_args),
    }
}

/// Generate an Ed25519 key pair and write it to disk.
fn keygen(args: &KeygenArgs) -> Result<()> {
    let signing = SigningKey::from_seed(&random_seed()?);
    let public_output = args
        .public_output
        .clone()
        .unwrap_or_else(|| public_key_path(&args.output));

    io::write(&args.output, encode_private_key(&signing).as_bytes())?;
    io::write(
        &public_output,
        encode_public_key(&signing.verifying_key()).as_bytes(),
    )?;

    println!("wrote private key to {}", args.output.display());
    println!("wrote public key to  {}", public_output.display());
    Ok(())
}

/// Sign an unsigned package with a private key.
fn package(args: &SignPackageArgs) -> Result<()> {
    let key_text = io::read_to_string(&args.key)?;
    let signing = decode_private_key(&key_text).context("failed to read private key")?;

    let unsigned = io::read(&args.input)?;
    let parsed = Package::parse(&unsigned)
        .with_context(|| format!("{} is not a valid .otapkg", args.input.display()))?;
    let signature = signing.sign(&parsed.signing_digest());

    let signed = append_signature(
        &unsigned,
        SignatureAlgorithm::Ed25519,
        &signature.to_bytes(),
    )
    .context("failed to append signature")?;

    io::write(&args.output, &signed)?;
    println!(
        "wrote {} ({} bytes, signed)",
        args.output.display(),
        signed.len()
    );
    Ok(())
}

/// Verify a signed package against a public key.
fn verify(args: &VerifyArgs) -> Result<()> {
    let key_text = io::read_to_string(&args.key)?;
    let public = decode_public_key(&key_text).context("failed to read public key")?;

    let bytes = io::read(&args.input)?;
    let parsed = Package::parse(&bytes)
        .with_context(|| format!("{} is not a valid .otapkg", args.input.display()))?;

    let signature = parsed
        .signature()
        .with_context(|| format!("{} is not signed", args.input.display()))?;
    if signature.algorithm != SignatureAlgorithm::Ed25519 {
        bail!(
            "unsupported signature algorithm: {}",
            signature.algorithm.as_str()
        );
    }

    public
        .verify(
            &parsed.signing_digest(),
            &Signature::from_slice(signature.bytes).context("malformed signature")?,
        )
        .context("package signature is invalid")?;

    println!("{}: signature ok", args.input.display());
    Ok(())
}

/// Derive the default public key path from the private key path.
fn public_key_path(private: &Path) -> PathBuf {
    let mut path = private.as_os_str().to_owned();
    path.push(".pub");
    PathBuf::from(path)
}

/// Read [`SEED_LEN`] bytes of entropy from the operating system.
///
/// Key generation reads `/dev/urandom`, so it is only supported on Unix hosts.
fn random_seed() -> Result<[u8; SEED_LEN]> {
    let mut seed = [0u8; SEED_LEN];
    std::fs::File::open("/dev/urandom")
        .context("failed to open /dev/urandom (key generation requires a Unix host)")?
        .read_exact(&mut seed)
        .context("failed to read entropy from /dev/urandom")?;
    Ok(seed)
}
