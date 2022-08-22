use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/modal")]
    Modal,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    let go_to_secure_button = {
        let onclick = Callback::from(move |_| navigator.push(&Route::Modal));
        html! {
            <button {onclick}>{"click to go to secure"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_secure_button}
        </>
    }
}

#[function_component(Modal)]
fn modal() -> Html {
    let modal_state: UseStateHandle<bool> = use_state(|| false);
    let modal = *modal_state;

    html! {
        <div>
            <h3>{ "Modal" }</h3>
            if modal {
                <div>{"this is the modal!"}</div>
            }
            <button onclick={move |_| modal_state.set(!modal)}>{"toggle"}</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let render_list: UseStateHandle<Vec<String>> = use_state(|| vec![String::from("hello")]);
    let input_value_handle: UseStateHandle<String> = use_state(String::default);
    let items = render_list.iter().map(|item| {
        return html! {
            <li>{item}</li>
        };
    });
    let onchange = {
        let input_value = input_value_handle.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(v) = input {
                input_value.set(v.value());
            }
        })
    };

    let add_item = {
        let input_value_handle = input_value_handle.clone();
        let list = render_list.clone();
        Callback::from(move |_| {
            let mut vec: Vec<String> = list.to_vec();
            let val: String = String::from(input_value_handle.as_str());
            if val.is_empty() {
                return;
            }
            vec.push(val);
            list.set(vec);
        })
    };

    html! {
        <>
            <div>{"this is the todolist"}</div>
            <input {onchange} type="text"/>
            <button onclick={add_item}>{"add item"}</button>
            {for items}
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Modal => html! {
            <Modal />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Welcome to Rust Web App!"}</h1>
            <BrowserRouter>
            <NavItems />
            <Switch<Route> render={switch} />
            </BrowserRouter>
            </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
