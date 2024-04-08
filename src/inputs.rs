use leptos::*;

/// A controlled input is an input whose value is monitored by a signal.
/// When the user updates the value, an event is fired. In this case the 
/// value is inserted into a <p> element.
#[component]
pub fn Controlled() -> impl IntoView {
    
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input=move |ev| {
                set_name.set(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }

}

/// An uncontrolled input waits for the user to ask for the value of the input
/// to be submitted. The NodeRef type is used to access the input value when
/// the user requests it.
#[component]
pub fn Uncontrolled() -> impl IntoView {

    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();

    // this function defines what happens when the user clicks on submit
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // prevents the page from reloading (default form behaviour)
        ev.prevent_default();

        // get the value typed into the input
        let value = input_element.get().expect("<input> should be mounted").value();

        // set a new value for the signal (name)
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit> // on_submit is defined below
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }

}

/// Textareas work a little different as they don't have a "value" field. Instead
/// a "prop" is used as a placeholder while an untracked value stands in for the
/// initial value.
#[component]
pub fn TextArea() -> impl IntoView {
    
    let (text, set_text) = create_signal("Textarea".to_string());

    view! {
        <textarea
            prop:value=move || text.get()
            on:input=move |ev| {
                set_text.set(event_target_value(&ev));
            }
        >
            /* plain-text initial value, does not change if the signal changes */
            {move || text.get_untracked()}
        </textarea>

        <p>"Text is: " {text}</p>
    }

}

/// A dropdown for selecting pre-defined values
#[component]
pub fn Select() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());
    view! {
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value.set(new_value);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value.get() == is
        >
            {is}
        </option>
    }
}