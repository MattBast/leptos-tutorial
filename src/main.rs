use leptos::*;

// import modules
mod button;
use button::Button;
mod progressbar;
use progressbar::ProgressBar;
mod iteration;
use iteration::{ StaticList, DynamicList };
mod inputs;
use inputs::{ Controlled, Uncontrolled, TextArea, Select };
mod error_handler;
use error_handler::ErrorHandler;
mod parent;
use parent::Parent;
mod html_children;
use html_children::WrapChildren;
mod todo;
use todo::ToDoApp;
mod async_load;
use async_load::{ AsyncLoad, LoadTwoServices };

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    // Returns a getter and setter for a variable whose value can change.
    // Signals are Leptos' way of managing these variables in the front end
    let (count, set_count) = create_signal(0);

    let large_count_msg = move || if count.get() > 5 {
        "Large"
    } else {
        "Small"
    };

    // this is a derived signal that is listening to the parent signal, `count`
    let double_count = move || count.get() * 2;

    // A static list of values (the values don't have to be static, just the list)
    let static_values = vec![0, 1, 2];

    view! {
        // create a button that increments the count when clicked
        <Button set_count=set_count count=count/>

        // add a break between elements
        <br/>

        // add a progress bar also using the count signal
        <ProgressBar progress=count/>

        <br/>

        // add a progress bar also using the count signal
        <ProgressBar progress=Signal::derive(double_count)/>

        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>

        // this is one way of using a control flow to decide what element to display
        
        <Show
            when=move || { count.get() > 5 }
            fallback=|| view! { <p>"Small"</p> }
        >
            <p>"Large"</p>
        </Show>

        // an if statement or a match statement would also work
        <p>{large_count_msg}</p>

        <StaticList static_values=static_values/>

        <DynamicList/>

        <Controlled/>

        <Uncontrolled/>

        <TextArea/>

        <Select/>

        <br/><br/>

        <ErrorHandler/>

        <Parent/>

        <WrapChildren>
            <Button set_count=set_count count=count/>
            "A"
            <Button set_count=set_count count=count/>
            "B"
            <Button set_count=set_count count=count/>
            "C"
        </WrapChildren>

        <ToDoApp/>

        <AsyncLoad/>

        <LoadTwoServices/>
        
    }
}