#!/usr/bin/env python3
"""Input validators for spec-backed MirBuilder family artifact generators."""

from __future__ import annotations

from typing import Any


def _require_kinds(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
    *,
    facts_kind: str,
    subject: str,
) -> None:
    if facts.get("kind") != facts_kind:
        raise SystemExit("unexpected facts kind")
    if plan.get("kind") != "HakoLifecyclePlan":
        raise SystemExit("unexpected plan kind")
    if oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected oracle kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("subject mismatch")


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {op["op"] for vector in oracle["vectors"] for op in vector["operations"]}


def _behavior_flags_false(plan: dict[str, Any], names: list[str]) -> None:
    behavior = plan.get("behavior", {})
    for name in names:
        if behavior.get(name) is not False:
            raise SystemExit(f"unexpected behavior flag: {name}")


def validate_binding_context(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::binding_context::BindingContext"
    methods = [
        "BindingContext::new",
        "BindingContext::is_empty",
        "BindingContext::len",
        "BindingContext::contains",
        "BindingContext::lookup",
        "BindingContext::insert",
        "BindingContext::remove",
        "BindingContext::clear_for_function_entry",
    ]
    _require_kinds(facts, plan, oracle, facts_kind="RustLifecycleAdapterFacts", subject=subject)
    binding_map = {row["id"]: row for row in facts["fields"]}.get("BindingContext.binding_map")
    if binding_map is None:
        raise SystemExit("missing BindingContext.binding_map facts")
    if binding_map.get("rust_type") != "BTreeMap<String, BindingId>":
        raise SystemExit("unexpected binding_map rust_type")
    if binding_map.get("deterministic_order_required") is not True:
        raise SystemExit("missing deterministic order fact")
    if binding_map.get("drop_class") != "TrivialMemory":
        raise SystemExit("binding_map drop must be TrivialMemory")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["BindingContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("BindingContext must be LocalBox")
    if plans["BindingContext.binding_map"]["plan_kind"] != "OrderedMapBox":
        raise SystemExit("binding_map must project to OrderedMapBox")
    if plans["BindingContext"]["cleanup_policy"] != "erase":
        raise SystemExit("BindingContext cleanup must erase")
    missing = sorted(set(methods[1:]) - {row["id"] for row in facts["methods"]})
    if missing:
        raise SystemExit(f"missing method facts: {missing}")
    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    for method, operation in {
        "BindingContext::new": "NewOrderedMap",
        "BindingContext::is_empty": "MapIsEmpty",
        "BindingContext::len": "MapLength",
        "BindingContext::contains": "MapHas",
        "BindingContext::lookup": "MapGet",
        "BindingContext::insert": "MapSet",
        "BindingContext::remove": "MapRemove",
        "BindingContext::clear_for_function_entry": "MapClear",
    }.items():
        body_fact = body_facts.get(method)
        if body_fact is None:
            raise SystemExit(f"missing body fact: {method}")
        if body_fact.get("operation") != operation:
            raise SystemExit(f"unexpected body operation for {method}")
        if body_fact.get("selected_field") != "binding_map":
            raise SystemExit(f"unexpected body field for {method}")
    for op in ["new", "is_empty", "len", "contains", "lookup", "insert", "remove", "clear_for_function_entry"]:
        if op not in _oracle_ops(oracle):
            raise SystemExit(f"missing oracle op: {op}")


def validate_box_compilation_context(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::context::BoxCompilationContext"
    _require_kinds(facts, plan, oracle, facts_kind="RustLifecycleFacts", subject=subject)
    field_names = ["variable_map", "value_origin_newbox", "value_types"]
    type_fact = {row["id"]: row for row in facts["type_facts"]}.get("BoxCompilationContext")
    if type_fact is None or type_fact.get("drop_fact") != "TrivialMemory":
        raise SystemExit("BoxCompilationContext drop fact mismatch")
    for field_name in field_names:
        field_id = f"BoxCompilationContext.{field_name}"
        field_fact = {row["id"]: row for row in facts["field_facts"]}.get(field_id)
        if field_fact is None:
            raise SystemExit(f"missing field fact: {field_id}")
        if field_fact.get("deterministic_order_required") is not True:
            raise SystemExit(f"missing deterministic order fact: {field_id}")
        if field_fact.get("drop_fact") != "TrivialMemory":
            raise SystemExit(f"unexpected drop fact: {field_id}")
        plan_entry = {row["id"]: row for row in plan["plans"]}.get(field_id)
        if plan_entry is None or plan_entry.get("plan_kind") != "OrderedMapBox":
            raise SystemExit(f"unexpected field plan: {field_id}")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    new_fact = body_facts.get("BoxCompilationContext::new")
    if new_fact is None or new_fact.get("operation") != "DefaultConstruct":
        raise SystemExit("constructor body fact mismatch")
    if new_fact.get("selected_fields") != field_names:
        raise SystemExit("constructor selected fields mismatch")
    empty_fact = body_facts.get("BoxCompilationContext::is_empty")
    if empty_fact is None or empty_fact.get("operation") != "CompositeMapIsEmpty":
        raise SystemExit("is_empty body fact mismatch")
    if empty_fact.get("selected_fields") != field_names:
        raise SystemExit("is_empty selected fields mismatch")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["BoxCompilationContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("BoxCompilationContext must be LocalBox")
    if plans["BoxCompilationContext::new"]["plan_kind"] != "DefaultConstruct":
        raise SystemExit("constructor plan mismatch")
    if plans["BoxCompilationContext::is_empty"]["plan_kind"] != "BorrowView":
        raise SystemExit("is_empty plan mismatch")
    for fact in ["BoxCompilationContext.escape_fact=LocalOnly", "BoxCompilationContext.drop_fact=TrivialMemory"]:
        if fact not in set(plans["BoxCompilationContext"].get("required_facts", [])):
            raise SystemExit(f"missing required fact: {fact}")
    for fact in [
        "BoxCompilationContext::new.returns.copy_kind=NonCopyOwned",
        "BoxCompilationContext::new.returns.drop_fact=TrivialMemory",
        "BoxCompilationContext::is_empty.receiver_borrow.kind=SharedRead",
        "BoxCompilationContext::is_empty.receiver_borrow.escapes=false",
    ]:
        required = plans["BoxCompilationContext::new"].get("required_facts", []) + plans["BoxCompilationContext::is_empty"].get("required_facts", [])
        if fact not in set(required):
            raise SystemExit(f"missing required fact: {fact}")
    for op in ["new", "is_empty"]:
        if op not in _oracle_ops(oracle):
            raise SystemExit(f"missing oracle op: {op}")


def validate_variable_context_simple_map(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::variable_context::VariableContext.simple_map"
    methods = {
        "VariableContext::lookup",
        "VariableContext::contains",
        "VariableContext::len",
        "VariableContext::is_empty",
        "VariableContext::insert",
        "VariableContext::remove",
    }
    excluded_methods = {
        "VariableContext::variable_map",
        "VariableContext::variable_map_mut",
        "VariableContext::snapshot",
        "VariableContext::restore",
    }
    _require_kinds(facts, plan, oracle, facts_kind="RustLifecycleFacts", subject=subject)
    variable_map = {row["id"]: row for row in facts["field_facts"]}.get("VariableContext.variable_map")
    if variable_map is None:
        raise SystemExit("missing VariableContext.variable_map facts")
    if variable_map.get("rust_type") != "BTreeMap<String, ValueId>":
        raise SystemExit("unexpected variable_map rust_type")
    if variable_map.get("deterministic_order_required") is not True:
        raise SystemExit("missing deterministic order fact")
    if variable_map.get("drop_fact") != "TrivialMemory":
        raise SystemExit("variable_map drop must be TrivialMemory")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["VariableContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("VariableContext must be LocalBox")
    if plans["VariableContext.variable_map"]["plan_kind"] != "OrderedMapBox":
        raise SystemExit("variable_map must project to OrderedMapBox")
    missing = sorted(methods - {row["id"] for row in facts["method_facts"]})
    if missing:
        raise SystemExit(f"missing method facts: {missing}")
    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    for method, operation in {
        "VariableContext::new": "NewOrderedMap",
        "VariableContext::lookup": "MapGet",
        "VariableContext::contains": "MapHas",
        "VariableContext::len": "MapLength",
        "VariableContext::is_empty": "MapIsEmpty",
        "VariableContext::insert": "MapSet",
        "VariableContext::remove": "MapRemove",
    }.items():
        body_fact = body_facts.get(method)
        if body_fact is None:
            raise SystemExit(f"missing body fact: {method}")
        if body_fact.get("operation") != operation:
            raise SystemExit(f"unexpected body operation for {method}")
        if body_fact.get("selected_field") != "variable_map":
            raise SystemExit(f"unexpected body field for {method}")
    missing_excluded = sorted(excluded_methods - {row["id"] for row in facts["excluded_methods"]})
    if missing_excluded:
        raise SystemExit(f"missing excluded methods: {missing_excluded}")
    for op in ["new", "is_empty", "len", "contains", "lookup", "insert", "remove"]:
        if op not in _oracle_ops(oracle):
            raise SystemExit(f"missing oracle op: {op}")


def validate_variable_context_immutable_borrow(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::variable_context::VariableContext.immutable_map_borrow"
    method_id = "VariableContext::variable_map"
    excluded = ["VariableContext::variable_map_mut", "VariableContext::snapshot", "VariableContext::restore"]
    _require_kinds(facts, plan, oracle, facts_kind="RustLifecycleFacts", subject=subject)
    if facts.get("base_facts") != "variable-context-simple-map-facts-v0.json":
        raise SystemExit("unexpected base facts")
    method_facts = facts.get("method_facts", [])
    if len(method_facts) != 1 or method_facts[0].get("id") != method_id:
        raise SystemExit("unexpected method fact id")
    borrow = method_facts[0].get("receiver_borrow", {})
    if borrow.get("kind") != "SharedRead" or borrow.get("scope") != "ReturnedBorrow":
        raise SystemExit("expected SharedRead returned borrow")
    if borrow.get("escapes") is not False or borrow.get("owner_carrying_required") is not True:
        raise SystemExit("borrow ownership facts mismatch")
    returns = method_facts[0].get("returns", {})
    if returns.get("borrow_view") != "OwnerCarryingBorrowView" or returns.get("access") != "read":
        raise SystemExit("unexpected borrow return surface")
    denied = {row["id"]: row for row in facts.get("denied_methods", [])}
    for name, reason in {
        "VariableContext::variable_map_mut": "ReturnedMutableBorrow",
        "VariableContext::snapshot": "SnapshotOwnedMapOutOfScope",
        "VariableContext::restore": "ReplaceOwnedOutOfScope",
    }.items():
        if denied.get(name, {}).get("deny_reason") != reason:
            raise SystemExit(f"missing denied method fact: {name}")
    excluded_consumers = {row["id"] for row in facts.get("excluded_consumers", [])}
    for name in ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers"]:
        if name not in excluded_consumers:
            raise SystemExit(f"missing excluded consumer: {name}")
    borrow_plan = {row["id"]: row for row in plan.get("plans", [])}.get(method_id)
    if borrow_plan is None:
        raise SystemExit("missing borrow plan")
    for key, value in {
        "plan_kind": "BorrowView",
        "access": "read",
        "escape_policy": "deny_if_escapes",
        "return_alias_policy": "owner_carrying_view_only",
    }.items():
        if borrow_plan.get(key) != value:
            raise SystemExit(f"borrow plan {key} mismatch")
    if borrow_plan.get("owner_carrying") is not True:
        raise SystemExit("borrow plan must be owner_carrying")
    required = set(borrow_plan.get("required_facts", []))
    for fact in [
        "receiver_borrow.kind=SharedRead",
        "receiver_borrow.scope=ReturnedBorrow",
        "receiver_borrow.escapes=false",
        "receiver_borrow.owner_carrying_required=true",
    ]:
        if fact not in required:
            raise SystemExit(f"missing required fact: {fact}")
    denied_plan = set(plan.get("denied", []))
    for name in excluded + ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]:
        if name not in denied_plan:
            raise SystemExit(f"missing denied plan boundary: {name}")
    _behavior_flags_false(plan, ["general_resolver_implemented", "converter_emission_added", "rust_lifetime_syntax_added", "carrier_phi_claim", "full_variable_context_claim"])
    ops = [op for vector in oracle.get("vectors", []) for op in vector.get("operations", [])]
    if not any(op.get("op") == "borrow_view" and op.get("method") in {"variable_map", method_id} for op in ops):
        raise SystemExit("missing borrow_view oracle op")
    for op in ["borrow_get", "borrow_len", "borrow_iteration_order"]:
        if not any(item.get("op") == op for item in ops):
            raise SystemExit(f"missing {op} oracle op")
    denied_vectors = set(oracle.get("denied_vectors", []))
    for name in ["variable_map_mut_returned_borrow", "snapshot", "restore", "carrier_extraction", "phi_planner_integration"]:
        if name not in denied_vectors:
            raise SystemExit(f"missing denied oracle vector: {name}")
    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "VariableContext immutable map borrow only":
        raise SystemExit("unexpected oracle promotion scope")
    for name in ["carrier_phi_claim", "full_variable_context_claim", "mirbuilder_wide_claim"]:
        if scope.get(name) is not False:
            raise SystemExit(f"unexpected oracle scope flag: {name}")


def validate_variable_context_snapshot_restore(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::variable_context::VariableContext.snapshot_restore"
    _require_kinds(facts, plan, oracle, facts_kind="RustLifecycleFacts", subject=subject)
    if facts.get("base_facts") != "variable-context-simple-map-facts-v0.json":
        raise SystemExit("unexpected base facts")
    method_facts = {row["id"]: row for row in facts.get("method_facts", [])}
    if set(method_facts) != {"VariableContext::snapshot", "VariableContext::restore"}:
        raise SystemExit("unexpected method facts")
    snapshot = method_facts["VariableContext::snapshot"]
    if snapshot.get("operation") != "CloneOwnedMap":
        raise SystemExit("snapshot operation must be CloneOwnedMap")
    snapshot_receiver = snapshot.get("receiver_borrow", {})
    if snapshot_receiver.get("kind") != "SharedRead" or snapshot_receiver.get("escapes") is not False:
        raise SystemExit("snapshot receiver borrow mismatch")
    snapshot_returns = snapshot.get("returns", {})
    if snapshot_returns.get("copy_kind") != "NonCopyOwned":
        raise SystemExit("snapshot copy kind mismatch")
    if snapshot_returns.get("deterministic_order_required") is not True:
        raise SystemExit("snapshot must require deterministic order")
    if snapshot_returns.get("drop_fact") != "TrivialMemory":
        raise SystemExit("snapshot drop fact must be TrivialMemory")
    if snapshot_returns.get("rust_type") != "BTreeMap<String, ValueId>":
        raise SystemExit("unexpected snapshot rust type")
    restore = method_facts["VariableContext::restore"]
    if restore.get("operation") != "ReplaceOwned":
        raise SystemExit("restore operation must be ReplaceOwned")
    restore_receiver = restore.get("receiver_borrow", {})
    if restore_receiver.get("kind") != "UniqueWrite" or restore_receiver.get("escapes") is not False:
        raise SystemExit("restore receiver borrow mismatch")
    moves = restore.get("argument_moves", [])
    if len(moves) != 1:
        raise SystemExit("restore must have one owned argument move")
    move = moves[0]
    if move.get("name") != "snapshot" or move.get("move_kind") != "ConsumeArgument":
        raise SystemExit("restore argument move mismatch")
    if move.get("deterministic_order_required") is not True or move.get("drop_fact") != "TrivialMemory":
        raise SystemExit("restore snapshot argument facts mismatch")
    cleanup = restore.get("old_value_cleanup", {})
    if cleanup.get("required_fact") != "VariableContext.variable_map.drop_fact=TrivialMemory":
        raise SystemExit("restore cleanup fact mismatch")
    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    for method, operation in {
        "VariableContext::snapshot": "CloneOwnedMap",
        "VariableContext::restore": "ReplaceOwnedMap",
    }.items():
        body_fact = body_facts.get(method)
        if body_fact is None:
            raise SystemExit(f"missing body fact: {method}")
        if body_fact.get("operation") != operation:
            raise SystemExit(f"unexpected body operation for {method}")
        if body_fact.get("selected_field") != "variable_map":
            raise SystemExit(f"unexpected body field for {method}")
    denied = {row["id"]: row for row in facts.get("denied_methods", [])}
    if denied.get("VariableContext::variable_map_mut", {}).get("deny_reason") != "ReturnedMutableBorrow":
        raise SystemExit("variable_map_mut must remain denied")
    excluded_consumers = set(facts.get("excluded_consumers", []))
    for name in ["CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]:
        if name not in excluded_consumers:
            raise SystemExit(f"missing excluded consumer: {name}")
    plans = {row["id"]: row for row in plan.get("plans", [])}
    if set(plans) != {"VariableContext::snapshot", "VariableContext::restore"}:
        raise SystemExit("unexpected plan entries")
    snapshot_plan = plans["VariableContext::snapshot"]
    if snapshot_plan.get("plan_kind") != "CloneOwnedMap" or snapshot_plan.get("result_plan") != "OwnedOrderedMap":
        raise SystemExit("snapshot plan mismatch")
    for fact in ["receiver_borrow.kind=SharedRead", "receiver_borrow.escapes=false", "returns.deterministic_order_required=true", "returns.drop_fact=TrivialMemory"]:
        if fact not in set(snapshot_plan.get("required_facts", [])):
            raise SystemExit(f"missing snapshot required fact: {fact}")
    restore_plan = plans["VariableContext::restore"]
    if restore_plan.get("plan_kind") != "ReplaceOwned" or restore_plan.get("old_value_cleanup") != "erase":
        raise SystemExit("restore plan mismatch")
    for fact in ["receiver_borrow.kind=UniqueWrite", "receiver_borrow.escapes=false", "argument.move_kind=ConsumeArgument", "argument.deterministic_order_required=true", "VariableContext.variable_map.drop_fact=TrivialMemory"]:
        if fact not in set(restore_plan.get("required_facts", [])):
            raise SystemExit(f"missing restore required fact: {fact}")
    _behavior_flags_false(plan, ["general_resolver_implemented", "converter_emission_added", "rust_lifetime_syntax_added", "carrier_phi_claim", "full_variable_context_claim"])
    ops = [op for vector in oracle.get("vectors", []) for op in vector.get("operations", [])]
    for op in ["new", "insert", "snapshot", "restore", "len", "contains"]:
        if not any(item.get("op") == op for item in ops):
            raise SystemExit(f"missing oracle op: {op}")
    restore_ops = [item for item in ops if item.get("op") == "restore"]
    if not restore_ops:
        raise SystemExit("missing restore oracle op")
    for requirement in ["ReplaceOwned", "old_map_cleanup=TrivialMemory"]:
        if requirement not in set(restore_ops[0].get("requires", [])):
            raise SystemExit(f"missing restore oracle requirement: {requirement}")
    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "VariableContext snapshot/restore only":
        raise SystemExit("unexpected promotion scope")
    for name in ["carrier_phi_claim", "full_variable_context_claim", "mirbuilder_wide_claim"]:
        if scope.get(name) is not False:
            raise SystemExit(f"unexpected oracle scope flag: {name}")
    for name in ["variable_map_mut_returned_borrow", "carrier_extraction", "phi_planner_integration"]:
        if name not in set(oracle.get("denied_vectors", [])):
            raise SystemExit(f"missing denied oracle vector: {name}")
