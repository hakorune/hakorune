"""Guard for the conservative same-basic-block MIR CSE row.

The active-surface parent imports this module only when the CSE row is the
current pointer.  The guard owns no registry entry and does not make CSE a
Call or backend migration claim.
"""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0"
KEY = "mir_cse_same_block_stats_determinism_r0_2026_09_01"

CSE_REL = Path("src/mir/passes/cse.rs")
SEMANTIC_REL = Path("src/mir/passes/semantic_simplification.rs")
OPT_DOC_REL = Path(
    "docs/development/current/main/design/current-optimization-mechanisms-ssot.md"
)
WORKSTREAM_REL = Path(
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"
)
GUARD_REL = Path("tools/checks/lib/mir_cse_same_block_guard.py")


def _read(root: Path, rel: Path) -> str:
    path = root / rel
    if not path.is_file():
        api.fail(f"CSE same-block owner is missing: {rel}")
    if rel.suffix == ".rs" and sum(1 for _ in path.open(encoding="utf-8")) >= 760:
        api.fail(f"CSE same-block owner reached the 760-line boundary: {rel}")
    return path.read_text(encoding="utf-8")


def check_cse_same_block_r0(
    state: dict, card: dict, root: Path, parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        parent_api.fail("same-block CSE row requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        parent_api.fail("same-block CSE row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        parent_api.fail("same-block CSE row must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        parent_api.fail("same-block CSE row pointer drifted")
    if state.get("next_execution_card_path") != str(parent_api.CARD_REL):
        parent_api.fail("same-block CSE row card pointer drifted")

    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        parent_api.fail("same-block CSE row is missing")
    status = row.get("status")
    if status not in {"selected_fast", "landed"}:
        parent_api.fail("same-block CSE row status is not finite")
    if row.get("implementation_permission") is not (status == "selected_fast"):
        parent_api.fail("same-block CSE permission/status drifted")

    required_allowed = {
        str(CSE_REL),
        str(SEMANTIC_REL),
        str(OPT_DOC_REL),
        str(GUARD_REL),
        str(parent_api.HELPER_REL),
        str(parent_api.STATE_REL),
        str(parent_api.CARD_REL),
        str(WORKSTREAM_REL),
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != required_allowed:
        parent_api.fail("same-block CSE allowed-file boundary drifted")

    cse = _read(root, CSE_REL)
    semantic = _read(root, SEMANTIC_REL)
    optimization_doc = _read(root, OPT_DOC_REL)
    workstream = _read(root, WORKSTREAM_REL)
    guard = _read(root, GUARD_REL)
    for token, label in (
        ("same-basic-block", "optimization documentation"),
        ("MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0", "workstream"),
        ("cse_in_function", "CSE owner"),
        ("semantic_simplification", "bundle consumer"),
    ):
        haystack = {
            "optimization documentation": optimization_doc,
            "workstream": workstream,
            "CSE owner": cse,
            "bundle consumer": semantic,
            "positive test": cse,
            "cross-block negative test": cse,
            "statistics test": cse,
            "bundle test": semantic,
            "CSE guard contract": guard,
        }[label]
        if token not in haystack:
            parent_api.fail(f"same-block CSE {label} evidence is missing: {token}")

    if status == "selected_fast":
        return

    for token, label, haystack in (
        (
            "cse_rewrites_only_duplicate_pure_instructions_in_same_block",
            "positive test",
            cse,
        ),
        (
            "cse_does_not_reuse_duplicate_pure_instructions_across_sibling_blocks",
            "cross-block negative test",
            cse,
        ),
        ("cse_counts_only_actual_copy_rewrites", "statistics test", cse),
        ("bundle_runs_landed_cse", "bundle test", semantic),
    ):
        if token not in haystack:
            parent_api.fail(f"same-block CSE {label} evidence is missing: {token}")

    loop = cse.find("for (_bid, block) in &mut function.blocks")
    expression_map = cse.find("let mut expression_map")
    if loop < 0 or expression_map < 0 or expression_map < loop:
        parent_api.fail("same-block CSE expression map is not scoped inside the block loop")
    if "let mut rewritten" not in cse or "if rewritten" not in cse:
        parent_api.fail("same-block CSE statistic is not tied to an actual rewrite")
    rewritten_at = cse.find("if rewritten")
    if "eliminated += 1" not in cse[rewritten_at:]:
        parent_api.fail("same-block CSE increments before confirming the Copy rewrite")
    for token in ("CrossBlockNoRewrite", "UnsupportedNoRewrite", "no cross-block fallback"):
        if token not in str(row.get("finite_states", [])) + str(row.get("fail_fast_boundary", "")):
            parent_api.fail(f"same-block CSE contract lost token: {token}")

    filters = parent_api.require_text_list(
        row.get("focused_test_filters"), "same-block CSE focused_test_filters"
    )
    names = parent_api.require_text_list(
        row.get("changed_test_names"), "same-block CSE changed_test_names"
    )
    listed = parent_api.cargo_test_names(root)
    for name in names:
        matches = [item for item in listed if item.endswith("::" + name)]
        if len(matches) != 1:
            parent_api.fail(f"same-block CSE test {name} is not uniquely listed by cargo")
        if not any(token in matches[0] for token in filters):
            parent_api.fail(f"same-block CSE test {name} has no focused filter")
    for token in filters:
        if not any(token in item for item in listed):
            parent_api.fail(f"same-block CSE focused filter has zero cargo-list matches: {token}")

    base = parent_api.require_text(row.get("coverage_base_commit"), "same-block CSE coverage_base_commit")
    changed_paths = parent_api.git_diff_paths(root, base)
    if not changed_paths <= set(allowed):
        parent_api.fail(
            "same-block CSE changed paths exceed allowed boundary: "
            f"{sorted(changed_paths - set(allowed))}"
        )
