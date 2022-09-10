use crate::route::Route;
use gloo_console::{console, log};
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::{EventTarget, HtmlElement, HtmlInputElement, UrlSearchParams};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::{navigator, prelude::*, switch::_SwitchProps::render};

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    let go_to_secure_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Modal));
        html! {
            <button {onclick}>{"click to go to secure"}</button>
        }
    };
    let go_to_pokemon_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Pokemon));
        html! {
            <button {onclick}>{"go to pokemon"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_secure_button}
            {go_to_pokemon_btn}
        </>
    }
}
