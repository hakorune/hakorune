//! Runtime Box conversion bridge for AST literal values.
//!
//! `syntax.rs` owns the passive frontend data model. Conversion between
//! `LiteralValue` and runtime `NyashBox` values is kept here so a future
//! frontend crate can move passive AST data without importing runtime boxes.

use super::LiteralValue;
use crate::box_trait::NyashBox;

/// Convert a frontend literal value into a runtime Box.
pub fn literal_to_nyash_box(value: &LiteralValue) -> Box<dyn NyashBox> {
    use crate::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};
    use crate::boxes::FloatBox;

    match value {
        LiteralValue::String(s) => Box::new(StringBox::new(s)),
        LiteralValue::Integer(i) => Box::new(IntegerBox::new(*i)),
        LiteralValue::TypedInteger { value, .. } => Box::new(IntegerBox::new(*value)),
        LiteralValue::Float(f) => Box::new(FloatBox::new(*f)),
        LiteralValue::Bool(b) => Box::new(BoolBox::new(*b)),
        LiteralValue::Null => Box::new(crate::boxes::null_box::NullBox::new()),
        LiteralValue::Void => Box::new(VoidBox::new()),
    }
}

/// Convert a runtime Box into a frontend literal value when possible.
pub fn literal_from_nyash_box(box_val: &dyn NyashBox) -> Option<LiteralValue> {
    use crate::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};
    use crate::boxes::FloatBox;

    if let Some(string_box) = box_val.as_any().downcast_ref::<StringBox>() {
        Some(LiteralValue::String(string_box.value.clone()))
    } else if let Some(int_box) = box_val.as_any().downcast_ref::<IntegerBox>() {
        Some(LiteralValue::Integer(int_box.value))
    } else if let Some(float_box) = box_val.as_any().downcast_ref::<FloatBox>() {
        Some(LiteralValue::Float(float_box.value))
    } else if let Some(bool_box) = box_val.as_any().downcast_ref::<BoolBox>() {
        Some(LiteralValue::Bool(bool_box.value))
    } else if box_val
        .as_any()
        .downcast_ref::<crate::boxes::null_box::NullBox>()
        .is_some()
    {
        Some(LiteralValue::Null)
    } else if box_val.as_any().downcast_ref::<VoidBox>().is_some() {
        Some(LiteralValue::Void)
    } else {
        None
    }
}
