/*!
 * Method Resolution System
 *
 * Type-safe function and method resolution at compile-time
 * ChatGPT5 Pro design for preventing runtime string-based resolution
 */

use crate::mir::policies::call_name_classification::{
    classify_call_name_v1, CallNameCalleeClassV1,
};
use crate::mir::{Callee, ValueId};
use hakorune_mir_defs::CanonicalGlobalTargetV1;
use std::collections::BTreeMap;

/// Resolve function call target to type-safe Callee
/// Implements the core logic of compile-time function resolution
pub fn resolve_call_target(
    name: &str,
    variable_map: &BTreeMap<String, ValueId>,
) -> Result<Callee, String> {
    let name_classification = classify_call_name_v1(name);

    // 1. Check for built-in/global functions first
    if name_classification.callee_class() == CallNameCalleeClassV1::BuiltinGlobal {
        if name == "print" {
            return Ok(Callee::Global(CanonicalGlobalTargetV1::builtin_print()));
        }
        return Err(format!(
            "unsupported builtin global target without a typed issuer: {name}"
        ));
    }

    // 2. Check for local variable containing function value
    if let Some(&value_id) = variable_map.get(name) {
        return Ok(Callee::Value(value_id));
    }

    // 3. Check for external/host functions
    if name_classification.callee_class() == CallNameCalleeClassV1::Extern {
        return Ok(Callee::Extern(name.to_string()));
    }

    // 4. Resolution failed - prevent runtime string-based resolution.
    //    Static receiver selection belongs to an exact catalog/source owner;
    //    this resolver must not manufacture a receiverless Method.
    Err(format!(
        "Unresolved function: '{}'. {}",
        name,
        suggest_resolution(name)
    ))
}

/// Suggest resolution for unresolved function
pub fn suggest_resolution(name: &str) -> String {
    match name {
        n if n.starts_with("console") => "Did you mean 'env.console.log' or 'print'?".to_string(),
        "log" | "println" => "Did you mean 'print' or 'env.console.log'?".to_string(),
        n if n.contains('.') => {
            "Qualified names should use 'env.' prefix for external calls.".to_string()
        }
        _ => "Check function name or ensure it's in scope.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_name_facts_preserve_resolution_priority() {
        let mut variables = BTreeMap::new();
        variables.insert("print".to_string(), ValueId(1));
        variables.insert("env.local".to_string(), ValueId(3));

        assert!(matches!(
            resolve_call_target("print", &variables).unwrap(),
            Callee::Global(target)
                if target == CanonicalGlobalTargetV1::builtin_print()
        ));
        assert!(matches!(
            resolve_call_target("env.local", &variables).unwrap(),
            Callee::Value(ValueId(3))
        ));
        assert!(matches!(
            resolve_call_target("system.clock", &variables).unwrap(),
            Callee::Extern(name) if name == "system.clock"
        ));
    }

    #[test]
    fn method_resolution_never_issues_receiverless_static_method() {
        let variables = BTreeMap::new();

        let result = resolve_call_target("length", &variables);

        assert!(result.is_err());
    }
}
