#!/usr/bin/env python3
"""CUT0-I0-ROOT0-RAW0 disconnected retained-root-witness guard."""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
BRIEF = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-design-stop-2026-07-22.md"
)
ROOT_FILE = SRC / "raw_root_completion.rs"
BODY = SRC / "root_body_completion.rs"
ROOT_BATCH = SRC / "module_draft_collector/root_batch.rs"
LEDGER = SRC / "raw_expansion_receipt_ledger.rs"
BUILDER = SRC.parent / "builder.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    brief = BRIEF.read_text()
    root = ROOT_FILE.read_text()
    body = BODY.read_text()
    root_batch = ROOT_BATCH.read_text()
    ledger = LEDGER.read_text()
    builder = BUILDER.read_text()

    for path in (ROOT_FILE, BODY, ROOT_BATCH, LEDGER, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"RAW0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-ROOT0-RAW0 is closed", "state closeout")
    if not any(
        marker in state
        for marker in ("ROOT0-CANON0 is next", "ROOT-RETENTION0-PREFLIGHT", "ROOT-RETENTION0-COMMIT")
    ):
        raise AssertionError("missing successor row: ROOT0-CANON0 or ROOT-RETENTION0")
    require(task, "Status: **closed — Candidate A implemented", "task closeout")
    require(brief, "ROOT0-RAW0 Candidate A is closed", "brief closeout")

    for fragment, label in (
        ("RawInvocationRootWitnessV1", "root witness"),
        ("RawCompleteInvocationV1", "raw complete product"),
        ("complete_raw_root", "raw root terminal"),
        ("CompletedRootBodyV1", "retained root body"),
        ("complete_required_root_batch", "atomic ledger root batch"),
        ("SelectedCallableMainMissing", "selected callable failure boundary"),
    ):
        require(root, fragment, label)

    require(body, "new_for_brand", "brand-bound root tracker")
    require(root_batch, "root_body: CompletedRootBodyV1", "collector root retention")
    require(root_batch, "commit_branded", "collector-issued branded receipts")
    require(ledger, "collector_brand() != Some(self.brand)", "receipt provenance check")
    require(builder, "mod raw_root_completion;", "RAW0 module registration")

    if "PendingMainDraftV1::into_draft" in root:
        raise AssertionError("RAW0 must not use the root-body erasure seam")
    if "ConditionFnPolicyV1::Optional" in root:
        raise AssertionError("RAW0 must not expose Optional condition policy")

    for path in SRC.glob("*.rs"):
        text = path.read_text()
        production_text = text.split("#[cfg(test)]", 1)[0]
        if "complete_raw_root(" in production_text and path != ROOT_FILE:
            raise AssertionError(f"RAW0 production consumer must remain zero: {path}")

    print("[cut0-i0-root0-raw0-guard] ok retained_root=1 atomic_root_batch=1 production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
