use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Instateam AB"</h1>
        <section class="Hero">
            <div class="Hero-content">
                <h2 class="Hero-heading">
                    "Niclas Åhdén" <span class="Hero-subtext">"CTO on-demand"</span>
                </h2>
                <p class="Hero-p">
                    "I help businesses by stepping in as a technical leader and taking on their pressing challenges."
                </p>
                <p class="Hero-p">
                    "Usually it's to drive product development, lead an important initiative, or improve the output of existing teams."
                </p>
            </div>
            <div class="Hero-portrait">
                <img src="/niclas-full.jpg" alt="Niclas Åhdén"/>
                <div class="Hero-overlay"></div>
            </div>
        </section>
        <section class="Challenges">
            <ul class="Challenges-list">
                <li class="Challenges-item">"Leadership"</li>
                <li class="Challenges-item">"Recruiting"</li>
                <li class="Challenges-item">"Product development"</li>
                <li class="Challenges-item">"Strategy and decision-making"</li>
            </ul>
        </section>
    }
}

#[component]
fn JobListing(#[prop(into)] title: String, #[prop(into)] href: String) -> impl IntoView {
    view! {
        <A class="JobListing" href=href>
            <h3>{title}</h3>
            <ul>
                <li>"Full-time"</li>
                <li>"Tyresö"</li>
                <li>"Hybrid (up to 50 % remote)"</li>
            </ul>
        </A>
    }
}
