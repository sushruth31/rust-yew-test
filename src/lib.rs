//! Two layers: a pure domain that builds and tests on the host, and a yew view layer
//! that only exists on wasm32.

pub mod config;
pub mod pokeapi;
pub mod todo;

#[cfg(target_arch = "wasm32")]
pub mod ui;
