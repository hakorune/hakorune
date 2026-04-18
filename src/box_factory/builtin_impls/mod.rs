/*!
 * Builtin Box fallback implementations.
 *
 * These modules remain only for runtime fallback / selfhost support paths.
 * Plugin-preferred routes may still use them when no external provider is active.
 * `compat_map_box.rs` is the deprecated MapBox constructor shim only; the
 * live MapBox implementation stays in `src/boxes/map_box.rs`.
 * `null_box.rs` stays as the surface/compat constructor for the runtime no-value family.
 */

pub mod array_box;
pub mod bool_box;
#[cfg(feature = "builtin-mapbox-compat")]
pub mod compat_map_box; // deprecated MapBox constructor shim
pub mod console_box; // builtin fallback for selfhost support (plugin-preferred)
pub mod integer_box;
pub mod string_box;

pub mod file_box; // core fallback for auto/core-ro modes
pub mod filehandle_box;
pub mod path_box;

pub mod null_box; // surface/compat alias over runtime no-value semantics
