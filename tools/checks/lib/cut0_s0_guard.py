#!/usr/bin/env python3
"""CUT0-S0 disconnected linear-owner guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    files = {
        name: (SRC / name).read_text()
        for name in (
            "module_lowering_invocation_state.rs",
            "module_lowering_invocation_candidate.rs",
            "module_invocation_drain.rs",
            "module_finalization_once.rs",
            "module_invocation_drain_s0_tests.rs",
            "module_finalization_once_p0.rs",
        )
    }
    for name, text in files.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"CUT0-S0 source must remain below 800 lines: {name}")

    require(files["module_lowering_invocation_state.rs"], "capture_main", "root capture transition")
    require(files["module_lowering_invocation_state.rs"], "complete_root", "root complete transition")
    require(files["module_lowering_invocation_candidate.rs"], "complete_success", "candidate success handoff")
    require(files["module_invocation_drain.rs"], "prepare_complete", "same-state drain admission")
    require(files["module_invocation_drain.rs"], "drain_candidate", "typed drain terminal")
    require(files["module_finalization_once.rs"], "finalize_drained_module_once", "post-drain finalizer")
    require(files["module_finalization_once.rs"], "Builder-free", "finalizer boundary contract")
    require(files["module_invocation_drain_s0_tests.rs"], "without_rebuilding_state", "same-state fixture")
    require(files["module_finalization_once_p0.rs"], "consume-once", "finalizer fixture")

    state = STATE.read_text()
    card = CARD.read_text()
    require(state, "CUT0-S0-OWNER0 is closed, CUT0-S0-COMPAT0 is next", "current CUT0-S0 pointer")
    require(card, "Status: **Active — CUT0-S0-OWNER0 closed; CUT0-S0-COMPAT0 next**", "active CUT0-S0 status")
    require(card, "production capture/commit = 0", "CUT0-S0 acceptance")

    production_files = [
        path
        for path in SRC.glob("*.rs")
        if not (path.name.endswith("_tests.rs") or path.name.endswith("_p0.rs") or path.name.endswith("_p0d.rs"))
        and path.name not in {"module_invocation_drain.rs", "module_finalization_once.rs", "module_lowering_invocation_candidate.rs"}
    ]
    forbidden_calls = (
        "ModuleLoweringInvocationCandidateV1::open(",
        "ModuleLoweringInvocationDrainOwnerV1::prepare_complete(",
        "finalize_drained_module_once(",
    )
    for path in production_files:
        text = path.read_text()
        for fragment in forbidden_calls:
            if fragment in text:
                raise AssertionError(f"CUT0-S0 production consumer in {path.name}: {fragment}")

    print("[cut0-s0-guard] ok disconnected production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
