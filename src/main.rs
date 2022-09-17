#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<yew_app::ui::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("yew-app is a wasm32 binary; serve it with `trunk serve`.");
    std::process::exit(1);
}
