window.__TAURI_ISOLATION_HOOK__ = (data) => {
    console.debug("[IsolationHook]", data)
    return data
}
