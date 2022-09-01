use gloo::timers::callback::Timeout;
use gloo_console::{console, log};
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::{EventTarget, HtmlElement, HtmlInputElement};
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

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub cb: Callback<MouseEvent>,
    pub label: String,
}

#[function_component(Item)]
fn item(props: &ItemProps) -> Html {
    return html! {
        <li onclick={props.cb.clone()}>{props.label.clone()}</li>
    };
}

#[function_component(Home)]
fn home() -> Html {
    let render_list: UseStateHandle<Vec<String>> = use_state(|| vec!["hello".to_string()]);
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
                <Item {cb} label={item.to_string()}  />
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

    let add_item = {
        let input_value_handle = input_value_handle.clone();
        let list = render_list.clone();
        Callback::from(move |_| {
            let mut vec = (*list).clone();
            let val: String = (*input_value_handle).to_string();
            vec.push(val);
            list.set(vec);
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
