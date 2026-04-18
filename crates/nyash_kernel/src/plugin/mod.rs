// Plugin-side runtime surface.
// Keep public host-service glue visible here, but keep compat quarantine internal-only.

pub mod array;
mod array_compat;
mod array_guard;
mod array_handle_cache;
mod array_runtime_aliases;
mod array_runtime_any;
mod array_runtime_facade;
mod array_runtime_substrate;
mod array_slot_append;
mod array_slot_capacity;
mod array_slot_load;
mod array_slot_store;
mod array_string_slot;
mod array_substrate;
pub mod birth;
mod compat_invoke_core;
pub mod console;
pub mod future;
mod handle_cache;
pub mod instance;
pub mod intarray;
pub mod invoke;
pub mod invoke_core;
pub mod map;
mod map_aliases;
mod map_compat;
mod map_debug;
mod map_key_codec;
mod map_probe;
mod map_runtime_data;
mod map_runtime_facade;
mod map_slot_load;
mod map_slot_store;
mod map_substrate;
pub(crate) mod module_string_dispatch;
pub mod runtime_data;
pub mod semantics;
pub mod string;
mod value_codec;

pub use array::*;
pub use birth::*;
pub use console::*;
pub use future::*;
pub use instance::*;
pub use intarray::*;
pub use invoke::*;
pub use invoke_core::*;
pub use map::*;
pub use runtime_data::*;
pub use semantics::*;
pub use string::*;
pub use value_codec::KernelTextSlot;
#[allow(unused_imports)]
pub(crate) use value_codec::KernelTextSlotState;
pub(crate) use value_codec::{
    freeze_owned_string_into_slot, issue_fresh_handle_from_arc, materialize_owned_string,
    materialize_owned_string_generic_fallback, materialize_owned_string_generic_fallback_for_site,
    owned_string_from_handle, publish_kernel_text_slot, with_kernel_text_slot_text,
    StringPublishSite,
};

#[cfg(test)]
#[cfg(test)]
mod wiring_tests {
    #[test]
    fn b3_public_wiring_contract_compiles() {
        // B3-closeout lock: keep future/invoke public entry wiring stable.
        // These bindings intentionally fail to compile if crate-root re-export changes.
        let _future_spawn_method_h: extern "C" fn(
            i64,
            i64,
            i64,
            i64,
            *const i64,
            *const i64,
        ) -> i64 = crate::nyash_future_spawn_method_h;
        let _future_spawn_instance3_i64: extern "C" fn(i64, i64, i64, i64) -> i64 =
            crate::nyash_future_spawn_instance3_i64;
        let _env_future_spawn_instance: extern "C" fn(i64, i64, i64, i64) -> i64 =
            crate::env_future_spawn_instance;
        let _env_future_new: extern "C" fn(i64) -> i64 = crate::env_future_new;
        let _env_future_set: extern "C" fn(i64, i64) -> i64 = crate::env_future_set;
        let _env_future_await: extern "C" fn(i64) -> i64 = crate::env_future_await;
        let _future_delay_i64: extern "C" fn(i64) -> i64 = crate::nyash_future_delay_i64;

        let _invoke3_i64: extern "C" fn(i64, i64, i64, i64, i64, i64) -> i64 =
            crate::nyash_plugin_invoke3_i64;
        let _invoke3_f64: extern "C" fn(i64, i64, i64, i64, i64, i64) -> f64 =
            crate::nyash_plugin_invoke3_f64;
        let _invoke_by_name_i64: extern "C" fn(i64, *const i8, i64, i64, i64) -> i64 =
            crate::nyash_plugin_invoke_by_name_i64;
        let _invoke3_tagged_i64: extern "C" fn(
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
        ) -> i64 = crate::nyash_plugin_invoke3_tagged_i64;
        let _invoke_tagged_v_i64: extern "C" fn(i64, i64, i64, i64, *const i64, *const i64) -> i64 =
            crate::nyash_plugin_invoke_tagged_v_i64;

        let _ = (
            _future_spawn_method_h,
            _future_spawn_instance3_i64,
            _env_future_spawn_instance,
            _env_future_new,
            _env_future_set,
            _env_future_await,
            _future_delay_i64,
            _invoke3_i64,
            _invoke3_f64,
            _invoke_by_name_i64,
            _invoke3_tagged_i64,
            _invoke_tagged_v_i64,
        );
    }
}
