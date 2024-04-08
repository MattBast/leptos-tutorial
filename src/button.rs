use leptos::*;

/// A button that increments count and changes to red when the value is odd.
#[component]
pub fn Button(
    /// A setter to allow for count to be updated.
    set_count: WriteSignal<i32>,
    /// Get the latest count.
    count: ReadSignal<i32>
) -> impl IntoView {
    view! {
        // create a button that increments the count when clicked
        <button
            // add click listener to update count when the button is clicked
            on:click=move |_| {
                 set_count.update(|n| *n += 1);
            }
            // add class listener that adds the `red` class when the count is odd
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}