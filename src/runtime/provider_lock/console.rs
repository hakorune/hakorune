use crate::runtime::core_services::ConsoleService;
use std::sync::{Arc, OnceLock};

static CONSOLEBOX_PROVIDER: OnceLock<Arc<dyn ConsoleService>> = OnceLock::new();

/// Set the global ConsoleBox provider (can only be called once)
pub fn set_consolebox_provider(provider: Arc<dyn ConsoleService>) -> Result<(), String> {
    CONSOLEBOX_PROVIDER
        .set(provider)
        .map_err(|_| "ConsoleBox provider already set".to_string())
}

/// Get the global ConsoleBox provider
pub fn get_consolebox_provider() -> Option<&'static Arc<dyn ConsoleService>> {
    CONSOLEBOX_PROVIDER.get()
}

/// Create a fresh ConsoleService instance using the provider-lock SSOT path.
pub fn new_consolebox_provider_instance() -> Result<Arc<dyn ConsoleService>, String> {
    let registered = get_consolebox_provider().ok_or_else(|| {
        "ConsoleBox provider is not initialized. Call Runtime::initialize() first.".to_string()
    })?;
    Ok(registered.clone())
}
