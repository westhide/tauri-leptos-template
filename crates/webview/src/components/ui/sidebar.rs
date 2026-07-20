use icons::PanelLeft;
use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::components::ui::{
    button::{Button, ButtonVariant},
    skeleton::Skeleton,
};

// ==========================================================
// Constants
// ==========================================================

const SIDEBAR_WIDTH: &str = "12rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";

// ==========================================================
// CSS variable inline style builder
// ==========================================================

fn sidebar_css_vars() -> String {
    format!(
        r#"--sidebar-width: {SIDEBAR_WIDTH};
--sidebar-width-icon: {SIDEBAR_WIDTH_ICON};
--sidebar-background: var(--primary-color-2, hsl(240 5.9% 96%));
--sidebar-foreground: var(--secondary-color-4, hsl(240 10% 10%));
--sidebar-border: var(--primary-color-6, hsl(240 5.9% 85%));
--sidebar-accent: var(--primary-color-4, hsl(240 4.8% 90%));
--sidebar-accent-foreground: var(--secondary-color-4, hsl(240 10% 10%));
--sidebar-ring: var(--primary-color-7, hsl(240 5% 75%))"#
    )
}

// ==========================================================
// Enums
// ==========================================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

impl SidebarState {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarState::Expanded => "expanded",
            SidebarState::Collapsed => "collapsed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    Offcanvas,
    #[default]
    Icon,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
        }
    }
}

// ==========================================================
// Context
// ==========================================================

#[derive(Clone)]
pub struct SidebarCtx {
    pub state: Signal<SidebarState>,
    pub side: RwSignal<SidebarSide>,
    open: RwSignal<bool>,
}

impl SidebarCtx {
    pub fn toggle(&self) {
        self.open.update(|value| *value ^= true);
    }

    pub fn open(&self) -> bool {
        self.open.get()
    }
}

pub fn use_sidebar() -> SidebarCtx {
    expect_context::<SidebarCtx>()
}

// ==========================================================
// SidebarProvider
// ==========================================================

#[component]
pub fn SidebarProvider(
    #[prop(default = true)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let side = RwSignal::new(SidebarSide::Left);
    let open = RwSignal::new(default_open);

    let state =
        Signal::derive(
            move || {
                if open.get() { SidebarState::Expanded } else { SidebarState::Collapsed }
            },
        );

    let ctx = SidebarCtx { state, side, open };
    provide_context(ctx);

    view! {
        <div
            data-slot="sidebar-wrapper"
            style=sidebar_css_vars()
            class="grid overflow-hidden w-full grid-cols-[auto_1fr] h-svh min-h-svh group/sidebar-wrapper md:group-data-[side=right]/sidebar-desktop:grid-cols-[1fr_auto]"
        >
            {children()}
        </div>
    }
}

// ==========================================================
// Sidebar
// ==========================================================

