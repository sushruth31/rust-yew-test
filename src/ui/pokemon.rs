use crate::config::POKEAPI_BASE_URL;
use crate::pokeapi::{self, Pokemon};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Fetch {
    Idle,
    Loading,
    Failed(String),
}

type Caught = UseStateHandle<Vec<Pokemon>>;

#[function_component]
pub fn PokemonPage() -> Html {
    let caught: Caught = use_state(Vec::new);
    let fetch = use_state(|| Fetch::Idle);
    let onclick = catch_handler(&caught, &fetch);

    html! {
        <section>
            <h2>{ "Pokédex" }</h2>
            <button {onclick} disabled={*fetch == Fetch::Loading}>{ "catch one" }</button>
            { status_view(&fetch) }
            <ul>{ for caught.iter().enumerate().map(|(slot, p)| card(slot, p, &caught)) }</ul>
        </section>
    }
}

fn status_view(fetch: &UseStateHandle<Fetch>) -> Html {
    match &**fetch {
        Fetch::Idle => html! {},
        Fetch::Loading => html! { <p>{ "fetching…" }</p> },
        Fetch::Failed(reason) => html! { <p role="alert">{ format!("failed: {reason}") }</p> },
    }
}

/// Removal is positional, not by name: after fifteen catches from a 151-wide dex a
/// duplicate species is more likely than not, and matching on name deletes both.
fn card(slot: usize, pokemon: &Pokemon, caught: &Caught) -> Html {
    let onclick = {
        let caught = caught.clone();
        Callback::from(move |_: MouseEvent| {
            let mut next = (*caught).clone();
            next.remove(slot);
            caught.set(next);
        })
    };
    html! {
        <li>
            <span>{ pokemon.display_name() }</span>
            <img width="96" height="96" alt={pokemon.display_name()} src={pokemon.sprite.clone()} />
            <button {onclick}>{ "release" }</button>
        </li>
    }
}

fn catch_handler(caught: &Caught, fetch: &UseStateHandle<Fetch>) -> Callback<MouseEvent> {
    let (caught, fetch) = (caught.clone(), fetch.clone());
    Callback::from(move |_: MouseEvent| {
        let (caught, fetch) = (caught.clone(), fetch.clone());
        fetch.set(Fetch::Loading);
        spawn_local(async move {
            match catch_one().await {
                Ok(pokemon) => {
                    caught.set([(*caught).clone(), vec![pokemon]].concat());
                    fetch.set(Fetch::Idle);
                }
                Err(reason) => fetch.set(Fetch::Failed(reason)),
            }
        });
    })
}

async fn catch_one() -> Result<Pokemon, String> {
    let id = pokeapi::random_id(&mut rand::thread_rng());
    let url = pokeapi::endpoint(POKEAPI_BASE_URL, id);
    let response = Request::get(&url).send().await.map_err(stringify)?;
    if !response.ok() {
        return Err(format!("pokeapi answered {}", response.status()));
    }
    let body = response.text().await.map_err(stringify)?;
    pokeapi::parse(&body).map_err(stringify)
}

fn stringify(error: impl std::fmt::Display) -> String {
    error.to_string()
}
