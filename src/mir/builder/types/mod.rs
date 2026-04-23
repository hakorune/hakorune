//! types: 型注釈/推論の薄い箱。
//! - annotation.rs（既知の戻り型などの注釈付け）。
//! - array_element.rs（Array<T> receiver-local element publication）。
//! - map_value.rs（MapBox existing-key receiver-local publication）。
//! - inference.rs（後段、挙動不変の観測強化と最小推論）。

pub mod annotation;
pub(in crate::mir::builder) mod array_element;
pub(in crate::mir::builder) mod map_value;
