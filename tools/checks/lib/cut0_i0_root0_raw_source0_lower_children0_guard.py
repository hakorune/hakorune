#!/usr/bin/env python3
"""CHILDREN0-S0 implementation boundary guard."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
SOURCE = (
    ROOT / "src/mir/compiler/raw_root_children.rs",
    ROOT / "src/mir/builder/raw_root_child_work.rs",
    ROOT / "src/mir/builder/raw_root_physical.rs",
    ROOT / "src/mir/builder/raw_root_physical/child_terminal.rs",
    ROOT / "src/mir/builder/module_invocation_brand0.rs",
    ROOT / "src/mir/builder/module_lowering_invocation.rs",
    ROOT / "src/mir/builder/module_draft_collector.rs",
    ROOT / "src/mir/compiler/raw_root_plan0.rs",
    ROOT / "src/mir/compiler/raw_root_eligibility.rs",
)


def main() -> int:
    state = STATE.read_text()
    expected = "RAW-SOURCE0-LOWER0-ROOT0-CHILDREN0-S0"
    if f'current_execution_row = "{expected}"' not in state:
        raise AssertionError("active row moved beyond CHILDREN0-S0")
    for path in SOURCE:
        if not path.exists():
            raise AssertionError(f"missing CHILDREN0 source: {path}")
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"source file is at/over 800 lines: {path}")
    joined = "\n".join(path.read_text() for path in SOURCE[:4])
    for forbidden in (
        "ModuleLoweringInvocationStateV1::capture_main",
        "ModuleLoweringInvocationStateV1::complete_root",
        "with_shell_collector",
        "sorted_method_entries",
        "catch_unwind",
        "execute_preflighted_module_invocation",
    ):
        if forbidden in joined:
            raise AssertionError(f"forbidden CHILDREN0 widening: {forbidden}")
    children = (ROOT / "src/mir/compiler/raw_root_children.rs").read_text()
    for required in (
        "prepare_children",
        "complete_all",
        "LexicalMethodName",
        "RawPreRootChildrenCompletionV1",
        "RawRootChildFailureSiteV1",
        "successful_prefix_count",
        "locator_drift_is_rejected_before_physical_effects",
        "second_child_failure_keeps_successful_prefix_and_stops_siblings",
    ):
        if required not in children:
            raise AssertionError(f"missing CHILDREN0 contract: {required}")
    if "typed_child_causes_map_to_existing_coarse_abort_reasons" not in joined:
        raise AssertionError("missing CHILDREN0 coarse abort mapping fixture")
    print("[cut0-i0-root0-raw-source0-lower-children0-guard] ok below_800=1 production_consumer=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
