use leptos::*;
use leptos_router::*;

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
mod search;
use search::SearchPage;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    
    view! {
        // if using routes, everything should go inside the `Router` tags
        <Router>
            <nav>
                // A tags can be used to navigate to different pages
                <A href="/">"Home"</A>
                <A href="/conditionals">"Conditionals"</A>
                <A href="/lists">"Lists"</A>
                <A href="/inputs">"Inputs"</A>
                <A href="/hierarchy">"Hierarchy"</A>
                <A href="/todo">"To Do"</A>
                <A href="/load_data">"Load Data"</A>
                <A href="/contacts">"Contacts"</A>
                <A href="/search">"Search"</A>
            </nav>
            <main>
                // All routes go here. If there are component elements that belong
                // on all pages (like navigation bars), put them outside the `Routes` 
                // tags
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/conditionals" view=ConditionalPage/>
                    <Route path="/lists" view=ListsPage/>
                    <Route path="/inputs" view=InputsPage/>
                    <Route path="/hierarchy" view=HierarchyPage/>
                    <Route path="/todo" view=ToDoPage/>
                    <Route path="/load_data" view=LoadDataPage/>
                    // Routes can be nested to help Leptos decide what parts of a page to 
                    // render and re-render. For instance, this contacts list loads a list
                    // and then loads different people as sub-sections of the page.
                    <Route path="/contacts" view=ContactList>
                        // This nested child looks for an id in the url like this: `/contacts/alice.`
                        // The id is picked up and used in the ContactInfo component.
                        <Route path=":id" view=ContactInfo/>
                        // If no id specified, fall back
                        <Route path="" view=|| view! {
                            <div class="select-user">
                                "Select a user to view contact info."
                            </div>
                        }/>
                    </Route>
                    <Route path="/search" view=SearchPage/>
                </Routes>
            </main>
        </Router>
        
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Returns a getter and setter for a variable whose value can change.
    // Signals are Leptos' way of managing these variables in the front end
    let (count, set_count) = create_signal(0);

    // this is a derived signal that is listening to the parent signal, `count`
    let double_count = move || count.get() * 2;

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
        
    }
}

#[component]
fn ConditionalPage() -> impl IntoView {
    
    let (count, set_count) = create_signal(0);

    let large_count_msg = move || if count.get() > 5 {
        "Large"
    } else {
        "Small"
    };

    view! {

        // create a button that increments the count when clicked
        <Button set_count=set_count count=count/>

        // this is one way of using a control flow to decide what element to display
        <Show
            when=move || { count.get() > 5 }
            fallback=|| view! { <p>"Small"</p> }
        >
            <p>"Large"</p>
        </Show>

        // an if statement or a match statement would also work
        <p>{large_count_msg}</p>

        <ErrorHandler/>
        
    }
}


#[component]
fn ListsPage() -> impl IntoView {
    
    // A static list of values (the values don't have to be static, just the list)
    let static_values = vec![0, 1, 2];

    view! {

        <StaticList static_values=static_values/>

        <DynamicList/>
        
    }
}


#[component]
fn InputsPage() -> impl IntoView {

    view! {

        <Controlled/>

        <Uncontrolled/>

        <TextArea/>

        <Select/>
        
    }
}


#[component]
fn HierarchyPage() -> impl IntoView {

    let (count, set_count) = create_signal(0);

    view! {

        <Parent/>

        <WrapChildren>
            <Button set_count=set_count count=count/>
            "A"
            <Button set_count=set_count count=count/>
            "B"
            <Button set_count=set_count count=count/>
            "C"
        </WrapChildren>
        
    }
}


#[component]
fn ToDoPage() -> impl IntoView {

    view! {

        <ToDoApp/>
        
    }
}


#[component]
fn LoadDataPage() -> impl IntoView {

    view! {

        <AsyncLoad/>

        <LoadTwoServices/>
        
    }
}


#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div>
            <div>
                <h3>"Contacts"</h3>
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>

            // <Outlet/> tells the parent (ContactList) where
            // to render child components (in this case ContactInfo). 
            <Outlet/>
        </div>
    }
}


#[component]
fn ContactInfo() -> impl IntoView {

    // We can access the :id param reactively with `use_params_map`.
    // There's a similar one called `use_query_map` for url queries.
    // Both of these functions return untyped values. `use_query` and
    // `use_params` return typed values.
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! {

        <div>
            <h4>{id}</h4>
            // some sort of API call could have been made to populate the rest of
            // this page with data.
        </div>
        
    }
}