use crate::runtime::core_services::ArrayService;
use std::sync::{Arc, OnceLock};

static ARRAYBOX_PROVIDER: OnceLock<Arc<dyn ArrayService>> = OnceLock::new();

/// Set the global ArrayBox provider (can only be called once)
pub fn set_arraybox_provider(provider: Arc<dyn ArrayService>) -> Result<(), String> {
    ARRAYBOX_PROVIDER
        .set(provider)
        .map_err(|_| "ArrayBox provider already set".to_string())
}

/// Get the global ArrayBox provider
pub fn get_arraybox_provider() -> Option<&'static Arc<dyn ArrayService>> {
    ARRAYBOX_PROVIDER.get()
}

/// Create a fresh ArrayService instance using the provider-lock SSOT path.
pub fn new_arraybox_provider_instance() -> Result<Arc<dyn ArrayService>, String> {
    let registered = get_arraybox_provider().ok_or_else(|| {
        "ArrayBox provider is not initialized. Call Runtime::initialize() first.".to_string()
    })?;
    Ok(registered.clone())
}
