use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="max-w-sm mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl">
            <div class="md:flex">
                <div class="md:shrink-0">
                    <img class="h-48 md:w-48 md:h-full w-full aspect-square rounded-lg" src="assets/images/profile.jpg" alt="Modern building architecture"> </img>
                </div>
                <div class="p-8">
                    <div class="uppercase tracking-wide text-sm text-indigo-500 font-semibold">Company retreats</div>
                    <a href="#" class="block mt-1 text-lg leading-tight font-medium text-black hover:underline">Incredible accommodation for your team</a>
                    <p class="mt-2 text-slate-500">Looking to take your team away on a retreat to enjoy awesome food and take in some sunshine? We have a list of places to do just that.</p>
                </div>
            </div>
        </div>

        // <div class="flex flex-wrap">
        //     <div class="basis-1/4 md:basis-1/3 bg-blue-200">
        //         <img class="m-3 rounded-lg" src="assets/images/profile.jpg" alt="image description"></img>
        //     </div>
        //     <div class="grow bg-red-200">
        //         <img class="basis-3/4 m-3 rounded-lg" src="assets/images/profile.jpg" alt="image description"></img>
    
        //     </div>
        // </div>
    }
}