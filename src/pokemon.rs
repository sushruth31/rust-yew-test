use gloo::timers::callback::Timeout;
use gloo_console::{console, log};
use rand::prelude::*;
use reqwest;
use serde_json::Value;
use wasm_bindgen::JsCast;
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
    let pokemon: UseStateHandle<Vec<PokemonData>> = use_state(Vec::default);
    let loading: UseStateHandle<bool> = use_state(|| false);
    let loading_copy = loading.clone();
    let pokemon_copy = pokemon.clone();
    let pokemon_copy_2 = pokemon.clone();
    let ondelete = Callback::from(move |pokemon_name: String| {
        let p = pokemon_copy_2.clone();
        let pokemon_state = p
            .to_vec()
            .into_iter()
            .filter(|pokemon| pokemon.name != pokemon_name)
            .collect::<Vec<PokemonData>>();
        p.set(pokemon_state);
    });
    //console::log_1(&format!("{pokemon:#?}").into());
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
                        let img_src = data["sprites"]["back_default"].as_str().unwrap();
                        let mut poke_vec = pokemon_state.clone().to_vec();
                        poke_vec.push(PokemonData {
                            name: name.into(),
                            img_src: img_src.into(),
                        });
                        pokemon_state.set(poke_vec);
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
            <PokemonView {ondelete} pokemon={(*pokemon_copy).clone()}/>
            <button {onclick}>{"Fetch Pokemon"}</button>
            </>
    }
}

#[derive(Properties, PartialEq)]
pub struct PokemonViewProps {
    pub pokemon: Vec<PokemonData>,
    pub ondelete: Callback<String>,
}

pub fn get_inner_html(e: MouseEvent) -> String {
    return e
        .target()
        .unwrap()
        .dyn_ref::<web_sys::Element>()
        .unwrap()
        .clone()
        .inner_html();
}

#[function_component(PokemonView)]
pub fn pokemon_view(props: &PokemonViewProps) -> Html {
    if !props.pokemon.is_empty() {
        let pokemon_cards = props.pokemon.iter().map(|pokemon| {
            let pokemon_copy = pokemon.clone();
            let ondelete = props.ondelete.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                let name = pokemon_copy.name.to_owned();
                ondelete.emit(name);
            });
            return html! {
                <>
                <div>{"Name: "} {pokemon.name.clone()}</div>
                <button {onclick}>{"Remove pokemon"}</button>
                <img width={200} height={200} src={pokemon.img_src.clone()}/>
                    </>
            };
        });
        return html! {
            <>
            {for pokemon_cards}
                </>
        };
    } else {
        return html! {};
    }
}
