//! Read-only SymbolRef resolution contract observation.
//!
//! This owner proves the contract that a ProgramJSON symbol id maps to a Rust
//! variable key/current ValueId before any SymbolRef lookup bridge is opened.

use super::MirBuilder;
use crate::mir::ValueId;

pub(in crate::mir::builder) const SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW: u8 = 1;
pub(in crate::mir::builder) const SCOPE_RENAMED_LOCAL_NO_SHADOW: u8 = 2;
pub(in crate::mir::builder) const SCOPE_SHADOWED_NAME_NOT_CLAIMED: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct SymbolRefResolutionContract {
    pub ok: bool,
    pub reason_code: u8,
    pub symbol_id: u32,
    pub source_name: String,
    pub expected_rust_variable_key: String,
    pub rust_variable_key_present: bool,
    pub rust_current_valueid_present: bool,
    pub rust_current_valueid_nonzero: bool,
    pub rust_current_valueid: Option<ValueId>,
    pub scope_contract_kind: u8,
    pub shadowing_claimed: bool,
    pub current_ssa_claimed: bool,
    pub local_ssa_materialization_claimed: bool,
    pub readonly: bool,
}

pub(in crate::mir::builder) struct CompareRhsSymbolRefContractObserver;

impl CompareRhsSymbolRefContractObserver {
    pub(in crate::mir::builder) fn observe_no_shadow_local(
        builder: &MirBuilder,
        symbol_id: u32,
        source_name: &str,
        expected_rust_variable_key: &str,
        scope_contract_kind: u8,
    ) -> SymbolRefResolutionContract {
        let current = builder
            .variable_ctx
            .variable_map
            .get(expected_rust_variable_key)
            .copied();
        SymbolRefResolutionContract {
            ok: current.is_some(),
            reason_code: if current.is_some() { 0 } else { 1 },
            symbol_id,
            source_name: source_name.to_string(),
            expected_rust_variable_key: expected_rust_variable_key.to_string(),
            rust_variable_key_present: current.is_some(),
            rust_current_valueid_present: current.is_some(),
            rust_current_valueid_nonzero: current.map(|value| value.0 > 0).unwrap_or(false),
            rust_current_valueid: current,
            scope_contract_kind,
            shadowing_claimed: false,
            current_ssa_claimed: false,
            local_ssa_materialization_claimed: false,
            readonly: true,
        }
    }

    pub(in crate::mir::builder) fn shadowed_name_not_claimed(
        symbol_id: u32,
        source_name: &str,
    ) -> SymbolRefResolutionContract {
        SymbolRefResolutionContract {
            ok: false,
            reason_code: 2,
            symbol_id,
            source_name: source_name.to_string(),
            expected_rust_variable_key: source_name.to_string(),
            rust_variable_key_present: false,
            rust_current_valueid_present: false,
            rust_current_valueid_nonzero: false,
            rust_current_valueid: None,
            scope_contract_kind: SCOPE_SHADOWED_NAME_NOT_CLAIMED,
            shadowing_claimed: false,
            current_ssa_claimed: false,
            local_ssa_materialization_claimed: false,
            readonly: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolref_contract_observes_simple_and_renamed_locals_readonly() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("symbolref_contract/0".to_string());
        builder.push_lexical_scope();

        let i_value = builder.alloc_value_for_test();
        builder
            .declare_local_in_current_scope("i", i_value)
            .expect("declare i");
        let count_value = builder.alloc_value_for_test();
        builder
            .declare_local_in_current_scope("count", count_value)
            .expect("declare count");

        let before = builder.variable_ctx.variable_map.clone();
        let simple = CompareRhsSymbolRefContractObserver::observe_no_shadow_local(
            &builder,
            1,
            "i",
            "i",
            SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW,
        );
        let renamed = CompareRhsSymbolRefContractObserver::observe_no_shadow_local(
            &builder,
            3,
            "count",
            "count",
            SCOPE_RENAMED_LOCAL_NO_SHADOW,
        );
        let shadowed = CompareRhsSymbolRefContractObserver::shadowed_name_not_claimed(4, "i");

        assert_eq!(builder.variable_ctx.variable_map, before);

        assert!(simple.ok);
        assert_eq!(simple.symbol_id, 1);
        assert_eq!(simple.source_name, "i");
        assert_eq!(simple.expected_rust_variable_key, "i");
        assert_eq!(simple.rust_current_valueid, Some(i_value));
        assert!(simple.rust_variable_key_present);
        assert!(simple.rust_current_valueid_present);
        assert!(simple.rust_current_valueid_nonzero);
        assert_eq!(
            simple.scope_contract_kind,
            SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW
        );

        assert!(renamed.ok);
        assert_eq!(renamed.symbol_id, 3);
        assert_eq!(renamed.source_name, "count");
        assert_eq!(renamed.expected_rust_variable_key, "count");
        assert_eq!(renamed.rust_current_valueid, Some(count_value));
        assert_eq!(renamed.scope_contract_kind, SCOPE_RENAMED_LOCAL_NO_SHADOW);

        assert!(!shadowed.ok);
        assert_eq!(
            shadowed.scope_contract_kind,
            SCOPE_SHADOWED_NAME_NOT_CLAIMED
        );
        assert!(!shadowed.shadowing_claimed);

        for row in [&simple, &renamed, &shadowed] {
            assert!(row.readonly);
            assert!(!row.current_ssa_claimed);
            assert!(!row.local_ssa_materialization_claimed);
        }
    }
}
