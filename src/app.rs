use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::pages::about;
use super::pages::cv;
use super::pages::home;
use super::pages::not_found;
use super::components::navbar;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div id="root">
            <Router
                base="/portfolio"
                fallback=move || view! { <not_found::PageNotFound></not_found::PageNotFound> }
            >
                <navbar::NavBar />
                // // <nav> and <main> will show on every route
                // <nav>
                //     <A href="">"Home"</A>
                //     <A href="cv" class="my-class">
                //         "CV"
                //     </A>
                //     <A href="about" class="about">
                //         "About"
                //     </A>
                // </nav>
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
