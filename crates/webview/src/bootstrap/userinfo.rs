use icons::{ChevronRight, LogOut, Settings};
use leptos::prelude::*;

use crate::components::ui::{
    avatar::{Avatar, AvatarImage, AvatarSize},
    dropdown_menu::{
        DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuContent,
        DropdownMenuSeparator, DropdownMenuTrigger,
    },
    sidebar::{SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem},
};

// ==========================================================
// UserInfo component
// ==========================================================

#[component]
pub fn UserInfo(
    #[prop(into)] logo: Signal<String>,
    #[prop(into)] name: Signal<String>,
    #[prop(into)] email: Signal<String>,
) -> impl IntoView {
    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger as_child=true>
                        <SidebarMenuButton size=SidebarMenuButtonSize::Lg>
                            <Avatar size=AvatarSize::Sm>
                                <AvatarImage attr:src=move || logo.get() />
                            </Avatar>
                            <div class="grid min-w-0 flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">{move || name.get()}</span>
                                <span class="truncate text-xs opacity-70">{move || email.get()}</span>
                            </div>
                            <ChevronRight class="size-4 shrink-0 ml-auto transition-transform duration-200 ease-out group-data-[open=true]:rotate-90" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <div class="flex items-center gap-2 py-1.5 px-1 text-left text-sm">
                            <Avatar size=AvatarSize::Sm>
                                <AvatarImage attr:src=move || logo.get() />
                            </Avatar>
                            <div class="grid min-w-0 flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">{move || name.get()}</span>
                                <span class="truncate text-xs opacity-70">{move || email.get()}</span>
                            </div>
                        </div>
                        <DropdownMenuSeparator />
                        <DropdownMenuAction>
                            <Settings class="size-4" />
                            <span>"设置"</span>
                        </DropdownMenuAction>
                        <DropdownMenuSeparator />
                        <DropdownMenuAction variant=DropdownMenuActionVariant::Destructive>
                            <LogOut class="size-4" />
                            <span>"退出登录"</span>
                        </DropdownMenuAction>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}
