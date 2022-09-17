//! Request shaping and response parsing for PokéAPI. Deliberately free of any HTTP
//! client: the fallible part is the JSON contract, and that is testable offline.

use rand::Rng;
use serde::Deserialize;

/// Generation I. The national dex runs further, but this is the slice the app draws
/// from and it keeps the sprite URLs predictable.
pub const DEX_SIZE: u32 = 151;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PokeError {
    #[error("response is not pokeapi json: {0}")]
    Malformed(String),
    #[error("`{0}` has no usable sprite")]
    NoSprite(String),
}

/// The subset of the payload the view needs; PokéAPI sends about 200 KB per pokemon.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pokemon {
    pub name: String,
    pub sprite: String,
}

impl Pokemon {
    /// `nidoran-f` becomes `Nidoran F`. The API is lowercase and hyphen separated.
    pub fn display_name(&self) -> String {
        self.name
            .split('-')
            .map(capitalize)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    name: String,
    sprites: Sprites,
}

/// Both fields are nullable in the API contract, which is why neither is unwrapped.
#[derive(Debug, Deserialize)]
struct Sprites {
    back_default: Option<String>,
    front_default: Option<String>,
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Tolerates a trailing slash on `base` so a stray one in the environment cannot
/// produce a double-slashed URL.
pub fn endpoint(base: &str, id: u32) -> String {
    format!("{}/pokemon/{id}", base.trim_end_matches('/'))
}

pub fn random_id(rng: &mut impl Rng) -> u32 {
    rng.gen_range(1..=DEX_SIZE)
}

/// A missing back sprite falls back to the front sprite; only a payload with neither
/// is an error. Unwrapping either would abort the whole wasm module.
pub fn parse(body: &str) -> Result<Pokemon, PokeError> {
    let response: Response =
        serde_json::from_str(body).map_err(|err| PokeError::Malformed(err.to_string()))?;
    let sprite = response
        .sprites
        .back_default
        .or(response.sprites.front_default)
        .ok_or_else(|| PokeError::NoSprite(response.name.clone()))?;
    Ok(Pokemon {
        name: response.name,
        sprite,
    })
}
