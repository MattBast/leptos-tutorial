use leptos::*;

/// Children can be nested in components much like you can with native HTML.
/// Functionality can then be provided to manipulate those children. Here they
/// get wrapped in <li> tags and added to a <ul> list.
#[component]
pub fn WrapChildren(children: Children) -> impl IntoView {
    
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <ul>{children}</ul>
    }

}