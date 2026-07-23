#!/usr/bin/env python3
"""RAW-SOURCE0-LOWER0-ROOT0-PLAN0 design/plan guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23.md"
)
PROD = (
    ROOT / "src/mir/compiler/mod.rs",
    ROOT / "src/runtime/mirbuilder_emit.rs",
)
PLAN = ROOT / "src/mir/compiler/raw_root_plan0.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    active = 'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT0-PLAN0"' in state
    if active:
        require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-PLAN0"', "execution row")
        require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23"', "latest card")
    else:
        require(state, "RAW-SOURCE0-LOWER0-ROOT0-PLAN0 are closed", "historical closeout")
        require(task, "Status: **Closed", "task closeout status")
    for fragment in (
        "RawRootKindV1",
        "Physical root identity",
        'main        = symbol "main", arity 0',
        'condition_fn = symbol "condition_fn", arity 1',
        "complete ordered source work schedule",
        "declaration/index plan",
        "callable catalog",
        "static-table specs/plans",
        "closure source sites",
        "runtime-input snapshot",
        "current_module reads = 0",
        "process-global method-slot mutation = 0",
        "production root consumer/executor = 0",
        "retry/fallback/catch_unwind",
        "ROOT0-PLAN0",
    ):
        require(task, fragment, f"plan contract {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    compiler_mod = (ROOT / "src/mir/compiler/mod.rs").read_text()
    require(
        compiler_mod,
        "pub(in crate::mir) mod raw_root_plan0;",
        "source-only root plan registration",
    )
    plan = PLAN.read_text()
    for fragment in (
        "RawPhysicalRootIdentityV1",
        "RawRootEnvironmentPlanV1",
        "RawRootKindV1",
        "script_plan_seals_physical_identity_and_schedule",
        "app_plan_keeps_source_arity_separate_from_physical_root",
    ):
        require(plan, fragment, f"implementation {fragment}")
    for fragment in (
        "begin_raw_draft(",
        "ModuleBuilderInvocationSessionV1",
        "RawExpansionReceiptLedgerV1",
        "capture_main(",
        "complete_root(",
        "finalize_module(",
        "reserve_method_slot(",
        "get_or_assign_type_id(",
    ):
        if fragment in plan:
            raise AssertionError(f"forbidden PLAN0 effect/reference: {fragment}")
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during ROOT0-PLAN0")
    if "begin_raw_draft(" in joined:
        callers = [
            path.relative_to(ROOT)
            for path in PROD
            if "begin_raw_draft(" in path.read_text() and path.name != "mod.rs"
        ]
        if callers:
            raise AssertionError(f"unexpected Raw draft consumer: {callers}")
    for path in (TASK, PLAN, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    mode = "active" if active else "closed"
    print(
        "[cut0-i0-root0-raw-source0-lower-root-plan0-guard] ok "
        f"mode={mode} plan=1 raw_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
