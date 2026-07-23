use icons::{common::IconType, icon_component::LeptosIcon};
use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use service::{
    config::{Config, server::SaasPlatform},
    models::namespace::login::LoginParams,
    routes::login::login,
    traits::from_ctx::FromCtx,
};

use crate::{
    components::ui::{
        button::Button,
        card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
        input::{Input, InputType},
        label::Label,
    },
    shared::logger::error,
    views::fallback::Loading,
};

#[component]
pub fn Login() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let password_type = RwSignal::new(InputType::Password);
    let show_password = RwSignal::new(false);

    let navigate = use_navigate();
    let error_message = RwSignal::new(None::<String>);

    let toggle_show_password = move |_| {
        show_password.update(|value| *value ^= true);
        if show_password.get_untracked() {
            password_type.set(InputType::Text);
        } else {
            password_type.set(InputType::Password);
        }
    };

    let handle_login = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username.get_untracked();
        let password = password.get_untracked();

        let config = Config::from_ctx();
        // TODO: captcha
        let SaasPlatform { captcha, .. } = config.server.saas_platform.clone();

        let navigate = navigate.clone();
        spawn_local(async move {
            let params = LoginParams {
                username,
                password,
                social_type: None,
                social_code: None,
                social_state: None,
                social_code_valid: None,
                captcha_verification: captcha,
            };
            match login(params).await {
                Ok(_data) => {
                    error_message.set(None);
                    navigate("/", Default::default());
                },
                Err(err) => {
                    error!(%err);
                    error_message.set(Some(err.to_string()))
                },
            }
        });
    };

    view! {
        <Transition fallback=Loading>
            <div class="grid place-items-center p-6 w-full md:p-10 min-h-svh">
                <div class="w-full max-w-sm">
                    <div class="grid gap-6">
                        <Card>
                            <CardHeader>
                                <CardTitle>登录</CardTitle>
                                <CardDescription>输入用户名和密码</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <form on:submit=handle_login>
                                    <div class="grid gap-6">
                                        <div class="grid gap-3">
                                            <Label html_for="username">用户名</Label>
                                            <Input
                                                id="username"
                                                required=true
                                                autocomplete="username"
                                                placeholder="请输入用户名"
                                                bind_value=username
                                                on:input=move |_| error_message.set(None)
                                            />
                                        </div>
                                        <div class="grid gap-3">
                                            <div class="grid gap-2 items-center grid-cols-[1fr_auto]">
                                                <Label html_for="password">密码</Label>
                                                <a href="#" class="text-sm hover:underline underline-offset-4">
                                                    忘记密码?
                                                </a>
                                            </div>
                                            <div class="relative">
                                                <Input
                                                    class="pr-10"
                                                    id="password"
                                                    required=true
                                                    r#type=password_type
                                                    autocomplete="current-password"
                                                    minlength=8
                                                    placeholder="请输入密码"
                                                    bind_value=password
                                                    on:input=move |_| error_message.set(None)
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
                                        <div role="alert" class="text-sm font-normal text-destructive">
                                            {error_message}
                                        </div>
                                        <div class="flex flex-col gap-3">
                                            <Button class="w-full">登录</Button>
                                        </div>
                                    </div>
                                    <div class="mt-4 text-sm text-center">
                                        "还没有账号？"
                                        <a href="/register" class="ml-2 underline underline-offset-4">
                                            "注册"
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
