//! The `ota` command-line interface.
//!
//! DEV.md exempts the binary entry point from the `unwrap`/`expect` ban, so the
//! crate-level allow keeps `clippy::unwrap_used` quiet for the `main` shim while
//! every subcommand still returns [`anyhow::Result`] and propagates errors.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod cli;
mod io;
mod model;
mod package;
mod sign;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Command};

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Package(args) => package::run(args),
        Command::Sign(args) => sign::run(args),
        Command::Model(args) => model::run(args),
    }
}
