"""Fail-closed guard for the policy retirement of optional method rewrites.

The retired Known/Unique/equals writers are intentionally removed as one
bounded source -> canonical Method(Some) slice.  This checker owns no new
registry row; the stable active-surface guard dispatches it only while this
row is selected.
"""

from __future__ import annotations

from pathlib import Path


ROW = "MIR-CALL-SAME-MODULE-REWRITE-KNOWN-POLICY-RETIRE-I0"
KEY = "same_module_rewrite_known_policy_retire_i0_2026_08_30"
PARENT_KEY = "same_module_all_producer_disposition_r0_2026_08_30"
D0_KEY = "same_module_rewrite_known_issuer_boundary_d0_2026_08_30"

KNOWN_REL = Path("src/mir/builder/rewrite/known.rs")
HEADER_REL = Path("src/mir/builder/rewrite/header_lookup.rs")
SPECIAL_REL = Path("src/mir/builder/rewrite/special.rs")
MOD_REL = Path("src/mir/builder/rewrite/mod.rs")
README_REL = Path("src/mir/builder/rewrite/README.md")
EMITTER_REL = Path("src/mir/builder/calls/unified_emitter.rs")
TERMINAL_REL = Path("src/mir/builder/calls/unified_emitter/physical_terminal.rs")
TESTS_REL = Path("src/mir/builder/calls/unified_emitter/physical_receipt_tests.rs")
FLAGS_REL = Path("src/config/env/builder_flags.rs")
B1_GUARD_REL = Path("tools/checks/lib/mir_call_global_target_b1_cutover_guard.py")
GUARD_REL = Path("tools/checks/lib/mir_call_d1b_rewrite_known_policy_guard.py")

LIVE_PIN_ROOTS = (
    Path("tools/selfhost"),
    Path("tools/smokes/v2/profiles/integration"),
    Path("tools/smokes/v2/profiles/quick"),
)
RETIRED_ENV_NAMES = (
    "NYASH_REWRITE_KNOWN_DEFAULT",
    "NYASH_BUILDER_REWRITE_INSTANCE",
    "NYASH_DEV_REWRITE_USERBOX",
    "NYASH_DEV_REWRITE_NEW_ORIGIN",
)
RETIRED_SOURCE_TOKENS = (
    "try_known_rewrite",
    "try_unique_suffix_rewrite",
    "try_known_or_unique",
    "try_special_equals",
    "SpecialEqualsRewrite",
    "KnownOrUniqueRewrite",
    "builder_rewrite_known_default",
    "builder_rewrite_instance_mode",
    "builder_dev_rewrite_userbox",
    "builder_dev_rewrite_new_origin",
)


def _fail(message: str) -> None:
    raise SystemExit(f"[mir-call-d1b-active-surface] rewrite-known policy-retire I0: {message}")


def _text(root: Path, relative: Path) -> str:
    path = root / relative
    if not path.is_file():
        _fail(f"missing owner: {relative}")
    return path.read_text(encoding="utf-8")


def _source_paths(root: Path) -> list[Path]:
    return sorted(
        path
        for prefix in (root / "src", root / "crates")
        if prefix.is_dir()
        for path in prefix.rglob("*.rs")
        if path.is_file()
    )


def _live_pin_paths(root: Path) -> list[Path]:
    paths: list[Path] = []
    for relative in LIVE_PIN_ROOTS:
        base = root / relative
        if base.is_dir():
            paths.extend(path for path in base.rglob("*.sh") if path.is_file())
    return sorted(paths)


def _check_no_retired_source_tokens(root: Path) -> None:
    for path in _source_paths(root):
        text = path.read_text(encoding="utf-8")
        for token in RETIRED_SOURCE_TOKENS:
            if token in text:
                _fail(f"retired source token remains: {path.relative_to(root)}::{token}")


def _check_no_retired_env_pins(root: Path) -> None:
    for path in _live_pin_paths(root):
        text = path.read_text(encoding="utf-8")
        for token in RETIRED_ENV_NAMES:
            if token in text:
                _fail(f"retired live env pin remains: {path.relative_to(root)}::{token}")


