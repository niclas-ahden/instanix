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

    // => Cache busting
    // We want to ensure that assets are cache busted on each change. We could
    // do that by hashing each asset and appending the hash to their file names.
    // We are not yet that granular, so instead we:
    //
    // 1. Hash the whole project directory before compiling.
    // 2. Set LEPTOS_SITE_PKG_DIR at compile time to a value like "pkg-$hash".
    // 3. At runtime we set the environment variable LEPTOS_SITE_PKG_DIR from said value.
    // 4. We use that to refer to our assets, and cache busting happens.
    //
    // We have to do this both for the front- and back-end, so similar logic
    // exists in both `lib.rs` and `main.rs`. In `lib.rs` we set a constant and
    // in `main.rs` we also set an environment variable (since Leptos uses that
    // for its configuration).
    pub const LEPTOS_SITE_PKG_DIR: &str = env!("LEPTOS_SITE_PKG_DIR");

    leptos::mount_to_body(move || {
        view! { <App pkg_dir=LEPTOS_SITE_PKG_DIR.to_string() /> }
    });
}
