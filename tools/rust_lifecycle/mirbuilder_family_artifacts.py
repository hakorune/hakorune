#!/usr/bin/env python3
"""Spec-backed MirBuilder family artifact generators."""

from __future__ import annotations

from pathlib import Path
from textwrap import dedent
from typing import Any

from extract_binding_context_facts import (
    SOURCE as BINDING_CONTEXT_SOURCE,
    extract_facts as extract_binding_context_facts,
)
from extract_box_compilation_context_facts import (
    SOURCE as BOX_COMPILATION_CONTEXT_SOURCE,
    extract_facts as extract_box_compilation_context_facts,
)
from extract_variable_context_simple_map_facts import (
    SOURCE as VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE,
    extract_facts as extract_variable_context_simple_map_facts,
)
from extract_variable_context_snapshot_restore_facts import (
    extract_facts as extract_variable_context_snapshot_restore_facts,
)
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_ordered_map_converter import (
    compile_box_compilation_context_methods,
    compile_binding_context_methods,
    compile_variable_context_simple_map_methods,
    compile_variable_context_snapshot_restore_methods,
)
from verified_hako_family_ir import op
from shared_family_generator import read_json, run_validated_family_generator


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
VARIABLE_CONTEXT_SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
VARIABLE_CONTEXT_FAMILY_ID = "hakorune_mir_builder::variable_context"


def _lines(text: str) -> list[str]:
    return dedent(text).strip("\n").splitlines()


def _require_kinds(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any], *, facts_kind: str, subject: str) -> None:
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
        if fact not in set(plans["BoxCompilationContext::new"].get("required_facts", []) + plans["BoxCompilationContext::is_empty"].get("required_facts", [])):
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


