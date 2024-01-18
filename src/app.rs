use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::notfound::NotFound;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let increment_count = create_server_action::<UpdateCount>();

    let count = create_resource(
        move || {
            (
                increment_count.version().get(),
                // clear.version().get(),
            )
        },
        |_| get_count(),
    );

    view! {
        <picture>
            <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
            <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" />
        </picture>

        <ActionForm action=increment_count>
            <button >"Click Me: " {move || count()}</button>
        </ActionForm>
    }
}

#[server(UpdateCount, "/api")]
pub async fn update_count() -> Result<(), ServerFnError> {
    println!("Upated count");

    let store = spin_sdk::key_value::Store::open_default()?;

    let count: u64 = store
        .get_json("{{project-name}}_count")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .unwrap_or_default();

    let updated_count = count + 1;

    store
        .set_json("{{project-name}}_count", &updated_count)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}

#[server(GetCount, "/api")]
pub async fn get_count() -> Result<u64, ServerFnError> {
    let store = spin_sdk::key_value::Store::open_default()?;

    let stored_count: u64 = store
        .get_json("{{project-name}}_count")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .ok_or_else(|| ServerFnError::ServerError("Failed to get count".to_string()))?;

    println!("Got stored {stored_count}");

    Ok(stored_count)
}
