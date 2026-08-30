"""Fail-closed guard for the StaticCurrentOwner ``me.method`` cutover.

This guard is deliberately a small sibling of the method-corridor guard.  It
checks one production edge: an exact source/catalog handoff is consumed before
legacy header observation and argument descent.  True DeclaredInstance and
all compatibility siblings remain outside this row.
"""

from __future__ import annotations

from pathlib import Path


ROW = "MIR-CALL-SAME-MODULE-STATIC-CURRENT-OWNER-HANDOFF-I0"
KEY = "same_module_static_current_owner_handoff_i0_2026_08_30"
PARENT_KEY = "same_module_all_producer_disposition_r0_2026_08_30"

HANDLERS_REL = Path("src/mir/builder/method_call_handlers.rs")
INGRESS_REL = Path("src/mir/builder/static_result_publication_ingress.rs")
BRIDGE_REL = Path(
    "src/mir/builder/calls/static_result_publication_physical_bridge.rs"
)
TESTS_REL = Path("src/mir/builder/calls/me_method_canonical_cutover_tests.rs")
CALLS_MOD_REL = Path("src/mir/builder/calls/mod.rs")
GUARD_REL = Path("tools/checks/lib/mir_call_d1b_me_method_cutover_guard.py")


def _fail(api: object, message: str) -> None:
    api.fail(f"StaticCurrentOwner me.method I0: {message}")


def _read(root: Path, relative: Path, api: object) -> str:
    path = root / relative
    if not path.is_file():
        _fail(api, f"missing owner: {relative}")
    return path.read_text(encoding="utf-8")


def _check_lines(root: Path, relatives: tuple[Path, ...], api: object) -> None:
    for relative in relatives:
        path = root / relative
        if not path.is_file():
            _fail(api, f"missing line-count owner: {relative}")
        line_count = len(path.read_text(encoding="utf-8").splitlines())
        if line_count >= 800:
            _fail(api, f"800-line hard stop reached: {relative}")
        if line_count >= 760:
            _fail(api, f"760-line split boundary reached: {relative}")


def _check_order(handler: str, api: object) -> None:
    start = handler.find("fn resolve_me_call_with_publication_ingress")
    end = handler.find("fn validate_prepared_me_arity_before_descent", start)
    if start < 0 or end <= start:
        _fail(api, "publication-ingress owner cannot be located")
    body = handler[start:end]
    ingress = body.find("take_static_result_publication_ingress_v1")
    prepare = body.find("let prepared = Self::prepare")
    if ingress < 0 or prepare < 0 or ingress > prepare:
        _fail(api, "exact ingress must precede legacy prepare/header observation")
    for token in ("observe_me_call_parameters", "generate_method_function_name"):
        if token in body:
            _fail(api, f"selected StaticCurrentOwner branch re-enters {token}")
    if "StaticResultPublicationIngressV1::Selected" not in body:
        _fail(api, "Selected handoff consumer is missing")
    if "StaticResultPublicationIngressV1::TargetOnly" not in body:
        _fail(api, "TargetOnly handoff consumer is missing")


def _check_landed_shape(root: Path, api: object) -> None:
    handler = _read(root, HANDLERS_REL, api)
    ingress = _read(root, INGRESS_REL, api)
    bridge = _read(root, BRIDGE_REL, api)
    calls_mod = _read(root, CALLS_MOD_REL, api)
    tests = _read(root, TESTS_REL, api)

    _check_order(handler, api)
    if "SameModuleCallableNamespaceV1::StaticBoxMethod" not in ingress:
        _fail(api, "source ingress does not enforce StaticBoxMethod namespace")
    if "RawInvocationRootLineageV1::Cataloged(_)" not in ingress:
        _fail(api, "non-static Cataloged sibling is not kept outside this ingress")
    if "StaticResultPublicationIngressV1::Unavailable" not in ingress:
        _fail(api, "outside-static ingress state is missing")

    if "source_argument_count: usize" not in bridge:
        _fail(api, "bridge does not receive the source arity before descent")
    projection = bridge.find("canonical_global_target_v1()")
    descent = bridge.find("descent.lower_all(builder)")
    if projection < 0 or descent < 0 or projection > descent:
        _fail(api, "typed Global projection is not before argument descent")
    if "static-result-bridge/source-arity" not in bridge:
        _fail(api, "selected source-arity rejection is missing")
    if "static-target-only/source-arity" not in bridge:
        _fail(api, "target-only source-arity rejection is missing")
    if "lower_selected_static_result_publication_v1" not in calls_mod:
        _fail(api, "selected bridge is not exported to the bounded route")

    for name in (
        "static_current_owner_target_is_taken_before_header_and_arguments",
        "static_current_owner_lowers_source_arguments_once_without_receiver_prefix",
        "static_current_owner_missing_target_rejects_before_arguments",
        "outside_static_current_owner_preserves_declared_instance_sibling",
        "static_current_owner_argument_failure_does_not_emit_retry_or_fallback",
    ):
        if f"fn {name}(" not in tests:
            _fail(api, f"focused test is missing: {name}")


def check_me_method_canonical_i0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        _fail(api, "requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        _fail(api, "row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        _fail(api, "current_design_stop must be none")
    if state.get("next_execution_card") != ROW:
        _fail(api, "execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        _fail(api, "execution card path drifted")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict) or parent.get("status") != "accepted_with_open_blockers":
        _fail(api, "SameModule parent is not blocker-open")
    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        _fail(api, "active I0 card is missing")
    if row.get("status") not in {"fast_open", "landed"}:
        _fail(api, "row status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        _fail(api, "row permission/status drifted")

    expected_allowed = {
        str(HANDLERS_REL),
        str(INGRESS_REL),
        str(BRIDGE_REL),
        str(TESTS_REL),
        str(CALLS_MOD_REL),
        "src/mir/source_call_target/README.md",
        "src/mir/builder/README.md",
        str(GUARD_REL),
        str(api.HELPER_REL),
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    declared = row.get("allowed_files")
    if not isinstance(declared, list) or set(declared) != expected_allowed:
        _fail(api, "allowed-file boundary drifted")

    _check_lines(
        root,
        (HANDLERS_REL, INGRESS_REL, BRIDGE_REL, CALLS_MOD_REL, GUARD_REL),
        api,
    )
    if row.get("status") == "fast_open":
        return

    _check_landed_shape(root, api)
    base = api.require_text(row.get("coverage_base_commit"), "coverage_base_commit")
    changed_paths = api.git_diff_paths(root, base)
    if not changed_paths.issubset(expected_allowed):
        _fail(api, f"changed paths escaped: {sorted(changed_paths - expected_allowed)}")
    api.check_test_coverage(root, row)
