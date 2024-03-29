#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use instanix::app::*;
    use instanix::fileserv::file_and_error_handler;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos::logging::log;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment

    // => Cache busting
    // We want to ensure that assets are cache busted on each change. We could
    // do that by hashing each asset and appending the hash to their file names.
    // We are not yet that granular, so instead we:
    //
    // 1. Hash the whole project directory before compiling.
    // 2. Set LEPTOS_SITE_PKG_DIR at compile time to a value like "pkg-$hash".
    // 3. Set LEPTOS_SITE_PKG_DIR to the same value at runtime (Nix does this).
    // 4. We use that to refer to our assets, and cache busting happens.
    //
    // We have to do this both for the front- and back-end, so the same logic
    // exists in both `lib.rs` and `main.rs`.
    pub const LEPTOS_SITE_PKG_DIR: &str = env!("LEPTOS_SITE_PKG_DIR");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App pkg_dir=LEPTOS_SITE_PKG_DIR.to_string() /> });

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App pkg_dir=LEPTOS_SITE_PKG_DIR.to_string() /> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    log!("Listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
