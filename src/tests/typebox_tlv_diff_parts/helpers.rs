use crate::box_trait::{NyashBox, StringBox};
use crate::runtime::plugin_loader_unified::PluginHost;
use std::env;

// RAII: environment variable guard (restores on drop)
pub(super) struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    pub(super) fn set(key: &'static str, val: &str) -> Self {
        let prev = env::var(key).ok();
        env::set_var(key, val);
        EnvGuard { key, prev }
    }

    pub(super) fn remove(key: &'static str) -> Self {
        let prev = env::var(key).ok();
        env::remove_var(key);
        EnvGuard { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => env::set_var(self.key, v),
            None => env::remove_var(self.key),
        }
    }
}

// Helper: read-lock the global plugin host and pass immutable ref to closure
pub(super) fn with_host<R>(f: impl FnOnce(&PluginHost) -> R) -> R {
    let host = crate::runtime::get_global_plugin_host();
    let guard = host.read().expect("plugin host RwLock poisoned");
    f(&*guard)
}

// ---- Test helpers (invoke wrappers) ----
pub(super) fn inv_ok(
    h: &PluginHost,
    box_ty: &str,
    method: &str,
    id: u32,
    args: &[Box<dyn NyashBox>],
) -> Option<Box<dyn NyashBox>> {
    h.invoke_instance_method(box_ty, method, id, args)
        .expect(&format!("invoke {}::{}", box_ty, method))
}

pub(super) fn inv_some(
    h: &PluginHost,
    box_ty: &str,
    method: &str,
    id: u32,
    args: &[Box<dyn NyashBox>],
) -> Box<dyn NyashBox> {
    inv_ok(h, box_ty, method, id, args)
        .unwrap_or_else(|| panic!("{}::{} returned None", box_ty, method))
}

pub(super) fn inv_void(
    h: &PluginHost,
    box_ty: &str,
    method: &str,
    id: u32,
    args: &[Box<dyn NyashBox>],
) {
    let _ = h
        .invoke_instance_method(box_ty, method, id, args)
        .expect(&format!("invoke {}::{}", box_ty, method));
}

pub(super) fn ensure_host() {
    let _ = crate::runtime::init_global_plugin_host("nyash.toml");
}

pub(super) fn create_plugin_instance(box_type: &str) -> (String, u32, Box<dyn NyashBox>) {
    let bx = with_host(|h| h.create_box(box_type, &[]).expect("create_box"));
    if let Some(p) = bx
        .as_any()
        .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
    {
        (box_type.to_string(), p.instance_id(), bx)
    } else {
        panic!("not a plugin box: {}", bx.type_name());
    }
}

pub(super) fn rand_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX_EPOCH");
    now.as_micros() as u64
}
