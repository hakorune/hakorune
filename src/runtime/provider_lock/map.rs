use crate::runtime::core_services::MapService;
use std::sync::{Arc, OnceLock};

static MAPBOX_PROVIDER: OnceLock<Arc<dyn MapService>> = OnceLock::new();

/// Set the global MapBox provider (can only be called once)
pub fn set_mapbox_provider(provider: Arc<dyn MapService>) -> Result<(), String> {
    MAPBOX_PROVIDER
        .set(provider)
        .map_err(|_| "MapBox provider already set".to_string())
}

/// Get the global MapBox provider
pub fn get_mapbox_provider() -> Option<&'static Arc<dyn MapService>> {
    MAPBOX_PROVIDER.get()
}

/// Create a fresh MapService instance using the provider-lock SSOT path.
pub fn new_mapbox_provider_instance() -> Result<Arc<dyn MapService>, String> {
    let registered = get_mapbox_provider().ok_or_else(|| {
        "MapBox provider is not initialized. Call Runtime::initialize() first.".to_string()
    })?;
    Ok(registered.clone())
}
