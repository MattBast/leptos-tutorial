use leptos::*;
use leptos_router::*;

async fn fetch_results(search_query: String) -> String {
    if search_query.is_empty() {
        String::new()
    }
    else {
        String::from("https:/website.com/") + &search_query
    }
}

#[component]
pub fn SearchPage() -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map();
    // search stored as ?q=
    // let search = move || query.get("q").cloned().unwrap_or_default();
    let search = move || query.with(|query| query.get("q").cloned().unwrap_or_default());
    // a resource driven by the search string
    let search_results = create_resource(search, fetch_results);

    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search/>
            <input type="submit"/>
        </Form>
        <Transition fallback=move || ()>
            <p>{search_results.get()}</p>
        </Transition>
    }
}