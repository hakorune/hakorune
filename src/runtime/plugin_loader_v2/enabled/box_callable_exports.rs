//! Neutral callable exports for BoxCallableRegistry seeding.
//!
//! This is a provider boundary, not the callable truth. PluginLoader keeps
//! resolving plugin metadata, then BoxCallableRegistry consumes these snapshots.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginCallableExport {
    Method {
        lib_name: String,
        box_type: String,
        type_id: u32,
        method_name: String,
        arity: u8,
        method_id: u32,
        returns_result: bool,
    },
    Lifecycle {
        lib_name: String,
        box_type: String,
        type_id: u32,
        birth_id: Option<u32>,
        fini_id: Option<u32>,
    },
}
