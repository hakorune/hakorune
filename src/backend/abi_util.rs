/*!
 * Backend ABI/utility consolidation (minimal)
 *
 * Shared helpers for handle/ptr/to_bool/compare/tag/invoke scaffolding.
 * Initial scope focuses on value coercions used by the MIR interpreter and JIT.
 */

use crate::backend::runtime_type_tag::{tag_from_vmvalue, tag_to_str};
use crate::backend::vm::VMValue;
use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox};
use crate::boxes::null_box::NullBox;
use std::sync::Arc;

/// Opaque handle type used by JIT/runtime bridges.
pub type Handle = u64;

/// Convert a VMValue to boolean using unified, fail-fast semantics (Phase 275 A1).
/// SSOT: docs/reference/language/types.md (Section 3: Boolean Context)
pub fn to_bool_vm(v: &VMValue) -> Result<bool, String> {
    match v {
        VMValue::Bool(b) => Ok(*b),
        VMValue::Integer(i) => Ok(*i != 0),
        VMValue::Void => Err("Void in boolean context".to_string()),
        VMValue::String(s) => Ok(!s.is_empty()),
        VMValue::Float(f) => Ok(*f != 0.0),
        VMValue::BoxRef(b) => {
            // Bridge boxes: allow unboxing to primitive for truthiness
            if let Some(bb) = b.as_any().downcast_ref::<BoolBox>() {
                return Ok(bb.value);
            }
            if let Some(ib) = b.as_any().downcast_ref::<IntegerBox>() {
                return Ok(ib.value != 0);
            }
            if let Some(sb) = b.as_any().downcast_ref::<StringBox>() {
                return Ok(!sb.value.is_empty());
            }
            // Null/Void surface aliases share one runtime no-value meaning.
            if NullBox::check_null(b.as_ref()) {
                return Err("Null/Void box in boolean context".to_string());
            }
            Err(format!("cannot coerce BoxRef({}) to bool", b.type_name()))
        }
        VMValue::Future(_) => Err("cannot coerce Future to bool".to_string()),
        // Phase 285A0: WeakRef in boolean context is TypeError
        VMValue::WeakBox(_) => Err("WeakRef in boolean context - use upgrade() first".to_string()),
    }
}

/// Nyash-style equality on VMValue with precise number-only coercion (Phase 275 B2).
pub fn eq_vm(a: &VMValue, b: &VMValue) -> bool {
    use VMValue::*;
    match (a, b) {
        (Integer(x), Integer(y)) => x == y,
        (Float(x), Float(y)) => x == y,
        (Bool(x), Bool(y)) => x == y,
        (String(x), String(y)) => x == y,
        (Void, Void) => true,
        // Precise Int↔Float equality (avoid accidental true via float rounding)
        (Integer(x), Float(y)) | (Float(y), Integer(x)) => {
            if y.is_nan() {
                return false;
            }
            if y.is_finite() && y.fract() == 0.0 {
                // Float is integral - check exact representability
                let y_int = *y as i64;
                if (y_int as f64) == *y {
                    return x == &y_int;
                }
            }
            false
        }
        (BoxRef(ax), BoxRef(by)) => Arc::ptr_eq(ax, by),
        // Treat BoxRef(NullBox/VoidBox/MissingBox) as equal to runtime Void for compatibility.
        (BoxRef(bx), Void) => {
            NullBox::check_null(bx.as_ref())
                || bx
                    .as_any()
                    .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                    .is_some()
        }
        (Void, BoxRef(bx)) => {
            NullBox::check_null(bx.as_ref())
                || bx
                    .as_any()
                    .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                    .is_some()
        }
        // Phase 285A0: WeakBox equality
        (WeakBox(wa), WeakBox(wb)) => {
            match (wa.upgrade(), wb.upgrade()) {
                (Some(arc_a), Some(arc_b)) => Arc::ptr_eq(&arc_a, &arc_b),
                (None, None) => true, // Both dropped
                _ => false,
            }
        }
        // WeakBox == Void when dropped
        (WeakBox(w), Void) | (Void, WeakBox(w)) => w.upgrade().is_none(),
        _ => false,
    }
}

/// Obtain a human-readable tag/type name for a VMValue.
/// (Delegates to runtime_type_tag.rs for SSOT consolidation)
pub fn tag_of_vm(v: &VMValue) -> &'static str {
    tag_to_str(tag_from_vmvalue(v))
}

/// Wrap a NyashBox object into a handle using runtime handle registry.
/// This keeps a single handle mechanism across backends.
/// ARCHIVED: JIT handle implementation moved to archive/jit-cranelift/ during Phase 15
pub fn handle_of(_boxref: Arc<dyn NyashBox>) -> Handle {
    // TODO: Implement handle registry for Phase 15 VM/LLVM backends
    // For now, use a simple 0-handle placeholder
    0
}

/// Try to resolve a handle back to a Box object.
/// ARCHIVED: JIT handle implementation moved to archive/jit-cranelift/ during Phase 15
pub fn handle_get(_h: Handle) -> Option<Arc<dyn NyashBox>> {
    // TODO: Implement handle registry for Phase 15 VM/LLVM backends
    None
}
