//! Basic box implementations
//!
//! This module contains the core basic Box types that implement the
//! fundamental data types in Nyash: String, Integer, Boolean, Void, File, and Error.

// Individual basic box implementations
mod bool_box;
mod error_box;
mod file_box;
mod integer_box;
mod string_box;
mod void_box;

// Re-export all basic box types
pub use bool_box::BoolBox;
pub use error_box::ErrorBox;
pub use file_box::FileBox;
pub use integer_box::IntegerBox;
pub use string_box::StringBox;
pub use void_box::VoidBox;
