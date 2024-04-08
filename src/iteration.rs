use leptos::*;
use rand::Rng;

#[component]
pub fn StaticList(static_values: Vec<i32>,) -> impl IntoView {
    
    view! {
        
        <ul>
            {
                static_values.into_iter()
                    .map(|n| view! { <li>{n}</li> })
                    .collect_view()
            }
        </ul>

    }
}

/// Define a fake database entry (row of data) to be displayed in a list
#[derive(Debug, Clone, PartialEq)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

impl DatabaseEntry {

    // Create a new database entry weith random values
    fn new() -> DatabaseEntry {
        DatabaseEntry { 
            key: rand::random::<u8>().to_string(),
            value: create_rw_signal(rand::thread_rng().gen_range(0..100)) 
        }
    }
}

#[component]
pub fn DynamicList() -> impl IntoView {
    
    // create some rows of data
    let (data, set_data) = create_signal(vec![
        DatabaseEntry::new(),
        DatabaseEntry::new(),
        DatabaseEntry::new(),
    ]);
    
    view! {
        
        // when we click, add a row
        <button on:click=move |_| {
            set_data.update(move |entry| {
                entry.push(DatabaseEntry::new())
            });
        }>
            "Add Value"
        </button>

        // when we click, update each row, doubling its value
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
        }>
            "Update Values"
        </button>
        
        // iterate over the rows and display each value
        <ul>
            <For
                each=move || data.get()
                key=|state| state.key.clone()
                children=move |child| {
                    view! {
                        
                        <li>
                            // Create a button for each entry. Delete button when it's clicked.
                            <button
                                on:click=move |_| {
                                    set_data.update(|data| {
                                        data.retain(|entry| entry.key != child.key)
                                    });
                                }
                            >
                                {child.value}
                            </button>
                        </li>

                    }
                }
            >
            </For>
        </ul>

    }
}