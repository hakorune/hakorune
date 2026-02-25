use crate::boxes::file::core_ro::CoreRoFileIo;
use crate::boxes::file::provider::{FileCaps, FileIo};
use crate::providers::ring1::file::nofs_fileio::NoFsFileIo;
use crate::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo;
use crate::runtime::get_global_ring0;
use std::sync::{Arc, OnceLock};

static FILEBOX_PROVIDER: OnceLock<Arc<dyn FileIo>> = OnceLock::new();

/// Set the global FileBox provider (can only be called once)
pub fn set_filebox_provider(provider: Arc<dyn FileIo>) -> Result<(), String> {
    FILEBOX_PROVIDER
        .set(provider)
        .map_err(|_| "FileBox provider already set".to_string())
}

/// Get the global FileBox provider
pub fn get_filebox_provider() -> Option<&'static Arc<dyn FileIo>> {
    FILEBOX_PROVIDER.get()
}

/// Convenience: fetch current FileBox provider capabilities (if initialized).
/// Returns None when no provider is registered yet.
pub fn get_filebox_caps() -> Option<FileCaps> {
    get_filebox_provider().map(|p| p.caps())
}

/// Create a fresh FileIo instance using the provider-lock SSOT path.
///
/// This is the only supported allocation route for FileBox/FileHandleBox runtime instances.
/// `mode_hint` is applied when the selected provider supports mode configuration.
pub fn new_filebox_provider_instance(mode_hint: Option<&str>) -> Result<Arc<dyn FileIo>, String> {
    let registered = get_filebox_provider().ok_or_else(|| {
        "FileBox provider is not initialized. Call Runtime::initialize() first.".to_string()
    })?;

    let provider: Arc<dyn FileIo> = if registered.as_ref().as_any().is::<Ring0FsFileIo>() {
        Arc::new(Ring0FsFileIo::new(get_global_ring0()))
    } else if registered.as_ref().as_any().is::<CoreRoFileIo>() {
        Arc::new(CoreRoFileIo::new())
    } else if registered.as_ref().as_any().is::<NoFsFileIo>() {
        Arc::new(NoFsFileIo)
    } else {
        // Unknown provider shape: keep legacy behavior by sharing the registered instance.
        // This is intentionally noisy so provider implementations can add factory support.
        get_global_ring0()
            .log
            .warn("[provider-lock] FileBox provider instance factory is not implemented for this provider type; reusing shared Arc");
        registered.clone()
    };

    if let Some(mode) = mode_hint {
        if let Some(ring0_provider) = provider.as_ref().as_any().downcast_ref::<Ring0FsFileIo>() {
            ring0_provider.set_mode(mode.to_string());
        }
    }

    Ok(provider)
}
