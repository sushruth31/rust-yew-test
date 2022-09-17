use super::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Real anchors instead of buttons driven by `use_navigator()`: nothing to unwrap,
/// and middle-click, ctrl-click and copy-link-address keep working.
#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav>
            <Link<Route> to={Route::Home}>{ "todo" }</Link<Route>>
            { " · " }
            <Link<Route> to={Route::Pokemon}>{ "pokédex" }</Link<Route>>
        </nav>
    }
}
