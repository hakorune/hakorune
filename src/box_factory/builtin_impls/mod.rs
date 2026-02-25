/*!
 * Builtin Box Implementations (Phase 15.5: Everything is Plugin Migration)
 *
 * 🎯 Purpose: Separated implementations for easy deletion during Phase 2
 * 🗓️ Timeline: These files will be deleted one by one in Phase 2.1-2.6
 *
 * Deletion Order (dependency-based):
 * 1. string_box.rs    - Phase 2.1 ✅ Plugin ready
 * 2. integer_box.rs   - Phase 2.2 ✅ Plugin ready
 * 3. bool_box.rs      - Phase 2.3 🔄 Plugin needed
 * 4. array_box.rs     - Phase 2.4 🔄 Plugin check needed
 * 5. map_box.rs       - Phase 2.5 🔄 Plugin check needed
 * 6. console_box.rs   - Phase 125 ✅ DELETED - Plugin-only (nyash-console-plugin)
 * 7. null_box.rs      - TBD: 🤔 Keep as language primitive?
 */

// Phase 2.1-2.6: Delete these modules one by one
pub mod array_box; // DELETE: Phase 2.4 (plugin check)
pub mod bool_box; // DELETE: Phase 2.3 (plugin needed)
pub mod console_box; // Phase 151: Builtin fallback for selfhost support (plugin-preferred)
pub mod integer_box; // DELETE: Phase 2.2 (plugin ready)
pub mod map_box; // DELETE: Phase 2.5 (plugin check)
pub mod string_box; // DELETE: Phase 2.1 (plugin ready)

// Fallback support (Phase 15.5: Fallback Guarantee)
pub mod file_box; // FALLBACK: Core-ro FileBox for auto/core-ro modes
pub mod filehandle_box; // Phase 113: FileHandleBox Nyash API
pub mod path_box; // Path provider-backed builtin bridge

// Special consideration
pub mod null_box; // DISCUSS: Keep as primitive?
