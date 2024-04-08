use leptos::*;

/// There isn't any need to use the `view!` macro if it's confusing to look
/// at both Rust and HTML code in the same file (as well as not been able to
/// benefit from all of Rusts dev tools). Instead the code underlying the macros
/// can be used. 

/// It's worth noting though that a bunch of optimizations make the `view!` approach
/// more performant. Not that the builder syntax is slow, just not as fast as `view!`.

// The builder and the view! syntax can't be used together. One must be picked. Hence
// this compoent hasn't been imported into the main.rs file.

//The first thing to notice is the absence of the #[compoent] macro.
pub fn no_macro() -> impl IntoView {

	// signals are still created in the same way
	let (count, set_count) = create_signal(0);

	// all html elements like divs have corresponding functions. Children
	// can be added with the `child()` function which takes either a single
	// child or a tuple of children (like this example does).
	div().child((

		// this button has an event listener added to it via the `ev` function
		button()
			.on(ev::click, move |_| set_count.update(|count| *count = 0))
			.child("Clear"),


		button()
			.on(ev::click, move |_| set_count.update(|count| *count -= 1))
			.child("-1"),

		button()
			.on(ev::click, move |_| set_count.update(|count| *count += 1))
			.child("+1"),

		// attributes, classes, properties and styles can be added via the
		// `attr()`, `class()`, `prop()` and `style()` functions.
		p()
			.attr("id", "foo")
			.class("classname")
			.style("color", "blue")
			.child(count.get()),

	))
}