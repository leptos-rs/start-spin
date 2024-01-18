use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/benwis_spin.css"/>

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
    let inc_count = create_server_action::<AdjustCount>();
    let count = create_resource(
        move || {
            (
                inc_count.version().get(),
                // clear.version().get(),
            )
        },
        |_| get_count(),
    );

    view! {
        <h1>"Here's my counter, so click me baby!"</h1>
        <h3>"Leptos served on Spin with SSR and Server Functions!"</h3>
        <ActionForm action=inc_count>
            <button >"Click Me: " {move || count.get()}</button>
        </ActionForm>
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
        let resp = expect_context::<leptos_spin::ResponseOptions>();
        resp.set_status(404);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(AdjustCount, "/api")]
pub async fn adjust_count() -> Result<(), ServerFnError> {
    println!("Adding 1 ah aha h");
    let store = spin_sdk::key_value::Store::open_default()?;
    let count: u64 = store
        .get_json("benwis_spin_count")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .unwrap_or_default();
    let new = count + 1;
    store
        .set_json("benwis_spin_count", &new)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}

#[server(GetCount, "/api")]
pub async fn get_count() -> Result<u64, ServerFnError> {
    let store = spin_sdk::key_value::Store::open_default()?;
    let stored_count: u64 = store
        .get_json("benwis_spin_count")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .ok_or_else(|| ServerFnError::ServerError("Failed to get count".to_string()))?;
    println!("Got stored {stored_count}");
    Ok(stored_count)
}
