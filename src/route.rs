use yew_router::{navigator, prelude::*, switch::_SwitchProps::render};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/modal")]
    Modal,
    #[at("/pokemon")]
    Pokemon,
    #[not_found]
    #[at("/404")]
    NotFound,
}
