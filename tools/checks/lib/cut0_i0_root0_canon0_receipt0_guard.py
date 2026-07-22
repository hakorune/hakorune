#!/usr/bin/env python3
"""Evidence guard for CUT0-I0-ROOT0-CANON0 RECEIPT0."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
PRODUCT = ROOT / "src/mir/builder/module_draft_collector/collected_product.rs"
BATCH = ROOT / "src/mir/builder/module_draft_collector/callable_batch.rs"
COMPLETION = ROOT / "src/mir/builder/canonical_root_completion.rs"
FIXTURE = ROOT / "src/mir/builder/canonical_root_completion_receipt0_p0.rs"
BUILDER = ROOT / "src/mir/builder.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
MANIFEST = (COLLECTOR, PRODUCT, BATCH, COMPLETION, FIXTURE, BUILDER, TASK, pathlib.Path(__file__))


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    collector = COLLECTOR.read_text()
    product = PRODUCT.read_text()
    batch = BATCH.read_text()
    completion = COMPLETION.read_text()
    fixture = FIXTURE.read_text()
    builder = BUILDER.read_text()
    task = TASK.read_text()
    state = STATE.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"RECEIPT0 file must remain below 800 lines: {path}")

    require(state, "CANON-FIXTURE0-20260722", "successor blocker")
    require(task, "Status: **Closed — RECEIPT0", "closed receipt task")
    require(product, "CollectedDraftAdmissionProductV1", "single receipt product")
    require(product, "collect_canonical_single", "single collector terminal")
    require(batch, "CollectedCallableCollectorBatchV1", "batch receipt product")
    require(batch, "collect_all_branded", "batch collector terminal")
    require(completion, "CanonicalSingleCollectedInvocationV1", "single collected state")
    require(completion, "CallableBatchCollectedInvocationV1", "batch collected state")
    require(completion, "collected.into_parts()", "by-value receipt consumption")
    require(completion, "receipt,\n                _seal: CanonicalSingleRootWitnessSealV1", "single root receipt retention")
    require(completion, "receipt,\n                capability,", "batch root receipt retention")
    require(builder, "mod canonical_root_completion_receipt0_p0", "focused fixture registration")
    require(fixture, "single_collection_moves_collector_and_exact_receipt_together", "single fixture")
    require(fixture, "foreign_or_duplicate_single_collection_rejects_without_prefix_mutation", "rejection fixture")
    require(fixture, "callable_batch_product_uses_collector_brand_for_the_whole_receipt", "batch fixture")

    forbidden = (
        "fn complete(\n        mut self,\n        receipt:",
        "InvocationBranded::from_source(brand, receipt",
        "receipt.clone()",
        "Arc<CallableCollectorBatchReceiptV1>",
        "Arc<CollectedDraftAdmissionReceiptV1>",
    )
    for fragment in forbidden:
        if fragment in completion or fragment in product or fragment in batch:
            raise AssertionError(f"forbidden loose/post-hoc receipt path: {fragment}")

    print(
        "[cut0-i0-root0-canon0-receipt0-guard] ok "
        "single_product=1 batch_product=1 receipt_retention=2 loose_complete_receipt=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
