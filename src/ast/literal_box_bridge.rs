//! Runtime Box conversion bridge for AST literal values.
//!
//! `syntax.rs` owns the passive frontend data model. Conversion between
//! `LiteralValue` and runtime `NyashBox` values is kept here so a future
//! frontend crate can move passive AST data without importing runtime boxes.

use super::LiteralValue;
use crate::box_trait::NyashBox;

impl LiteralValue {
    /// LiteralValueをNyashBoxに変換
    pub fn to_nyash_box(&self) -> Box<dyn NyashBox> {
        use crate::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};
        use crate::boxes::FloatBox;

        match self {
            LiteralValue::String(s) => Box::new(StringBox::new(s)),
            LiteralValue::Integer(i) => Box::new(IntegerBox::new(*i)),
            LiteralValue::TypedInteger { value, .. } => Box::new(IntegerBox::new(*value)),
            LiteralValue::Float(f) => Box::new(FloatBox::new(*f)),
            LiteralValue::Bool(b) => Box::new(BoolBox::new(*b)),
            LiteralValue::Null => Box::new(crate::boxes::null_box::NullBox::new()),
            LiteralValue::Void => Box::new(VoidBox::new()),
        }
    }

    /// NyashBoxからLiteralValueに変換
    pub fn from_nyash_box(box_val: &dyn NyashBox) -> Option<LiteralValue> {
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
}
