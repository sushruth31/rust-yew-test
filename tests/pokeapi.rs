use rand::rngs::StdRng;
use rand::SeedableRng;
use yew_app::pokeapi::{endpoint, parse, random_id, PokeError, Pokemon, DEX_SIZE};

fn payload(back: &str, front: &str) -> String {
    format!(
        r#"{{"name":"bulbasaur","id":1,"sprites":{{"back_default":{back},"front_default":{front}}}}}"#
    )
}

#[test]
fn parse_prefers_the_back_sprite() {
    let body = payload(r#""https://img/back.png""#, r#""https://img/front.png""#);
    let pokemon = parse(&body).expect("both sprites present");
    assert_eq!(pokemon.sprite, "https://img/back.png");
    assert_eq!(pokemon.name, "bulbasaur");
}

#[test]
fn parse_falls_back_to_the_front_sprite_when_back_is_null() {
    let body = payload("null", r#""https://img/front.png""#);
    assert_eq!(
        parse(&body).map(|p| p.sprite),
        Ok("https://img/front.png".to_owned())
    );
}

#[test]
fn parse_reports_an_error_instead_of_panicking_when_every_sprite_is_null() {
    let body = payload("null", "null");
    assert_eq!(parse(&body), Err(PokeError::NoSprite("bulbasaur".into())));
}

#[test]
fn parse_rejects_a_payload_missing_the_sprites_object() {
    let err = parse(r#"{"name":"bulbasaur"}"#).expect_err("sprites is required");
    assert!(matches!(err, PokeError::Malformed(_)));
}

#[test]
fn parse_rejects_html_error_pages_returned_with_a_200() {
    let err = parse("<!DOCTYPE html><h1>502</h1>").expect_err("html is not json");
    assert!(matches!(err, PokeError::Malformed(_)));
}

#[test]
fn parse_ignores_the_hundreds_of_fields_the_view_does_not_use() {
    let body = r#"{"name":"pikachu","base_experience":112,"moves":[{"move":{"name":"thunder"}}],
        "sprites":{"back_default":"b.png","front_default":"f.png","other":{"dream_world":{}}}}"#;
    let pokemon = parse(body).expect("unknown fields are skipped");
    assert_eq!(pokemon.name, "pikachu");
}

#[test]
fn endpoint_never_double_slashes_however_the_base_url_is_written() {
    let expected = "https://pokeapi.co/api/v2/pokemon/25";
    assert_eq!(endpoint("https://pokeapi.co/api/v2", 25), expected);
    assert_eq!(endpoint("https://pokeapi.co/api/v2/", 25), expected);
    assert_eq!(endpoint("https://pokeapi.co/api/v2///", 25), expected);
}

#[test]
fn random_id_stays_inside_the_dex_and_reaches_both_ends() {
    let mut rng = StdRng::seed_from_u64(11);
    let ids: Vec<u32> = (0..20_000).map(|_| random_id(&mut rng)).collect();

    assert!(ids.iter().all(|id| (1..=DEX_SIZE).contains(id)));
    assert_eq!(ids.iter().min(), Some(&1), "lower bound is inclusive");
    assert_eq!(
        ids.iter().max(),
        Some(&DEX_SIZE),
        "upper bound is inclusive"
    );
}

#[test]
fn display_name_title_cases_each_hyphenated_segment() {
    let named = |name: &str| Pokemon {
        name: name.to_owned(),
        sprite: "s.png".to_owned(),
    };
    assert_eq!(named("pikachu").display_name(), "Pikachu");
    assert_eq!(named("nidoran-f").display_name(), "Nidoran F");
    assert_eq!(named("mr-mime").display_name(), "Mr Mime");
}
