//! ring1 PathService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::runtime::provider_lock::PathService;
use crate::{box_trait::NyashBox, boxes::path_box::PathBox, runtime::provider_lock};
use std::path::{Component, Path};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Ring1PathService;

impl Ring1PathService {
    pub fn new() -> Self {
        Self
    }
}

/// Create the canonical runtime PathBox through the ring1 path provider seam.
pub fn new_path_box() -> Result<Box<dyn NyashBox>, String> {
    if provider_lock::get_pathbox_provider().is_none() {
        let _ = provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()));
    }
    Ok(Box::new(PathBox::try_new()?))
}

impl PathService for Ring1PathService {
    fn join(&self, base: &str, rest: &str) -> String {
        if base.ends_with('/') || base.ends_with('\\') {
            format!("{base}{rest}")
        } else {
            format!("{base}/{rest}")
        }
    }

    fn dirname(&self, path: &str) -> String {
        Path::new(path)
            .parent()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn basename(&self, path: &str) -> String {
        Path::new(path)
            .file_name()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn extname(&self, path: &str) -> String {
        Path::new(path)
            .extension()
            .map(|x| format!(".{}", x.to_string_lossy()))
            .unwrap_or_default()
    }

    fn is_abs(&self, path: &str) -> bool {
        Path::new(path).is_absolute() || path.contains(":\\")
    }

    fn normalize(&self, path: &str) -> String {
        let mut parts: Vec<String> = Vec::new();
        let p = Path::new(path);
        let mut absolute = false;
        let mut prefix: Option<String> = None;

        for comp in p.components() {
            match comp {
                Component::Prefix(pre) => {
                    prefix = Some(pre.as_os_str().to_string_lossy().to_string());
                }
                Component::RootDir => {
                    absolute = true;
                }
                Component::CurDir => {}
                Component::ParentDir => {
                    if !parts.is_empty() {
                        parts.pop();
                    }
                }
                Component::Normal(seg) => parts.push(seg.to_string_lossy().to_string()),
            }
        }

        let body = parts.join("/");
        match (prefix, absolute, body.is_empty()) {
            (Some(pre), _, true) => pre,
            (Some(pre), _, false) => format!("{pre}/{body}"),
            (None, true, true) => "/".to_string(),
            (None, true, false) => format!("/{body}"),
            (None, false, _) => body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring1_path_service_basics() {
        let provider = Ring1PathService::new();
        assert_eq!(provider.join("apps", "tests"), "apps/tests");
        assert_eq!(provider.dirname("apps/tests/main.hako"), "apps/tests");
        assert_eq!(provider.basename("apps/tests/main.hako"), "main.hako");
        assert_eq!(provider.extname("apps/tests/main.hako"), ".hako");
        assert!(!provider.is_abs("apps/tests/main.hako"));
        assert_eq!(
            provider.normalize("./apps/./tests/../tests/main.hako"),
            "apps/tests/main.hako"
        );
    }

    #[test]
    fn ring1_path_new_box_returns_pathbox() {
        let boxed = new_path_box().unwrap();
        assert!(boxed.as_any().downcast_ref::<PathBox>().is_some());
    }
}
