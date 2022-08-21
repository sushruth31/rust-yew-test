use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use wasm_logger;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub current: u32,
}

impl Default for ItemProps {
    fn default() -> Self {
        return Self { current: 0 };
    }
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    return html! {
        <><h1>{props.current}</h1></>
    };
}

#[function_component(App)]
pub fn app() -> Html {
    let render_count: UseStateHandle<u32> = use_state(|| 0);
    let count = *render_count;
    let render_input: UseStateHandle<String> = use_state(String::default);
    let input = (*render_input).clone();
    let interval: u32 = 1000;
    use_effect_with_deps(
        move |_| {
            let handle = Timeout::new(interval, move || render_count.set(count + 1));
            return move || {
                handle.cancel();
            };
        },
        count,
    );
    return html! {
        <>
            <h1>{"Welcome To The Rust App"}</h1>
            <Item current={count}/>
            <div>{input}</div>
            <input onchange={move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let inputval = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(v) = inputval {
                render_input.set(v.value());
            }
            }}/>
            </>
    };
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
