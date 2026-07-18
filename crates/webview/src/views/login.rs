use icons::{common::IconType, icon_component::LeptosIcon};
use leptos::prelude::*;

use crate::components::ui::{
    button::Button,
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    input::{Input, InputType},
    label::Label,
};

#[component]
pub fn Login() -> impl IntoView {
    let show_password = RwSignal::new(false);
    let password_input_ty = RwSignal::new(InputType::Password);

    let toggle_show_password = move |_| {
        show_password.update(|value| *value ^= true);
        if show_password.get_untracked() {
            password_input_ty.set(InputType::Text);
        } else {
            password_input_ty.set(InputType::Password);
        }
    };

    view! {
        <div class="grid place-items-center p-6 w-full md:p-10 min-h-svh">
            <div class="w-full max-w-sm">
                <div class="grid gap-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Login</CardTitle>
                            <CardDescription>Enter username and password</CardDescription>
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
                                        <div class="grid grid-cols-[1fr_auto] items-center gap-2">
                                            <Label html_for="password">Password</Label>
                                            <a
                                                href="#"
                                                class="text-sm hover:underline underline-offset-4"
                                            >
                                                Forgot your password?
                                            </a>
                                        </div>
                                        <div class="relative">
                                            <Input
                                                class="pr-10"
                                                id="password"
                                                required=true
                                                r#type=password_input_ty
                                                autocomplete="current-password"
                                                minlength=8
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
                                    <div class="flex flex-col gap-3">
                                        <Button class="w-full">Login</Button>
                                    </div>
                                </div>
                                <div class="mt-4 text-sm text-center">
                                    "Don't have an account?"
                                    <a href="#" class="underline underline-offset-4">
                                        Sign up
                                    </a>
                                </div>
                            </form>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
