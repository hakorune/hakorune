"""Small helper owners for the stable D1B active-surface dispatcher."""

from __future__ import annotations

from pathlib import Path
import tomllib


BOXSHAPE_MAINTENANCE_ROW = "MIRBUILDER-BOXSHAPE-MAINTENANCE-T0"
BOXSHAPE_MAINTENANCE_MANIFEST = Path(
    "docs/development/current/main/investigations/"
    "mirbuilder-cleanup-retirement1-next-edge-census-2026-08-25.toml"
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
