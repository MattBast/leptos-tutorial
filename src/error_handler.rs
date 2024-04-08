use leptos::*;

/// Use the Result type to identify when an error is raised. Wrap the
/// html elements in an ErrorBoundary tag to handle the error.
#[component]
pub fn ErrorHandler() -> impl IntoView {
	
	let (value, set_value) = create_signal(Ok(0));

	// when input changes, try to parse a number from the input
	let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

	view! {
		<label>
            "Type a number (or something that's not a number!)"
            <input on:input=on_input/>
            
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|_errors| view! {
                    <div class="error">
                        <p>"You did not type a number!"</p>
                    </div>
                }
            >
                <p>
                    "You entered "
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>

        </label>
	}
}