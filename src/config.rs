//! Configuration resolved at build time by `build.rs`, which also validates it.

/// Base URL of the PokéAPI-compatible service, normalised without a trailing slash.
pub const POKEAPI_BASE_URL: &str = env!("POKEAPI_BASE_URL");
