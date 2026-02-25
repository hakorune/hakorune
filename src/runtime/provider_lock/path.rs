use std::sync::{Arc, OnceLock};

pub trait PathService: Send + Sync {
    fn join(&self, base: &str, rest: &str) -> String;
    fn dirname(&self, path: &str) -> String;
    fn basename(&self, path: &str) -> String;
    fn extname(&self, path: &str) -> String;
    fn is_abs(&self, path: &str) -> bool;
    fn normalize(&self, path: &str) -> String;
}

static PATHBOX_PROVIDER: OnceLock<Arc<dyn PathService>> = OnceLock::new();

/// Set the global PathBox provider (can only be called once)
pub fn set_pathbox_provider(provider: Arc<dyn PathService>) -> Result<(), String> {
    PATHBOX_PROVIDER
        .set(provider)
        .map_err(|_| "PathBox provider already set".to_string())
}

/// Get the global PathBox provider
pub fn get_pathbox_provider() -> Option<&'static Arc<dyn PathService>> {
    PATHBOX_PROVIDER.get()
}

/// Obtain a PathService instance using the provider-lock SSOT path.
///
/// PathService is currently stateless, so this returns a cloned `Arc`
/// for the registered provider instance.
pub fn get_pathbox_provider_instance() -> Result<Arc<dyn PathService>, String> {
    let registered = get_pathbox_provider().ok_or_else(|| {
        "PathBox provider is not initialized. Call Runtime::initialize() first.".to_string()
    })?;
    Ok(registered.clone())
}

/// Compatibility alias.
///
/// Kept temporarily so existing docs/scripts do not break while callers
/// migrate to `get_pathbox_provider_instance`.
pub fn new_pathbox_provider_instance() -> Result<Arc<dyn PathService>, String> {
    get_pathbox_provider_instance()
}
