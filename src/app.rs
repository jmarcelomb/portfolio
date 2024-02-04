use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::routes::cv;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div id="root">
            <Router base="/portfolio" fallback=move || view! { <PageNotFound/> }>
                // <nav> and <main> will show on every route
                <nav>
                    <A href="">"Home"</A>
                    <A href="cv" class="my-class">
                        "CV"
                    </A>
                    <A href="about" class="about">
                        "About"
                    </A>
                </nav>
                <main>
                    <Routes base="/portfolio".to_string()>
                        <Route path="/" view=move || view! { <HomeRoot/> }/>
                        <Route path="/cv" view=move || view! { <cv::Home></cv::Home> }/>
                        <Route path="/about" view=About/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
  view! { <p>"About Marcelo Page :D"</p> }
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-gray-900 ">
            <div class="container flex items-center min-h-screen px-6 py-12 mx-auto">
                <div class="flex flex-col items-center max-w-sm mx-auto text-center">
                    <p class="p-3 text-sm font-medium text-blue-500 rounded-full bg-blue-50 dark:bg-gray-800">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="2"
                            stroke="currentColor"
                            class="w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
                            ></path>
                        </svg>
                    </p>
                    <h1 class="mt-3 text-2xl font-semibold text-gray-800 dark:text-white md:text-3xl">
                        "Page not found"
                    </h1>
                    <p class="mt-4 text-gray-500 dark:text-gray-400">
                        "The page you are looking for doesn't exist. Here are some helpful links:"
                    </p>

                    <div class="flex items-center w-full mt-6 gap-x-3 shrink-0 sm:w-auto">
                        <button class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="w-5 h-5 rtl:rotate-180"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M6.75 15.75L3 12m0 0l3.75-3.75M3 12h18"
                                ></path>
                            </svg>

                            <span>"Go back"</span>
                        </button>

                        <button class="w-1/2 px-5 py-2 text-sm tracking-wide text-white transition-colors duration-200 bg-blue-500 rounded-lg shrink-0 sm:w-auto hover:bg-blue-600 dark:hover:bg-blue-500 dark:bg-blue-600">
                            "Take me home"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn HomeRoot() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">
                "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
            </p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 { "Click me!".to_string() } else { count().to_string() }}

                " | Some more text"
            </button>
        </div>
    }
}
