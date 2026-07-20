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
            Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarHeader,
            SidebarInset, SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger,
            SidebarVariant,
        },
    },
};

#[component]
pub fn Bootstrap(children: Children) -> impl IntoView {
    let orgs = vec![SwitcherItem { name: "MedClaw".into() }, SwitcherItem {
        name: "杭州美联数字医疗科技有限公司".into(),
    }];

    let items = vec![
        MenuItem {
            title: "医患管理".into(),
            url: "#".into(),
            icon: IconType::SquareTerminal,
            is_active: RwSignal::new(true),
            children: vec![
                MenuSubItem { title: "患者列表".into(), url: "#".into() },
                MenuSubItem { title: "健康档案".into(), url: "#".into() },
            ],
        },
        MenuItem {
            title: "AI模型".into(),
            url: "#".into(),
            icon: IconType::Boxes,
            is_active: RwSignal::new(false),
            children: vec![MenuSubItem { title: "Gemini".into(), url: "#".into() }, MenuSubItem {
                title: "claude".into(),
                url: "#".into(),
            }],
        },
        MenuItem {
            title: "Settings".into(),
            url: "#".into(),
            icon: IconType::Settings,
            is_active: RwSignal::new(false),
            children: vec![MenuSubItem { title: "TODO".into(), url: "#".into() }],
        },
    ];

    let logo = RwSignal::new("/assets/images/logo.png".to_string());
    let name = RwSignal::new("unknown".to_string());
    let email = RwSignal::new("unknown@unknown.com".to_string());

    view! {
        <SidebarProvider>
            <Sidebar variant=SidebarVariant::Sidebar collapsible=SidebarCollapsible::Icon side=SidebarSide::Left>
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
                        <Separator orientation=Signal::derive(move || { SeparatorOrientation::Vertical }) />
                        <span>"TODO: Header"</span>
                    </div>
                </header>
                {children()}
            </SidebarInset>
        </SidebarProvider>
    }
}
