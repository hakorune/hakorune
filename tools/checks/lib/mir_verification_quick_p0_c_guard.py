"""Guard the deterministic full-lib baseline runner/quick wiring row."""

from __future__ import annotations

import hashlib
from pathlib import Path
import re
import tomllib

import mir_call_d1b_active_surface_guard as api


ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-C-RUNNER-WIRE-R0"
KEY = "verification_health_quick_lib_baseline_p0_c_runner_wire_r0_2026_09_01"
REFRESH_ROW = "DEV-GATE-LIB-BASELINE-REFRESH-R0"
REFRESH_KEY = "verification_health_quick_lib_baseline_refresh_r0_2026_09_01"
VARMAP_RECONCILE_ROW = "DEV-GATE-COREPLAN-VARMAP-BOUNDARY-RECONCILE-D0"
VARMAP_RECONCILE_KEY = "verification_health_coreplan_varmap_boundary_reconcile_d0_2026_09_01"
VARMAP_ROLE_CENSUS_ROW = "DEV-GATE-COREPLAN-VARMAP-ROLE-CENSUS-PRUNE-R0"
VARMAP_ROLE_CENSUS_KEY = "dev_gate_coreplan_varmap_role_census_prune_r0_2026_09_01"
VARMAP_RESEAL_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-GENERIC-BODY-V1-R0"
VARMAP_RESEAL_KEY = "dev_gate_coreplan_varmap_reseal_generic_body_v1_r0_2026_09_01"
VARMAP_CARRIER_PIPELINE_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-CARRIER-PIPELINE-R0"
VARMAP_CARRIER_PIPELINE_KEY = "dev_gate_coreplan_varmap_reseal_carrier_pipeline_r0_2026_09_01"
VARMAP_LOOP_COND_BC_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-BC-R0"
VARMAP_LOOP_COND_BC_KEY = "dev_gate_coreplan_varmap_reseal_loop_cond_bc_r0_2026_09_01"
VARMAP_LOOP_TRUE_BC_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-TRUE-BC-R0"
VARMAP_LOOP_TRUE_BC_KEY = "dev_gate_coreplan_varmap_reseal_loop_true_bc_r0_2026_09_01"
VARMAP_LOOP_COND_CONTINUE_ONLY_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-CONTINUE-ONLY-R0"
VARMAP_LOOP_COND_CONTINUE_ONLY_KEY = "dev_gate_coreplan_varmap_reseal_loop_cond_continue_only_r0_2026_09_01"
VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-CONTINUE-WITH-RETURN-PHI-R0"
VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_KEY = "dev_gate_coreplan_varmap_reseal_loop_cond_continue_with_return_phi_r0_2026_09_01"
BASELINE = Path("tools/checks/manifests/cargo_lib_red_baseline.toml")
INVENTORY = Path("tools/checks/manifests/cargo_lib_red_baseline.tests.txt")
FAILURES = Path("tools/checks/manifests/cargo_lib_red_baseline.failures.txt")
QUICK_STEPS = Path("tools/checks/lib/dev_gate_quick_steps.sh")

VARMAP_ROOTS = (
    Path("src/mir/builder/control_flow/plan"),
    Path("src/mir/builder/ssa"),
)
VARMAP_TEST_ONLY_SITES = frozenset(
    {
        *(f"src/mir/builder/control_flow/plan/composer/coreloop_v2_nested_minimal.rs#{n}" for n in range(1, 5)),
        "src/mir/builder/control_flow/plan/features/generic_loop_body/nested_depth_observer_tests.rs#1",
        "src/mir/builder/control_flow/plan/features/generic_loop_located_composer_tests.rs#1",
        "src/mir/builder/control_flow/plan/features/generic_loop_whole_parity_tests.rs#1",
        *(f"src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs#{n}" for n in range(1, 5)),
        *(f"src/mir/builder/control_flow/plan/normalizer/tests.rs#{n}" for n in range(1, 4)),
        "src/mir/builder/control_flow/plan/parts/associated_source/located_hook_tests.rs#1",
        "src/mir/builder/control_flow/plan/parts/associated_source/located_parity_tests.rs#1",
    }
)
VARMAP_DISCONNECTED_SITES = frozenset(
    {"src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs#1"}
)
VARMAP_CANONICAL_SITES = frozenset(
    {"src/mir/builder/control_flow/plan/parts/var_map_scope.rs#1"}
)


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"P0-C field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"P0-C list is malformed: {name}")
    return list(value)


