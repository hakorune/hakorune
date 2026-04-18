/*!
 * Builtin Box Factory (Phase 15.5: Transitioning to "Everything is Plugin")
 *
 * ⚠️ MIGRATION IN PROGRESS: Phase 15.5 Core Box Unification
 * 🎯 Goal: Remove builtin priority, make all Boxes plugin-based
 * 📋 Current: builtin > user > plugin (PROBLEMATIC)
 * 🚀 Target: plugin > user > builtin_compat (Phase 1) → plugin-only (Phase 3)
 *
 * Implementation Strategy:
 * - Phase 0: ✅ Separate implementations to builtin_impls/ (easy deletion)
 * - Phase 1: 🚧 Add strict_plugin_first policy + access guards
 * - Phase 2: 🔄 Delete builtin_impls/ files one by one
 * - Phase 3: ❌ Delete BuiltinBoxFactory entirely
 */

use super::BoxFactory;
use super::RuntimeError;
use crate::box_trait::NyashBox;

// Separated implementations (Phase 0: ✅ Complete)
use super::builtin_impls;

/// Factory for builtin Box types
pub struct BuiltinBoxFactory;

impl BuiltinBoxFactory {
    pub fn new() -> Self {
        Self
    }
}

impl BoxFactory for BuiltinBoxFactory {
    fn create_box(
        &self,
        name: &str,
        args: &[Box<dyn NyashBox>],
    ) -> Result<Box<dyn NyashBox>, RuntimeError> {
        // Phase 0: ✅ Route to separated implementations (easy deletion)
        match name {
            // Phase 2.1-2.2: DELETE when plugins are confirmed working
            "StringBox" => builtin_impls::string_box::create(args),
            "IntegerBox" => builtin_impls::integer_box::create(args),

            // Phase 2.3: DELETE when BoolBox plugin is created
            "BoolBox" => builtin_impls::bool_box::create(args),

            // Collection constructors are owned by ring1 provider seams.
            "ArrayBox" => Ok(crate::providers::ring1::array::new_array_box()),
            "MapBox" => Ok(crate::providers::ring1::map::new_map_box()),

            // Phase 151: selfhost fallback, owned by the ring1 console seam.
            "ConsoleBox" => Ok(crate::providers::ring1::console::new_console_box()),

            // Phase 15.5: Fallback support (auto/core-ro modes)
            "FileBox" => builtin_impls::file_box::create(args),
            "PathBox" => crate::providers::ring1::path::new_path_box()
                .map_err(|message| RuntimeError::InvalidOperation { message }),

            // Phase 113: FileHandleBox Nyash API
            "FileHandleBox" => builtin_impls::filehandle_box::create(args),

            // Surface/compat alias for the runtime no-value family.
            "NullBox" => builtin_impls::null_box::create(args),

            // Leave other types to other factories (user/plugin)
            _ => Err(RuntimeError::InvalidOperation {
                message: format!("Unknown Box type: {}", name),
            }),
        }
    }

    fn box_types(&self) -> Vec<&str> {
        vec![
            // Primitive wrappers
            "StringBox",
            "IntegerBox",
            "BoolBox",
            // Collections/common
            "ArrayBox",
            "MapBox",
            // Phase 151: ConsoleBox builtin fallback for selfhost support
            "ConsoleBox",
            // Fallback support
            "FileBox",
            "PathBox",
            "FileHandleBox", // Phase 113
            "NullBox",
        ]
    }

    fn is_builtin_factory(&self) -> bool {
        true
    }
}
