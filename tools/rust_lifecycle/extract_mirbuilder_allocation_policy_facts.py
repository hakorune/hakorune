#!/usr/bin/env python3
"""Extract MirBuilder allocation-policy facts from live Rust source."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require
from mirbuilder_allocation_policy_facts import (
    directability_decision,
    resolve_allocation_policy,
    run_drift_probes,
    validate_allocation_policy_facts,
)


ROOT = Path(__file__).resolve().parents[2]
ID_ALLOC = ROOT / "src/mir/builder/utils/id_alloc.rs"
FUNCTION_IMPL = ROOT / "src/mir/function/function_impl.rs"
PARAMETER_SETUP = ROOT / "src/mir/builder/calls/parameter_setup.rs"
HEADER_PHI_PREBUILD = ROOT / "src/mir/builder/control_flow/joinir/merge/header_phi_prebuild.rs"
COMPILATION_CONTEXT = ROOT / "src/mir/builder/compilation_context.rs"
VALUE_ID = ROOT / "crates/hakorune_mir_core/src/value_id.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-allocation-policy-facts-v0.json"
)


def _read(path: Path) -> str:
    return path.read_text()


def _require_all(source: str, fragments: list[tuple[str, str]]) -> None:
    for fragment, label in fragments:
        require(fragment in source, f"missing allocation policy source shape: {label}")


def _source_refs() -> dict[str, str]:
    return {
        "id_alloc": str(ID_ALLOC.relative_to(ROOT)),
        "function_impl": str(FUNCTION_IMPL.relative_to(ROOT)),
        "parameter_setup": str(PARAMETER_SETUP.relative_to(ROOT)),
        "header_phi_prebuild": str(HEADER_PHI_PREBUILD.relative_to(ROOT)),
        "compilation_context": str(COMPILATION_CONTEXT.relative_to(ROOT)),
        "value_id": str(VALUE_ID.relative_to(ROOT)),
    }


def extract_facts() -> dict[str, Any]:
    id_alloc = _read(ID_ALLOC)
    function_impl = _read(FUNCTION_IMPL)
    parameter_setup = _read(PARAMETER_SETUP)
    header_phi = _read(HEADER_PHI_PREBUILD)
    compilation_context = _read(COMPILATION_CONTEXT)
    value_id = _read(VALUE_ID)

    _require_all(
        id_alloc,
        [
            ("pub(crate) fn next_value_id(&mut self)", "MirBuilder::next_value_id"),
            ("loop {", "candidate retry loop"),
            ("if let Some(ref mut f) = self.scope_ctx.current_function", "current_function selector"),
            ("f.next_value_id() // Function context", "function-local candidate source"),
            ("self.core_ctx.next_value()", "module-global candidate source"),
            ("if !self.comp_ctx.reserved_value_ids.contains(&candidate)", "reserved membership predicate"),
            ("return candidate;", "accepted candidate return"),
            ("Reserved ID - try next one (loop continues)", "reserved retry comment"),
        ],
    )
    _require_all(
        function_impl,
        [
            ("let param_count = signature.params.len() as u32;", "param count source"),
            ("let total_value_ids = param_count;", "param id range source"),
            ("let initial_counter = total_value_ids.max(1);", "max(param_count, 1) seed"),
            ("for i in 0..total_value_ids", "param id prepopulation range"),
            ("pre_params.push(ValueId::new(i));", "param id prepopulation"),
            ("pub fn next_value_id(&mut self) -> ValueId", "MirFunction::next_value_id"),
            ("let id = ValueId::new(self.next_value_id);", "function allocator take"),
            ("self.next_value_id += 1;", "function allocator increment"),
            ("pub fn reserve_parameter_value_ids(&mut self)", "counter floor repair function"),
            ("if self.next_value_id < param_count as u32", "counter floor repair predicate"),
            ("self.next_value_id = param_count as u32;", "counter floor repair effect"),
        ],
    )
    _require_all(
        parameter_setup,
        [
            ("if param_idx < f.params.len()", "static parameter binding reuse predicate"),
            ("f.params[param_idx]", "static parameter binding reuse"),
            ("let new_pid = f.next_value_id();", "static compatibility fallback allocation"),
            ("if f.params.is_empty()", "instance compatibility fallback predicate"),
            ("let me_id = ValueId(0);", "instance me fallback id"),
            ("let me_id = f.params[0];", "instance me binding reuse"),
            ("let pid = f.params[param_idx];", "instance parameter binding reuse"),
            ("let pid = f.next_value_id();", "instance compatibility fallback allocation"),
        ],
    )
    _require_all(
        header_phi,
        [
            ("function_params: &BTreeMap<String, Vec<ValueId>>", "JoinIR parameter input"),
            ("let reserved_phi_dsts = loop_header_phi_info.reserved_value_ids();", "PHI destination source"),
            ("let mut reserved_value_ids = reserved_phi_dsts.clone();", "reserved set starts from PHI destinations"),
            ("for params in function_params.values()", "JoinIR parameter union loop"),
            ("reserved_value_ids.insert(param);", "JoinIR parameter inserted into reserved set"),
            ("builder.comp_ctx.reserved_value_ids = reserved_value_ids.clone();", "reserved set ReplaceSnapshot"),
            ("Ok((loop_header_phi_info, merge_entry_block, reserved_value_ids))", "reserved set returned"),
        ],
    )
    _require_all(
        compilation_context,
        [
            ("pub reserved_value_ids: HashSet<ValueId>", "reserved set storage owner"),
            ("pub fn is_reserved_value_id(&self, id: ValueId) -> bool", "reserved membership helper"),
            ("self.reserved_value_ids.contains(&id)", "reserved membership observation"),
            ("pub fn reserve_value_id(&mut self, id: ValueId)", "manual reserve helper"),
            ("self.reserved_value_ids.insert(id);", "manual reserve effect"),
            ("pub fn clear_reserved_value_ids(&mut self)", "reserved set clear helper"),
            ("self.reserved_value_ids.clear();", "reserved set clear effect"),
        ],
    )
    _require_all(
        value_id,
        [
            ("pub struct ValueId(pub u32);", "ValueId newtype"),
            ("pub const INVALID: Self = ValueId(u32::MAX);", "formal invalid sentinel"),
            ("pub fn new(id: u32) -> Self", "ValueId constructor"),
            ("pub fn next(&mut self) -> ValueId", "ValueIdGenerator next"),
            ("let id = ValueId(self.next_id);", "ValueIdGenerator take"),
            ("self.next_id += 1;", "ValueIdGenerator increment"),
        ],
    )

    facts: dict[str, Any] = {
        "schema_version": 0,
        "kind": "MirBuilderAllocationPolicyFactsV1",
        "subject": "hakorune_mir_builder::MirBuilder::next_value_id",
        "source": _source_refs(),
        "candidate_selection": {
            "predicate": "CurrentFunctionPresent",
            "present_source": "MirFunctionNextValueId",
            "absent_source": "CoreContextNextValue",
            "selection_frequency": "PerCandidateAttempt",
        },
        "function_allocator": {
            "state": "MirFunction.next_value_id",
            "operation": "TakeThenIncrement",
            "param_ids": "[0, param_count)",
            "initial_counter_seed": "max(param_count, 1)",
            "zero_floor_policy": True,
            "transport": "ValueIdAsI64",
        },
        "module_allocator": {
            "state": "CoreContext.value_gen",
            "operation": "GeneratorNext",
            "first_candidate": 0,
            "transport": "ValueIdAsI64",
        },
        "exclusion_set": {
            "storage_owner": "CompilationContext",
            "producer": "JoinIrHeaderPhiPrebuild",
            "members": ["PhiDestinations", "JoinIrFunctionParameters"],
            "update_kind": "ReplaceSnapshot",
            "consumer": "MirBuilder::next_value_id",
            "predicate": "CandidateNotInReservedSet",
            "observation": "MembershipOnly",
            "rejected_candidate_effect": "Consumed",
            "retry": "GenerateNextCandidate",
            "lifetime": "JoinIrMergeTemporary",
        },
        "parameter_initialization": {
            "prepopulation": "ParameterIdPrepopulation",
            "counter_seed": "FunctionCounterSeed",
            "binding_reuse": "ParameterBindingReuse",
            "counter_floor_repair": "ParameterCounterFloorRepair",
            "compatibility_fallback": "ParameterSetupCompatibilityFallbackUnselected",
        },
        "sentinel_policy": {
            "function_initial_floor": 1,
            "zero_reserved_by_function_constructor_policy": True,
            "formal_invalid_sentinel": "u32::MAX",
            "formal_invalid_exclusion_claim": False,
        },
        "boundary_facts": [
            "CurrentFunctionOptionTransportUnselected",
            "ReservedValueSetTransportUnselected",
            "ParameterSetupCompatibilityFallbackUnselected",
            "ReservedSetLifetimeProofRequired",
            "FormalInvalidSentinelPolicyUnselected",
            "AllocationCounterOverflowPolicyUnselected",
        ],
        "claims": {
            "core_context_reserved_skip_claim": 0,
            "generated_hako_changed": 0,
            "executable_allocation_policy_claim": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }
    facts["resolved_policy"] = resolve_allocation_policy(facts)
    facts["directability"] = directability_decision(facts)
    validate_allocation_policy_facts(facts)
    return facts


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=REFERENCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    facts = extract_facts()
    if args.drift_probes:
        run_drift_probes(facts)

    return report_or_emit(
        facts=facts,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rust-lifecycle-mirbuilder-allocation-policy-facts-v0"),
            ("mirbuilder_allocation_policy_facts", "green"),
            ("resolved_policy", facts["resolved_policy"]["kind"]),
            ("directability", facts["directability"]["decision"]),
            ("executable_allocation_policy_claim", "0"),
            ("generated_hako_changed", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
