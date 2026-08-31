#!/usr/bin/env python3
"""Guard the locator-to-ledger crosswalk design stop."""

from __future__ import annotations

from pathlib import Path
import re

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-D0"
KEY = "mir_call_me_declared_instance_locator_value_crosswalk_d0_2026_09_02"
PARENT_KEY = "mir_call_me_declared_instance_receiver_value_owner_d0_2026_09_01"
I0_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-I0"
I0_KEY = "mir_call_me_declared_instance_locator_value_crosswalk_i0_2026_09_01"


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"locator-value crosswalk D0 field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"locator-value crosswalk D0 list is missing or empty: {name}")
    return value


def _tokens(text: str, label: str, required: tuple[str, ...]) -> None:
    for token in required:
        if token not in text:
            api.fail(f"locator-value crosswalk D0 {label} lacks {token}")


def check_declared_instance_locator_value_crosswalk_d0(
    state: dict, card: dict, _root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("locator-value crosswalk D0 must remain design_stop")
    for field in ("current_execution_row", "current_design_stop", "next_design_card"):
        if state.get(field) != ROW:
            api.fail(f"locator-value crosswalk D0 pointer drifted: {field}")
    if state.get("next_execution_card") != "none":
        api.fail("locator-value crosswalk D0 must not open implementation")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"locator-value crosswalk D0 section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("locator-value crosswalk D0 task id drifted")
    if _text(row, "status") != "accepted_design_stop":
        api.fail("locator-value crosswalk D0 status drifted")
    if row.get("implementation_permission") is not False:
        api.fail("locator-value crosswalk D0 implementation permission must be false")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict):
        api.fail(f"locator-value crosswalk D0 parent is missing: {PARENT_KEY}")
    _tokens(
        _text(parent, "next_design_row"),
        "parent handoff",
        (ROW,),
    )
    _tokens(
        _text(row, "decision"),
        "decision",
        ("non-Clone callback-scoped view", "existing generic take-once value", "family-specific receiver loan"),
    )
    _tokens(
        _text(row, "source_authority"),
        "source authority",
        ("caller_owner", "receiver_site", "receiver_binding"),
    )
    _tokens(
        _text(row, "canonical_issuer"),
        "canonical issuer",
        ("No new issuer", "DeclaredInstanceCallLocatorViewV1", "CallableSemanticLoweringState"),
    )
    _tokens(
        _text(row, "fail_fast_boundary"),
        "fail-fast boundary",
        ("before receiver effects", "argument descent", "MIR publication", "fallback", "retry"),
    )
    _tokens(
        _text(row, "census_boundary"),
        "census boundary",
        ("Installed package locator callback", "live callable session owner", "request-local receiver ValueId"),
    )
    forbidden = "\n".join(_list(row, "non_authority"))
    _tokens(
        forbidden,
        "non-authority",
        ("variable_map", "param0", "args[0]", "ValueId(0)", "family-specific loan", "second resolver"),
    )

    states = _list(row, "finite_states")
    for state_name in (
        "NoRootDeclaredInstanceCall",
        "RelationRowUnavailable",
        "OwnerMismatch",
        "SessionMismatch",
        "BindingMismatch",
        "DuplicateCandidate",
        "EntryValueUnavailable",
        "ReadyToBorrow",
        "AlreadyTaken",
        "Residual",
        "NestedOrUpvarOutside",
    ):
        if not any(item.startswith(state_name + ":") for item in states):
            api.fail(f"locator-value crosswalk D0 finite states lack {state_name}")

    tasks = _list(row, "ordered_tasks")
    for prefix in ("D0-A:", "D0-B:", "D0-C:", "I0 later:"):
        if not any(item.startswith(prefix) for item in tasks):
            api.fail(f"locator-value crosswalk D0 tasks lack {prefix}")
    _tokens(
        _text(row, "acceptance"),
        "acceptance",
        ("finite bidirectional crosswalk", "before effects", "no ValueId"),
    )
    _tokens(
        _text(row, "no_safe_slice"),
        "NoSafeSlice",
        ("locator view", "locator-to-ledger crosswalk", "second authority", "fallback", "retry"),
    )
    _tokens(
        _text(row, "non_claims"),
        "non-claims",
        ("No receiver ValueId production", "no Method(Some)", "no selected-C/Hako/C change"),
    )
    _tokens(
        _text(row, "worker_audit_result"),
        "worker audit",
        ("three D0 audits", "CallableSemanticLoweringState", "crosswalk is zero"),
    )
    evidence = "\n".join(_list(row, "evidence"))
    _tokens(
        evidence,
        "evidence",
        ("declared_instance_locator.rs", "normal_callable_semantic_lowering_state.rs", "normal_callable_semantic_loan_port.rs"),
    )
    if _text(row, "next_design_row") != ROW:
        api.fail("locator-value crosswalk D0 next design row drifted")

    expected = {
        "docs/development/current/main/CURRENT_STATE.toml",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "tools/checks/lib/mir_declared_instance_locator_value_crosswalk_d0_guard.py",
    }
    if set(_list(row, "allowed_files")) != expected:
        api.fail("locator-value crosswalk D0 allowed-file boundary drifted")
    forbidden_files = "\n".join(_list(row, "forbidden_files"))
    _tokens(forbidden_files, "forbidden boundary", ("src/mir/", "Call schema", "VM/backend/JSON/runtime"))
    print(f"[{api.TAG}] locator-value crosswalk D0 design-stop contract ok")