def binding_context_spec() -> FamilyArtifactSpec:
    facts = extract_binding_context_facts(BINDING_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "binding-context-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in compile_binding_context_methods(facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_binding_context_artifact.py",
        generator_version="binding-context-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json",
        family_comment="hakorune_mir_builder::binding_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="BindingContext",
            field_name="binding_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_lines=_lines("""
            local ctx = new BindingContext()
            if ctx.binding_map.keys_value.length() != 0 {
                print("binding_context_new_empty=fail")
                return 1
            }
            if ctx.binding_map.keys_value.length() != 0 {
                print("binding_context_new_len=fail")
                return 2
            }

            BindingContextApi.insert(ctx, "x", 1)
            if BindingContextApi.lookup(ctx, "x") != 1 {
                print("binding_context_lookup_x=fail")
                return 3
            }
            if ctx.binding_map.keys_value.length() != 1 {
                print("binding_context_len_after_insert=fail")
                return 4
            }
            if ctx.binding_map.keys_value.length() != 1 {
                print("binding_context_empty_after_insert=fail")
                return 5
            }
            if BindingContextApi.remove(ctx, "x") != 1 {
                print("binding_context_remove_x=fail")
                return 6
            }
            if BindingContextApi.lookup(ctx, "x") != null {
                print("binding_context_lookup_removed=fail")
                return 7
            }
            if ctx.binding_map.keys_value.length() != 0 {
                print("binding_context_empty_after_remove=fail")
                return 8
            }

            local contains_ctx = new BindingContext()
            if BindingContextApi.contains(contains_ctx, "x") != 0 {
                print("binding_context_contains_empty=fail")
                return 9
            }
            BindingContextApi.insert(contains_ctx, "x", 1)
            if BindingContextApi.contains(contains_ctx, "x") != 1 {
                print("binding_context_contains_x=fail")
                return 10
            }

            local order_ctx = new BindingContext()
            BindingContextApi.insert(order_ctx, "b", 2)
            BindingContextApi.insert(order_ctx, "a", 1)
            if order_ctx.binding_map.keys_value.length() != 2 {
                print("binding_context_order_len=fail")
                return 11
            }
            if BindingContextApi.lookup(order_ctx, "a") != 1 {
                print("binding_context_lookup_a=fail")
                return 12
            }
            if BindingContextApi.lookup(order_ctx, "b") != 2 {
                print("binding_context_lookup_b=fail")
                return 13
            }
            local clear_ctx = new BindingContext()
            BindingContextApi.insert(clear_ctx, "a", 1)
            BindingContextApi.clear_for_function_entry(clear_ctx)
            if clear_ctx.binding_map.keys_value.length() != 0 {
                print("binding_context_clear_empty=fail")
                return 14
            }

            print("binding_context_derived_artifact=ok")
            return 0
        """),
        family_id="hakorune_mir_builder::binding_context",
        state="DerivedShadow",
        source_rust_file=ROOT / "crates/hakorune_mir_builder/src/binding_context.rs",
        hako_path=OUT_DIR / "binding_context.hako",
        facts_path=FIXTURES / "binding-context-adapter-facts-v0.json",
        plan_path=FIXTURES / "binding-context-plan-v0.json",
        oracle_path=FIXTURES / "binding-context-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "binding-context-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "binding-context-derived-artifact-verifier-result-v0.json",
        recipe_subject="hakorune_mir_builder::binding_context::BindingContext",
        selected_body_count="all_non_test_methods",
        api_name="BindingContextApi",
        api_methods=api_methods,
        api_trailing_blank_line=True,
        methods=[
            BehaviorMethodSpec(id="BindingContext::new", rust_operation="BTreeMap::new", hako_operation="OrderedMap.create", emits="birth initializes me.binding_map"),
            BehaviorMethodSpec(id="BindingContext::is_empty", rust_operation="BTreeMap::is_empty", hako_operation="OrderedMapBox.length == 0 as i64_bool_v0", emits="BindingContextApi.is_empty(ctx)"),
            BehaviorMethodSpec(id="BindingContext::len", rust_operation="BTreeMap::len", hako_operation="OrderedMapBox.length", emits="BindingContextApi.len(ctx)"),
            BehaviorMethodSpec(id="BindingContext::contains", rust_operation="BTreeMap::contains_key", hako_operation="OrderedMapBox.has as i64_bool_v0", emits="BindingContextApi.contains(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::lookup", rust_operation="BTreeMap::get(...).copied", hako_operation="OrderedMapBox.get", emits="BindingContextApi.lookup(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::insert", rust_operation="BTreeMap::insert", hako_operation="OrderedMapBox.set", emits="BindingContextApi.insert(ctx, name, binding_id)"),
            BehaviorMethodSpec(id="BindingContext::remove", rust_operation="BTreeMap::remove", hako_operation="OrderedMapBox.remove", emits="BindingContextApi.remove(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::clear_for_function_entry", rust_operation="BTreeMap::clear", hako_operation="OrderedMapBox.clear", emits="BindingContextApi.clear_for_function_entry(ctx)"),
        ],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "all_non_test_methods", "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["OrderedMap.create", "OrderedMapBox.length == 0 as i64_bool_v0", "OrderedMapBox.length", "OrderedMapBox.has as i64_bool_v0", "OrderedMapBox.get", "OrderedMapBox.set", "OrderedMapBox.remove", "OrderedMapBox.clear"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "reason": "pure-first global helper ABI expects scalar i64 returns in this pilot"},
        denied_boundaries=["selfhost mainline selection", "HakoAdopted native source decision", "MirBuilder-wide lifecycle parity", "runtime try-Hako-then-Rust fallback"],
    )


def variable_context_simple_map_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map", "VariableContext::variable_map_mut", "VariableContext::snapshot", "VariableContext::restore"]
    facts = extract_variable_context_simple_map_facts(VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE)
    plan = read_json(FIXTURES / "variable-context-simple-map-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in compile_variable_context_simple_map_methods(facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py",
        generator_version="variable-context-simple-map-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="VariableContext",
            field_name="variable_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_lines=_lines("""
            local ctx = new VariableContext()
            if VariableContextApi.is_empty(ctx) != 1 {
                print("variable_context_new_empty=fail")
                return 1
            }
            if VariableContextApi.len(ctx) != 0 {
                print("variable_context_new_len=fail")
                return 2
            }

            VariableContextApi.insert(ctx, "x", 42)
            if VariableContextApi.lookup(ctx, "x") != 42 {
                print("variable_context_lookup_x=fail")
                return 3
            }
            if VariableContextApi.len(ctx) != 1 {
                print("variable_context_len_after_insert=fail")
                return 4
            }
            if VariableContextApi.is_empty(ctx) != 0 {
                print("variable_context_empty_after_insert=fail")
                return 5
            }
            if VariableContextApi.remove(ctx, "x") != 42 {
                print("variable_context_remove_x=fail")
                return 6
            }
            if VariableContextApi.lookup(ctx, "x") != null {
                print("variable_context_lookup_removed=fail")
                return 7
            }

            local contains_ctx = new VariableContext()
            if VariableContextApi.contains(contains_ctx, "x") != 0 {
                print("variable_context_contains_empty=fail")
                return 8
            }
            VariableContextApi.insert(contains_ctx, "x", 1)
            if VariableContextApi.contains(contains_ctx, "x") != 1 {
                print("variable_context_contains_x=fail")
                return 9
            }

            local ssa_ctx = new VariableContext()
            VariableContextApi.insert(ssa_ctx, "x", 1)
            VariableContextApi.insert(ssa_ctx, "x", 2)
            VariableContextApi.insert(ssa_ctx, "x", 4)
            if VariableContextApi.lookup(ssa_ctx, "x") != 4 {
                print("variable_context_ssa_update=fail")
                return 10
            }

            print("variable_context_simple_map_derived_artifact=ok")
            return 0
        """),
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_simple_map.hako",
        facts_path=FIXTURES / "variable-context-simple-map-facts-v0.json",
        plan_path=FIXTURES / "variable-context-simple-map-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-simple-map-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "variable-context-simple-map-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "variable-context-simple-map-derived-artifact-verifier-result-v0.json",
        pilot_scope="VariableContext_simple_map_only",
        recipe_subject="hakorune_mir_builder::variable_context::VariableContext.simple_map",
        selected_body_count="simple_map_methods_only",
        api_name="VariableContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="VariableContext::lookup", rust_operation="BTreeMap::get(...).copied", hako_operation="OrderedMapBox.get", emits="VariableContextApi.lookup(ctx, name)"),
            BehaviorMethodSpec(id="VariableContext::contains", rust_operation="BTreeMap::contains_key", hako_operation="OrderedMapBox.has as i64_bool_v0", emits="VariableContextApi.contains(ctx, name)"),
            BehaviorMethodSpec(id="VariableContext::len", rust_operation="BTreeMap::len", hako_operation="OrderedMapBox.length", emits="VariableContextApi.len(ctx)"),
            BehaviorMethodSpec(id="VariableContext::is_empty", rust_operation="BTreeMap::is_empty", hako_operation="OrderedMapBox.length == 0 as i64_bool_v0", emits="VariableContextApi.is_empty(ctx)"),
            BehaviorMethodSpec(id="VariableContext::insert", rust_operation="BTreeMap::insert", hako_operation="OrderedMapBox.set", emits="VariableContextApi.insert(ctx, name, value_id)"),
            BehaviorMethodSpec(id="VariableContext::remove", rust_operation="BTreeMap::remove", hako_operation="OrderedMapBox.remove", emits="VariableContextApi.remove(ctx, name)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "simple_map_methods_only", "full_variable_context_claim": 0, "excluded_methods": excluded, "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["OrderedMapBox.get", "OrderedMapBox.has as i64_bool_v0", "OrderedMapBox.length", "OrderedMapBox.length == 0 as i64_bool_v0", "OrderedMapBox.set", "OrderedMapBox.remove"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "value_id_transport": "i64"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def box_compilation_context_spec() -> FamilyArtifactSpec:
    facts = extract_box_compilation_context_facts(BOX_COMPILATION_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "box-compilation-context-plan-v0.json")
    oracle = read_json(FIXTURES / "box-compilation-context-oracle-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in compile_box_compilation_context_methods(facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_box_compilation_context_artifact.py",
        generator_version="box-compilation-context-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json",
        family_comment="hakorune_mir_builder::context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="BoxCompilationContext",
            fields=[
                FieldSpec(name="variable_map", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="value_origin_newbox", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="value_types", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
            ],
        ),
        main_lines=[],
        main_operations=[
            op("NewBox", target="ctx", box="BoxCompilationContext"),
            op("StaticCall", target="is_empty_result", callee="BoxCompilationContextApi.is_empty", args=["ctx"]),
            op(
                "AssertEq",
                left="is_empty_result",
                right=1,
                fail_message="box_compilation_context_new_empty=fail",
                fail_code=1,
            ),
            op("Print", text="box_compilation_context_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::context",
        state="DerivedShadow",
        source_rust_file=BOX_COMPILATION_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "box_compilation_context.hako",
        facts_path=FIXTURES / "box-compilation-context-facts-v0.json",
        plan_path=FIXTURES / "box-compilation-context-plan-v0.json",
        oracle_path=FIXTURES / "box-compilation-context-oracle-v0.json",
        recipe_path=FIXTURES / "box-compilation-context-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "box-compilation-context-derived-artifact-verifier-result-v0.json",
        pilot_scope="BoxCompilationContext_ctor_is_empty_only",
        recipe_subject="hakorune_mir_builder::context::BoxCompilationContext",
        selected_body_count="constructor_is_empty_only",
        api_name="BoxCompilationContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(
                id="BoxCompilationContext::new",
                rust_operation="DefaultConstruct",
                hako_operation="BoxCompilationContext.birth",
                emits="BoxCompilationContext.birth initializes three ordered maps",
            ),
            BehaviorMethodSpec(
                id="BoxCompilationContext::is_empty",
                rust_operation="CompositeMapIsEmpty",
                hako_operation="BoxCompilationContextBox.all_fields_empty",
                emits="BoxCompilationContextApi.is_empty(ctx)",
            ),
        ],
        excluded_methods=["BoxCompilationContext::size_info"],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "constructor_is_empty_only", "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["DefaultConstruct", "NewOrderedMap", "AllFieldsMapIsEmpty"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "box_birth": "three ordered maps"},
        extra_manifest_fields={"excluded_methods": ["BoxCompilationContext::size_info"]},
    )


def variable_context_immutable_borrow_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map_mut", "VariableContext::snapshot", "VariableContext::restore"]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py",
        generator_version="variable-context-immutable-borrow-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="VariableContext", field_name="variable_map", field_type="OrderedMapBox", initializer="OrderedMap.create()"),
        main_lines=_lines("""
            local ctx = new VariableContext()
            local view = VariableContextApi.variable_map(ctx)
            if VariableMapViewApi.is_empty(view) != 1 {
                print("variable_context_borrow_view_empty=fail")
                return 1
            }
            if VariableMapViewApi.len(view) != 0 {
                print("variable_context_borrow_view_len=fail")
                return 2
            }
            if VariableMapViewApi.contains(view, "x") != 0 {
                print("variable_context_borrow_view_contains=fail")
                return 3
            }
            if VariableMapViewApi.lookup(view, "x") != null {
                print("variable_context_borrow_view_lookup=fail")
                return 4
            }

            print("variable_context_immutable_borrow_derived_artifact=ok")
            return 0
        """),
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_immutable_borrow.hako",
        facts_path=FIXTURES / "variable-context-immutable-borrow-facts-v0.json",
        plan_path=FIXTURES / "variable-context-immutable-borrow-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-immutable-borrow-oracle-vectors-v0.json",
        pilot_scope="VariableContext_immutable_borrow_only",
        static_boxes=[
            StaticBoxSpec(
                name="VariableContextApi",
                methods=[
                    ApiMethodSpec(
                        signature="variable_map(ctx)",
                        operations=[{"kind": "ReturnSource", "source": "ctx.variable_map"}],
                    )
                ],
            ),
            StaticBoxSpec(
                name="VariableMapViewApi",
                methods=[
                    ApiMethodSpec(signature="is_empty(view): i64", operations=[{"kind": "MapIsEmpty", "source": "view"}]),
                    ApiMethodSpec(signature="len(view): i64", operations=[{"kind": "MapLength", "source": "view"}]),
                    ApiMethodSpec(signature="contains(view, name): i64", operations=[{"kind": "MapHas", "source": "view", "key": "name"}]),
                    ApiMethodSpec(signature="lookup(view, name)", operations=[{"kind": "MapGet", "source": "view", "key": "name"}]),
                ],
            ),
        ],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def variable_context_snapshot_restore_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map", "VariableContext::variable_map_mut", "VariableContext::lookup", "VariableContext::require", "VariableContext::insert", "VariableContext::remove", "VariableContext::contains", "VariableContext::len", "VariableContext::is_empty", "CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]
    facts = extract_variable_context_snapshot_restore_facts(VARIABLE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "variable-context-snapshot-restore-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in compile_variable_context_snapshot_restore_methods(facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py",
        generator_version="variable-context-snapshot-restore-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="VariableContext",
            field_name="variable_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_lines=_lines("""
            local ctx = new VariableContext()
            local snapshot = VariableContextApi.snapshot(ctx)
            VariableContextApi.restore(ctx, snapshot)

            print("variable_context_snapshot_restore_derived_artifact=ok")
            return 0
        """),
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_snapshot_restore.hako",
        facts_path=FIXTURES / "variable-context-snapshot-restore-facts-v0.json",
        plan_path=FIXTURES / "variable-context-snapshot-restore-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-snapshot-restore-oracle-vectors-v0.json",
        pilot_scope="VariableContext_snapshot_restore_only",
        api_name="VariableContextApi",
        api_methods=api_methods,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        extra_manifest_fields={"excluded_methods": excluded},
    )


_GENERATORS = {
    "binding_context": (binding_context_spec, validate_binding_context, lambda spec: extract_binding_context_facts(BINDING_CONTEXT_SOURCE), "generated_binding_context_artifact=unchanged"),
    "box_compilation_context": (box_compilation_context_spec, validate_box_compilation_context, lambda spec: extract_box_compilation_context_facts(BOX_COMPILATION_CONTEXT_SOURCE), "generated_box_compilation_context_artifact=unchanged"),
    "variable_context_simple_map": (variable_context_simple_map_spec, validate_variable_context_simple_map, lambda spec: extract_variable_context_simple_map_facts(VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE), "generated_variable_context_simple_map_artifact=unchanged"),
    "variable_context_immutable_borrow": (variable_context_immutable_borrow_spec, validate_variable_context_immutable_borrow, lambda spec: read_json(spec.facts_path), "generated_variable_context_immutable_borrow_artifact=unchanged"),
    "variable_context_snapshot_restore": (variable_context_snapshot_restore_spec, validate_variable_context_snapshot_restore, lambda spec: extract_variable_context_snapshot_restore_facts(VARIABLE_CONTEXT_SOURCE), "generated_variable_context_snapshot_restore_artifact=unchanged"),
}


def run_mirbuilder_family_artifact_generator(name: str, *, check: bool) -> None:
    try:
        spec_factory, validator, facts_loader, unchanged_label = _GENERATORS[name]
    except KeyError as exc:
        raise SystemExit(f"unknown MirBuilder family artifact generator: {name}") from exc

    spec = spec_factory()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = []
    if recipe_text is not None and spec.recipe_path is not None:
        outputs.append((spec.recipe_path, recipe_text))
    if verifier_text is not None and spec.verifier_path is not None:
        outputs.append((spec.verifier_path, verifier_text))
    outputs.extend([(spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)])
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label=unchanged_label,
        load_facts=lambda: facts_loader(spec),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validator,
        outputs_factory=lambda: outputs,
    )
