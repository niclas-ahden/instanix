use crate::routes::{
    // blog::*,
    blog::avoiding_danger_using_types::*,
    home::*,
    recruiting::back_end::*,
    recruiting::front_end::*,
    recruiting::thank_you::*,
    recruiting::*,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/instarst.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=DM+Serif+Display&family=Fira+Code&family=Inter:wght@400;700&display=swap"
        />
        <Title text="Instateam"/>
        <Router>
            <nav class="MainNav">
                <ul>
                    <li>
                        <A href="" exact=true>
                            "Home"
                        </A>
                    </li>
                    <li>
                        <A href="/recruiting">"Recruiting"</A>
                    </li>
                    <li>
                        <A href="/blog">"Blog"</A>
                    </li>
                </ul>
            </nav>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| { view! { <Home/> } }
                    />
                    <Route
                        path="/blog"
                        view=|| { view! { <AvoidingDangerUsingTypes/> } }
                    />
                    <Route
                        path="/blog/avoiding-danger-using-types"
                        view=|| { view! { <AvoidingDangerUsingTypes/> } }
                    />
                    <Route
                        path="/recruiting"
                        view=|| { view! { <OpenPositions/> } }
                    />
                    <Route
                        path="/recruiting/typescript-front-end-developer"
                        view=|| { view! { <FrontEndDeveloper/> } }
                    />
                    <Route
                        path="/recruiting/back-end-developer"
                        view=|| { view! { <BackEndDeveloper/> } }
                    />
                    <Route
                        path="/recruiting/thank-you"
                        view=|| { view! { <ThankYou/> } }
                    />
                </Routes>
            </main>
            <footer class="Footer">
                <p class="Footer-copyright">"© 2023 Instateam AB"</p>
            </footer>
        </Router>
    }
}
