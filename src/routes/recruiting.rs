#![allow(unused_variables)]
use leptos::html::*;
use leptos::*;
use leptos_router::*;
pub mod back_end;
pub mod front_end;
pub mod thank_you;

#[component]
pub fn OpenPositions() -> impl IntoView {
    view! {
        <section class="Recruiting">
            <h2>"Open positions (on behalf of clients)"</h2>
            <p>
                "Are you looking for a new challenge? I'm currently looking to fill these positions on behalf of clients. I'm happy to share more information about the roles and the companies in response to your application."
            </p>
            <p>"Which role fits you best?"</p>
            <ul class="JobListings">
                <li>
                    <JobListing
                        title="Front-end developer — TypeScript"
                        href="/recruiting/typescript-front-end-developer#top"
                    />
                </li>
                <li>
                    <JobListing
                        title="Back-end developer"
                        href="/recruiting/back-end-developer#top"
                    />
                </li>
            </ul>
            <p>
                "The roles will be filled as soon as possible and I'm actively talking to candidates now. I don't want to miss out on talking to you, so please don't hold off on applying."
            </p>
            <p>
                "Don't worry about perfecting your application, you won't be judged by it. I'll contact you to find a time to talk, and you'll have ample opportunity to convey your experience and skills. Just send it 👌"
            </p>
        </section>
    }
}

#[component]
fn JobListing(#[prop(into)] title: String, #[prop(into)] href: String) -> impl IntoView {
    view! {
        <a class="JobListing" href=href>
            <h3 class="JobListing-title">{title}</h3>
            <ul class="JobListing-details">
                <li class="JobListing-time">"Full-time"</li>
                <li class="JobListing-location">"Tyresö"</li>
                <li class="JobListing-remote">"Hybrid (up to 50 % remote)"</li>
            </ul>
        </a>
    }
}


/*
// TODO: All code below is currently unused. Implement form handling so that the forms are usable
// (currently they log an Application to stdout, but that's not terribly useful). Either persist
// the applications in a database and notify me somewhow, or send the applications as emails
// synchronously (so that we can display an error in case it didn't work).
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Role {
    FrontEnd,
    BackEnd,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Application {
    role: Role,
    name: String,
    email: String,
    text: String,
}

#[component]
fn ApplicationForm(role: Role) -> impl IntoView {
    let name_input: NodeRef<Input> = create_node_ref();
    let email_input: NodeRef<Input> = create_node_ref();
    let text_input: NodeRef<Textarea> = create_node_ref();

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name =
            name_input()
            .expect("name input to exist")
            .value();
        let email =
            email_input()
            .expect("email input to exist")
            .value();
        let text =
            text_input()
            .expect("text input to exist")
            .value();
        let application = Application {
            role: role.clone(),
            name,
            email,
            text,
        };
        spawn_local(async {
            let _ = apply_for_role(application).await;
        });
    };

    view! {
        <form class="ApplicationForm" on:submit=on_submit>
            <Input type_="text" label="Namn" required=true node_ref=name_input/>
            <Input type_="email" label="E-post" required=true node_ref=email_input/>
            <Textarea label="Meddelande" node_ref=text_input/>
            <input class="ApplicationForm-submit" type="submit" value="Skicka ansökan"/>
        </form>
    }
}

#[component]
fn Input(
    #[prop(into)] label: String,
    #[prop(into)] type_: String,
    required: bool,
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <label class="ApplicationForm-label">
            {label}
            <input
                class="ApplicationForm-input"
                prop:type=type_
                prop:required=required.then_some("required")
                node_ref=node_ref
            />
        </label>
    }
}

#[component]
fn Textarea(#[prop(into)] label: String, node_ref: NodeRef<Textarea>) -> impl IntoView {
    view! {
        <label class="ApplicationForm-label">
            {label} <textarea class="ApplicationForm-textarea" node_ref=node_ref></textarea>
        </label>
    }
}

#[server(ApplyForRole, "/api")]
pub async fn apply_for_role(application: Application) -> Result<(), ServerFnError> {
    // TODO: Persist the application.
    println!("Application received: {:?}", application);
    Ok(())
}
*/