#[component]
pub fn Sidebar(
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_sidebar();

    if ctx.side.get_untracked() != side {
        ctx.side.set(side);
    }

    let state = ctx.state;

    let collapsible_str = move || {
        if state.get() == SidebarState::Collapsed { collapsible.as_str() } else { "" }
    };

    let side_str = side.as_str();
    let variant_str = variant.as_str();

    view! {
        <div
            data-sidebar-desktop=""
            data-state=move || state.get().as_str()
            data-collapsible=collapsible_str
            data-variant=variant_str
            data-side=side_str
            data-slot="sidebar"
            class="text-[var(--sidebar-foreground)] group/sidebar-desktop peer"
        >
            // Sidebar gap (spacer)
            <div
                data-sidebar-gap=""
                data-slot="sidebar-gap"
                class="relative bg-transparent duration-200 ease-out w-[var(--sidebar-width)] transition-[width] group-data-[collapsible=icon]/sidebar-desktop:w-[var(--sidebar-width-icon)] group-data-[variant=floating]/sidebar-desktop:w-[var(--sidebar-width)] group-data-[variant=inset]/sidebar-desktop:w-[var(--sidebar-width)] group-data-[variant=floating]/sidebar-desktop:group-data-[collapsible=icon]/sidebar-desktop:w-[calc(var(--sidebar-width-icon)+1rem)] group-data-[variant=inset]/sidebar-desktop:group-data-[collapsible=icon]/sidebar-desktop:w-[calc(var(--sidebar-width-icon)+1rem)] group-data-[collapsible=offcanvas]/sidebar-desktop:w-0"
            />

            // Sidebar container (fixed position)
            <div
                data-sidebar-container=""
                data-slot="sidebar-container"
                class=tw_merge!(
                    "fixed z-10 top-0 bottom-0 hidden md:grid w-[var(--sidebar-width)] h-svh box-border transition-[left,right,width] duration-200 ease-out",
                    // Position based on side
                    "data-[side=left]:left-0 data-[side=left]:group-data-[collapsible=offcanvas]/sidebar-desktop:left-[calc(var(--sidebar-width)*-1)]",
                    "data-[side=right]:right-0 data-[side=right]:group-data-[collapsible=offcanvas]/sidebar-desktop:right-[calc(var(--sidebar-width)*-1)]",
                    // Icon mode
                    "group-data-[collapsible=icon]/sidebar-desktop:overflow-visible group-data-[collapsible=icon]/sidebar-desktop:w-[var(--sidebar-width-icon)]",
                    // Variant borders
                    "group-data-[variant=sidebar]/sidebar-desktop:group-data-[side=left]/sidebar-desktop:border-r group-data-[variant=sidebar]/sidebar-desktop:group-data-[side=right]/sidebar-desktop:border-l border-[var(--sidebar-border)]",
                    // Floating/inset padding
                    "group-data-[variant=floating]/sidebar-desktop:p-2 group-data-[variant=inset]/sidebar-desktop:p-2",
                    // Floating/inset icon width
                    "group-data-[variant=floating]/sidebar-desktop:group-data-[collapsible=icon]/sidebar-desktop:w-[calc(var(--sidebar-width-icon)+1rem+2px)]",
                    "group-data-[variant=inset]/sidebar-desktop:group-data-[collapsible=icon]/sidebar-desktop:w-[calc(var(--sidebar-width-icon)+1rem+2px)]",
                    class
                )
            >
                <div
                    data-sidebar-inner=""
                    data-slot="sidebar-inner"
                    class="grid w-full h-full box-border bg-[var(--sidebar-background)] grid-rows-[auto_1fr_auto] group-data-[collapsible=icon]/sidebar-desktop:overflow-visible group-data-[variant=floating]/sidebar-desktop:border group-data-[variant=floating]/sidebar-desktop:border-[var(--sidebar-border)] group-data-[variant=floating]/sidebar-desktop:rounded-lg group-data-[variant=floating]/sidebar-desktop:shadow-sm"
                >
                    {children()}
                </div>
            </div>
        </div>
    }
}

// ==========================================================
// SidebarTrigger
// ==========================================================

