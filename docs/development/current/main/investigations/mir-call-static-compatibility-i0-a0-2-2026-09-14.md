---
Status: selected__closeout__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A0-2
Date: 2026-09-14
Priority: typed merged-source lineage transport and parser-brand co-seal
Parent: mir-call-static-compatibility-catalog-target-d0-2026-09-14.md
NextCard: none__a1_static_parent_co_seal_d0
Implementation permission: true for this transport responsibility only
---

# Static compatibility A0-2 typed import-lineage transport I0

## Six-line brief

```text
Decision: issue merged-source segment/import lineage once from the existing merge owner and co-seal it to one parser invocation.
Source authority + canonical issuer: TextMergePlan/PreparedSourceWithImports emits typed lineage; the normal materializer attaches it to the parser-branded source product.
Non-authority: runtime alias HashMap, LineSpan thread-local diagnostics, AST/MIR/name/path reconstruction, compatibility labels and test-only receipts.
Fail-fast boundary: duplicate canonical path, alias rebinding/conflict, unresolved target, range gap/overlap, foreign brand or missing lineage reaches a named merge/materializer/parser terminal before semantic admission.
Smallest next slice: root plus nested import segment identity, edge, alias, DFS ordinal and global/local ranges through source_hint to one parser invocation.
Non-claims: no A1 static-parent co-seal, MixedProgram source-admission switch, fallback restoration, old static-terminal deletion, Windows proof or R7 completion.
```

## Bounded owner and handoff

1. Add a small lineage model under `src/runner/modes/common_util/resolve/strip/`
   (prefer a sibling module rather than inflating `merge.rs`). The product must
   carry root/import identity, canonical path, import edge origin, requested and
   resolved path, alias/concrete binding, global/local line ranges, DFS ordinal,
   parent relation and exactly-once coverage.
2. Extend `TextMergePlan` in `resolve/strip/merge.rs` to issue that product from
   its existing DFS. The merge owner remains the sole issuer; do not reconstruct
   rows after the merge.
3. Extend `PreparedSourceWithImports` in `source_hint.rs` to transport typed
   lineage beside `code` and the existing runtime alias map. Keep the alias map
   for lowering compatibility; it is not semantic lineage authority.
4. Thread the product through the existing normal materializer handoff in
   `normal_callable.rs` / `src/mir/modes.rs` (or the current compile-request
   transport owner) and co-seal it with the existing parser source identity,
   digest and invocation brand. Builder/MIR code must not infer it from AST,
   names, paths or aliases.

## Ordered implementation tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Define typed lineage rows and reject terminals | The schema names source identity, edge, alias, ranges, DFS order, parent and coverage without duplicating semantic declaration rows. |
| 2 | Issue from merge DFS | Root and nested imports are emitted once in deterministic DFS order; duplicate canonical paths and alias rebinding/conflict stop at the merge owner. |
| 3 | Transport through source preparation | `PreparedSourceWithImports` carries code, runtime aliases and typed lineage without changing alias-lowering behavior. |
| 4 | Co-seal at parser invocation | One parser brand/source identity owns the lineage; missing, foreign, gap or overlap products fail before source admission. |
| 5 | Focused evidence and guard | Positive root+nested order/alias/path/range/brand/exactly-once coverage passes; negative unresolved, duplicate, conflict, gap/overlap, foreign-brand and missing-lineage cases hit named terminals. |
| 6 | Closeout docs | Owner README/reference and CURRENT_STATE receipt record the production edge and any classified red; no new CI lane is added. |

## Exact retention and deletion

Retain `using_import_boxes`, the runtime alias map, `LineSpan` diagnostics and
the existing compatibility origin while their consumers remain. This I0 has no
physical deletion set. Do not touch A1 static rows, source-admission
classification, generic fallback, `legacy-fallback-retired`, or unrelated
callers. A later row may retire the alias map only after all semantic consumers
are switched and caller-zero is proven.

## Acceptance and limits

The selected source-preparation/parser path must observe one typed lineage
product with exact coverage and a single parser brand. Failures are affine:
the merge plan, lineage product and parser source sibling are consumed by a
named terminal; no AST-only or compatibility retry is introduced. Keep
`merge.rs` below the 760-line design boundary and every changed source below
the 800-line hard stop; split the lineage model before crossing either limit.

The row is ready for `fast` implementation. If the parser attach point becomes
multiple competing issuers, exact segment coverage cannot be proven by the
merge owner, or aliases/paths must be reconstructed downstream, return to
`design_stop` as `NoSafeSlice` instead of widening this task.

## Implementation checkpoint — 2026-09-14

A0-2 is implemented as one typed transport slice. The merge owner now issues
`MergedSourceLineageV1` rows from its existing DFS, including root/import edges,
canonical paths, aliases/bindings, parent links, DFS ordinal, and global/local
line ranges. `PreparedSourceWithImports` carries the product beside the retained
runtime alias map. The selected normal materializer passes it through the
parser-branded source handoff and co-seals the parser invocation witness.

Focused evidence:

- `runner::modes::common_util::source_hint::normal_tests::normal_preparation_preserves_local_with_and_without_prelude`
- `runner::modes::common_util::normal_callable::tests::merged_lineage_is_co_sealed_to_the_parser_invocation`
- `runner::modes::common_util::resolve::strip::import_lineage::tests::lineage_accepts_exact_root_and_nested_coverage`
- `runner::modes::common_util::resolve::strip::import_lineage::tests::lineage_rejects_duplicate_and_gap`
- `cargo check --profile quick -j4` and `cargo test --profile quick --lib --no-run` completed successfully.
- `git diff --check` and `tools/checks/current_state_pointer_guard.sh` are green.

The repository-wide formatter check still reports pre-existing formatting
drift outside this slice; it is recorded as baseline tooling debt rather than a
current-change failure. This checkpoint does not claim A1 static-parent
co-seal, source-admission switching, fallback restoration, legacy-edge
deletion, Windows lifecycle proof, or R7 completion.
