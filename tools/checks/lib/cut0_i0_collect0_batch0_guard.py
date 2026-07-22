#!/usr/bin/env python3
"""CUT0-I0-COLLECT0-BATCH0 disconnected callable-batch guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
BUILDER = ROOT / "src/mir/builder.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
BATCH = ROOT / "src/mir/builder/module_draft_collector/callable_batch.rs"
SOURCE = ROOT / "src/mir/builder/module_invocation_callable_batch.rs"
TRANSACTION = ROOT / (
    "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
)
FIXTURE = ROOT / (
    "src/mir/builder/resolved_lowering/callable_batch_collection_p0.rs"
)
SHELL = ROOT / "src/mir/builder/module_lowering_shell.rs"
SRC = ROOT / "src"

ALLOWED = {
    BUILDER.relative_to(ROOT),
    COLLECTOR.relative_to(ROOT),
    BATCH.relative_to(ROOT),
    SOURCE.relative_to(ROOT),
    TRANSACTION.relative_to(ROOT),
    FIXTURE.relative_to(ROOT),
    SHELL.relative_to(ROOT),
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    builder = BUILDER.read_text()
    collector = COLLECTOR.read_text()
    batch = BATCH.read_text()
    source = SOURCE.read_text()
    transaction = TRANSACTION.read_text()
    fixture = FIXTURE.read_text()
    shell = SHELL.read_text()

    for path in (*[pathlib.Path(p) for p in ALLOWED], pathlib.Path(__file__)):
        if len((ROOT / path).read_text().splitlines()) >= 800:
            raise AssertionError(f"BATCH0 file must remain below 800 lines: {path}")

    require(
        state,
        "CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof",
        "S0 closeout",
    )
    require(
        state,
        "CUT0-I0-COLLECT0-BATCH0 is closed as a disconnected atomic callable-batch proof",
        "BATCH0 closeout",
    )
    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "session closeout")
    require(state, "CUT0-I0-ROOT0-BRAND0 is closed as a disconnected real branded physical-owner proof", "ROOT0 brand successor")
    require(task, "### CUT0-I0-COLLECT0-BATCH0 — closed", "task row")
    require(task, "late collision -> collector delta = 0", "atomic acceptance")
    require(task, "recursive capability preserved exactly once", "recursive acceptance")
    require(task, "CUT0-I0-SESSION0", "successor task")

    require(builder, "mod module_invocation_callable_batch;", "source registration")
    require(collector, "mod callable_batch;", "collector registration")
    require(collector, "CallableCollectorBatchReceiptV1", "collector re-export")
    for fragment, label in (
        ("prepare_callable_batch", "whole-batch preflight"),
        ("collect_all", "infallible collect terminal"),
        ("DuplicateBatchKey", "batch duplicate key"),
        ("DuplicateBatchSymbol", "batch duplicate symbol"),
        ("CanonicalRejectDuplicate", "canonical policy fixed"),
    ):
        require(batch, fragment, label)
    for fragment, label in (
        ("CallableBatchSourceProofV1", "source authority"),
        ("seal_callable_batch", "callable co-seal"),
        ("CanonicalRecursiveCallableModuleCapabilityV1", "recursive capability"),
        ("CallableBatchShellFactV1", "shell fact"),
        ("CanonicalCallableKeyV1", "catalog key projection"),
        ("SourcePlanMismatch", "foreign source-plan rejection"),
    ):
        require(source, fragment, label)
    for fragment, label in (
        ("prepare_collector_batch", "verified owner bridge"),
        ("PreparedCallableCollectorInvocationV1", "source-retaining owner"),
        ("CallableCollectorDraftEntryV1", "typed collector row"),
        ("source: &'a VerifiedResolvedCallableModuleV1", "source retained"),
        ("publish_into", "legacy publication remains present"),
    ):
        require(transaction, fragment, label)
    require(
        shell,
        "install_callable_batch_shell_fact_for_test",
        "disconnected shell capability terminal",
    )
    for fragment, label in (
        ("exact_catalog_batch_co_seals_one_physical_receipt", "success fixture"),
        ("late_collector_collision_rejects_without_delta", "late collision fixture"),
        ("recursive_batch_preserves_one_shell_capability_marker", "recursive fixture"),
        ("foreign_callable_brand_fails_before_co_seal", "foreign brand fixture"),
        ("callable_family_cannot_pair_with_a_foreign_verified_source_plan", "foreign source fixture"),
    ):
        require(fixture, fragment, label)

    forbidden = (
        "prepare_collector_batch(",
        "seal_callable_batch(",
        "install_callable_batch_shell_fact_for_test",
        "CallableBatchSourceProofV1",
    )
    consumers = []
    for path in SRC.rglob("*.rs"):
        if path.relative_to(ROOT) in ALLOWED:
            continue
        text = path.read_text()
        for fragment in forbidden:
            if fragment in text:
                consumers.append(f"{path.relative_to(ROOT)}:{fragment}")
    if consumers:
        raise AssertionError("BATCH0 production consumers: " + ", ".join(consumers))

    if transaction.count("fn publish_into(") != 1:
        raise AssertionError("existing callable publish_into owner must remain singular")

    print(
        "[cut0-i0-collect0-batch0-guard] ok "
        "whole_preflight=1 collect_all=1 recursive_marker=1 production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
