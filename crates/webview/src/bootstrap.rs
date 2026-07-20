#[cfg(feature = "hydrate")]
pub mod hydrate;
pub mod menu;
#[cfg(server)]
pub mod shell;
pub mod switcher;
pub mod userinfo;

use icons::common::IconType;
use leptos::prelude::*;

use crate::{
    bootstrap::{
        menu::{Menu, MenuItem, MenuSubItem},
        switcher::{Switcher, SwitcherItem},
        userinfo::UserInfo,
    },
    components::ui::{
        separator::{Separator, SeparatorOrientation},
        sidebar::{
            Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarInset, SidebarProvider,
            SidebarRail, SidebarTrigger,
        },
    },
};

#[component]
pub fn Bootstrap(children: Children) -> impl IntoView {
    let orgs = vec![SwitcherItem { name: "MedClaw".into() }];

    let items = vec![
        MenuItem {
            title: "租户管理".into(),
            url: "#".into(),
            icon: IconType::SquareTerminal,
            is_active: RwSignal::new(true),
            children: vec![MenuSubItem { title: "实例列表".into(), url: "#".into() }],
        },
        MenuItem {
            title: "用户管理".into(),
            url: "#".into(),
            icon: IconType::Users,
            is_active: RwSignal::new(true),
            children: vec![
                MenuSubItem { title: "用户列表".into(), url: "#".into() },
                MenuSubItem { title: "菜单管理".into(), url: "#".into() },
            ],
        },
        MenuItem {
            title: "AI模型".into(),
            url: "#".into(),
            icon: IconType::Boxes,
            is_active: RwSignal::new(false),
            children: vec![MenuSubItem { title: "Openai".into(), url: "#".into() }, MenuSubItem {
                title: "Claude".into(),
                url: "#".into(),
            }],
        },
        MenuItem {
            title: "设置".into(),
            url: "#".into(),
            icon: IconType::Settings,
            is_active: RwSignal::new(false),
            children: vec![MenuSubItem { title: "配置中心".into(), url: "#".into() }],
        },
    ];

    let logo = RwSignal::new("/assets/images/logo.png".to_string());
    let name = RwSignal::new("medclaw".to_string());
    let email = RwSignal::new("medclaw@medclaw.com".to_string());

    view! {
        <SidebarProvider>
            <Sidebar>
                <SidebarHeader>
                    <Switcher items=orgs />
                </SidebarHeader>
                <SidebarContent>
                    <Menu items=items />
                </SidebarContent>
                <SidebarFooter>
                    <UserInfo logo=logo name=name email=email />
                </SidebarFooter>
                <SidebarRail />
            </Sidebar>
            <SidebarInset>
                <header class="grid grid-cols-1 items-center px-4 h-14 border-b shrink-0 border-[var(--sidebar-border)] bg-[var(--primary-color-1)]">
                    <div class="grid grid-flow-col auto-cols-max gap-3 items-center">
                        <SidebarTrigger />
                        <Separator orientation=RwSignal::new(SeparatorOrientation::Vertical) />
                        <span>"控制台"</span>
                    </div>
                </header>
                {children()}
            </SidebarInset>
        </SidebarProvider>
    }
}
