use leptos::*;

#[component]
pub fn ThankYou() -> impl IntoView {
    view! {
        <section class="Recruiting ThankYou">
            <h2>"Thank you!"</h2>
            <p>
                "Your call is scheduled and you'll recieve a confirmation and a calendar invitation via e-mail."
            </p>
            <p>"Feel free to reach out in case you have any questions before the call:"</p>
            <div class="ContactCard">
                <img src="/niclas-round.png" alt=""/>
                <div class="ContactCard-details">
                    <strong>"Niclas Åhdén"</strong>
                    <span>"Interim CTO"</span>
                    <a href="mailto:niclas@instateam.se">"niclas@instateam.se"</a>
                    <a href="tel:+46705650510">"070 565 05 10"</a>
                </div>
            </div>
        </section>
    }
}
