use super::PluginLoaderV2;
use crate::bid::{BidError, BidResult};
use crate::runtime::get_global_ring0;

pub(super) fn load_config(loader: &mut PluginLoaderV2, config_path: &str) -> BidResult<()> {
    let canonical = std::fs::canonicalize(config_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| config_path.to_string());
    if std::path::Path::new(&canonical)
        .file_name()
        .and_then(|s| s.to_str())
        == Some("nyash.toml")
        && !std::path::Path::new("hako.toml").exists()
    {
        crate::runtime::deprecations::warn_nyash_toml_used_once();
    }
    let content = std::fs::read_to_string(&canonical).map_err(|e| {
        get_global_ring0().log.error(&format!(
            "[plugin/init] failed to read {}: {}",
            canonical, e
        ));
        BidError::PluginError
    })?;
    let parsed_cfg =
        crate::config::nyash_toml_v2::NyashConfigV2::from_str(&content).map_err(|e| {
            get_global_ring0().log.error(&format!(
                "[plugin/init] failed to parse {}: {}",
                canonical, e
            ));
            BidError::PluginError
        })?;
    let parsed_toml = toml::from_str::<toml::Value>(&content).map_err(|e| {
        get_global_ring0().log.error(&format!(
            "[plugin/init] failed to parse TOML {}: {}",
            canonical, e
        ));
        BidError::PluginError
    })?;

    loader.config_path = Some(canonical.clone());
    loader.config = Some(parsed_cfg);
    loader.config_toml = Some(parsed_toml);
    if let Some(cfg) = loader.config.as_ref() {
        let mut labels: Vec<String> = Vec::new();
        for (_lib, def) in &cfg.libraries {
            for bt in &def.boxes {
                labels.push(format!("BoxRef:{}", bt));
            }
        }
        crate::runtime::cache_versions::bump_many(&labels);
    }
    Ok(())
}
