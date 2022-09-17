use yew_router::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pokemon")]
    Pokemon,
    #[not_found]
    #[at("/404")]
    NotFound,
}
