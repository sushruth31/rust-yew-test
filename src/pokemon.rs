use gloo::timers::callback::Timeout;
use gloo_console::{console, log};
use rand::prelude::*;
use reqwest;
use serde_json::Value;
use wasm_bindgen_futures;
use wasm_logger;
use web_sys::console;
use yew::prelude::*;
use yew_router::{navigator, prelude::*, switch::_SwitchProps::render};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct PokemonData {
    name: String,
    img_src: String,
}

#[function_component(Pokemon)]
pub fn pokemon() -> Html {
    let pokemon: UseStateHandle<Option<PokemonData>> = use_state(|| None);
    let loading: UseStateHandle<bool> = use_state(|| false);
    let loading_copy = loading.clone();
    let pokemon_copy = pokemon.clone();
    //console::log_1(&format!("{:?}", pokemon).into());
    let onclick = Callback::from(move |e: MouseEvent| {
        let pokemon_state = pokemon.clone();
        let loading = loading.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            let mut rng = rand::thread_rng();
            let poke_id: u32 = rng.gen_range(1..100);
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", poke_id);
            match reqwest::get(url).await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        let text = response.text().await.ok().unwrap();
                        let data: Value = serde_json::from_str(&text).unwrap();
                        let name = data["name"].as_str().unwrap();
                        let img_src = data["sprites"]["front_default"].as_str().unwrap();
                        pokemon_state.set(Some(PokemonData {
                            name: name.into(),
                            img_src: img_src.into(),
                        }));
                        loading.set(false);
                    }
                }

                Err(response) => {
                    log!(response.to_string());
                }
            }
        })
    });
    html! {
        <>
            <h4>{"Your Pokemon"}</h4>
            if *loading_copy {
                <div>{"Fetching..."}</div>
            }
            <PokemonView pokemon={(*pokemon_copy).clone()}/>
            <button {onclick}>{"Fetch Pokemon"}</button>
            </>
    }
}

#[derive(Properties, PartialEq)]
pub struct PokemonViewProps {
    pokemon: Option<PokemonData>,
}

#[function_component(PokemonView)]
pub fn pokemon_view(props: &PokemonViewProps) -> Html {
    if let Some(pokemon) = &props.pokemon {
        return html! {
            <>
                <div>{"Pokemon Details"}</div>
                <div>{"Name: "} {pokemon.name.clone()}</div>
                </>
        };
    } else {
        return html! {};
    }
}
