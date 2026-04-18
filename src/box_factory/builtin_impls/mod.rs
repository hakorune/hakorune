/*!
 * Builtin Box fallback implementations.
 *
 * These modules remain only for runtime fallback / selfhost support paths.
 * Plugin-preferred routes may still use them when no external provider is active.
 * ArrayBox/MapBox/PathBox/ConsoleBox construction goes through ring1 provider
 * seams; do not add standalone builtin fallbacks here.
 * `null_box.rs` stays as the surface/compat constructor for the runtime no-value family.
 */

pub mod bool_box;
pub mod integer_box;
pub mod string_box;

pub mod file_box; // core fallback for auto/core-ro modes
pub mod filehandle_box;

pub mod null_box; // surface/compat alias over runtime no-value semantics
