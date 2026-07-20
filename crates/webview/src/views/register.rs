use icons::{common::IconType, icon_component::LeptosIcon};
use leptos::prelude::*;

use crate::{
    components::ui::{
        button::Button,
        card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
        input::{Input, InputType},
        label::Label,
    },
    views::fallback::Loading,
};

#[component]
pub fn Register() -> impl IntoView {
    let show_password = RwSignal::new(false);
    let password_type = RwSignal::new(InputType::Password);
    let show_confirm_password = RwSignal::new(false);
    let confirm_password_type = RwSignal::new(InputType::Password);

    let toggle_show_password = move |_| {
        show_password.update(|value| *value ^= true);
        if show_password.get_untracked() {
            password_type.set(InputType::Text);
        } else {
            password_type.set(InputType::Password);
        }
    };

    let toggle_show_confirm_password = move |_| {
        show_confirm_password.update(|value| *value ^= true);
        if show_confirm_password.get_untracked() {
            confirm_password_type.set(InputType::Text);
        } else {
            confirm_password_type.set(InputType::Password);
        }
    };

    let handle_register = |_| {};

    view! {
        <Transition fallback=Loading>
            <div class="grid place-items-center p-6 w-full md:p-10 min-h-svh">
                <div class="w-full max-w-sm">
                    <div class="grid gap-6">
                        <Card>
                            <CardHeader>
                                <CardTitle>Register</CardTitle>
                                <CardDescription>Create a new account</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <form>
                                    <div class="grid gap-6">
                                        <div class="grid gap-3">
                                            <Label html_for="username">Username</Label>
                                            <Input
                                                id="username"
                                                required=true
                                                autocomplete="username"
                                                placeholder="Enter your username"
                                            />
                                        </div>
                                        <div class="grid gap-3">
                                            <Label html_for="password">Password</Label>
                                            <div class="relative">
                                                <Input
                                                    class="pr-10"
                                                    id="password"
                                                    required=true
                                                    r#type=password_type
                                                    autocomplete="new-password"
                                                    minlength=8
                                                    placeholder="Enter your password"
                                                />
                                                <button
                                                    class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                                    type="button"
                                                    on:click=toggle_show_password
                                                >
                                                    {move || {
                                                        if show_password.get() {
                                                            view! {
                                                                <LeptosIcon icon=IconType::EyeOff class="size-4" />
                                                            }
                                                        } else {
                                                            view! { <LeptosIcon icon=IconType::Eye class="size-4" /> }
                                                        }
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                        <div class="grid gap-3">
                                            <Label html_for="confirm-password">Confirm Password</Label>
                                            <div class="relative">
                                                <Input
                                                    class="pr-10"
                                                    id="confirm-password"
                                                    required=true
                                                    r#type=confirm_password_type
                                                    autocomplete="new-password"
                                                    minlength=8
                                                    placeholder="Confirm your password"
                                                />
                                                <button
                                                    class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                                    type="button"
                                                    on:click=toggle_show_confirm_password
                                                >
                                                    {move || {
                                                        if show_confirm_password.get() {
                                                            view! {
                                                                <LeptosIcon icon=IconType::EyeOff class="size-4" />
                                                            }
                                                        } else {
                                                            view! { <LeptosIcon icon=IconType::Eye class="size-4" /> }
                                                        }
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                        <div class="flex flex-col gap-3">
                                            <Button class="w-full" on:click=handle_register>
                                                Register
                                            </Button>
                                        </div>
                                    </div>
                                    <div class="mt-4 text-sm text-center">
                                        "Already have an account?"
                                        <a href="/login" class="ml-2 underline underline-offset-4">
                                            "Login"
                                        </a>
                                    </div>
                                </form>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </Transition>
    }
}
