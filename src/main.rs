use gloo::timers::callback::Timeout;
use gloo_console::{console, log};
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::{EventTarget, HtmlElement, HtmlInputElement, UrlSearchParams};
use yew::prelude::*;
use yew_app::navbar::NavBar;
use yew_app::pokemon::Pokemon;
use yew_app::route::Route;
use yew_hooks::prelude::*;
use yew_router::{navigator, prelude::*, switch::_SwitchProps::render};

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

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub cb: Callback<MouseEvent>,
    pub status: Status,
    pub text: String,
}

#[function_component(Item)]
fn item(props: &ItemProps) -> Html {
    return html! {
        <li onclick={props.cb.clone()}>{props.text.clone()}</li>
    };
}

#[derive(Clone, PartialEq)]
pub enum Status {
    Done,
    NotDone,
}

#[derive(Clone)]
pub struct ListData {
    pub status: Status,
    pub text: String,
}

#[function_component(Home)]
fn home() -> Html {
    let render_list: UseStateHandle<Vec<ListData>> = use_state(|| Vec::new());
    let input_value_handle: UseStateHandle<AttrValue> = use_state(AttrValue::default);
    let selected_item: UseStateHandle<AttrValue> = use_state(AttrValue::default);

    let on_delete = {
        let selected_item = selected_item.clone();
        Callback::from(move |e: MouseEvent| {
            if !selected_item.is_empty() {
                selected_item.set(AttrValue::from(""));
            }
        })
    };

    let add_item = {
        let text = input_value_handle.clone();
        let list = render_list.clone();
        Callback::from(move |e: MouseEvent| {
            if text.is_empty() {
                return;
            }
            let mut list_copy = (*list).clone();
            list_copy.push(ListData {
                status: Status::NotDone,
                text: text.to_string(),
            });
            list.set(list_copy);
            text.set(AttrValue::from(""));
        })
    };

    let items = render_list.iter().map(|item| {
        let selected = selected_item.clone();
        let cb = Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target().and_then(|event_target: web_sys::EventTarget| {
                event_target.dyn_into::<web_sys::Element>().ok()
            }) {
                let text = target.inner_html();
                selected.set(AttrValue::from(text));
            }
        });
        return html! {
            <>
                <Item {cb} status={item.status.to_owned()} text={item.text.to_string()} />
                </>
        };
    });
    let onchange = {
        let input_value = input_value_handle.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(v) = input {
                input_value.set(AttrValue::from(v.value()));
            }
        })
    };

    html! {
        <>
            <div>{"this is the todolist"}</div>
            <div>{"The selected item is"} {" "} {selected_item.to_string()}</div>
            <input {onchange} type="text"/>
            <button onclick={add_item}>{"add item"}</button>
            <button onclick={on_delete}>{"remove item"}</button>
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
        Route::Pokemon => html! {
            <Pokemon />
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
            <NavBar />
            <Switch<Route> render={switch} />
            </BrowserRouter>
            </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
