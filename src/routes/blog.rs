use leptos::*;
use leptos_router::*;

pub mod avoiding_danger_using_types;

struct Article {
    title: String,
    slug: String,
    ingress: String,
}

#[component]
pub fn Blog() -> impl IntoView {
    let articles = vec![
        Article {
            title: "Avoiding danger using types".to_string(),
            slug: "avoiding-danger-using-types".to_string(),
            ingress:
                "Use types to enforce validation rules, checks, and generally tighten constraints."
                    .to_string(),
        },
        Article {
            title: "Docker inline caching".to_string(),
            slug: "avoiding-danger-using-types".to_string(),
            ingress: "Caching dependencies in Docker...".to_string(),
        },
        Article {
            title: "Ruby → Sorbet → Haskell → Rust".to_string(),
            slug: "avoiding-danger-using-types".to_string(),
            ingress: "When I see a tomato there is much that I can doubt.".to_string(),
        },
        Article {
            title: "Leptos language server in neovim".to_string(),
            slug: "avoiding-danger-using-types".to_string(),
            ingress: "...".to_string(),
        },
    ];

    view! {
        <section class="Blog">
            <ul>
                {articles
                    .into_iter()
                    .map(|a| {
                        view! {
                            <li>
                                <h3>
                                    <A href=format!("/blog/{0}#top", a.slug)>
                                        {a.title}
                                    </A>
                                </h3>
                                <p>{a.ingress}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
