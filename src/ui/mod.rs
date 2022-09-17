//! The yew view layer. Compiled only for wasm32 so the host test build stays clean
//! of browser dependencies.

mod navbar;
mod pokemon;
mod route;
mod todo;

use navbar::NavBar;
use pokemon::PokemonPage;
use route::Route;
use todo::TodoPage;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <TodoPage /> },
        Route::Pokemon => html! { <PokemonPage /> },
        Route::NotFound => html! { <p>{ "404 — no such page" }</p> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <h1>{ "Yew playground" }</h1>
            <NavBar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
