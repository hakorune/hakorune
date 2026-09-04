"""Small helper owners for the stable D1B active-surface dispatcher."""

from __future__ import annotations

from pathlib import Path
import tomllib


BOXSHAPE_MAINTENANCE_ROW = "MIRBUILDER-BOXSHAPE-MAINTENANCE-T0"
BOXSHAPE_MAINTENANCE_MANIFEST = Path(
    "docs/development/current/main/investigations/"
    "mirbuilder-cleanup-retirement1-next-edge-census-2026-08-25.toml"
)
T3_CLEANUP_ROW = "MIRBUILDER-T3-CLEANUP-R0"
T3_CLEANUP_COHORT = "route_selection_test_facade"
T3_CLEANUP_MANIFEST = Path(
    "docs/development/current/main/investigations/"
    "mirbuilder-cleanup-retirement1-next-edge-census-2026-08-25.toml"
)
T3_PLAN_CANON_COHORT = "plan_canon_facade"
T3_PLAN_CANON_TASK = "MIRBUILDER-PLAN-CANON-FACADE-RETIRE-R0"
T3_PLAN_CANON_FILES = (
    "src/mir/builder/control_flow/plan/canon/README.md",
    "src/mir/builder/control_flow/plan/canon/generic_loop.rs",
    "src/mir/builder/control_flow/plan/canon/generic_loop/condition.rs",
    "src/mir/builder/control_flow/plan/canon/generic_loop/step.rs",
    "src/mir/builder/control_flow/plan/canon/generic_loop/step/placement.rs",
    "src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/decision.rs",
    "src/mir/builder/control_flow/plan/canon/generic_loop/types.rs",
    "src/mir/builder/control_flow/plan/canon/mod.rs",
)
LEGACY_PHI_CANDIDATE_ROW = "MIRBUILDER-LEGACY-PHI-CANDIDATE-RETIRE-R0"
LEGACY_PHI_CANDIDATE_FILES = (
    "src/mir/builder/ssa/phi_input_materializer/legacy_candidate.rs",
    "src/mir/builder/ssa/phi_input_materializer/legacy_candidate_cfg.rs",
    "src/mir/builder/ssa/phi_input_materializer/legacy_candidate_tests.rs",
)


