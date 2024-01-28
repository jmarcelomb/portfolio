mod app;

use app::*;
use leptos::*;


#[component]
fn TestHome() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
            <button
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
    }
}


pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    // mount_to_body(|| {
    //     view! { <App /> }
    // });
    mount_to_body(|| {
        view! { <TestHome /> }
    });
}
