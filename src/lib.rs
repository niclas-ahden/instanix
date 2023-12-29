pub mod app;
pub mod error_template;
pub mod fileserv;
pub mod routes;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "hydrate")]
use crate::app::*;
#[cfg(feature = "hydrate")]
use leptos::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
        view! { <App/> }
    });
}
