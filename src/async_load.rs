use leptos::*;
use gloo_timers::future::TimeoutFuture;

/// Mocks calling an external service and waiting for a response
#[component]
pub fn AsyncLoad() -> impl IntoView {
    
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // This is the source signal. A future is created everytime it changes.
        move || count.get(),
        // This function makes the call to an external service when the signal changes.
        |value| async move { load_data_10(value).await },
    );

    // Keep track of whether data has been loaded or is currently loading
    let is_loading = move || if async_data.loading().get() { "Loading..." } else { "Idle." };

    view! {
        
        <button
            on:click=move |_| {
                 set_count.update(|n| *n += 1);
            }
        >
            "Click to load data"
        </button>

        <p>"Raw count: " {move || count.get()}</p>

        {
            move || match async_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => view! { <p>"Data from service: " {data}</p> }.into_view()
            }
        }

        <p>{is_loading}</p>

    }

}

/// This function mocks calling an external service.
/// It returns a the number passed multiplied by ten 
/// after a one second delay.
async fn load_data_10(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value * 10
}

/// Same as function above except it takes longer to resolve
/// and multiplies by 20.
async fn load_data_20(value: i32) -> i32 {
    TimeoutFuture::new(1_500).await;
    value * 20
}

/// Uses the Suspense tag to load two external services
#[component]
pub fn LoadTwoServices() -> impl IntoView {
    
    let (count, set_count) = create_signal(0);
    let (count_2, set_count_2) = create_signal(0);

    let data_a = create_resource(
        move || count.get(),
        |value| async move { load_data_10(value).await },
    );

    let a_is_loading = move || if data_a.loading().get() { "Loading..." } else { "" };

    let data_b = create_resource(
        move || count_2.get(),
        |value| async move { load_data_20(value).await },
    );

    let b_is_loading = move || if data_b.loading().get() { "Loading..." } else { "" };

    view! {
        
        <button
            on:click=move |_| {
                 set_count.update(|n| *n += 1);
                 set_count_2.update(|n| *n += 1);
            }
        >
            "Click to load data"
        </button>


        <Transition
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || {
                data_a.get().map(|data| view! {<p>"Data a: " {data} " " {a_is_loading}</p>})
            }}
            {move || {
                data_b.get().map(|data| view! {<p>"Data b: " {data} " " {b_is_loading}</p>})
            }}
        </Transition>

    }

}