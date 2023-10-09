use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_|
        set_count.update(|count| {
            *count += 1;
        });

    view! {
      <div class="w-full m-4">

        <h1 class="text-2xl">"Full-stack Rust boilerplate"</h1>


        <p class="text-lg font-extrabold">
        <span class="text-red-700">"Rust "</span>
        "with"
        <span class="text-purple-800">" Webassembly"</span>
        </p>

        <p class="font-serif">"Actix"</p>
        <p class="font-serif">"Leptos"</p>
        <p class="font-serif">"TailwindCSS"</p>

        <button class="mt-4 bg-slate-500 px-4 py-2 rounded-full transition-all hover:scale-90" on:click=on_click>"Click Me: " {count}</button>

      </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
