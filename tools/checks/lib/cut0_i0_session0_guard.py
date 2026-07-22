#!/usr/bin/env python3
"""CUT0-I0-SESSION0 disconnected Builder-transaction guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
BUILDER = ROOT / "src/mir/builder.rs"
SESSION = ROOT / "src/mir/builder/module_invocation_session.rs"
FIXTURE = ROOT / "src/mir/builder/module_invocation_session_p0.rs"
CORE = ROOT / "crates/hakorune_mir_builder/src/core_context.rs"
VALUE = ROOT / "crates/hakorune_mir_core/src/value_id.rs"
BLOCK = ROOT / "crates/hakorune_mir_core/src/basic_block_id.rs"

ALLOWED = {
    BUILDER.relative_to(ROOT),
    SESSION.relative_to(ROOT),
    FIXTURE.relative_to(ROOT),
    CORE.relative_to(ROOT),
    VALUE.relative_to(ROOT),
    BLOCK.relative_to(ROOT),
    pathlib.Path(__file__).relative_to(ROOT),
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    builder = BUILDER.read_text()
    session = SESSION.read_text()
    fixture = FIXTURE.read_text()
    core = CORE.read_text()
    value = VALUE.read_text()
    block = BLOCK.read_text()

    for path in (ROOT / p for p in ALLOWED):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"SESSION0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "state closeout")
    require(state, "CUT0-I0-ROOT0 is next", "successor pointer")
    require(task, "### CUT0-I0-SESSION0 — closed", "task closeout")
    require(task, "CUT0-I0-ROOT0", "successor task")
    require(builder, "mod module_invocation_session;", "session registration")
    require(builder, "mod module_invocation_session_p0;", "fixture registration")

    for fragment, label in (
        ("BuilderInvocationConfigV1", "config snapshot"),
        ("BuilderCoreCursorV1", "five-counter cursor"),
        ("BuilderCoreIdSeedV1", "sealed seed"),
        ("BuilderCoreSeedPolicyV1", "typed seed policy"),
        ("ContinueLive", "raw parity seed"),
        ("Fresh", "canonical seed"),
        ("using_import_boxes", "imports snapshot"),
        ("plugin_method_sigs", "plugin signature snapshot"),
        ("source_file", "explicit source snapshot"),
        ("prepare_external_commit", "commit-ready witness"),
        ("PreparedBuilderExternalCommitV1", "one-shot prepared product"),
        ("BuilderCommitReadinessErrorV1", "typed readiness error"),
        ("FunctionStateOpen", "function-state readiness error"),
    ):
        require(session, fragment, label)
    for fragment, label in (
        ("from_cursors", "core cursor install"),
        ("ValueIdGenerator::from_next_id", "value seed"),
        ("BasicBlockIdGenerator::from_next_id", "block seed"),
    ):
        require(core, fragment, label)
    require(value, "from_next_id", "value generator constructor")
    require(block, "from_next_id", "block generator constructor")
    for fragment, label in (
        ("snapshot_installs_all_explicit_builder_inputs", "config fixture"),
        ("continue_live_and_fresh_seed_all_five_core_cursors", "seed parity fixture"),
        ("dropping_failed_candidate_leaves_live_builder_unchanged", "drop invariance fixture"),
        ("commit_readiness_rejects_open_slot_state_before_external_commit", "failure witness fixture"),
        ("commit_readiness_rejects_function_owned_residue", "function residue fixture"),
        ("prepared_commit_moves_candidate_once_and_reuse_is_fresh", "one-shot commit fixture"),
    ):
        require(fixture, fragment, label)

    if "candidate.current_source_file" in session:
        raise AssertionError("candidate must not resolve source file through ambient fallback")
    if "plugin_sigs::load_plugin_method_sigs" in session:
        raise AssertionError("candidate session must not reload plugin signatures")

    forbidden = (
        "BuilderInvocationConfigV1::snapshot",
        "ModuleBuilderInvocationSessionV1::open(",
        "PreparedBuilderExternalCommitV1",
        "BuilderCoreSeedPolicyV1",
    )
    consumers = []
    for path in ROOT.glob("src/**/*.rs"):
        if path.relative_to(ROOT) in ALLOWED:
            continue
        text = path.read_text()
        for fragment in forbidden:
            if fragment in text:
                consumers.append(f"{path.relative_to(ROOT)}:{fragment}")
    if consumers:
        raise AssertionError("SESSION0 production consumers: " + ", ".join(consumers))

    print(
        "[cut0-i0-session0-guard] ok config=1 five_cursors=1 "
        "readiness=1 production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
