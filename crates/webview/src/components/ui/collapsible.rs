use icons::ChevronRight;
use leptos::prelude::*;
use tw_merge::tw_merge;

// ==========================================================
// Collapsible
// ==========================================================

/// A collapsible/expandable section. Wrap a `CollapsibleTrigger` and
/// `CollapsibleContent` inside this component.
#[component]
pub fn Collapsible(
    open: RwSignal<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let merged = tw_merge!("", class);

    provide_context(CollapsibleCtx { open });

    view! {
        <div data-collapsible="" class=merged>
            {children()}
        </div>
    }
}

#[derive(Clone)]
pub struct CollapsibleCtx {
    pub open: RwSignal<bool>,
}

// ==========================================================
// CollapsibleTrigger
// ==========================================================

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleCtx>();
    let merged = tw_merge!("cursor-pointer", class);

    view! {
        <div
            data-collapsible-trigger=""
            class=merged
            on:click=move |_| ctx.open.update(|value| *value ^= true)
            aria-expanded=move || ctx.open.get().to_string()
        >
            {children()}
        </div>
    }
}

// ==========================================================
// CollapsibleContent
// ==========================================================

#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleCtx>();
    let merged = tw_merge!("grid overflow-hidden transition-all duration-300", class);

    view! {
        <div
            data-collapsible-content=""
            class=merged
            style=move || {
                if ctx.open.get() {
                    "grid-template-rows: 1fr; opacity: 1;"
                } else {
                    "grid-template-rows: 0fr; opacity: 0;"
                }
            }
        >
            <div class="overflow-hidden min-h-0">{children()}</div>
        </div>
    }
}

// ==========================================================
// CollapsibleChevron — animated chevron icon tied to collapsible state
// ==========================================================

#[component]
pub fn CollapsibleChevron(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CollapsibleCtx>();
    let base = tw_merge!("size-4 shrink-0 transition-transform duration-200 ease-out", class);

    view! {
        <span class=move || { if ctx.open.get() { format!("{base} rotate-90") } else { base.clone() } }>
            <ChevronRight class="size-4" />
        </span>
    }
}
