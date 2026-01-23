use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

/// Benchmark creating errors using anyhow! macro
#[divan::bench]
fn create_anyhow_error() -> anyhow::Error {
    anyhow!("something went wrong")
}

/// Benchmark error creation with formatting
#[divan::bench]
fn create_formatted_error() -> anyhow::Error {
    let value = 42;
    anyhow!("failed to process value: {}", value)
}

/// Benchmark converting from std::io::Error
#[divan::bench]
fn convert_from_io_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::from(io_err)
}

/// Benchmark adding context to errors
#[divan::bench]
fn add_context() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).context("failed to read configuration")?;
    Ok(())
}

/// Benchmark error chain creation
#[divan::bench]
fn create_error_chain() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
    Err(io_err)
        .context("failed to open file")
        .context("failed to load configuration")
        .context("application startup failed")?;
    Ok(())
}

/// Benchmark downcast operations
#[divan::bench]
fn downcast_error() -> bool {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = anyhow::Error::from(io_err);
    err.downcast_ref::<io::Error>().is_some()
}
