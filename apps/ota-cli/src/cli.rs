//! Command-line surface for the `ota` binary.
//!
//! Every clap type lives here so the command handlers in [`crate::package`] and
//! [`crate::sign`] stay focused on behaviour.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use ota_package::Quantization;

/// Build, sign and inspect ai-ota update packages.
#[derive(Debug, Parser)]
#[command(name = "ota", version, about, long_about = None)]
pub struct Cli {
    /// Command group to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level command groups.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build and inspect `.otapkg` packages.
    Package(PackageArgs),
    /// Generate keys, sign packages and verify signatures.
    Sign(SignArgs),
    /// Compare models and check target fit.
    Model(ModelArgs),
}

/// `ota package` subcommands.
#[derive(Debug, Args)]
pub struct PackageArgs {
    /// Package operation to run.
    #[command(subcommand)]
    pub command: PackageCommand,
}

/// Operations on `.otapkg` files.
#[derive(Debug, Subcommand)]
pub enum PackageCommand {
    /// Assemble an unsigned `.otapkg` from a model payload.
    Build(BuildArgs),
    /// Parse a `.otapkg`, verify its payload and print its manifest.
    Inspect(InspectArgs),
}

/// Arguments for `ota package build`.
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Model or artifact name, for example `vision-v3`.
    #[arg(long)]
    pub model: String,
    /// Semantic version of the model, for example `1.2.0`.
    #[arg(long)]
    pub version: String,
    /// Target identifier the payload is built for, for example `stm32h7`.
    #[arg(long)]
    pub target: String,
    /// Weight quantization of the payload.
    #[arg(long, value_enum, default_value_t = QuantizationArg::Int8)]
    pub quantization: QuantizationArg,
    /// Payload file to package.
    #[arg(long)]
    pub input: PathBuf,
    /// Destination `.otapkg` file.
    #[arg(long)]
    pub output: PathBuf,
    /// Payload chunk size in bytes.
    #[arg(long, default_value_t = ota_package::DEFAULT_CHUNK_SIZE)]
    pub chunk_size: u32,
}

/// Arguments for `ota package inspect`.
#[derive(Debug, Args)]
pub struct InspectArgs {
    /// `.otapkg` file to inspect.
    pub input: PathBuf,
}

/// Weight quantization selected on the command line.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum QuantizationArg {
    /// Not a quantized model.
    None,
    /// 8-bit integer weights.
    Int8,
    /// 4-bit integer weights.
    Int4,
    /// 16-bit float weights.
    Fp16,
    /// 32-bit float weights.
    Fp32,
    /// Mixed precision.
    Mixed,
}

impl From<QuantizationArg> for Quantization {
    fn from(value: QuantizationArg) -> Self {
        match value {
            QuantizationArg::None => Self::None,
            QuantizationArg::Int8 => Self::Int8,
            QuantizationArg::Int4 => Self::Int4,
            QuantizationArg::Fp16 => Self::Fp16,
            QuantizationArg::Fp32 => Self::Fp32,
            QuantizationArg::Mixed => Self::Mixed,
        }
    }
}

/// `ota sign` subcommands.
#[derive(Debug, Args)]
pub struct SignArgs {
    /// Signing operation to run.
    #[command(subcommand)]
    pub command: SignCommand,
}

/// Key and signature operations.
#[derive(Debug, Subcommand)]
pub enum SignCommand {
    /// Generate an Ed25519 key pair.
    Keygen(KeygenArgs),
    /// Sign an unsigned `.otapkg` package.
    Package(SignPackageArgs),
    /// Verify a signed `.otapkg` package against a public key.
    Verify(VerifyArgs),
}

/// Arguments for `ota sign keygen`.
#[derive(Debug, Args)]
pub struct KeygenArgs {
    /// Path to write the private key to.
    #[arg(long)]
    pub output: PathBuf,
    /// Path to write the public key to. Defaults to `<output>.pub`.
    #[arg(long)]
    pub public_output: Option<PathBuf>,
}

/// Arguments for `ota sign package`.
#[derive(Debug, Args)]
pub struct SignPackageArgs {
    /// Private key file produced by `ota sign keygen`.
    #[arg(long)]
    pub key: PathBuf,
    /// Unsigned `.otapkg` file to sign.
    #[arg(long)]
    pub input: PathBuf,
    /// Destination for the signed package.
    #[arg(long)]
    pub output: PathBuf,
}

/// Arguments for `ota sign verify`.
#[derive(Debug, Args)]
pub struct VerifyArgs {
    /// Public key file produced by `ota sign keygen`.
    #[arg(long)]
    pub key: PathBuf,
    /// Signed `.otapkg` file to verify.
    pub input: PathBuf,
}

/// `ota model` subcommands.
#[derive(Debug, Args)]
pub struct ModelArgs {
    /// Model operation to run.
    #[command(subcommand)]
    pub command: ModelCommand,
}

/// Model comparison and fit checks.
#[derive(Debug, Subcommand)]
pub enum ModelCommand {
    /// Compare two `.otapkg` packages (manifest + payload hash).
    Diff(DiffArgs),
    /// Check whether a package fits a target profile.
    Fit(FitArgs),
}

/// Arguments for `ota model diff`.
#[derive(Debug, Args)]
pub struct DiffArgs {
    /// First `.otapkg` file.
    pub left: PathBuf,
    /// Second `.otapkg` file.
    pub right: PathBuf,
}

/// Arguments for `ota model fit`.
#[derive(Debug, Args)]
pub struct FitArgs {
    /// `.otapkg` file to check.
    pub input: PathBuf,
    /// Target RAM budget in bytes.
    #[arg(long, default_value_t = 16384)]
    pub ram_bytes: u64,
    /// Target flash budget in bytes.
    #[arg(long, default_value_t = 1048576)]
    pub flash_bytes: u64,
    /// Maximum package size in bytes.
    #[arg(long, default_value_t = 1048576)]
    pub max_package_bytes: u64,
    /// Model arena size in bytes (user-supplied hint).
    #[arg(long, default_value_t = 0)]
    pub arena_bytes: u64,
}