def check_declared_instance_locator_value_crosswalk_i0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "fast":
        api.fail("locator-value crosswalk I0 requires fast work_mode")
    for field in ("current_execution_row", "next_execution_card"):
        if state.get(field) != I0_ROW:
            api.fail(f"locator-value crosswalk I0 pointer drifted: {field}")
    if state.get("current_design_stop") != "none":
        api.fail("locator-value crosswalk I0 must clear current_design_stop")

    row = card.get(I0_KEY)
    if not isinstance(row, dict):
        api.fail(f"locator-value crosswalk I0 section is missing: {I0_KEY}")
    if _text(row, "task_id") != I0_ROW:
        api.fail("locator-value crosswalk I0 task id drifted")
    if _text(row, "status") != "fast_open":
        api.fail("locator-value crosswalk I0 status must be fast_open")
    if row.get("implementation_permission") is not True:
        api.fail("locator-value crosswalk I0 implementation permission must be true")

    locator_path = root / "src/mir/normal_callable_semantic_package/declared_instance_locator.rs"
    lowering_port_path = root / "src/mir/normal_callable_semantic_package/install/lowering_port.rs"
    receiver_path = root / "src/mir/builder/normal_callable_semantic_receiver_crosswalk.rs"
    raw_path = root / "src/mir/builder/recursive_child_lowering.rs"
    method_path = root / "src/mir/builder/method_call_handlers.rs"
    tests = (
        root / "src/mir/normal_callable_semantic_package/declared_instance_locator_tests.rs",
        root / "src/mir/normal_callable_semantic_package/resolved_selected_handoff_tests.rs",
        root / "src/mir/builder/normal_callable_semantic_receiver_crosswalk_tests.rs",
    )
    production = (locator_path, lowering_port_path, receiver_path, raw_path, method_path)
    for path in (*production, *tests):
        if not path.is_file():
            api.fail(f"locator-value crosswalk I0 source is missing: {path}")
        if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
            api.fail(f"locator-value crosswalk I0 source reached the 760-line boundary: {path}")

    locator = locator_path.read_text(encoding="utf-8")
    lowering_port = lowering_port_path.read_text(encoding="utf-8")
    receiver = receiver_path.read_text(encoding="utf-8")
    raw = raw_path.read_text(encoding="utf-8")
    method = method_path.read_text(encoding="utf-8")
    _tokens(
        locator,
        "locator source",
        (
            "DeclaredInstanceCallLocatorScopeV1",
            "take_exact_relation",
            "RelationUnavailable",
            "AlreadyTaken",
            "self.consumed.insert",
        ),
    )
    relation_check = locator.index("relation.rows().get")
    locator_take = locator.index("self.consumed.insert", relation_check)
    locator_callback = locator.index("callback(DeclaredInstanceCallRelationViewV1", locator_take)
    if not relation_check < locator_take < locator_callback:
        api.fail("locator-value crosswalk I0 must validate, take, then invoke the callback")
    if re.search(
        r"#\[derive\([^]]*(?:Clone|Copy)[^]]*\)\]\s*pub\(in crate::mir\) struct DeclaredInstanceCallLocatorScopeV1",
        locator,
        flags=re.DOTALL,
    ):
        api.fail("locator-value crosswalk I0 scope must remain non-Clone/non-Copy")
    for token in ("Rc<", "RefCell<", "variable_map", "args[0]", "ValueId(0)"):
        if token in locator:
            api.fail(f"locator-value crosswalk I0 locator contains forbidden token: {token}")

    _tokens(
        lowering_port,
        "package lowering port",
        (
            "with_selected_cataloged_lowering_input_signature_and_declared_instance_locator",
            "declared_instance_consumed",
            "DeclaredInstanceLocatorNotConsumed",
        ),
    )
    _tokens(
        receiver,
        "receiver crosswalk",
        (
            "take_exact_receiver_value",
            "ReceiverBindingMismatch",
            "ReceiverSiteUnavailable",
            "AlreadyTaken",
            "value_for_exact_binding",
            "consumed_variables.insert",
        ),
    )
    if receiver.index("value_for_exact_binding") > receiver.index("consumed_variables.insert"):
        api.fail("receiver value must be verified before its exact source site is consumed")
    for token in ("variable_map", "param0", "args[0]", "ValueId(0)", "Callee"):
        if token in receiver:
            api.fail(f"locator-value crosswalk I0 receiver source contains forbidden token: {token}")

    _tokens(
        raw,
        "raw capability",
        (
            "declared_instance_locator",
            "with_declared_instance_locator_scope",
            "take_declared_instance_receiver_value_inner_v1",
            "take_exact_relation",
            "take_exact_receiver_value",
        ),
    )
    _tokens(
        method,
        "method consumer",
        (
            "take_declared_instance_receiver_value_v1",
            "DeclaredInstanceReceiverIngressV1::Unarmed",
            "Self::prepare",
            "DeclaredInstanceReceiverIngressV1::Ready(value)",
            "prepare_me_call_execution_with_receiver_v1",
        ),
    )
    if method.index("take_declared_instance_receiver_value_v1") > method.index(
        "validate_prepared_me_arity_before_descent"
    ):
        api.fail("receiver capability must be consumed before argument descent preflight")

    test_text = "\n".join(path.read_text(encoding="utf-8") for path in tests)
    for test_name in _list(row, "changed_test_names"):
        if f"fn {test_name}(" not in test_text:
            api.fail(f"locator-value crosswalk I0 changed test is missing: {test_name}")
    _tokens(
        test_text,
        "focused tests",
        (
            "DeclaredInstanceCallLocatorTakeErrorV1::AlreadyTaken",
            "DeclaredInstanceLocatorNotConsumed",
            "ExactReceiverValueErrorV1::AlreadyTaken",
        ),
    )
    print(f"[{api.TAG}] locator-value crosswalk I0 structure ok")
