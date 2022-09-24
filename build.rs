//! Resolves build-time configuration and injects it into the crate.
//!
//! A wasm bundle has no process environment at run time, so the API base URL has to be
//! baked in here. PokeAPI is public and needs no key, so the default is the real
//! service and a clean clone builds with no setup. An override is still validated —
//! pointing at a local mirror is supported, pointing at nonsense is not.

use std::process::exit;

const VAR: &str = "POKEAPI_BASE_URL";
const DEFAULT: &str = "https://pokeapi.co/api/v2";

fn main() {
    println!("cargo:rerun-if-env-changed={VAR}");
    println!("cargo:rerun-if-changed=build.rs");

    let value = std::env::var(VAR).unwrap_or_else(|_| DEFAULT.to_string());
    if !value.starts_with("http://") && !value.starts_with("https://") {
        eprintln!("error: {VAR} must be an absolute http:// or https:// url; got {value:?}.");
        exit(1);
    }
    println!("cargo:rustc-env={VAR}={}", value.trim_end_matches('/'));
}
