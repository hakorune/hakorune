//! Compare RHS SymbolRef lookup bridge.
//!
//! This owner opens actual lookup only for contract-verified no-shadow local
//! rows. It returns an existing ValueId and performs no allocation or mutation.

use super::compare_rhs_symbolref_contract::{
    SymbolRefResolutionContract, SCOPE_RENAMED_LOCAL_NO_SHADOW, SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW,
};
use super::compare_rhs_valueid_resolution_bridge::{
    CompareRhsValueIdResolutionResponse, REASON_OK,
};

pub(in crate::mir::builder) struct CompareRhsSymbolRefLookupBridge;

impl CompareRhsSymbolRefLookupBridge {
    pub(in crate::mir::builder) fn resolve_symbol_ref_no_shadow_local(
        contract: &SymbolRefResolutionContract,
    ) -> CompareRhsValueIdResolutionResponse {
        let allowed_scope = matches!(
            contract.scope_contract_kind,
            SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW | SCOPE_RENAMED_LOCAL_NO_SHADOW
        );
        let resolved = contract.ok
            && allowed_scope
            && contract.rust_variable_key_present
            && contract.rust_current_valueid_present
            && contract.rust_current_valueid_nonzero
            && !contract.shadowing_claimed
            && !contract.current_ssa_claimed
            && !contract.local_ssa_materialization_claimed;
        CompareRhsValueIdResolutionResponse {
            ok: resolved,
            reason_code: if resolved { REASON_OK } else { 1 },
            rhs_value_id_present: resolved,
            rhs_value_id: if resolved {
                contract.rust_current_valueid
            } else {
                None
            },
            emitted_constant: false,
            constant_kind_code: 0,
            constant_i64: 0,
            used_symbol_lookup: resolved,
            symbol_id: contract.symbol_id,
            valueid_allocated: false,
            mutation_performed: false,
            mutation_kind_code: 0,
            local_ssa_finalize_compare_executed: false,
            mir_compare_emitted: false,
            mir_branch_emitted: false,
            route_selection: false,
            runtime_route_switch: false,
            programjson_runtime_authority: false,
            source_selfhost_claim: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::compare_rhs_symbolref_contract::{
        CompareRhsSymbolRefContractObserver, SCOPE_SHADOWED_NAME_NOT_CLAIMED,
    };

    #[test]
    fn symbolref_lookup_bridge_returns_existing_no_shadow_valueids_only() {
        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("symbolref_lookup_bridge/0".to_string());
        builder.push_lexical_scope();

        let i_value = builder.alloc_value_for_test();
        builder
            .declare_local_in_current_scope("i", i_value)
            .expect("declare i");
        let count_value = builder.alloc_value_for_test();
        builder
            .declare_local_in_current_scope("count", count_value)
            .expect("declare count");

        let before_next = builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id;
        let before_map = builder.variable_ctx.variable_map.clone();

        let simple_contract = CompareRhsSymbolRefContractObserver::observe_no_shadow_local(
            &builder,
            1,
            "i",
            "i",
            SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW,
        );
        let renamed_contract = CompareRhsSymbolRefContractObserver::observe_no_shadow_local(
            &builder,
            3,
            "count",
            "count",
            SCOPE_RENAMED_LOCAL_NO_SHADOW,
        );
        let unmapped_contract = SymbolRefResolutionContract {
            ok: false,
            reason_code: 1,
            symbol_id: 99,
            source_name: "missing".to_string(),
            expected_rust_variable_key: "missing".to_string(),
            rust_variable_key_present: false,
            rust_current_valueid_present: false,
            rust_current_valueid_nonzero: false,
            rust_current_valueid: None,
            scope_contract_kind: SCOPE_SAME_FUNCTION_LOCAL_NO_SHADOW,
            shadowing_claimed: false,
            current_ssa_claimed: false,
            local_ssa_materialization_claimed: false,
            readonly: true,
        };
        let shadowed_contract = SymbolRefResolutionContract {
            ok: true,
            reason_code: 0,
            symbol_id: 4,
            source_name: "i".to_string(),
            expected_rust_variable_key: "i".to_string(),
            rust_variable_key_present: true,
            rust_current_valueid_present: true,
            rust_current_valueid_nonzero: true,
            rust_current_valueid: Some(i_value),
            scope_contract_kind: SCOPE_SHADOWED_NAME_NOT_CLAIMED,
            shadowing_claimed: false,
            current_ssa_claimed: false,
            local_ssa_materialization_claimed: false,
            readonly: true,
        };

        let simple =
            CompareRhsSymbolRefLookupBridge::resolve_symbol_ref_no_shadow_local(&simple_contract);
        let renamed =
            CompareRhsSymbolRefLookupBridge::resolve_symbol_ref_no_shadow_local(&renamed_contract);
        let unmapped =
            CompareRhsSymbolRefLookupBridge::resolve_symbol_ref_no_shadow_local(&unmapped_contract);
        let shadowed =
            CompareRhsSymbolRefLookupBridge::resolve_symbol_ref_no_shadow_local(&shadowed_contract);

        let after_next = builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id;
        assert_eq!(after_next, before_next);
        assert_eq!(builder.variable_ctx.variable_map, before_map);

        assert!(simple.ok);
        assert_eq!(simple.rhs_value_id, Some(i_value));
        assert!(simple.used_symbol_lookup);
        assert_eq!(simple.symbol_id, 1);

        assert!(renamed.ok);
        assert_eq!(renamed.rhs_value_id, Some(count_value));
        assert!(renamed.used_symbol_lookup);
        assert_eq!(renamed.symbol_id, 3);

        assert!(!unmapped.ok);
        assert!(!unmapped.rhs_value_id_present);
        assert!(!unmapped.used_symbol_lookup);
        assert_eq!(unmapped.symbol_id, 99);

        assert!(!shadowed.ok);
        assert!(!shadowed.rhs_value_id_present);
        assert!(!shadowed.used_symbol_lookup);

        for response in [&simple, &renamed, &unmapped, &shadowed] {
            assert!(!response.valueid_allocated);
            assert!(!response.emitted_constant);
            assert!(!response.mutation_performed);
            assert!(!response.local_ssa_finalize_compare_executed);
            assert!(!response.mir_compare_emitted);
            assert!(!response.mir_branch_emitted);
            assert!(!response.route_selection);
            assert!(!response.runtime_route_switch);
            assert!(!response.programjson_runtime_authority);
            assert!(!response.source_selfhost_claim);
        }
    }
}
