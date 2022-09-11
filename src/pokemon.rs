use gloo::timers::callback::Timeout;
use gloo_console::{console, log};
use reqwest;
use serde_json::Value;
use wasm_bindgen_futures;
use wasm_logger;
use web_sys::console;
use yew::prelude::*;
use yew_router::{navigator, prelude::*, switch::_SwitchProps::render};

#[derive(Debug, Properties, PartialEq)]
pub struct PokemonData {
    name: String,
    img_src: String,
}

#[function_component(Pokemon)]
pub fn pokemon() -> Html {
    let pokemon: UseStateHandle<Option<PokemonData>> = use_state(|| None);
    console::log_1(&format!("{:?}", pokemon).into());
    let onclick = Callback::from(move |e: MouseEvent| {
        let pokemon_state = pokemon.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let url = "https://pokeapi.co/api/v2/pokemon/22";
            match reqwest::get(url).await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::OK {
                        let text = response.text().await.ok().unwrap();
                        let data: Value = serde_json::from_str(&text).unwrap();
                        let name = data["name"].as_str().unwrap();
                        let img_src = data["sprites"]["front_default"].as_str().unwrap();
                        let pokemon = PokemonData {
                            name: name.into(),
                            img_src: img_src.into(),
                        };
                        pokemon_state.set(Some(pokemon));
                        log!("fetched!");
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
            <button {onclick}>{"Fetch Pokemon"}</button>
            </>
    }
}
