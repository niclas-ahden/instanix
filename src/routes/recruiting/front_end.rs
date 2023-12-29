use leptos::*;

#[component]
pub fn FrontEndDeveloper() -> impl IntoView {
    view! {
        <section class="JobDescription">
            <h2>"Front-end developer — TypeScript"</h2>
            <p>
                <em>
                    "Instateam is recruiting for this position on behalf of a client. You will receive all the details about the company in our first call."
                </em>
            </p>
            <h3>"The Company"</h3>
            <p>
                "A well-established company in Tyresö which helps Sweden's largest retailers with their in-store communication (price signs, offers etc.). They offer a market-leading software that is used by thousands of stores to create, print, and publish signage. It's developed in-house by an experienced team and they're now transitioning it from a desktop app to a modern web app which you'll help build."
            </p>
            <p>
                "They're 34 employees, of which 10 work in the IT department. Everyone has a big impact on the direction and design of their products, there's no overbearing bureaucracy, and decisions can be made quickly."
            </p>
            <h3>"The position"</h3>
            <p>
                "You will be developing the next version of Sweden's leading retail signage software. It's a major project that is highly requested by customers and will enable both reaching new customers and improving the whole product."
            </p>
            <p>
                "You'll start this project from scratch together with a back-end developer whom is being recruited at the same time as you. You'll get to make the technical decisions to make this a success, while having no tech debt to hold you back, and you'll have the whole team from the current software to rely on for assistance. You won't be in a silo."
            </p>
            <p>
                "The project will be led by the same Product Owner whom designed and led the development of the current software, so you'll have ample access to domain knowledge, clear feature requirements, and customer understanding."
            </p>
            <p>
                "You'll build a user-friendly, performant, and well-built front-end for designing retails signs (think " <a href="www.canva.com">"www.canva.com"</a>")."
            </p>
            <p>
                "The front-end will be written in TypeScript (probably React, but we'll decide on that together) and we're open to different languages on the back-end (we'll prioritize finding a great candidate and then use the language that they're most comfortable with)."
            </p>
            <p>
                "We think we've got a rare opportunity to write a great piece of software from scratch which will quite literally be the best of its kind in the country (and perhaps carry the company into other markets as well). We're looking forward to your application!"
            </p>
            <h3>"Are you a great fit?"</h3>
            <ul>
                <li>"3+ years of professional front-end development experience"</li>
                <li>"Comfortable with making and owning long-term technical decisions"</li>
                <li>"Experience of building complex but user-friendly UIs"</li>
                <li>"You think performance is a feature, not an afterthought"</li>
                <li>"Comfortable with CI & CD, containers, Linux, and AWS or other IaaS"</li>
                <li>"Good communication and collaboration skills"</li>
                <li>"A drive and desire to make what you're working on better"</li>
                <li>"You think customer feedback and solving real issues is more fulfilling than jumping on the next framework"</li>
            </ul>
            <h3>"Additional skills"</h3>
            <ul>
                <li>
                    "Deeper knowledge of UX/UI design, user testing etc."
                </li>
                <li>
                    "Graphical design (if you've used graphical design software a lot, you'll understand the needs of our users when they design a retail sign)"
                </li>
                <li>
                    "Full-stack or an interest to broaden beyond the front-end"
                </li>
            </ul>
            <h3>"Basic information"</h3>
            <dl>
                <dt>"Employment:"</dt>
                <dd>"Full-time"</dd>
                <dt>"Location:"</dt>
                <dd>"Tyresö"</dd>
                <dt>"Remote policy:"</dt>
                <dd>"Hybrid (up to 50 % remote)"</dd>
                <dt>"Start date:"</dt>
                <dd>"As soon as possible (0-3 months)"</dd>
                <dt>"Swedish Collective agreement (Kollektivavtal):"</dt>
                <dd>"Yes"</dd>
            </dl>
            <h3>"Apply now"</h3>
            <p>
                "Send an email to "
                <a href="mailto:niclas@instateam.se">"niclas@instateam.se"</a>
                " and mention which role you're applying for, and include something that gives me a clue of your history (e.g. CV, LinkedIn, git forge with relevant projects). We'll book a time for a call and take it from there!"
            </p>
            <p>
                "Candidates are being interviewed right now so please don't delay your application. It'd be a pity if we didn't get a chance to talk to you 👌"
            </p>
            <p class="JobDescription-contact">
                <strong>"Niclas Åhdén"</strong>
                <span>"Interim IT Manager"</span>
                <a href="mailto:niclas@instateam.se">"niclas@instateam.se"</a>
            </p>
        </section>
    }
}
