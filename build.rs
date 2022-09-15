//! Validates build-time configuration and injects it into the crate.
//!
//! A wasm bundle has no process environment at run time, so configuration has to be
//! resolved here. Anything missing or malformed fails the build with the name of the
//! offending variable rather than shipping a bundle that points nowhere.

use std::process::exit;

const VAR: &str = "POKEAPI_BASE_URL";

fn main() {
    println!("cargo:rerun-if-env-changed={VAR}");
    println!("cargo:rerun-if-changed=build.rs");

    let value = std::env::var(VAR).unwrap_or_else(|_| abort("is not set"));
    if !value.starts_with("http://") && !value.starts_with("https://") {
        abort("must be an absolute http:// or https:// url");
    }
    println!("cargo:rustc-env={VAR}={}", value.trim_end_matches('/'));
}

fn abort(reason: &str) -> ! {
    eprintln!("error: {VAR} {reason}.");
    eprintln!("       copy .env.example to .env, then: set -a && . ./.env && set +a");
    exit(1);
}
