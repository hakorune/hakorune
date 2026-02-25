//! Phase 256.8.5: Common JoinIR helpers for pattern lowering

use crate::mir::join_ir::{JoinFunction, JoinModule};

/// Phase 256.8.5: Extract entry function from JoinModule (SSOT)
///
/// Priority: join_module.entry → fallback to "main"
///
/// # Arguments
///
/// * `join_module` - The JoinModule to extract entry function from
/// * `pattern_name` - Pattern name for error messages (e.g., "pattern2", "pattern6")
///
/// # Returns
///
/// Reference to the entry JoinFunction
///
/// # Errors
///
/// Returns error if entry function not found
///
/// # Example
///
/// ```ignore
/// use super::common::get_entry_function;
///
/// let main_func = get_entry_function(&join_module, "pattern4")?;
/// let join_input_slots = main_func.params.clone();
/// ```
pub(crate) fn get_entry_function<'a>(
    join_module: &'a JoinModule,
    pattern_name: &str,
) -> Result<&'a JoinFunction, String> {
    if let Some(entry_id) = join_module.entry {
        join_module.functions.get(&entry_id)
            .ok_or_else(|| format!("[{}] Entry function {:?} not found", pattern_name, entry_id))
    } else {
        join_module.get_function_by_name("main")
            .ok_or_else(|| format!("[{}] JoinModule has no 'main' function", pattern_name))
    }
}
