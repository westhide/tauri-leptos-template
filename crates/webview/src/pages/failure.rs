use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::shared::{consts::HOME_PAGE, logger::error};

pub fn error_fallback(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view! { <Failure errors /> }
}

#[component]
pub fn Failure(errors: ArcRwSignal<Errors>) -> impl IntoView {
    error!(errors= ?*errors.read());

    let navigate = use_navigate();

    let go_back = move |_| {
        if let Ok(history) = window().history() {
            history.back().ok();
        } else {
            navigate(HOME_PAGE, Default::default());
        }
    };

    // TODO
    let error_list = errors.read().iter();

    view! {
        <div class="grid place-items-center py-24 px-6 min-h-full sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-red-400">"Error"</p>
                {{
                    let errors = errors.clone();
                    move || {
                        let items: Vec<_> = errors
                            .read()
                            .iter()
                            .map(|(id, err)| {
                                view! { <li class="font-mono break-all">{format!("{id}: {err}")}</li> }
                            })
                            .collect();

                        view! {
                            <div class="mx-auto mt-8 max-w-lg">
                                <ul class="p-4 space-y-2 text-sm text-left text-red-300 rounded-lg border bg-red-950/50 border-red-900/50">
                                    {items}
                                </ul>
                            </div>
                        }
                    }
                }}

                <div class="grid place-items-center mt-10">
                    <button
                        on:click=go_back
                        class="py-2.5 px-3.5 text-sm font-semibold text-white bg-indigo-500 rounded-md hover:bg-indigo-400 shadow-xs focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    >
                        Go back
                    </button>
                </div>
            </div>
        </div>
    }
}