def check_boxshape_maintenance(state: dict, root: Path, api) -> None:
    """Validate the one reusable, manifest-driven BoxShape maintenance lane."""
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} must be fast or closeout")
    if state.get("current_execution_row") != BOXSHAPE_MAINTENANCE_ROW:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} pointer row drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} must clear current_design_stop")
    expected_next = BOXSHAPE_MAINTENANCE_ROW if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} next_execution_card drifted")
    manifest_rel = str(BOXSHAPE_MAINTENANCE_MANIFEST)
    if state.get("latest_card_path") != manifest_rel:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} latest card drifted")

    with (root / BOXSHAPE_MAINTENANCE_MANIFEST).open("rb") as handle:
        manifest = tomllib.load(handle)
    row = manifest.get("boxshape_maintenance_t0")
    if not isinstance(row, dict) or row.get("task_id") != BOXSHAPE_MAINTENANCE_ROW:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} manifest row is missing")
    expected_status = "selected_fast" if mode == "fast" else "landed"
    if row.get("status") != expected_status:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} status drifted")
    if row.get("implementation_permission") is (mode != "fast"):
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} implementation permission drifted")

    budget_paths = row.get("line_budget_paths")
    if not isinstance(budget_paths, list) or not budget_paths:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} line budget paths are missing")
    pre_split_paths = row.get("pre_split_over_limit_paths", [])
    if not isinstance(pre_split_paths, list) or not all(
        isinstance(path, str) and path.strip() for path in pre_split_paths
    ):
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} pre_split_over_limit_paths are malformed")
    for rel in budget_paths:
        path = root / rel
        if not path.is_file():
            api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} owner is missing: {rel}")
        line_count = sum(1 for _ in path.open(encoding="utf-8"))
        if line_count >= 760 and not (
            mode == "fast" and rel in pre_split_paths
        ):
            api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} owner reached 760 lines: {rel}")

    for field, should_exist in (("required_text", True), ("forbidden_text", False)):
        entries = row.get(field)
        if not isinstance(entries, list):
            api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} {field} is missing")
        for entry in entries:
            rel, separator, token = entry.partition("||")
            if not separator or not rel or not token:
                api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} malformed {field}: {entry!r}")
            present = token in (root / rel).read_text(encoding="utf-8")
            if present is not should_exist:
                api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} {field} mismatch: {entry!r}")

    base = row.get("base_head")
    if not isinstance(base, str) or not base:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} base_head is missing")
    diff = api.subprocess.run(
        ["git", "diff", "--numstat", base, "--", "src"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    delta = 0
    for line in diff.splitlines():
        added, deleted, _ = line.split("\t", 2)
        if "-" in {added, deleted}:
            api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} binary src delta is forbidden")
        delta += int(added) - int(deleted)
    untracked = api.subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "--", "src"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    for rel in untracked:
        path = root / rel
        if not path.is_file():
            api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} untracked source is not a file: {rel}")
        delta += sum(1 for _ in path.open(encoding="utf-8"))

    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or not allowed:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} allowed_files are missing")
    tracked = api.subprocess.run(
        ["git", "diff", "--name-only", base, "--"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    unexpected = (set(tracked) | set(untracked)) - set(allowed)
    if unexpected:
        api.fail(
            f"{BOXSHAPE_MAINTENANCE_ROW} changed files exceed the manifest: "
            f"{sorted(unexpected)}"
        )
    if delta > 0:
        api.fail(f"{BOXSHAPE_MAINTENANCE_ROW} src line delta is positive: {delta}")
    print(f"[{api.TAG}] row={BOXSHAPE_MAINTENANCE_ROW} delegated=boxshape-maintenance delta={delta}")


def check_t3_cleanup(state: dict, root: Path, api) -> None:
    """Validate one reusable caller-zero T3 cleanup cohort."""
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{T3_CLEANUP_ROW} must be fast or closeout")
    if state.get("current_execution_row") != T3_CLEANUP_ROW:
        api.fail(f"{T3_CLEANUP_ROW} pointer row drifted")
    cohort = state.get("current_execution_cohort")
    if cohort not in {T3_CLEANUP_COHORT, T3_PLAN_CANON_COHORT}:
        api.fail(f"{T3_CLEANUP_ROW} cohort drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{T3_CLEANUP_ROW} must clear current_design_stop")
    if state.get("next_design_card") != "none":
        api.fail(f"{T3_CLEANUP_ROW} must not open a second design card")

    manifest_rel = str(T3_CLEANUP_MANIFEST)
    for key in ("latest_card_path", "current_execution_design"):
        if state.get(key) != manifest_rel:
            api.fail(f"{T3_CLEANUP_ROW} {key} drifted")
    next_card = state.get("next_execution_card")
    if mode == "fast" and next_card != T3_CLEANUP_ROW:
        api.fail(f"{T3_CLEANUP_ROW} fast next_execution_card drifted")
    if mode == "closeout" and next_card != "none":
        next_path = root / str(state.get("next_execution_card_path", ""))
        if not next_path.is_file():
            api.fail(f"{T3_CLEANUP_ROW} selected next task path is missing")
        if str(next_card) not in next_path.read_text(encoding="utf-8"):
            api.fail(f"{T3_CLEANUP_ROW} selected next task is not owned by its path")

    with (root / T3_CLEANUP_MANIFEST).open("rb") as handle:
        manifest = tomllib.load(handle)
    task_id = (
        "MIRBUILDER-ROUTE-SELECTION-TEST-FACADE-R0"
        if cohort == T3_CLEANUP_COHORT
        else T3_PLAN_CANON_TASK
    )
    rows = [row for row in manifest.get("candidate", []) if row.get("id") == task_id]
    if len(rows) != 1:
        api.fail(f"{T3_CLEANUP_ROW} manifest candidate is missing or duplicated")
    row = rows[0]
    expected_status = "selected_fast" if mode == "fast" else "landed"
    if row.get("status") != expected_status:
        api.fail(f"{T3_CLEANUP_ROW} candidate status drifted")
    if row.get("implementation_permission") is not (mode == "fast"):
        api.fail(f"{T3_CLEANUP_ROW} implementation permission drifted")

    if cohort == T3_CLEANUP_COHORT:
        source_rel = "src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs"
        source = root / source_rel
        if not source.is_file() or sum(1 for _ in source.open(encoding="utf-8")) >= 800:
            api.fail(f"{T3_CLEANUP_ROW} source owner missing or reached 800 lines")
        present = "selection_for_test" in source.read_text(encoding="utf-8")
        if present is not (mode == "fast"):
            api.fail(f"{T3_CLEANUP_ROW} selected helper presence does not match mode")
    else:
        for source_rel in T3_PLAN_CANON_FILES:
            present = (root / source_rel).is_file()
            if present is not (mode == "fast"):
                api.fail(f"{T3_CLEANUP_ROW} plan/canon file presence drifted: {source_rel}")

    base = row.get("base_head")
    if not isinstance(base, str) or not base:
        api.fail(f"{T3_CLEANUP_ROW} base_head is missing")
    changed = api.subprocess.run(
        ["git", "diff", "--name-only", base, "--"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    allowed = set(row.get("allowed_files", []))
    unexpected = set(changed) - allowed
    if unexpected:
        api.fail(f"{T3_CLEANUP_ROW} changed files exceed manifest: {sorted(unexpected)}")
    diff = api.subprocess.run(
        ["git", "diff", "--numstat", base, "--", "src"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    delta = 0
    for line in diff.splitlines():
        added, deleted, _ = line.split("\t", 2)
        if "-" in {added, deleted}:
            api.fail(f"{T3_CLEANUP_ROW} binary src delta is forbidden")
        delta += int(added) - int(deleted)
    if delta > 0:
        api.fail(f"{T3_CLEANUP_ROW} src line delta is positive: {delta}")
    print(f"[{api.TAG}] row={T3_CLEANUP_ROW} cohort={cohort} delta={delta}")


def check_wasm_legacy_reader_stop_r0(
    state: dict,
    root: Path,
    api,
    *,
    row: str,
    reader: str,
    stop_tag: str,
    required: tuple[str, ...],
    owners: tuple[str, ...],
) -> None:
    """Validate one historical WASM reader-stop contract."""
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if state.get("current_design_stop") != "none" or state.get("next_design_card") != "none":
        api.fail(f"{row} must not have an open design stop")
    expected_next = row if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{row} next_execution_card drifted")
    final_rel = str(api.FINAL_PIPELINE_REL)
    for key in ("next_execution_card_path", "latest_card_path"):
        if state.get(key) != final_rel:
            api.fail(f"{row} {key} drifted")
    card_text = (root / api.FINAL_PIPELINE_REL).read_text(encoding="utf-8")
    for token in (row, reader, stop_tag, *required):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")
    expected = (
        ("status = fast_open", "implementation permission = true")
        if mode == "fast"
        else ("status = landed", "implementation permission = false")
    )
    for token in expected:
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")
    if len(card_text.splitlines()) > 1000:
        api.fail(f"{row} final-pipeline SSOT exceeds the 1000-line hard limit")
    for rel in owners:
        path = root / rel
        if not path.is_file() or sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner missing or reached 800 lines: {rel}")


def check_legacy_reader_stop_r0(state: dict, root: Path, api) -> None:
    """Validate M7-S scheduling; source tests own cohort semantics."""
    row = "MIR-CALL-LEGACY-READER-STOP-R0"
    mode = state.get("work_mode")
    cohort = state.get("current_execution_cohort")
    if not isinstance(cohort, str) or not cohort or not all(
        char.islower() or char.isdigit() or char == "_" for char in cohort
    ):
        api.fail(f"{row} requires one finite snake_case cohort token")
    if mode not in {"fast", "closeout"} or state.get("current_execution_row") != row:
        api.fail(f"{row} pointer mode/row drifted")
    if state.get("current_design_stop") != "none" or state.get("next_design_card") != "none":
        api.fail(f"{row} must not have an open design stop")
    next_card = state.get("next_execution_card")
    if mode == "fast" and next_card != row:
        api.fail(f"{row} fast next_execution_card drifted")
    if mode == "closeout" and next_card != "none":
        next_path = root / str(state.get("next_execution_card_path", ""))
        if not next_path.is_file() or str(next_card) not in next_path.read_text(encoding="utf-8"):
            api.fail(f"{row} selected next task is not owned by its path")
    final_rel = str(api.FINAL_PIPELINE_REL)
    for key in ("next_execution_card_path", "latest_card_path", "current_execution_design"):
        if state.get(key) != final_rel:
            api.fail(f"{row} {key} drifted")
    card_text = (root / api.FINAL_PIPELINE_REL).read_text(encoding="utf-8")
    for token in (row, cohort, "new guard=0", "new receipt=0", "fixed failure-name set unchanged"):
        if token not in card_text:
            api.fail(f"{row}/{cohort} contract is missing: {token}")
    expected = (
        ("status = fast_open", "implementation permission = true")
        if mode == "fast"
        else ("status = landed", "implementation permission = false")
    )
    for token in expected:
        if token not in card_text:
            api.fail(f"{row}/{cohort} contract is missing: {token}")
    if len(card_text.splitlines()) > 1000:
        api.fail(f"{row} final-pipeline SSOT exceeds the 1000-line hard limit")
    print(f"[{api.TAG}] row={row} cohort={cohort}")


def check_legacy_phi_candidate_retire_r0(state: dict, root: Path, api) -> None:
    """Validate the finite caller-zero legacy PHI subtree retirement."""
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} must be fast or closeout")
    if state.get("current_execution_row") != LEGACY_PHI_CANDIDATE_ROW:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} pointer row drifted")
    if state.get("current_execution_cohort") != "legacy_phi_candidate":
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} cohort drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} must clear current_design_stop")
    expected_next = LEGACY_PHI_CANDIDATE_ROW if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} next_execution_card drifted")
    manifest_rel = str(BOXSHAPE_MAINTENANCE_MANIFEST)
    if state.get("latest_card_path") != manifest_rel:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} latest card drifted")
    if state.get("current_execution_design") != manifest_rel:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} execution design drifted")
    if state.get("next_execution_card_path") != manifest_rel:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} next card path drifted")

    with (root / BOXSHAPE_MAINTENANCE_MANIFEST).open("rb") as handle:
        manifest = tomllib.load(handle)
    rows = [row for row in manifest.get("candidate", []) if row.get("id") == LEGACY_PHI_CANDIDATE_ROW]
    if len(rows) != 1:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} manifest row is missing or duplicated")
    row = rows[0]
    expected_status = "selected_fast" if mode == "fast" else "landed"
    if row.get("status") != expected_status:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} manifest status drifted")
    if row.get("implementation_permission") is not (mode == "fast"):
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} implementation permission drifted")
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or not all(isinstance(path, str) and path for path in allowed):
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} allowed_files are missing")
    base = row.get("base_head")
    if not isinstance(base, str) or not base:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} base_head is missing")
    changed = api.subprocess.run(
        ["git", "diff", "--name-only", base, "--"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    unexpected = set(changed) - set(allowed)
    if unexpected:
        api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} changed files exceed manifest: {sorted(unexpected)}")
    for rel in LEGACY_PHI_CANDIDATE_FILES:
        exists = (root / rel).is_file()
        if mode == "fast" and not exists:
            api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} selected owner is missing: {rel}")
        if mode == "closeout" and exists:
            api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} retired owner remains: {rel}")
    inventory = (root / "tools/checks/manifests/cargo_lib_red_baseline.tests.txt").read_text(encoding="utf-8")
    ssa_inventory = (root / "tools/checks/lib/resolved_binding_ssa_inventory.py").read_text(encoding="utf-8")
    if mode == "closeout":
        if "legacy_candidate_tests::" in inventory:
            api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} retired tests remain in baseline inventory")
        if any(name in ssa_inventory for name in ("legacy_candidate.rs", "legacy_candidate_cfg.rs", "legacy_candidate_tests.rs")):
            api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} SSA split inventory still names retired files")
        source = (root / "src/mir/builder/ssa/phi_input_materializer.rs").read_text(encoding="utf-8")
        if "legacy_candidate" in source:
            api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} facade still names retired modules")
        for rel, token in (
            ("src/mir/builder/ssa/phi_input_materializer/function_repair.rs", "materialize_all_phi_inputs"),
            ("src/mir/builder/ssa/phi_input_materializer/edge_rematerialization.rs", "rematerialize_for_pred"),
        ):
            if token not in (root / rel).read_text(encoding="utf-8"):
                api.fail(f"{LEGACY_PHI_CANDIDATE_ROW} replacement owner missing: {rel}")
    print(f"[{api.TAG}] row={LEGACY_PHI_CANDIDATE_ROW} mode={mode} retired={mode == 'closeout'}")