def _check_landed_files(root: Path, row: dict) -> None:
    for path in (BASELINE, INVENTORY, FAILURES, QUICK_STEPS):
        if not (root / path).is_file():
            api.fail(f"P0-C landed owner is missing: {path}")
    quick = (root / QUICK_STEPS).read_text(encoding="utf-8")
    if quick.count("cargo_lib_red_baseline.py") != 1:
        api.fail("P0-C quick wiring must invoke the runner exactly once")
    if "dev_gate_cmd_step" not in quick:
        api.fail("P0-C quick wiring must use a command step")
    evidence = _text(row, "implementation_evidence")
    for token in ("three", "exact", "cargo_lib_red_baseline.py"):
        if token not in evidence.lower():
            api.fail(f"P0-C implementation evidence lacks {token}")


def check_verification_quick_p0_c_runner_wire_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("P0-C requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        api.fail("P0-C is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("P0-C must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        api.fail("P0-C execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("P0-C card path drifted")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"P0-C section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("P0-C task id drifted")
    if _text(row, "parent_row") != "DEV-GATE-QUICK-LIB-BASELINE-P0":
        api.fail("P0-C parent row drifted")
    status = _text(row, "status")
    if status not in {"fast_open", "landed"}:
        api.fail("P0-C status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("P0-C permission/status drifted")

    decision = _text(row, "decision").lower()
    for token in ("fixed full-lib", "deterministic baseline", "known failure"):
        if token not in decision:
            api.fail(f"P0-C decision lacks {token}")
    for token in ("cargo test result summary", "cargo --list inventory", "sorted baseline"):
        if token not in _text(row, "source_authority"):
            api.fail(f"P0-C source authority lacks {token}")
    for token in ("added", "disappeared", "stack abort"):
        if token not in _text(row, "fail_fast_boundary"):
            api.fail(f"P0-C fail-fast boundary lacks {token}")
    for token in ("No claim that dev_gate quick is wholly green", "no DeclaredInstance", "no Call-schema"):
        if token not in _text(row, "non_claims"):
            api.fail(f"P0-C non-claims lack {token}")

    allowed = set(_list(row, "allowed_files"))
    expected_allowed = {
        "tools/checks/lib/cargo_lib_red_baseline.py",
        "tools/checks/lib/tests/test_cargo_lib_red_baseline.py",
        str(BASELINE),
        str(INVENTORY),
        str(FAILURES),
        str(QUICK_STEPS),
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        str(api.HELPER_REL),
        "docs/tools/check-scripts-index.md",
        str(api.STATE_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        str(api.CARD_REL),
    }
    if allowed != expected_allowed:
        api.fail("P0-C allowed-file boundary drifted")

    filters = _list(row, "focused_test_filters")
    if filters != ["test_cargo_lib_red_baseline"]:
        api.fail("P0-C focused test filter drifted")
    if status == "landed":
        if _text(row, "implementation_commit") == "pending":
            api.fail("P0-C landed row still has pending implementation")
        _check_landed_files(root, row)
    print(f"[{api.TAG}] P0-C baseline runner contract ok status={status}")


def _canonical_lines_sha256(lines: list[str]) -> str:
    payload = "" if not lines else "\n".join(lines) + "\n"
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def check_verification_quick_lib_baseline_refresh_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("baseline refresh requires fast or closeout work_mode")
    if state.get("current_execution_row") != REFRESH_ROW:
        api.fail("baseline refresh is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("baseline refresh must clear current_design_stop")
    if state.get("next_execution_card") != REFRESH_ROW:
        api.fail("baseline refresh execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("baseline refresh card path drifted")

    row = card.get(REFRESH_KEY)
    if not isinstance(row, dict):
        api.fail(f"baseline refresh section is missing: {REFRESH_KEY}")
    if _text(row, "task_id") != REFRESH_ROW:
        api.fail("baseline refresh task id drifted")
    if _text(row, "parent_row") != "MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0":
        api.fail("baseline refresh parent row drifted")
    status = _text(row, "status")
    if status not in {"fast_open", "landed"}:
        api.fail("baseline refresh status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("baseline refresh permission/status drifted")

    expected_allowed = {
        str(BASELINE),
        str(INVENTORY),
        str(api.STATE_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        str(api.CARD_REL),
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
    }
    if set(_list(row, "allowed_files")) != expected_allowed:
        api.fail("baseline refresh allowed-file boundary drifted")
    if _list(row, "focused_test_filters") != ["test_cargo_lib_red_baseline"]:
        api.fail("baseline refresh focused filter drifted")

    with (root / BASELINE).open("rb") as stream:
        manifest = tomllib.load(stream)
    expected_fields = {
        "expected_status": "FAILED",
        "expected_exit_code": 101,
        "expected_passed": 7394,
        "expected_failed": 139,
        "expected_ignored": 29,
        "expected_measured": 0,
        "expected_filtered": 0,
        "inventory_sha256": "f049ea4f066c7b027215f4c5edb74000bcf6f8962868c2a3744f5a919b5f8ca0",
        "failures_sha256": "86b8c383eb3d20f1851f33278e30fd431cae97dcc716aad9ac2fe13b586d9041",
    }
    for name, expected in expected_fields.items():
        if manifest.get(name) != expected:
            api.fail(f"baseline refresh manifest drifted: {name}")

    inventory = (root / INVENTORY).read_text(encoding="utf-8").splitlines()
    failures = (root / FAILURES).read_text(encoding="utf-8").splitlines()
    if len(inventory) != 7562 or inventory != sorted(set(inventory)):
        api.fail("baseline refresh inventory must contain 7562 unique sorted tests")
    if len(failures) != 139 or failures != sorted(set(failures)):
        api.fail("baseline refresh failure receipt must keep 139 unique sorted names")
    if _canonical_lines_sha256(inventory) != expected_fields["inventory_sha256"]:
        api.fail("baseline refresh inventory SHA drifted")
    if _canonical_lines_sha256(failures) != expected_fields["failures_sha256"]:
        api.fail("baseline refresh failure SHA drifted")
    quick = (root / QUICK_STEPS).read_text(encoding="utf-8")
    runner_steps = [
        line for line in quick.splitlines()
        if line.startswith("dev_gate_cmd_step ") and "cargo_lib_red_baseline.py" in line
    ]
    if len(runner_steps) != 1:
        api.fail("baseline refresh requires exactly one existing quick runner step")
    print(f"[{api.TAG}] baseline refresh receipt ok status={status}")


def check_verification_coreplan_varmap_boundary_reconcile_d0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("CorePlan varmap reconciliation requires design_stop")
    if state.get("current_execution_row") != VARMAP_RECONCILE_ROW:
        api.fail("CorePlan varmap reconciliation is not selected")
    if state.get("current_design_stop") != VARMAP_RECONCILE_ROW:
        api.fail("CorePlan varmap reconciliation design-stop pointer drifted")
    if state.get("next_execution_card") != "none__design_stop":
        api.fail("CorePlan varmap reconciliation must not open implementation")

    row = card.get(VARMAP_RECONCILE_KEY)
    if not isinstance(row, dict):
        api.fail(f"CorePlan varmap reconciliation section is missing: {VARMAP_RECONCILE_KEY}")
    if _text(row, "task_id") != VARMAP_RECONCILE_ROW:
        api.fail("CorePlan varmap reconciliation task id drifted")
    if _text(row, "status") != "active_design_stop":
        api.fail("CorePlan varmap reconciliation status drifted")
    if row.get("implementation_permission") is not False:
        api.fail("CorePlan varmap reconciliation must keep implementation closed")
    decision = _text(row, "decision")
    for token in ("51", "48", "reseal", "prune", "retire"):
        if token not in decision:
            api.fail(f"CorePlan varmap reconciliation decision lacks {token}")

    pattern = re.compile(r"variable_map\s*\.\s*insert\s*\(")
    sites = []
    for relative in (
        Path("src/mir/builder/control_flow/plan"),
        Path("src/mir/builder/ssa"),
    ):
        for path in sorted((root / relative).rglob("*.rs")):
            text = path.read_text(encoding="utf-8")
            sites.extend((path, match.start()) for match in pattern.finditer(text))
    if len(sites) != 51:
        api.fail(f"CorePlan varmap reconciliation premise drifted: sites={len(sites)}")
    guard = (root / "tools/checks/coreplan_varmap_boundary_inventory_guard.sh").read_text(
        encoding="utf-8"
    )
    if "max_insert_count = 48" not in guard:
        api.fail("CorePlan varmap historical upper-bound premise drifted")
    print(f"[{api.TAG}] CorePlan varmap reconciliation premise ok sites=51 bound=48")


def _collect_varmap_sites(root: Path) -> tuple[list[tuple[str, int, str]], list[tuple[str, int, str]]]:
    pattern = re.compile(r"variable_map\s*\.\s*(insert|remove|clear)\s*\(")
    occurrences: dict[str, int] = {}
    writes: list[tuple[str, int, str]] = []
    for relative in VARMAP_ROOTS:
        for path in sorted((root / relative).rglob("*.rs")):
            text = path.read_text(encoding="utf-8")
            for match in pattern.finditer(text):
                path_key = str(path.relative_to(root))
                occurrences[path_key] = occurrences.get(path_key, 0) + 1
                writes.append(
                    (path_key, occurrences[path_key], match.group(1))
                )
    return writes, [site for site in writes if site[2] in {"remove", "clear"}]


def check_verification_coreplan_varmap_role_census_prune_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    """Validate the finite role census without changing CorePlan semantics."""
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("CorePlan varmap role census requires fast or closeout")
    if state.get("current_execution_row") != VARMAP_ROLE_CENSUS_ROW:
        api.fail("CorePlan varmap role census is not selected")
    if state.get("current_design_stop") != "none":
        api.fail("CorePlan varmap role census must clear current_design_stop")
    if state.get("next_execution_card") != VARMAP_ROLE_CENSUS_ROW:
        api.fail("CorePlan varmap role census execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("CorePlan varmap role census card path drifted")

    row = card.get(VARMAP_ROLE_CENSUS_KEY)
    if not isinstance(row, dict):
        api.fail(f"CorePlan varmap role census section is missing: {VARMAP_ROLE_CENSUS_KEY}")
    if _text(row, "task_id") != VARMAP_ROLE_CENSUS_ROW:
        api.fail("CorePlan varmap role census task id drifted")
    status = _text(row, "status")
    if status not in {"fast_open", "landed"}:
        api.fail("CorePlan varmap role census status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("CorePlan varmap role census permission/status drifted")
    if _text(row, "parent_row") != VARMAP_RECONCILE_ROW:
        api.fail("CorePlan varmap role census parent drifted")
    decision = _text(row, "decision").lower()
    for token in ("raw=51", "test-only=16", "disconnected=1", "live=34", "canonical=1", "reseal=33"):
        if token not in decision:
            api.fail(f"CorePlan varmap role census decision lacks {token}")
    for token in ("raising the bound", "fixtures", "selected-c"):
        if token not in " ".join(_list(row, "non_authority")).lower():
            api.fail(f"CorePlan varmap role census non-authority lacks {token}")
    allowed = {
        "tools/checks/coreplan_varmap_boundary_inventory_guard.sh",
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }
    if set(_list(row, "allowed_files")) != allowed:
        api.fail("CorePlan varmap role census allowed-file boundary drifted")

    writes, remove_or_clear = _collect_varmap_sites(root)
    insert_sites = [site for site in writes if site[2] == "insert"]
    site_ids = {f"{path}#{ordinal}" for path, ordinal, _ in insert_sites}
    if len(insert_sites) != 51 or len(site_ids) != 51:
        api.fail(f"CorePlan varmap role census raw inventory drifted: inserts={len(insert_sites)}")
    if remove_or_clear:
        api.fail("CorePlan varmap role census found forbidden remove/clear")
    if site_ids & VARMAP_TEST_ONLY_SITES != VARMAP_TEST_ONLY_SITES:
        api.fail("CorePlan varmap test-only role inventory drifted")
    if site_ids & VARMAP_DISCONNECTED_SITES != VARMAP_DISCONNECTED_SITES:
        api.fail("CorePlan varmap disconnected role inventory drifted")
    if site_ids & VARMAP_CANONICAL_SITES != VARMAP_CANONICAL_SITES:
        api.fail("CorePlan varmap canonical owner inventory drifted")
    live_sites = site_ids - VARMAP_TEST_ONLY_SITES - VARMAP_DISCONNECTED_SITES
    reseal_sites = live_sites - VARMAP_CANONICAL_SITES
    if len(live_sites) != 34 or len(reseal_sites) != 33:
        api.fail(
            "CorePlan varmap role counts drifted: "
            f"test_only={len(site_ids & VARMAP_TEST_ONLY_SITES)} "
            f"disconnected={len(site_ids & VARMAP_DISCONNECTED_SITES)} "
            f"live={len(live_sites)} canonical={len(live_sites & VARMAP_CANONICAL_SITES)} "
            f"reseal={len(reseal_sites)}"
        )
    canonical_path = root / "src/mir/builder/control_flow/plan/parts/var_map_scope.rs"
    canonical_text = canonical_path.read_text(encoding="utf-8")
    if "publish_emission_cache" not in canonical_text:
        api.fail("CorePlan varmap canonical cache owner is missing")
    guard = (root / "tools/checks/coreplan_varmap_boundary_inventory_guard.sh").read_text(
        encoding="utf-8"
    )
    if "max_insert_count = 48" in guard or "variable_map_direct_insert_sites=48" in guard:
        api.fail("CorePlan varmap role guard still contains the stale 48-count contract")
    if "role-aware" not in guard.lower():
        api.fail("CorePlan varmap role guard is not role-aware")
    print(
        f"[{api.TAG}] CorePlan varmap role census ok "
        "raw=51 test_only=16 disconnected=1 live=34 canonical=1 reseal=33"
    )


def check_verification_coreplan_varmap_reseal_generic_body_v1_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    """Keep the next generic-loop reseal at a design-only boundary."""
    mode = state.get("work_mode")
    if mode not in {"design_stop", "fast", "closeout"}:
        api.fail("CorePlan generic-loop reseal mode is invalid")
    if state.get("current_execution_row") != VARMAP_RESEAL_ROW:
        api.fail("CorePlan generic-loop reseal is not selected")
    if mode == "design_stop":
        if state.get("current_design_stop") != VARMAP_RESEAL_ROW:
            api.fail("CorePlan generic-loop reseal design-stop pointer drifted")
        if state.get("next_execution_card") != "none__design_stop":
            api.fail("CorePlan generic-loop reseal must not open implementation")
    else:
        if state.get("current_design_stop") != "none":
            api.fail("CorePlan generic-loop reseal must clear current_design_stop")
        if state.get("next_execution_card") != VARMAP_RESEAL_ROW:
            api.fail("CorePlan generic-loop reseal execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("CorePlan generic-loop reseal card path drifted")

    row = card.get(VARMAP_RESEAL_KEY)
    if not isinstance(row, dict):
        api.fail(f"CorePlan generic-loop reseal section is missing: {VARMAP_RESEAL_KEY}")
    if _text(row, "task_id") != VARMAP_RESEAL_ROW:
        api.fail("CorePlan generic-loop reseal task id drifted")
    status = _text(row, "status")
    expected_statuses = {"active_design_stop"} if mode == "design_stop" else {"fast_open", "landed"}
    if status not in expected_statuses:
        api.fail("CorePlan generic-loop reseal status drifted")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("CorePlan generic-loop reseal permission/status drifted")
    if _text(row, "parent_row") != VARMAP_ROLE_CENSUS_ROW:
        api.fail("CorePlan generic-loop reseal parent drifted")
    decision = _text(row, "decision").lower()
    for token in ("five", "publish_emission_cache", "current_bindings", "no accepted shape"):
        if token not in decision:
            api.fail(f"CorePlan generic-loop reseal decision lacks {token}")
    if set(_list(row, "allowed_files")) != {
        "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs",
        "tools/checks/coreplan_varmap_boundary_inventory_guard.sh",
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }:
        api.fail("CorePlan generic-loop reseal allowed-file boundary drifted")
    if _text(row, "no_safe_slice").lower().find("second owner") < 0:
        api.fail("CorePlan generic-loop reseal must reject a second owner")

    writes, remove_or_clear = _collect_varmap_sites(root)
    target = [site for site in writes if site[0] == "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs"]
    expected_direct_sites = {0} if status == "landed" else {0, 5}
    if len(target) not in expected_direct_sites or any(site[2] != "insert" for site in target):
        api.fail(
            "CorePlan generic-loop reseal source inventory drifted: "
            f"sites={len(target)} expected_one_of={sorted(expected_direct_sites)}"
        )
    if remove_or_clear:
        api.fail("CorePlan generic-loop reseal found remove/clear under its boundary")
    if len((root / "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs").read_text(encoding="utf-8").splitlines()) > 760:
        api.fail("CorePlan generic-loop reseal source is at or beyond the 760-line design boundary")
    helper = root / "src/mir/builder/control_flow/plan/parts/var_map_scope.rs"
    if "publish_emission_cache" not in helper.read_text(encoding="utf-8"):
        api.fail("CorePlan generic-loop reseal helper owner is missing")
    print(
        f"[{api.TAG}] CorePlan generic-loop reseal row ok "
        f"status={status} direct_sites={len(target)}"
    )


def check_verification_coreplan_varmap_reseal_carrier_pipeline_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    """Validate the four-site carrier/pipeline cache-only reseal row."""
    mode = state.get("work_mode")
    if mode not in {"design_stop", "fast", "closeout"}:
        api.fail("CorePlan carrier/pipeline reseal mode is invalid")
    if state.get("current_execution_row") != VARMAP_CARRIER_PIPELINE_ROW:
        api.fail("CorePlan carrier/pipeline reseal is not selected")
    if mode == "design_stop":
        if state.get("current_design_stop") != VARMAP_CARRIER_PIPELINE_ROW:
            api.fail("CorePlan carrier/pipeline reseal design-stop pointer drifted")
        if state.get("next_execution_card") != "none__design_stop":
            api.fail("CorePlan carrier/pipeline reseal must not open implementation")
    else:
        if state.get("current_design_stop") != "none":
            api.fail("CorePlan carrier/pipeline reseal must clear current_design_stop")
        if state.get("next_execution_card") != VARMAP_CARRIER_PIPELINE_ROW:
            api.fail("CorePlan carrier/pipeline reseal execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("CorePlan carrier/pipeline reseal card path drifted")

    row = card.get(VARMAP_CARRIER_PIPELINE_KEY)
    if not isinstance(row, dict):
        api.fail(f"CorePlan carrier/pipeline reseal section is missing: {VARMAP_CARRIER_PIPELINE_KEY}")
    if _text(row, "task_id") != VARMAP_CARRIER_PIPELINE_ROW:
        api.fail("CorePlan carrier/pipeline reseal task id drifted")
    status = _text(row, "status")
    expected_statuses = {"active_design_stop"} if mode == "design_stop" else {"fast_open", "landed"}
    if status not in expected_statuses:
        api.fail("CorePlan carrier/pipeline reseal status drifted")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("CorePlan carrier/pipeline reseal permission/status drifted")
    if _text(row, "parent_row") != VARMAP_RESEAL_ROW:
        api.fail("CorePlan carrier/pipeline reseal parent drifted")
    decision = _text(row, "decision").lower()
    for token in ("four", "publish_emission_cache", "current_bindings", "boxshape"):
        if token not in decision:
            api.fail(f"CorePlan carrier/pipeline reseal decision lacks {token}")
    if "second owner" not in _text(row, "no_safe_slice").lower():
        api.fail("CorePlan carrier/pipeline reseal must reject a second owner")
    allowed = {
        "src/mir/builder/control_flow/plan/features/carrier_merge.rs",
        "src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs",
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }
    if set(_list(row, "allowed_files")) != allowed:
        api.fail("CorePlan carrier/pipeline reseal allowed-file boundary drifted")

    writes, remove_or_clear = _collect_varmap_sites(root)
    target_paths = {
        "src/mir/builder/control_flow/plan/features/carrier_merge.rs",
        "src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs",
    }
    target = [site for site in writes if site[0] in target_paths]
    expected_direct_sites = {0} if status == "landed" else {0, 4}
    if len(target) not in expected_direct_sites or any(site[2] != "insert" for site in target):
        api.fail(
            "CorePlan carrier/pipeline reseal source inventory drifted: "
            f"sites={len(target)} expected_one_of={sorted(expected_direct_sites)}"
        )
    if remove_or_clear:
        api.fail("CorePlan carrier/pipeline reseal found remove/clear under its boundary")
    for relative in sorted(target_paths):
        source = root / relative
        if len(source.read_text(encoding="utf-8").splitlines()) > 760:
            api.fail(f"CorePlan carrier/pipeline source reached the 760-line boundary: {relative}")
        if len(target) == 0 and "publish_emission_cache" not in source.read_text(encoding="utf-8"):
            api.fail(f"CorePlan carrier/pipeline source lacks the canonical cache helper: {relative}")
    helper = root / "src/mir/builder/control_flow/plan/parts/var_map_scope.rs"
    if "publish_emission_cache" not in helper.read_text(encoding="utf-8"):
        api.fail("CorePlan carrier/pipeline helper owner is missing")
    print(
        f"[{api.TAG}] CorePlan carrier/pipeline reseal row ok "
        f"status={status} direct_sites={len(target)}"
    )


def check_verification_coreplan_varmap_reseal_loop_cond_bc_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    """Validate the six-site loop-cond break/continue cache-only reseal row."""
    mode = state.get("work_mode")
    if mode not in {"design_stop", "fast", "closeout"}:
        api.fail("CorePlan loop-cond reseal mode is invalid")
    if state.get("current_execution_row") != VARMAP_LOOP_COND_BC_ROW:
        api.fail("CorePlan loop-cond reseal is not selected")
    if mode == "design_stop":
        if state.get("current_design_stop") != VARMAP_LOOP_COND_BC_ROW:
            api.fail("CorePlan loop-cond reseal design-stop pointer drifted")
        if state.get("next_execution_card") != "none__design_stop":
            api.fail("CorePlan loop-cond reseal must not open implementation")
    else:
        if state.get("current_design_stop") != "none":
            api.fail("CorePlan loop-cond reseal must clear current_design_stop")
        if state.get("next_execution_card") != VARMAP_LOOP_COND_BC_ROW:
            api.fail("CorePlan loop-cond reseal execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("CorePlan loop-cond reseal card path drifted")

    row = card.get(VARMAP_LOOP_COND_BC_KEY)
    if not isinstance(row, dict):
        api.fail(f"CorePlan loop-cond reseal section is missing: {VARMAP_LOOP_COND_BC_KEY}")
    if _text(row, "task_id") != VARMAP_LOOP_COND_BC_ROW:
        api.fail("CorePlan loop-cond reseal task id drifted")
    status = _text(row, "status")
    expected_statuses = {"active_design_stop"} if mode == "design_stop" else {"fast_open", "landed"}
    if status not in expected_statuses:
        api.fail("CorePlan loop-cond reseal status drifted")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("CorePlan loop-cond reseal permission/status drifted")
    if _text(row, "parent_row") != VARMAP_CARRIER_PIPELINE_ROW:
        api.fail("CorePlan loop-cond reseal parent drifted")
    decision = _text(row, "decision").lower()
    for token in ("six", "publish_emission_cache", "current_bindings", "boxshape"):
        if token not in decision:
            api.fail(f"CorePlan loop-cond reseal decision lacks {token}")
    if "second owner" not in _text(row, "no_safe_slice").lower():
        api.fail("CorePlan loop-cond reseal must reject a second owner")
    allowed = {
        "src/mir/builder/control_flow/plan/features/loop_cond_bc.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs",
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }
    if set(_list(row, "allowed_files")) != allowed:
        api.fail("CorePlan loop-cond reseal allowed-file boundary drifted")

    writes, remove_or_clear = _collect_varmap_sites(root)
    target_paths = {
        "src/mir/builder/control_flow/plan/features/loop_cond_bc.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs",
        "src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs",
    }
    target = [site for site in writes if site[0] in target_paths]
    expected_direct_sites = {0} if status == "landed" else {0, 6}
    if len(target) not in expected_direct_sites or any(site[2] != "insert" for site in target):
        api.fail(
            "CorePlan loop-cond reseal source inventory drifted: "
            f"sites={len(target)} expected_one_of={sorted(expected_direct_sites)}"
        )
    if remove_or_clear:
        api.fail("CorePlan loop-cond reseal found remove/clear under its boundary")
    for relative in sorted(target_paths):
        source = root / relative
        if len(source.read_text(encoding="utf-8").splitlines()) > 760:
            api.fail(f"CorePlan loop-cond source reached the 760-line boundary: {relative}")
        if len(target) == 0 and "publish_emission_cache" not in source.read_text(encoding="utf-8"):
            api.fail(f"CorePlan loop-cond source lacks the canonical cache helper: {relative}")
    helper = root / "src/mir/builder/control_flow/plan/parts/var_map_scope.rs"
    if "publish_emission_cache" not in helper.read_text(encoding="utf-8"):
        api.fail("CorePlan loop-cond reseal helper owner is missing")
    print(
        f"[{api.TAG}] CorePlan loop-cond reseal row ok "
        f"status={status} direct_sites={len(target)}"
    )


def _check_coreplan_varmap_reseal_single_site(
    state: dict,
    card: dict,
    root: Path,
    *,
    row_name: str,
    row_key: str,
    parent_row: str,
    label: str,
    target_paths: set[str],
    expected_direct_sites: int,
    expected_direct_sites_token: str,
    allowed_files: set[str],
) -> None:
    mode = state.get("work_mode")
    if mode not in {"design_stop", "fast", "closeout"}:
        api.fail(f"CorePlan {label} reseal mode is invalid")
    if state.get("current_execution_row") != row_name:
        api.fail(f"CorePlan {label} reseal is not selected")
    if mode == "design_stop":
        if state.get("current_design_stop") != row_name or state.get("next_execution_card") != "none__design_stop":
            api.fail(f"CorePlan {label} design-stop pointer drifted")
    elif state.get("current_design_stop") != "none" or state.get("next_execution_card") != row_name:
        api.fail(f"CorePlan {label} execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail(f"CorePlan {label} card path drifted")

    row = card.get(row_key)
    if not isinstance(row, dict) or _text(row, "task_id") != row_name:
        api.fail(f"CorePlan {label} reseal section/task is missing")
    status = _text(row, "status")
    expected_statuses = {"active_design_stop"} if mode == "design_stop" else {"fast_open", "landed"}
    if status not in expected_statuses:
        api.fail(f"CorePlan {label} reseal status drifted")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail(f"CorePlan {label} reseal permission/status drifted")
    if _text(row, "parent_row") != parent_row:
        api.fail(f"CorePlan {label} reseal parent drifted")
    decision = _text(row, "decision").lower()
    for token in (expected_direct_sites_token, "publish_emission_cache", "current_bindings", "cache-only"):
        if token not in decision:
            api.fail(f"CorePlan {label} decision lacks {token}")
    if "second owner" not in _text(row, "no_safe_slice").lower():
        api.fail(f"CorePlan {label} reseal must reject a second owner")
    if set(_list(row, "allowed_files")) != allowed_files:
        api.fail(f"CorePlan {label} allowed-file boundary drifted")

    writes, remove_or_clear = _collect_varmap_sites(root)
    target = [site for site in writes if site[0] in target_paths]
    expected_sites = {0} if status == "landed" else {0, expected_direct_sites}
    if len(target) not in expected_sites or any(site[2] != "insert" for site in target):
        api.fail(f"CorePlan {label} source inventory drifted: sites={len(target)}")
    if remove_or_clear:
        api.fail(f"CorePlan {label} reseal found remove/clear")
    for relative in sorted(target_paths):
        source = root / relative
        if len(source.read_text(encoding="utf-8").splitlines()) > 760:
            api.fail(f"CorePlan {label} source reached the 760-line boundary")
        if not target and "publish_emission_cache" not in source.read_text(encoding="utf-8"):
            api.fail(f"CorePlan {label} source lacks the canonical cache helper")
    helper = root / "src/mir/builder/control_flow/plan/parts/var_map_scope.rs"
    if "publish_emission_cache" not in helper.read_text(encoding="utf-8"):
        api.fail(f"CorePlan {label} reseal helper owner is missing")
    print(f"[{api.TAG}] CorePlan {label} reseal row ok status={status} direct_sites={len(target)}")


def _coreplan_varmap_reseal_allowed_files(target_path: str) -> set[str]:
    return {
        target_path,
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }


def check_verification_coreplan_varmap_reseal_loop_true_bc_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    _check_coreplan_varmap_reseal_single_site(
        state,
        card,
        root,
        row_name=VARMAP_LOOP_TRUE_BC_ROW,
        row_key=VARMAP_LOOP_TRUE_BC_KEY,
        parent_row=VARMAP_LOOP_COND_BC_ROW,
        label="loop-true",
        target_paths={
            "src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs"
        },
        expected_direct_sites=2,
        expected_direct_sites_token="two",
        allowed_files=_coreplan_varmap_reseal_allowed_files(
            "src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs"
        ),
    )


def check_verification_coreplan_varmap_reseal_loop_cond_continue_only_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    _check_coreplan_varmap_reseal_single_site(
        state,
        card,
        root,
        row_name=VARMAP_LOOP_COND_CONTINUE_ONLY_ROW,
        row_key=VARMAP_LOOP_COND_CONTINUE_ONLY_KEY,
        parent_row=VARMAP_CARRIER_PIPELINE_ROW,
        label="continue-only",
        target_paths={
            "src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs"
        },
        expected_direct_sites=1,
        expected_direct_sites_token="one",
        allowed_files=_coreplan_varmap_reseal_allowed_files(
            "src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs"
        ),
    )

def check_verification_coreplan_varmap_reseal_loop_cond_continue_with_return_phi_r0(state: dict, card: dict, root: Path, _parent_api=api) -> None:
    _check_coreplan_varmap_reseal_single_site(
        state,
        card,
        root,
        row_name=VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_ROW,
        row_key=VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_KEY,
        parent_row=VARMAP_LOOP_COND_CONTINUE_ONLY_ROW,
        label="continue-with-return-PHI",
        target_paths={"src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_phi_materializer.rs"},
        expected_direct_sites=1,
        expected_direct_sites_token="one",
        allowed_files=_coreplan_varmap_reseal_allowed_files("src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_phi_materializer.rs"),
    )
