//! Minimal GC tracing helpers (skeleton)
//!
//! Downcast-based child edge enumeration for builtin containers.
//! This is a non-invasive helper to support diagnostics and future collectors.

use std::sync::Arc;

use crate::box_trait::NyashBox;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceUnavailable {
    MapStorage,
    ModuleRoots,
}

impl TraceUnavailable {
    pub fn reason(self) -> &'static str {
        match self {
            Self::MapStorage => "map-storage-unavailable",
            Self::ModuleRoots => "module-roots-unavailable",
        }
    }
}

/// Visit child boxes of a given object and invoke `visit(child)` for each.
/// This function recognizes builtin containers (ArrayBox/MapBox) and is a no-op otherwise.
pub fn trace_children(
    obj: &dyn NyashBox,
    visit: &mut dyn FnMut(Arc<dyn NyashBox>),
) -> Result<(), TraceUnavailable> {
    // ArrayBox
    if let Some(arr) = obj.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
        arr.with_items_read(|items| {
            for it in items.iter() {
                let arc: Arc<dyn NyashBox> = Arc::from(it.clone_box());
                visit(arc);
            }
        });
        return Ok(());
    }
    // MapBox
    if let Some(map) = obj.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
        for child in map
            .native_trace_children()
            .map_err(|_| TraceUnavailable::MapStorage)?
        {
            visit(child);
        }
    }
    Ok(())
}