def check_method_call_handlers_policy_split_s0(
    state: dict, root: Path, api
) -> None:
    """Validate the queued, behavior-neutral method-handler owner split."""
    row = api.METHOD_CALL_HANDLERS_POLICY_SPLIT_S0_ROW
    card_path = api.METHOD_CALL_HANDLERS_POLICY_SPLIT_S0_CARD
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{row} must clear current_design_stop")
    expected_next = row if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{row} next_execution_card drifted")
    if state.get("latest_card_path") != str(card_path):
        api.fail(f"{row} latest card path drifted")
    if state.get("current_execution_design") != str(card_path):
        api.fail(f"{row} current execution design drifted")
    if state.get("next_execution_card_path") != str(card_path):
        api.fail(f"{row} next execution card path drifted")

    with (root / card_path).open("rb") as handle:
        import tomllib

        card = tomllib.load(handle)
    task = card.get(api.METHOD_CALL_HANDLERS_POLICY_SPLIT_S0_KEY)
    if not isinstance(task, dict) or task.get("task_id") != row:
        api.fail(f"{row} manifest row is missing")
    expected_status = "selected_fast" if mode == "fast" else "landed"
    if task.get("status") != expected_status:
        api.fail(f"{row} manifest status drifted")
    if task.get("implementation_permission") is not (mode == "fast"):
        api.fail(f"{row} implementation permission drifted")

    parent = root / "src/mir/builder/method_call_handlers.rs"
    child = root / "src/mir/builder/method_call_handlers/static_current_owner_policy.rs"
    if not parent.is_file() or not child.is_file():
        api.fail(f"{row} split owner is missing")
    parent_lines = sum(1 for _ in parent.open(encoding="utf-8"))
    child_lines = sum(1 for _ in child.open(encoding="utf-8"))
    if parent_lines >= 760:
        api.fail(f"{row} parent reached 760 lines: {parent_lines}")
    if child_lines >= 800:
        api.fail(f"{row} child reached 800 lines: {child_lines}")

    parent_text = parent.read_text(encoding="utf-8")
    child_text = child.read_text(encoding="utf-8")
    required_parent = "mod static_current_owner_policy;"
    required_child = "resolve_me_call_with_publication_ingress"
    if required_parent not in parent_text:
        api.fail(f"{row} parent module declaration is missing")
    if required_child not in child_text:
        api.fail(f"{row} moved policy implementation is missing")
    if "fn resolve_me_call_with_publication_ingress<Port>" in parent_text:
        api.fail(f"{row} policy implementation remains in parent")
    if child_text.count("fn resolve_me_call_with_publication_ingress<Port>") != 1:
        api.fail(f"{row} moved policy definition is not unique")

    allowed = task.get("allowed_files")
    if not isinstance(allowed, list) or not allowed:
        api.fail(f"{row} allowed_files are missing")
    base = task.get("base_head")
    if not isinstance(base, str) or not base:
        api.fail(f"{row} base_head is missing")
    changed = api.subprocess.run(
        ["git", "diff", "--name-only", base, "--"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    untracked = api.subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    unexpected = (set(changed) | set(untracked)) - set(allowed)
    if unexpected:
        api.fail(f"{row} changed files escaped allowed_files: {sorted(unexpected)}")
    print(
        f"[{api.TAG}] row={row} delegated=method-handler-policy-split "
        f"parent={parent_lines} child={child_lines}"
    )


def check_raw_root_cleanup(row: str, key: str, guard: Path, card: dict, root: Path, api) -> None:
    item = card.get(key)
    if not isinstance(item, dict) or item.get("status") not in {"selected_fast", "landed"}:
        api.fail(f"{row} manifest entry is not selected_fast or landed")
    runner = "bash" if guard.suffix == ".sh" else "python3"
    if api.subprocess.run([runner, str(root / guard)], cwd=root).returncode:
        api.fail(f"{row} delegated guard failed")


def dispatch_coreplan_varmap_reseal_row(
    row: str, state: dict, card: dict, root: Path, api
) -> None:
    """Dispatch future varmap reseal rows through one manifest-driven checker."""
    from mir_verification_quick_p0_c_guard import (
        _check_coreplan_varmap_reseal_single_site,
        _coreplan_varmap_reseal_allowed_files,
    )

    matches = [
        (key, value)
        for key, value in card.items()
        if isinstance(value, dict) and value.get("task_id") == row
    ]
    if len(matches) != 1:
        api.fail(f"CorePlan varmap reseal row must have one manifest entry: {row!r}")
    row_key, manifest_row = matches[0]
    target_paths = manifest_row.get("target_paths")
    if not isinstance(target_paths, list) or not target_paths or not all(
        isinstance(path, str) and path.strip() for path in target_paths
    ):
        api.fail(f"CorePlan varmap reseal target_paths are malformed: {row!r}")
    expected_direct_sites = manifest_row.get("expected_direct_sites")
    if not isinstance(expected_direct_sites, int) or expected_direct_sites <= 0:
        api.fail(f"CorePlan varmap reseal expected_direct_sites is malformed: {row!r}")
    expected_direct_sites_token = manifest_row.get("expected_direct_sites_token")
    label = manifest_row.get("label")
    parent_row = manifest_row.get("parent_row")
    if not all(
        isinstance(value, str) and value.strip()
        for value in (expected_direct_sites_token, label, parent_row)
    ):
        api.fail(f"CorePlan varmap reseal metadata is malformed: {row!r}")
    _check_coreplan_varmap_reseal_single_site(
        state,
        card,
        root,
        row_name=row,
        row_key=row_key,
        parent_row=parent_row,
        label=label,
        target_paths=set(target_paths),
        expected_direct_sites=expected_direct_sites,
        expected_direct_sites_token=expected_direct_sites_token,
        allowed_files=_coreplan_varmap_reseal_allowed_files(target_paths[0]),
    )
