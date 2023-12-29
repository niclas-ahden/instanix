use leptos::*;

#[component]
pub fn AvoidingDangerUsingTypes()  -> impl IntoView {
    view! {
        <section class="BlogPost">
            <h2>"Avoiding danger using types"</h2>
            <span class="Tag">Development</span>
            <span class="Tag">Haskell</span>
            <p>
                "I work on a system which continuously spends real money without user input. It creates social media ads and bugs could have severe real-life impact (gross overspending, spamming infinite ads, brand damage etc.)"
            </p>

            <p>
                "What would you do if you caused a bug that erroneously spent $100 000? How could you prevent those bugs and how far would you go?"
            </p>

            <p>
                "We could use spending caps, monitoring, and other external protections, but how do we change our way of writing code to protect ourselves?"
            </p>

            <h3>"An example of danger"</h3>

            <p>
                "Most ad platforms ask you to set an account-wide currency. This means you won't be specifying a currency when you actually buy an ad. That makes it easy for end-users, but could trip you up as an API user dealing with different accounts."
            </p>
            <p>
                "For example, Meta's Ad API looks roughly like this:"
            </p>

            <pre class="Code">
                <span class="Highlight-function">"createAd"</span>
                " :: "
                <span class="Highlight-type">"Account"</span>
                " -> "
                <span class="Highlight-type">"Int"</span>
                " -> "
                <span class="Highlight-type">"AdContent"</span>
                " -> "
                <span class="Highlight-type">"_"</span>
                <br />
                <span class="Highlight-function">"createAd"</span>
                " account budget adContent = [...]"
            </pre>

            <p>
                "You provide the account, the budget, and the content of the ad."
            </p>
            <p>
                "One day a user wants an ad for 10 000 SEK, so we call the API with a budget of 10 000. The next day we see that over 90 000 SEK has been spent ... what?!"
            </p>
            <p>
                "Oh, our user had their account set to USD, so we ordered an ad for 9 times more than we wanted. Ouch!"
            </p>
            <p>
                "How do we avoid this? Check the currency of the account before creating each ad?"
            </p>
            <p>
                "Let's write a function for that:"
            </p>


            <pre class="Code">
                <span class="Highlight-function">"isAccountUsingSEK"</span>
                " :: "
                <span class="Highlight-type">"Account"</span>
                " -> "
                <span class="Highlight-type">"Bool"</span>
                <br />
                <span class="Highlight-function">"isAccountUsingSEK"</span>
                " account =
    "
    <span class="Highlight-control">"case"</span>
    " accountCurrency account "
    <span class="Highlight-control">"of"</span>
    <span class="Highlight-comment">" -- Imaginary function"</span>
    "
        "
        <span class="Highlight-string">"\"SEK\""</span>
        " -> True
        _ -> False"
            </pre>

            <p>
                "OK, so we always want to run "
<span class="Highlight-inline Highlight-function">isAccountUsingSEK</span>
" before "
<span class="Highlight-inline Highlight-function">createAd</span>
"."
            </p>

            <p>
                "But what if we forget? Six months from now, when we're in a hurry, will we remember?"
            </p>

            <p>
                "Should we perform the currency check inside "
<span class="Highlight-inline Highlight-function">createAd</span>
" to ensure it always happens? We could, but then we're extending "
<span class="Highlight-inline Highlight-function">createAd</span>
"'s responsibility and making testing more difficult. How else can we ensure it's always checked?"
            </p>

            <h3>
                "Types to the rescue"
            </h3>

            <p>
                "We could change the signature of "
                <span class="Highlight-inline Highlight-function">createAd</span>
                " so that it no longer takes an "
                <span class="Highlight-inline Highlight-type">Account</span>
                " — it takes an "
                <span class="Highlight-inline Highlight-type">AccountUsingSEK</span>
                "."
            </p>
            <p>
                "We could then ensure that the "
                <em>only</em>
                " way to create an "
                <span class="Highlight-inline Highlight-type">AccountUsingSEK</span>
                " is by calling "
                <span class="Highlight-inline Highlight-function">isAccountUsingSEK</span>
                ". This neatly wraps up the whole problem:"
            </p>

            <pre class="Code">
                <span class="Highlight-control">"data"</span>
                " "
                <span class="Highlight-type">"AccountUsingSEK"</span>
" = {"
                <span class="Highlight-comment">" -- Hide this constructor from createAd"</span>
"
    id: "<span class="Highlight-type">"Text"</span>",
    [...]
}

"

                <span class="Highlight-function">"isAccountUsingSEK"</span>
                " :: "
                <span class="Highlight-type">"Account"</span>
                " -> "
                <span class="Highlight-type">"Maybe AccountUsingSEK"</span>
                <br />
                <span class="Highlight-function">"isAccountUsingSEK"</span>
                " account =
    "
    <span class="Highlight-control">"case"</span>
    " accountCurrency account "
    <span class="Highlight-control">"of"</span>
    <span class="Highlight-comment">" -- Imaginary function"</span>
    "
        "
        <span class="Highlight-string">"\"SEK\""</span>
        " -> "
        <span class="Highlight-type">"Just AccountUsingSEK"</span>
" { id: account.id }
        _ -> "
        <span class="Highlight-type">"Nothing"</span>
"

"
                <span class="Highlight-function">createAd</span>
                " :: "
                <span class="Highlight-type">AccountUsingSEK</span>
                " -> "
                <span class="Highlight-type">Int</span>
                " -> "
                <span class="Highlight-type">AdContent</span>
                " -> "
                <span class="Highlight-type">_</span>
                <br />
                <span class="Highlight-function">createAd</span>
                " account budget adContent = [...]"

            </pre>

            <p>
                "It's now impossible to call "
<span class="Highlight-inline Highlight-function">createAd</span>
" without checking the currency of the account. If we test "
<span class="Highlight-inline Highlight-function">isAccountUsingSEK</span>
" thoroughly, and ensure that there's no other public constructor of "
<span class="Highlight-inline Highlight-type">AccountUsingSEK</span>
", we have provided a lot of confidence in the system."
            </p>

            <p>
                "It's a small change, and it may seem trivial, but it greatly reduces risk. Some argue that it's costly in terms of performance and complexity. YMMV, but to me, on this project, it's a bargain given the importance of correctness."
            </p>
            <p>
                <em>The code above is pseudo-Haskell and the types have been simplified for clarity.</em>
            </p>
        </section>
    }
}
