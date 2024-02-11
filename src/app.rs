use leptos::*;
use leptos_darkmode;
use leptos_meta::*;
use leptos_router::*;

use super::components::navbar;
use super::pages::about;
use super::pages::cv;
use super::pages::home;
use super::pages::not_found;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let darkmode = leptos_darkmode::Darkmode::init();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="assets/images/logo_circle.png"/>
        // This set's the darkmode tag on the html class, if the darkmode is enabled
        <Html lang="en" class=move || if darkmode.is_dark() { "dark" } else { "" }/>
        // Set your tailwindcss theme, for example the background color of the body
        <Body class="bg-white dark:bg-gray-800"/>

        <div id="root">
            <Router
                base="/portfolio"
                fallback=move || view! { <not_found::PageNotFound></not_found::PageNotFound> }
            >
                <navbar::NavBar></navbar::NavBar>
                <main>
                    <Routes base="/portfolio".to_string()>
                        <Route path="/" view=move || view! { <home::Home></home::Home> }/>
                        <Route path="/cv" view=move || view! { <cv::Home></cv::Home> }/>
                        <Route path="/about" view=about::About/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
