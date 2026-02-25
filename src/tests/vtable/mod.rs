// Legacy boundary cases (pre-JoinIR). Disable by default.
#[cfg(feature = "legacy-tests")]
#[path = "../vtable_map_boundaries.rs"]
pub mod vtable_map_boundaries;
#[path = "../vtable_map_ext.rs"]
pub mod vtable_map_ext;
#[path = "../vtable_strict.rs"]
pub mod vtable_strict;
#[path = "../vtable_string.rs"]
pub mod vtable_string;
#[cfg(feature = "legacy-tests")]
#[path = "../vtable_string_boundaries.rs"]
pub mod vtable_string_boundaries;
