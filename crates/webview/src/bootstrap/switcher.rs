use icons::{ChevronRight, House};
use leptos::prelude::*;

use crate::components::ui::{
    dropdown_menu::{DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuTrigger},
    sidebar::{SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem},
};

// ==========================================================
// Data types
// ==========================================================

#[derive(Clone)]
pub struct SwitcherItem {
    pub name: String,
}

#[component]
pub fn Switcher(items: Vec<SwitcherItem>) -> impl IntoView {
    let items_for_display = items.clone();

    let index = RwSignal::new(0usize);

    let display_name =
        move || items_for_display.get(index.get()).map(|i| i.name.clone()).unwrap_or_default();

    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger as_child=true>
                        <SidebarMenuButton size=SidebarMenuButtonSize::Lg class="group">
                            <House class="size-4 shrink-0" />
                            <div class="grid place-items-center h-full text-sm text-left">
                                <span class="w-full truncate font-semibold">
                                    {display_name}
                                </span>
                            </div>
                            <ChevronRight class="size-4 shrink-0 ml-auto transition-transform duration-200 ease-out group-data-[open=true]:rotate-90" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <div class="p-2 text-xs opacity-70">"租户"</div>
                        {items.into_iter().enumerate().map(|(idx, item)| {
                            view! {
                                <DropdownMenuAction on:click=move |_| index.set(idx)>
                                    <span>{idx + 1}</span>
                                    <span>{item.name}</span>
                                </DropdownMenuAction>
                            }
                        }).collect::<Vec<_>>()}
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}
