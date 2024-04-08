use leptos::*;

/// Parent-Child communication can be achieved via passing `WriteSignal`s as
/// arguments into components. This can become messy when there are multiple
/// layers of children. In this case, we can use a `context`. This is also more
/// performant because the variable doesn't need to be loaded into the in-between
/// layers.
#[component]
pub fn Parent() -> impl IntoView {
    
    // this is the boolean variable we'll be passing to the grandchild component
    let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);

    view! {
        <p>"Toggled: " {toggled}</p>
        <Child/>
    }

}


#[component]
fn Child() -> impl IntoView {
    
    view! {
        <br/>
        <Grandchild/>
        <br/>
    }

}


#[component]
fn Grandchild() -> impl IntoView {
    
    // use_context searches up the context tree, hoping to find a `WriteSignal<bool>`. 
    // In this case we're looking for `set_toggled`. Error handling is required in
    // case the setter isn't found.
    let setter = use_context::<WriteSignal<bool>>()
        .expect("to have found the setter provided");

    view! {
        <button
            // function to toggle the boolean between true and false
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Click to toggle"
        </button>
    }

}