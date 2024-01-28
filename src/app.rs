use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home increment=5/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
    <div class="container bg-gray-500 items-center">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

        <h1 class="text-cyan-500">"Welcome to Leptos"</h1>
        <h2><i class="text-green-600">"On Github Pages"</i></h2>

        <button class="bg-yellow-600 rounded-xl m-5"
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: "
            {count}
        </button>


    </div>
    }
}