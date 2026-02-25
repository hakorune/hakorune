// IntArrayCore helpers for AOT/VM bridge (handle-based, ring1 numeric core)
// API (Hako-facing via externcall):
// - nyash.intarray.new_h(len)   -> handle (IntArrayCore)
// - nyash.intarray.len_h(h)     -> i64
// - nyash.intarray.get_hi(h,i)  -> i64
// - nyash.intarray.set_hii(h,i,v) -> i64 (0=ok, non-zero=error)

use nyash_rust::{
    box_trait::{BoxCore, NyashBox, StringBox},
    boxes::basic::BoolBox,
    runtime::host_handles as handles,
};
use std::any::Any;
use std::sync::RwLock;

/// Minimal numeric core: contiguous i64 buffer + length.
/// This box is intended for internal numeric kernels (matmul_core 等) 専用で、
/// 一般APIは .hako 側のラッパー（MatI64 等）から利用する。
#[derive(Debug)]
pub struct IntArrayCore {
    base: nyash_rust::box_trait::BoxBase,
    data: RwLock<Vec<i64>>,
}

impl IntArrayCore {
    pub fn new(len: i64) -> Self {
        let n = if len <= 0 { 0 } else { len as usize };
        IntArrayCore {
            base: nyash_rust::box_trait::BoxBase::new(),
            data: RwLock::new(vec![0; n]),
        }
    }

    pub fn len_i64(&self) -> i64 {
        self.data.read().unwrap().len() as i64
    }

    pub fn get_i64(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let i = idx as usize;
        let guard = self.data.read().unwrap();
        guard.get(i).copied()
    }

    pub fn set_i64(&self, idx: i64, v: i64) -> bool {
        if idx < 0 {
            return false;
        }
        let i = idx as usize;
        let mut guard = self.data.write().unwrap();
        if i >= guard.len() {
            return false;
        }
        guard[i] = v;
        true
    }
}

impl BoxCore for IntArrayCore {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IntArrayCore(len={})", self.data.read().unwrap().len())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for IntArrayCore {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(&format!(
            "IntArrayCore(len={})",
            self.data.read().unwrap().len()
        ))
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(o) = other.as_any().downcast_ref::<IntArrayCore>() {
            BoolBox::new(*self.data.read().unwrap() == *o.data.read().unwrap())
        } else {
            BoolBox::new(false)
        }
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(IntArrayCore {
            base: self.base.clone(),
            data: RwLock::new(self.data.read().unwrap().clone()),
        })
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        // Identity semantics are not required here; clone is fine.
        self.clone_box()
    }
}

// --- Extern API (handle-based) ---

fn get_core(handle: i64) -> Option<std::sync::Arc<dyn NyashBox>> {
    if handle <= 0 {
        return None;
    }
    handles::get(handle as u64)
}

#[export_name = "nyash.intarray.new_h"]
pub extern "C" fn nyash_intarray_new_h(len: i64) -> i64 {
    let core = IntArrayCore::new(len);
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(core);
    let h = handles::to_handle_arc(arc) as i64;
    if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
        eprintln!("[INTARRAY] new_h(len={}) -> handle={}", len, h);
    }
    h
}

#[export_name = "nyash.intarray.len_h"]
pub extern "C" fn nyash_intarray_len_h(handle: i64) -> i64 {
    if let Some(obj) = get_core(handle) {
        if let Some(core) = obj.as_any().downcast_ref::<IntArrayCore>() {
            return core.len_i64();
        }
    }
    0
}

#[export_name = "nyash.intarray.get_hi"]
pub extern "C" fn nyash_intarray_get_hi(handle: i64, idx: i64) -> i64 {
    if let Some(obj) = get_core(handle) {
        if let Some(core) = obj.as_any().downcast_ref::<IntArrayCore>() {
            if let Some(v) = core.get_i64(idx) {
                return v;
            }
        }
    }
    0
}

#[export_name = "nyash.intarray.set_hii"]
pub extern "C" fn nyash_intarray_set_hii(handle: i64, idx: i64, val: i64) -> i64 {
    if let Some(obj) = get_core(handle) {
        if let Some(core) = obj.as_any().downcast_ref::<IntArrayCore>() {
            return if core.set_i64(idx, val) { 0 } else { 1 };
        }
    }
    1
}
