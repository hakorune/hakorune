use super::loader::PluginLoaderV2;
use crate::bid::BidResult;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

static GLOBAL_LOADER_V2: Lazy<Arc<RwLock<PluginLoaderV2>>> =
    Lazy::new(|| Arc::new(RwLock::new(PluginLoaderV2::new())));

pub fn get_global_loader_v2() -> Arc<RwLock<PluginLoaderV2>> {
    GLOBAL_LOADER_V2.clone()
}

pub fn init_global_loader_v2(config_path: &str) -> BidResult<()> {
    let loader = get_global_loader_v2();
    let mut loader = super::errors::from_rwlock_write(loader.write())?;
    loader.load_config(config_path)?;
    drop(loader);
    let loader = get_global_loader_v2();
    let loader = super::errors::from_rwlock_read(loader.read())?;
    loader.load_all_plugins()
}

pub fn shutdown_plugins_v2() -> BidResult<()> {
    let loader = get_global_loader_v2();
    let loader = super::errors::from_rwlock_read(loader.read())?;
    loader.shutdown_singletons();
    Ok(())
}
