use leptos::prelude::*;

use crate::components::ui::spinner::Spinner;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="grid place-items-center min-h-full">
            <Spinner class="size-12 text-muted-foreground" />
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="grid place-items-center py-24 px-6 min-h-full sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-indigo-400">404</p>
                <h1 class="mt-4 text-5xl font-semibold tracking-tight text-white sm:text-7xl text-balance">
                    Page not found
                </h1>
                <div class="grid place-items-center mt-10">
                    <a
                        href="/"
                        class="py-2.5 px-3.5 text-sm font-semibold text-white bg-indigo-500 rounded-md hover:bg-indigo-400 shadow-xs focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    >
                        Go back home
                    </a>
                </div>
            </div>
        </div>
    }
}
