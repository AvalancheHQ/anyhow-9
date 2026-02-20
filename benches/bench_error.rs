use anyhow::{anyhow, Context};
use std::fmt;
use std::io;

fn main() {
    divan::main();
}

// ---------------------------------------------------------------------------
// Error creation
// ---------------------------------------------------------------------------

#[divan::bench]
fn error_from_std_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::new(io_err)
}

#[divan::bench]
fn error_from_message() -> anyhow::Error {
    anyhow::Error::msg("something went wrong")
}

#[divan::bench]
fn error_via_anyhow_macro() -> anyhow::Error {
    anyhow!("something went wrong: {}", 42)
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[divan::bench]
fn error_with_context() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let result: Result<(), io::Error> = Err(io_err);
    result.context("failed to open config file").unwrap_err()
}

#[divan::bench]
fn error_with_lazy_context() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let result: Result<(), io::Error> = Err(io_err);
    result
        .with_context(|| format!("failed to open {}", "config.toml"))
        .unwrap_err()
}

#[divan::bench]
fn error_with_chained_contexts() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
    anyhow::Error::new(io_err)
        .context("while reading configuration")
        .context("during application startup")
}

// ---------------------------------------------------------------------------
// Downcast
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct CustomError {
    code: u32,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error (code {})", self.code)
    }
}

impl std::error::Error for CustomError {}

#[divan::bench]
fn downcast_ref_success(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow::Error::new(CustomError { code: 42 }))
        .bench_local_refs(|error| {
            error.downcast_ref::<CustomError>().unwrap();
        });
}

#[divan::bench]
fn downcast_ref_failure(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow::Error::new(CustomError { code: 42 }))
        .bench_local_refs(|error| {
            let _ = error.downcast_ref::<io::Error>();
        });
}

#[divan::bench]
fn downcast_value(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow::Error::new(CustomError { code: 42 }))
        .bench_values(|error| {
            let _ = error.downcast::<CustomError>();
        });
}

// ---------------------------------------------------------------------------
// Error chain
// ---------------------------------------------------------------------------

#[divan::bench]
fn chain_length(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
            anyhow::Error::new(io_err)
                .context("layer 1")
                .context("layer 2")
                .context("layer 3")
        })
        .bench_local_refs(|error| error.chain().count());
}

// ---------------------------------------------------------------------------
// Display formatting
// ---------------------------------------------------------------------------

#[divan::bench]
fn display_simple(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow!("something went wrong"))
        .bench_local_refs(|error| {
            fmt::format(format_args!("{}", error))
        });
}

#[divan::bench]
fn display_with_chain(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
            anyhow::Error::new(io_err)
                .context("while reading configuration")
                .context("during application startup")
        })
        .bench_local_refs(|error| {
            fmt::format(format_args!("{:#}", error))
        });
}

#[divan::bench]
fn debug_with_chain(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
            anyhow::Error::new(io_err)
                .context("while reading configuration")
                .context("during application startup")
        })
        .bench_local_refs(|error| {
            fmt::format(format_args!("{:?}", error))
        });
}
