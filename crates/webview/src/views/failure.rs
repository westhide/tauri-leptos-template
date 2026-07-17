use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::shared::logger::error;

#[component]
pub fn Failure(errors: ArcRwSignal<Errors>) -> impl IntoView {
    error!(errors= ?*errors.read());

    let navigate = use_navigate();

    let go_back = move |_| {
        if let Ok(history) = window().history() {
            history.back().ok();
        } else {
            navigate("/", Default::default());
        }
    };

    let error_list = errors.read().iter();

    view! {
        <div class="grid min-h-full place-items-center px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-red-400">"Error"</p>
                {{
                    let errors = errors.clone();
                    move || {
                        let items: Vec<_> = errors
                            .read()
                            .iter()
                            .map(|(id, err)| {
                                view! {
                                    <li class="font-mono break-all">{format!("{id}: {err}")}</li>
                                }
                            })
                            .collect();

                        view! {
                            <div class="mt-8 mx-auto max-w-lg">
                                <ul class="text-left text-sm text-red-300 bg-red-950/50 rounded-lg p-4 space-y-2 border border-red-900/50">
                                    {items}
                                </ul>
                            </div>
                        }
                    }
                }}

                <div class="mt-10 grid place-items-center">
                    <button
                        on:click=go_back
                        class="rounded-md bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                    >
                        Go back
                    </button>
                </div>
            </div>
        </div>
    }
}
