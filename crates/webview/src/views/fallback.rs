use leptos::prelude::*;

#[component]
pub fn Fallback() -> impl IntoView {
    view! {
        <div class="grid min-h-full place-items-center px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-indigo-400">404</p>
                <h1 class="mt-4 text-5xl font-semibold tracking-tight text-balance text-white sm:text-7xl">
                    Page not found
                </h1>
                <div class="mt-10 grid place-items-center">
                    <a
                        href="/"
                        class="rounded-md bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    >
                        Go back home
                    </a>
                </div>
            </div>
        </div>
    }
}
