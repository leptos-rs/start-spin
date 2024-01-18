use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
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
        <picture class="img">
            <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
            <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400" />
        </picture>

        <ActionForm action=increment_count>
            <button >"Click Me: " {move || count.get()}</button>
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
