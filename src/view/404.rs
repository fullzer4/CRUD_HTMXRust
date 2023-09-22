#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND)
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
