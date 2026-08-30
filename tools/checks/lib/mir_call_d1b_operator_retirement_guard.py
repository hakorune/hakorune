#!/usr/bin/env python3
"""Fail-closed structural guard for the Builder operator-call retirement I0."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys
import tomllib


TAG = "mir-call-d1b-operator-retire"
ROW = "MIR-CALL-SAME-MODULE-OPERATOR-CALL-RETIRE-I0"
CARD_KEY = "same_module_operator_call_retire_i0_2026_08_30"
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
)
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")

SELECTOR_KEYS = (
    "NYASH_BUILDER_OPERATOR_BOX_ALL_CALL",
    "NYASH_BUILDER_OPERATOR_BOX_ADD_CALL",
    "NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL",
)
PUBLISHER_REL = (
    Path("src/mir/builder/ops/arithmetic.rs"),
    Path("src/mir/builder/ops/comparison.rs"),
    Path("src/mir/builder/ops/unary.rs"),
    Path("src/mir/builder/ops/mod.rs"),
)
SCRIPT_REL = (
    Path("tools/dev_env.sh"),
    Path("tools/selfhost/proof/run_stageb_compiler_vm.sh"),
    Path("tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh"),
    Path("tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh"),
)


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] {message}")


def load_toml(path: Path) -> dict:
    try:
        with path.open("rb") as stream:
            value = tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot load {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path} is not a TOML table")
    return value


def text(root: Path, rel: Path) -> str:
    path = root / rel
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        fail(f"cannot read {rel}: {exc}")


def changed_paths(root: Path, base: str) -> set[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base}..HEAD"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"cannot inspect changed paths from {base}: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line.strip()}


def check_operator_retirement_i0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("operator retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        fail("operator retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("operator retirement must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        fail("operator retirement pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("operator retirement card pointer drifted")

    row = card.get(CARD_KEY)
    if not isinstance(row, dict):
        fail(f"{CARD_KEY} section is missing")
    if row.get("task_id") != ROW:
        fail("operator retirement task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("operator retirement status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("operator retirement permission/status drifted")

    expected_allowed = {
        "src/config/env/builder_flags.rs",
        "src/mir/compiler/normal_default_pipeline.rs",
        "src/mir/compiler/raw_published_compile.rs",
        "src/mir/compiler/mod.rs",
        "src/mir/compiler/lowering_input.rs",
        "src/mir/compiler/operator_call_retirement_tests.rs",
        "src/mir/compiler/README.md",
        "src/mir/builder/ops/arithmetic.rs",
        "src/mir/builder/ops/comparison.rs",
        "src/mir/builder/ops/unary.rs",
        "src/mir/builder/ops/mod.rs",
        "src/mir/builder/ops/README.md",
        "src/runner/modes/common_util/resolve/strip/prelude.rs",
        "tools/dev_env.sh",
        "tools/selfhost/proof/run_stageb_compiler_vm.sh",
        "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh",
        "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh",
        "docs/guides/operator-boxes.md",
        "docs/reference/environment-variables.md",
        "tools/checks/lib/mir_call_d1b_operator_retirement_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "docs/development/current/main/CURRENT_STATE.toml",
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("operator retirement allowed-file boundary drifted")

    flags_rel = Path("src/config/env/builder_flags.rs")
    flags = text(root, flags_rel)
    for token in (
        "BuilderOperatorCallIngressPolicyV1",
        "BuilderOperatorCallIngressErrorV1",
        "validate_builder_operator_call_ingress_v1",
    ):
        if token not in flags:
            fail(f"operator selector owner is missing {token}")
    for getter in (
        "pub fn builder_operator_box_all_call",
        "pub fn builder_operator_box_add_call",
        "pub fn builder_operator_box_compare_call",
    ):
        if getter in flags:
            fail(f"retired Builder getter reappeared: {getter}")

    compiler = text(root, Path("src/mir/compiler/mod.rs"))
    normal = text(root, Path("src/mir/compiler/normal_default_pipeline.rs"))
    raw = text(root, Path("src/mir/compiler/raw_published_compile.rs"))
    if compiler.count("validate_builder_operator_call_ingress_once_v1()") != 3:
        fail("resolved ingress validator count is not exactly three")
    if normal.count("validate_builder_operator_call_ingress_once_v1()") != 1:
        fail("normal ingress validator count is not exactly one")
    if raw.count("validate_builder_operator_call_ingress_once_v1()") != 1:
        fail("raw-published ingress validator count is not exactly one")
    if compiler.count("fn validate_builder_operator_call_ingress_once_v1") != 1:
        fail("operator validator helper count is not exactly one")

    for rel in PUBLISHER_REL:
        source = text(root, rel)
        for token in (
            "NYASH_BUILDER_OPERATOR_BOX_",
            "builder_operator_box_",
            "Operator.apply",
            "typed_global_target_from_selected_symbol",
        ):
            if token in source:
                fail(f"retired operator publisher token remains in {rel}: {token}")

    prelude = text(root, Path("src/runner/modes/common_util/resolve/strip/prelude.rs"))
    if 'env_bool("NYASH_OPERATOR_BOX_ALL")' not in prelude:
        fail("runtime operator prelude selector disappeared")
    if "NYASH_BUILDER_OPERATOR_BOX_ALL_CALL" in prelude:
        fail("Builder selector still drives runtime prelude injection")

    for rel in SCRIPT_REL:
        if any(key in text(root, rel) for key in SELECTOR_KEYS):
            fail(f"repo-owned Builder selector writer remains in {rel}")

    for rel in (
        flags_rel,
        Path("src/mir/compiler/mod.rs"),
        Path("src/mir/compiler/normal_default_pipeline.rs"),
        Path("src/mir/compiler/raw_published_compile.rs"),
        Path("src/mir/compiler/lowering_input.rs"),
        *PUBLISHER_REL,
        Path("src/runner/modes/common_util/resolve/strip/prelude.rs"),
        Path("tools/checks/lib/mir_call_d1b_operator_retirement_guard.py"),
    ):
        if len(text(root, rel).splitlines()) >= 800:
            fail(f"operator retirement owner reached the 800-line hard stop: {rel}")

    if row.get("status") == "landed":
        base = row.get("coverage_base_commit")
        if not isinstance(base, str) or not base.strip():
            fail("landed operator retirement lacks coverage_base_commit")
        unexpected = changed_paths(root, base) - expected_allowed
        if unexpected:
            fail(f"operator retirement changed paths exceed boundary: {sorted(unexpected)}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: mir_call_d1b_operator_retirement_guard.py ROOT")
    root = Path(sys.argv[1]).resolve()
    check_operator_retirement_i0(
        load_toml(root / STATE_REL),
        load_toml(root / CARD_REL),
        root,
    )
    print(f"[{TAG}] row={ROW} ok")


if __name__ == "__main__":
    main()
