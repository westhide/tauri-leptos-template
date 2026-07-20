use icons::{common::IconType, icon_component::LeptosIcon};
use leptos::prelude::*;

use crate::components::ui::{
    collapsible::{Collapsible, CollapsibleChevron, CollapsibleContent, CollapsibleTrigger},
    sidebar::{
        SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
        SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
    },
};

// ==========================================================
// Data types
// ==========================================================

#[derive(Clone)]
pub struct MenuItem {
    pub title: String,
    pub url: String,
    pub icon: IconType,
    pub is_active: RwSignal<bool>,
    pub children: Vec<MenuSubItem>,
}

#[derive(Clone)]
pub struct MenuSubItem {
    pub title: String,
    pub url: String,
}

// ==========================================================
// Menu component
// ==========================================================

#[component]
pub fn Menu(items: Vec<MenuItem>) -> impl IntoView {
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"菜单"</SidebarGroupLabel>
            <SidebarMenu>
                {items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <Collapsible open=item.is_active>
                                <SidebarMenuItem>
                                    <CollapsibleTrigger>
                                        <SidebarMenuButton is_active=item.is_active>
                                            <LeptosIcon icon=item.icon class="size-4 shrink-0" />
                                            <span>{item.title}</span>
                                            <CollapsibleChevron class="ml-auto" />
                                        </SidebarMenuButton>
                                    </CollapsibleTrigger>
                                    <CollapsibleContent>
                                        <SidebarMenuSub>
                                            {item
                                                .children
                                                .into_iter()
                                                .map(|child| {
                                                    view! {
                                                        <SidebarMenuSubItem>
                                                            <SidebarMenuSubButton href=child.url>
                                                                <span>{child.title}</span>
                                                            </SidebarMenuSubButton>
                                                        </SidebarMenuSubItem>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </SidebarMenuSub>
                                    </CollapsibleContent>
                                </SidebarMenuItem>
                            </Collapsible>
                        }
                    })
                    .collect::<Vec<_>>()}
            </SidebarMenu>
        </SidebarGroup>
    }
}
