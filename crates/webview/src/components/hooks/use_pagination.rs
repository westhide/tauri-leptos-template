use leptos::{prelude::*, wasm_bindgen, web_sys};
use leptos_router::{
    NavigateOptions,
    hooks::{use_location, use_navigate},
};
use time::Date;

use crate::shared::consts::QueryParams;

pub struct QueryUtils;

impl QueryUtils {
    pub fn extract(query_key: String) -> Memo<Option<String>> {
        let location = use_location();

        Memo::new(move |_| location.query.with(|q| q.get(&query_key)))
    }

    /// Core method to handle URL updates with search parameter modifications
    fn update_url_with_params<F>(param_modifier: F)
    where
        F: FnOnce(&web_sys::UrlSearchParams),
    {
        let location = window().location();
        let Ok(current_search) = location.search() else { return };
        let navigate = use_navigate();

        // Parse existing query parameters
        let Ok(url_search_params) = web_sys::UrlSearchParams::new_with_str(&current_search) else {
            return
        };

        // Apply the parameter modifications
        param_modifier(&url_search_params);

        let Ok(pathname) = location.pathname() else { return };

        // Build new URL
        let new_url = if url_search_params.to_string().as_string().unwrap_or_default().is_empty() {
            pathname
        } else {
            format!(
                "{}?{}",
                pathname,
                url_search_params.to_string().as_string().unwrap_or_default()
            )
        };

        // Update browser history and navigate
        let Ok(history) = window().history() else { return };
        let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));

        let options = NavigateOptions { scroll: false, ..NavigateOptions::default() };
        navigate(&new_url, options);
    }

    pub fn update_dates_url(start: Option<Date>, end: Option<Date>) {
        Self::update_url_with_params(|params| {
            // Format: YYYY-MM-DD
            if let Some(start_date) = start {
                params.set(QueryParams::START_DATE, &start_date.to_string());
            } else {
                params.delete(QueryParams::START_DATE);
            }

            if let Some(end_date) = end {
                params.set(QueryParams::END_DATE, &end_date.to_string());
            } else {
                params.delete(QueryParams::END_DATE);
            }
        });
    }

    pub fn remove_from_url(query_key: &str) {
        Self::update_url_with_params(|params| {
            params.delete(query_key);
        });
    }

    /// Silently update a single query param using replaceState (no history entry, no router re-render).
    /// Use this for reactive UI state like theme/preset that should be bookmarkable but not navigable.
    pub fn replace_param(query_key: &str, value: &str) {
        let location = window().location();
        let Ok(current_search) = location.search() else { return };
        let Ok(params) = web_sys::UrlSearchParams::new_with_str(&current_search) else { return };
        params.set(query_key, value);
        let Ok(pathname) = location.pathname() else { return };
        let new_url =
            format!("{}?{}", pathname, params.to_string().as_string().unwrap_or_default());
        let Ok(history) = window().history() else { return };
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
    }
}

const FIRST_PAGE: u32 = 1;

#[derive(Clone)]
pub struct PaginationContext {
    pub current_page: Memo<u32>,
    pub page_href: Callback<u32, String>,
    pub prev_href: Signal<String>,
    pub next_href: Signal<String>,
    pub is_first_page: Signal<bool>,
    pub aria_current: Callback<u32, &'static str>,
}

pub fn use_pagination() -> PaginationContext {
    let location = use_location();
    let current_page_str = QueryUtils::extract(QueryParams::PAGE.to_string());

    let current_page = Memo::new(move |_| {
        current_page_str().and_then(|s| s.parse::<u32>().ok()).unwrap_or(FIRST_PAGE)
    });

    let page_href = Callback::new(move |page: u32| {
        location.query.with(|q| {
            let mut params: Vec<String> = q
                .clone()
                .into_iter()
                .filter(|(key, _)| key != QueryParams::PAGE)
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();

            params.push(format!("{}={}", QueryParams::PAGE, page));

            format!("?{}", params.join("&"))
        })
    });

    let prev_href = Signal::derive(move || {
        let current = current_page.get();
        if current > FIRST_PAGE { page_href.run(current - 1) } else { "#".to_string() }
    });

    let next_href = Signal::derive(move || {
        let current = current_page.get();
        page_href.run(current + 1)
    });

    let is_first_page = Signal::derive(move || current_page.get() <= FIRST_PAGE);

    let aria_current =
        Callback::new(
            move |page: u32| if current_page.get() == page { QueryParams::PAGE } else { "" },
        );

    PaginationContext { current_page, page_href, prev_href, next_href, is_first_page, aria_current }
}