def _check_shape(root: Path) -> None:
    if (root / KNOWN_REL).exists():
        _fail("known.rs must be physically retired in this row")
    if (root / HEADER_REL).exists():
        _fail("header_lookup.rs must be physically retired in this row")

    special = _text(root, SPECIAL_REL)
    module = _text(root, MOD_REL)
    emitter = _text(root, EMITTER_REL)
    terminal = _text(root, TERMINAL_REL)
    tests = _text(root, TESTS_REL)
    flags = _text(root, FLAGS_REL)
    readme = _text(root, README_REL)
    b1_guard = _text(root, B1_GUARD_REL)

    if "try_early_str_like_to_dst" not in special:
        _fail("early str-like owner disappeared")
    if "pub mod special;" not in module:
        _fail("rewrite module no longer exposes the retained special owner")
    for token in ("pub mod known;", "pub mod header_lookup;"):
        if token in module:
            _fail(f"retired module export remains: {token}")
    if "try_early_str_like_to_dst" not in emitter:
        _fail("unified emitter lost the retained early str-like ingress")
    for token in ("SpecialEqualsRewrite", "KnownOrUniqueRewrite"):
        if token in emitter or token in terminal:
            _fail(f"retired alternate label remains: {token}")
    for token in ("rewrite_retire_user_method", "rewrite_retire_equals"):
        if token not in tests:
            _fail(f"focused canonical Method(Some) witness is missing: {token}")
    for token in RETIRED_SOURCE_TOKENS:
        if token in flags:
            _fail(f"retired selector reader remains in builder_flags.rs: {token}")
    if "caller-zero" not in readme or "Method(Some(receiver))" not in readme:
        _fail("rewrite README does not record the retirement boundary")
    if "src/mir/builder/rewrite/known.rs" in b1_guard:
        # The B1 guard may mention the retired path only as a historical
        # baseline, but it must not count it as a live adapter user.
        if "COMPATIBILITY_ADAPTER_USERS" in b1_guard and "Path(\"src/mir/builder/rewrite/known.rs\")" in b1_guard:
            _fail("B1 adapter census still treats known.rs as a live user")

    for relative in (SPECIAL_REL, MOD_REL, EMITTER_REL, TERMINAL_REL, TESTS_REL, FLAGS_REL, GUARD_REL):
        if len(_text(root, relative).splitlines()) >= 760:
            _fail(f"source/guard reached the 760-line split boundary: {relative}")


def check_rewrite_known_policy_retire_i0(
    state: dict, card: dict, root: Path, parent_api=None
) -> None:
    if parent_api is None:
        import mir_call_d1b_active_surface_guard as parent_api

    if state.get("work_mode") not in {"fast", "closeout"}:
        _fail("requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        _fail("row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        _fail("current_design_stop must be none")
    if state.get("next_execution_card") != ROW:
        _fail("execution pointer drifted")
    if state.get("next_execution_card_path") != str(parent_api.CARD_REL):
        _fail("execution card path drifted")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict) or parent.get("status") != "accepted_with_open_blockers":
        _fail("SameModule parent census is not blocker-open")
    d0 = card.get(D0_KEY)
    if not isinstance(d0, dict) or d0.get("status") != "accepted_policy_retirement":
        _fail("rewrite/known policy decision is not accepted")
    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        _fail("active policy-retirement row is missing")
    if row.get("status") not in {"fast_open", "landed"}:
        _fail("row status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        _fail("row permission/status drifted")

    expected_allowed = {
        str(KNOWN_REL),
        str(HEADER_REL),
        str(SPECIAL_REL),
        str(MOD_REL),
        str(README_REL),
        str(EMITTER_REL),
        str(TERMINAL_REL),
        str(TESTS_REL),
        str(FLAGS_REL),
        str(B1_GUARD_REL),
        str(GUARD_REL),
        str(parent_api.HELPER_REL),
        str(parent_api.STATE_REL),
        str(parent_api.CARD_REL),
        "tools/selfhost/proof/selfhost_smoke.sh",
        "tools/smokes/v2/profiles/integration/json/json_query_vm_llvm.sh",
        "tools/smokes/v2/profiles/integration/selfhost/selfhost_mir_min_vm.sh",
        "tools/smokes/v2/profiles/integration/selfhost/selfhost_mir_m2_eq_false_vm.sh",
        "tools/smokes/v2/profiles/quick/core/json_query_min_vm.sh",
        "docs/design/instance-dispatch-and-birth.md",
        "docs/design/using-and-dispatch.md",
        "docs/development/builder/unified-method-resolution.md",
        "docs/development/builder/BOXES.md",
        "docs/development/selfhosting/quickstart.md",
        "docs/reference/language/quick-reference.md",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    declared = row.get("allowed_files")
    if not isinstance(declared, list) or set(declared) != expected_allowed:
        _fail("allowed-file boundary drifted")

    _check_shape(root)
    _check_no_retired_source_tokens(root)
    _check_no_retired_env_pins(root)

    if row.get("status") == "landed":
        base = parent_api.require_text(row.get("coverage_base_commit"), "rewrite policy coverage_base_commit")
        changed_paths = parent_api.git_diff_paths(root, base)
        if not changed_paths.issubset(expected_allowed):
            _fail(f"changed paths escaped: {sorted(changed_paths - expected_allowed)}")
        parent_api.check_test_coverage(root, row)
