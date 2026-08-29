"""Private guard for the bounded caller-zero Cataloged print retirement."""

from pathlib import Path


CATALOGED_PRINT_RETIRE_ROW = (
    "MIR-CALL-SAME-MODULE-CATALOGED-PRINT-CALLER-ZERO-RETIRE-I0"
)
CATALOGED_PRINT_RETIRE_KEY = "same_module_cataloged_print_caller_zero_retire_i0_2026_08_30"


def check_cataloged_print_caller_zero_retire_i0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("cataloged print retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != CATALOGED_PRINT_RETIRE_ROW:
        api.fail("cataloged print retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("cataloged print retirement must clear current_design_stop")
    if state.get("next_execution_card") != CATALOGED_PRINT_RETIRE_ROW:
        api.fail("cataloged print retirement pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("cataloged print retirement card pointer drifted")

    row = card.get(CATALOGED_PRINT_RETIRE_KEY)
    if not isinstance(row, dict):
        api.fail(f"{CATALOGED_PRINT_RETIRE_KEY} section is missing")
    if row.get("task_id") != CATALOGED_PRINT_RETIRE_ROW:
        api.fail("cataloged print retirement task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        api.fail("cataloged print retirement status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        api.fail("cataloged print retirement permission/status drifted")

    route_rel = Path("src/mir/builder/calls/function_call_preflight_route.rs")
    build_rel = Path("src/mir/builder/calls/build.rs")
    tests_rel = Path("src/mir/builder/calls/function_call_installed_gc_builtin_tests.rs")
    print_rel = Path("src/mir/builder/stmts/print_stmt.rs")
    route = (root / route_rel).read_text()
    build = (root / build_rel).read_text()
    tests = (root / tests_rel).read_text()
    print_owner = (root / print_rel).read_text()
    completion_start = route.find("fn prepare_ordinary_function_completion_v1")
    completion_end = route.find("fn is_installed_non_unified_gc_builtin_v1")
    if completion_start < 0 or completion_end < completion_start:
        api.fail("cataloged print ordinary completion owner cannot be located")
    completion = route[completion_start:completion_end]
    print_pos = completion.find('name == "print"')
    caller_pos = completion.find("else if let Some(caller) = caller")
    if print_pos < 0 or caller_pos < 0 or print_pos > caller_pos:
        api.fail("cataloged print retirement is not before caller target preparation")
    for token in (
        "BuiltinPrintCataloged",
        "PreparedRawOrdinaryFunctionCompletionV1::Retired",
        "caller.is_some()",
    ):
        if token not in completion and token not in route:
            api.fail(f"cataloged print retirement evidence is missing: {token}")
    if "CanonicalGlobalTargetV1::builtin_print()" not in print_owner:
        api.fail("dedicated ASTNode::Print owner no longer projects builtin_print")
    for token in (
        "cataloged_print_caller_zero_retires_before_target_synthesis",
        "cataloged_print_rejection_does_not_descend_or_publish",
        "RawOrdinaryFunctionRetirementV1::BuiltinPrintCataloged",
    ):
        if token not in tests:
            api.fail(f"cataloged print retirement test evidence is missing: {token}")
    if "PreparedRawOrdinaryFunctionCompletionV1::Retired" not in build:
        api.fail("ordinary completion consumer does not retain the typed retirement path")
    for path in (route_rel, build_rel, tests_rel, print_rel):
        if sum(1 for _ in (root / path).open()) >= 760:
            api.fail(f"cataloged print retirement source reached the 760-line boundary: {path}")

    expected_allowed = {
        str(route_rel),
        str(build_rel),
        str(tests_rel),
        "src/mir/builder/calls/function_call_preflight_route_tests.rs",
        "src/mir/builder/calls/README.md",
        str(api.HELPER_REL),
        "tools/checks/lib/mir_call_d1b_cataloged_print_guard.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        api.fail("cataloged print retirement allowed-file boundary drifted")
