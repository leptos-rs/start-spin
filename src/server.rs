use leptos::ServerFn;
use leptos_spin::{render_best_match_to_stream, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_{{project-name}}(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "{{project-name}}".to_owned();

    // Register server functions
    crate::pages::home::GetCount::register_explicit().unwrap();
    crate::pages::home::UpdateCount::register_explicit().unwrap();

    let app_router = crate::routes::AppRouter;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, app_fn, &conf.leptos_options).await
}
