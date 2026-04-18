/*!
 * Builtin Box fallback implementations.
 *
 * These modules remain only for runtime fallback / selfhost support paths.
 * Plugin-preferred routes may still use them when no external provider is active.
 * MapBox construction goes through the ring1 map provider seam; do not add a
 * standalone builtin MapBox fallback here.
 * `null_box.rs` stays as the surface/compat constructor for the runtime no-value family.
 */

pub mod array_box;
pub mod bool_box;
pub mod console_box; // builtin fallback for selfhost support (plugin-preferred)
pub mod integer_box;
pub mod string_box;

pub mod file_box; // core fallback for auto/core-ro modes
pub mod filehandle_box;
pub mod path_box;

pub mod null_box; // surface/compat alias over runtime no-value semantics