#[component]
pub fn SidebarTrigger(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = use_sidebar();

    let merged = tw_merge!("inline-grid size-7 place-items-center p-0!", class);

    view! {
        <Button
            variant=ButtonVariant::Ghost
            class=merged
            on:click=move |_| ctx.toggle()
            attr:data-sidebar="trigger"
            attr:data-slot="sidebar-trigger"
        >
            <PanelLeft class="size-4" />
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

// ==========================================================
// SidebarRail
// ==========================================================

#[component]
pub fn SidebarRail() -> impl IntoView {
    let ctx = use_sidebar();

    view! {
        <button
            data-sidebar-rail=""
            data-slot="sidebar-rail"
            class="hidden absolute top-0 bottom-0 z-20 p-0 w-4 bg-transparent border-none transition-all duration-200 ease-out -translate-x-1/2 sm:grid cursor-ew-resize after:absolute after:top-0 after:bottom-0 after:left-1/2 after:w-0.5 after:content-[''] group-data-[side=left]/sidebar-desktop:right-[-1rem] group-data-[side=left]/sidebar-desktop:cursor-w-resize group-data-[side=right]/sidebar-desktop:left-0 group-data-[side=right]/sidebar-desktop:cursor-e-resize group-data-[side=left]/sidebar-desktop:group-data-[state=collapsed]/sidebar-desktop:cursor-e-resize group-data-[side=right]/sidebar-desktop:group-data-[state=collapsed]/sidebar-desktop:cursor-w-resize group-data-[collapsible=offcanvas]/sidebar-desktop:translate-x-0 group-data-[collapsible=offcanvas]/sidebar-desktop:after:left-full group-data-[side=left]/sidebar-desktop:group-data-[collapsible=offcanvas]/sidebar-desktop:right-[-0.5rem] group-data-[side=right]/sidebar-desktop:group-data-[collapsible=offcanvas]/sidebar-desktop:left-[-0.5rem] hover:after:bg-[var(--sidebar-border)] hover:group-data-[collapsible=offcanvas]/sidebar-desktop:bg-[var(--sidebar-background)]"
            aria-label="Toggle Sidebar"
            tabindex="-1"
            title="Toggle Sidebar"
            on:click=move |_| ctx.toggle()
        />
    }
}

// ==========================================================
// SidebarInset
// ==========================================================

#[component]
pub fn SidebarInset(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!(
        r#"relative grid w-full bg-[var(--primary-color-1,#fff)]
peer-data-[variant=inset]/sidebar-desktop:rounded-xl peer-data-[variant=inset]/sidebar-desktop:m-2 peer-data-[variant=inset]/sidebar-desktop:ml-0
peer-data-[variant=inset]/sidebar-desktop:shadow-sm
peer-data-[variant=inset]/sidebar-desktop:peer-data-[state=collapsed]/sidebar-desktop:ml-2
peer-data-[variant=inset]/sidebar-desktop:peer-data-[side=right]/sidebar-desktop:mr-0 peer-data-[variant=inset]/sidebar-desktop:peer-data-[side=right]/sidebar-desktop:ml-2
peer-data-[variant=inset]/sidebar-desktop:peer-data-[side=right]/sidebar-desktop:peer-data-[state=collapsed]/sidebar-desktop:mr-2"#,
        class
    );
    view! {
        <main data-sidebar-inset="" data-slot="sidebar-inset" class=merged>
            {children()}
        </main>
    }
}

// ==========================================================
// SidebarHeader, SidebarContent, SidebarFooter
// ==========================================================

#[component]
pub fn SidebarHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!("grid p-2 gap-2", class);
    view! {
        <div data-sidebar="header" data-slot="sidebar-header" class=merged>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!(
        "grid overflow-x-hidden overflow-y-auto min-h-0 gap-2 group-data-[collapsible=icon]/sidebar-desktop:overflow-visible",
        class
    );
    view! {
        <div data-sidebar="content" data-slot="sidebar-content" class=merged>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!("grid p-2 gap-2", class);
    view! {
        <div data-sidebar="footer" data-slot="sidebar-footer" class=merged>
            {children()}
        </div>
    }
}

// ==========================================================
// SidebarGroup, SidebarGroupLabel, SidebarGroupContent, SidebarGroupAction
// ==========================================================

#[component]
pub fn SidebarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!("relative min-w-0 p-2", class);
    view! {
        <div data-sidebar="group" data-slot="sidebar-group" class=merged>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!(
        "grid grid-flow-col auto-cols-max items-center h-8 px-2 rounded-md text-[var(--sidebar-foreground)] text-xs font-medium opacity-70 outline-none transition-[margin,opacity] duration-200 ease-out group-data-[collapsible=icon]/sidebar-desktop:-mt-8 group-data-[collapsible=icon]/sidebar-desktop:opacity-0 [&_svg]:size-4 [&_svg]:shrink-0",
        class
    );
    view! {
        <div data-sidebar="group-label" data-slot="sidebar-group-label" class=merged>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!("w-full text-sm", class);
    view! {
        <div data-sidebar="group-content" data-slot="sidebar-group-content" class=merged>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupAction(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!(
        "absolute top-3.5 right-3 grid w-5 place-items-center p-0 border-none rounded-md aspect-square bg-transparent text-[var(--sidebar-foreground)] cursor-pointer outline-none transition-[transform,opacity,visibility] duration-150 ease-out hover:bg-[var(--sidebar-accent)] hover:text-[var(--sidebar-accent-foreground)] group-data-[collapsible=icon]/sidebar-desktop:opacity-0 group-data-[collapsible=icon]/sidebar-desktop:pointer-events-none group-data-[collapsible=icon]/sidebar-desktop:invisible [&_svg]:size-4",
        class
    );
    view! {
        <button data-sidebar="group-action" data-slot="sidebar-group-action" class=merged>
            {children()}
        </button>
    }
}

// ==========================================================
// SidebarMenu, SidebarMenuItem
// ==========================================================

#[component]
pub fn SidebarMenu(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!("grid w-full min-w-0 p-0 m-0 gap-1 list-none", class);
    view! {
        <ul data-sidebar="menu" data-slot="sidebar-menu" class=merged>
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!("relative", class);
    view! {
        <li data-sidebar="menu-item" data-slot="sidebar-menu-item" class=merged>
            {children()}
        </li>
    }
}

// ==========================================================
// SidebarMenuButton enums + component
// ==========================================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

impl SidebarMenuButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonVariant::Default => "default",
            SidebarMenuButtonVariant::Outline => "outline",
        }
    }

    fn tw_class(&self) -> &'static str {
        match self {
            SidebarMenuButtonVariant::Default => "",
            SidebarMenuButtonVariant::Outline => {
                "bg-[var(--primary-color-1,#fff)] shadow-[0_0_0_1px_var(--sidebar-border)] hover:shadow-[0_0_0_1px_var(--sidebar-accent)]"
            },
        }
    }
}

// Base menu button styles (shared, static)
const MENU_BUTTON_BASE: &str = r#"relative grid grid-cols-[auto_1fr_auto] overflow-hidden w-full box-border items-center p-2 border-none rounded-md text-sm
bg-transparent text-[var(--sidebar-foreground)] cursor-pointer gap-2
outline-none text-left no-underline transition-[width,height,padding] duration-200 ease-out
hover:bg-[var(--sidebar-accent)] hover:text-[var(--sidebar-accent-foreground)]
focus-visible:shadow-[0_0_0_2px_var(--sidebar-ring)]
active:bg-[var(--sidebar-accent)] active:text-[var(--sidebar-accent-foreground)]
disabled:opacity-50 disabled:pointer-events-none aria-disabled:opacity-50 aria-disabled:pointer-events-none
data-[active=true]:bg-[var(--sidebar-accent)] data-[active=true]:text-[var(--sidebar-accent-foreground)] data-[active=true]:font-medium
group-data-[collapsible=icon]/sidebar-desktop:grid-cols-[1fr] group-data-[collapsible=icon]/sidebar-desktop:aspect-square group-data-[collapsible=icon]/sidebar-desktop:p-0 group-data-[collapsible=icon]/sidebar-desktop:gap-0 group-data-[collapsible=icon]/sidebar-desktop:place-items-center
[&_svg]:size-4 [&_svg]:shrink-0
[&>:first-child]:shrink-0
[&>span]:overflow-hidden [&>span]:text-ellipsis [&>span]:whitespace-nowrap
[&>span]:transition-opacity [&>span]:duration-200
group-data-[collapsible=icon]/sidebar-desktop:[&>:not(:first-child)]:opacity-0 group-data-[collapsible=icon]/sidebar-desktop:[&>:not(:first-child)]:absolute group-data-[collapsible=icon]/sidebar-desktop:[&>:not(:first-child)]:pointer-events-none"#;

#[component]
pub fn SidebarMenuButton(
    #[prop(optional, into)] is_active: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] variant: SidebarMenuButtonVariant,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.tw_class();
    let merged = tw_merge!(MENU_BUTTON_BASE, variant_class, class);

    view! {
        <button
            data-sidebar="menu-button"
            data-slot="sidebar-menu-button"
            data-variant=variant.as_str()
            data-active=move || if is_active.get() { "true" } else { "false" }
            class=merged
        >
            {children()}
        </button>
    }
}

// ==========================================================
// SidebarMenuAction
// ==========================================================

#[component]
pub fn SidebarMenuAction(
    #[prop(optional)] show_on_hover: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let show = if show_on_hover { "true" } else { "false" };
    let merged = tw_merge!(
        r#"absolute top-1.5 right-1 grid w-5 place-items-center p-0 border-none rounded-md
aspect-square bg-transparent text-[var(--sidebar-foreground)] cursor-pointer outline-none
hover:bg-[var(--sidebar-accent)] hover:text-[var(--sidebar-accent-foreground)]
group-data-[collapsible=icon]/sidebar-desktop:opacity-0 group-data-[collapsible=icon]/sidebar-desktop:pointer-events-none
group-data-[collapsible=icon]/sidebar-desktop:invisible [&_svg]:size-4"#,
        class
    );
    view! {
        <button data-sidebar="menu-action" data-slot="sidebar-menu-action" data-show-on-hover=show class=merged>
            {children()}
        </button>
    }
}

// ==========================================================
// SidebarMenuBadge
// ==========================================================

#[component]
pub fn SidebarMenuBadge(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!(
        r#"absolute top-1.5 right-1 grid min-w-5 h-5 place-items-center px-1 rounded-md
text-[var(--sidebar-foreground)] text-xs tabular-nums font-medium pointer-events-none
select-none transition-opacity duration-200 group-data-[collapsible=icon]/sidebar-desktop:opacity-0
group-data-[collapsible=icon]/sidebar-desktop:pointer-events-none"#,
        class
    );
    view! {
        <div data-sidebar="menu-badge" data-slot="sidebar-menu-badge" class=merged>
            {children()}
        </div>
    }
}

// ==========================================================
// SidebarMenuSkeleton
// ==========================================================

#[component]
pub fn SidebarMenuSkeleton(#[prop(optional)] show_icon: bool) -> impl IntoView {
    view! {
        <div
            data-sidebar="menu-skeleton"
            data-slot="sidebar-menu-skeleton"
            class="grid gap-2 items-center px-2 h-8 rounded-md grid-cols-[auto_1fr]"
        >
            {if show_icon { view! { <Skeleton class="rounded-md size-4" /> }.into_any() } else { view! {}.into_any() }}
            <Skeleton class="h-4" />
        </div>
    }
}

// ==========================================================
// SidebarSeparator
// ==========================================================

#[component]
pub fn SidebarSeparator() -> impl IntoView {
    view! {
        <div
            data-sidebar="separator"
            data-slot="sidebar-separator"
            class="mx-2 w-auto h-px shrink-0 bg-[var(--sidebar-border)]"
            role="separator"
        />
    }
}

// ==========================================================
// SidebarMenuSub, SidebarMenuSubItem, SidebarMenuSubButton
// ==========================================================

#[component]
pub fn SidebarMenuSub(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let merged = tw_merge!(
        r#"grid py-0.5 px-2.5 border-l border-[var(--sidebar-border)] mx-3.5 gap-1 list-none translate-x-px
group-data-[collapsible=icon]/sidebar-desktop:overflow-hidden group-data-[collapsible=icon]/sidebar-desktop:max-h-0
group-data-[collapsible=icon]/sidebar-desktop:p-0 group-data-[collapsible=icon]/sidebar-desktop:m-0
group-data-[collapsible=icon]/sidebar-desktop:opacity-0 group-data-[collapsible=icon]/sidebar-desktop:pointer-events-none
group-data-[collapsible=icon]/sidebar-desktop:invisible"#,
        class
    );
    view! {
        <ul data-sidebar="menu-sub" data-slot="sidebar-menu-sub" class=merged>
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!("relative", class);
    view! {
        <li data-sidebar="menu-sub-item" data-slot="sidebar-menu-sub-item" class=merged>
            {children()}
        </li>
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

impl SidebarMenuSubButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuSubButtonSize::Sm => "sm",
            SidebarMenuSubButtonSize::Md => "md",
        }
    }

    fn tw_class(&self) -> &'static str {
        match self {
            SidebarMenuSubButtonSize::Sm => "text-xs",
            SidebarMenuSubButtonSize::Md => "text-sm",
        }
    }
}

const MENU_SUB_BUTTON_BASE: &str = r#"grid overflow-hidden w-full min-w-0 h-7 box-border items-center px-2 border-none rounded-md
bg-transparent text-[var(--sidebar-foreground)] cursor-pointer gap-2 outline-none no-underline
hover:bg-[var(--sidebar-accent)] hover:text-[var(--sidebar-accent-foreground)]
focus-visible:shadow-[0_0_0_2px_var(--sidebar-ring)]
active:bg-[var(--sidebar-accent)] active:text-[var(--sidebar-accent-foreground)]
disabled:opacity-50 disabled:pointer-events-none aria-disabled:opacity-50 aria-disabled:pointer-events-none
data-[active=true]:bg-[var(--sidebar-accent)] data-[active=true]:text-[var(--sidebar-accent-foreground)]
[&_svg]:size-4 [&_svg]:shrink-0 [&_svg]:text-[var(--sidebar-accent-foreground)]
[&>span:last-child]:overflow-hidden [&>span:last-child]:text-ellipsis [&>span:last-child]:whitespace-nowrap
"#;

#[component]
pub fn SidebarMenuSubButton(
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: SidebarMenuSubButtonSize,
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    let data_active = if is_active { "true" } else { "false" };
    let size_class = size.tw_class();
    let merged = tw_merge!(MENU_SUB_BUTTON_BASE, size_class, class);

    view! {
        <a
            data-sidebar="menu-sub-button"
            data-slot="sidebar-menu-sub-button"
            data-size=size.as_str()
            data-active=data_active
            class=merged
            href=href
        >
            {children()}
        </a>
    }
}
