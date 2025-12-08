use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod api;
mod charts;
mod utils;

use components::{Dashboard, Header, Sidebar};

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app-container">
            <Header />
            <div class="app-body">
                <Sidebar />
                <Dashboard />
            </div>
        </div>
    }
}
