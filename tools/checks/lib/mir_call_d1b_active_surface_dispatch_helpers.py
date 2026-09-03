"""Small helper owners for the stable D1B active-surface dispatcher."""

from __future__ import annotations

from pathlib import Path


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
