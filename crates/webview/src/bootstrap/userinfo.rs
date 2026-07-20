use icons::{ChevronRight, LogOut, Settings};
use leptos::prelude::*;

use crate::components::ui::{
    avatar::{Avatar, AvatarImage, AvatarSize},
    dropdown_menu::{
        DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuContent,
        DropdownMenuSeparator, DropdownMenuTrigger,
    },
    sidebar::{SidebarMenu, SidebarMenuButton, SidebarMenuItem},
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
                        <SidebarMenuButton>
                            <Avatar size=AvatarSize::Sm>
                                <AvatarImage attr:src=move || logo.get() />
                            </Avatar>
                            <div class="grid min-w-0 text-sm leading-tight text-left">
                                <span class="font-semibold truncate">{move || name.get()}</span>
                                <span class="text-xs opacity-70 truncate">{move || email.get()}</span>
                            </div>
                            <ChevronRight class="ml-auto transition-transform duration-200 ease-out size-4 shrink-0 group-data-[open=true]:rotate-90" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <div class="grid gap-2 items-center py-1.5 px-1 text-sm text-left grid-cols-[auto_1fr]">
                            <Avatar size=AvatarSize::Sm>
                                <AvatarImage attr:src=move || logo.get() />
                            </Avatar>
                            <div class="grid min-w-0 text-sm leading-tight text-left">
                                <span class="font-semibold truncate">{move || name.get()}</span>
                                <span class="text-xs opacity-70 truncate">{move || email.get()}</span>
                            </div>
                        </div>
                        <DropdownMenuSeparator class="my-2" />
                        <DropdownMenuAction>
                            <Settings class="size-4" />
                            <span>"设置"</span>
                        </DropdownMenuAction>
                        <DropdownMenuSeparator class="my-2" />
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
