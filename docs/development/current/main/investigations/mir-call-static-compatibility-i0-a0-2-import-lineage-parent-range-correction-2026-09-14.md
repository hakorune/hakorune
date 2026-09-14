---
Status: closed__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A0-2-IMPORT-LINEAGE-PARENT-RANGE-CORRECTION
Date: 2026-09-14
Priority: correct nested import parent identity and preserve original source ranges
Parent: mir-call-static-compatibility-i0-a0-2-2026-09-14.md
NextCard: MIR-CALL-STATIC-COMPATIBILITY-A3-PACKAGE-ADMISSION-D0
Implementation permission: true for this bounded transport correction only
---

# A0-2 import-lineage parent and source-range correction

## Six-line brief

```text
Decision: derive lineage parent links from the direct-import DFS and keep one output line for every stripped using statement.
Source authority + canonical issuer: collect_using_and_strip_with_edges and dfs_text_with_imports issue the merge-owned lineage consumed by MergedSourceLineageV1.
Non-authority: the resolver's transitive closure, compact compatibility stripping, merged-text reinspection, aliases, AST/MIR reconstruction, and post-hoc parent inference.
Fail-fast boundary: the merged lineage is rejected before parser admission when segment identity, edge identity, or source coverage is inconsistent.
Smallest next slice: nested fixture evidence covering root→prelude→nested parents and original local line counts.
Non-claims: no A3 package admission, MixedProgram publication, caller cutover, legacy deletion, StringBox repair, Windows proof, or missing/foreign-lineage consumer validation.
```

## Findings and bounded correction

The original A0-2 transport re-scanned a resolver-provided transitive closure
with the root as the parent for every path. That made an import such as
`root -> A -> B` record `B.parent = root` when `B` was first encountered through
the closure. The merge owner now walks the direct imports with
`dfs_text_with_imports`; the recursive call supplies the canonical importing
file, so the first visit fixes both DFS order and parent identity. Resolver
closure entries are retained only as unseen compatibility observers and cannot
replace the direct traversal's parent relation.

The lineage-aware strip path now preserves a blank output line for every
removed `using` statement. Compact compatibility stripping remains unchanged.
Consequently each segment's local range continues to describe the original
file: a `using` on line 1 does not move a declaration on line 2 to line 1.

## Evidence

Focused command (one Cargo process, four build jobs):

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick -j4 --lib runner::modes::common_util::source_hint::normal_tests::normal_preparation_preserves_local_with_and_without_prelude -- --exact --nocapture
```

Result: one test passed, zero failed. The fixture asserts three segments and
two edges, `prelude.parent = root`, `nested.parent = prelude`, and original
local line counts of 7 for the prelude and 4 for the root. `git diff --check`
also passes. Changed source remains below the 760-line design boundary
(`using.rs` is 738 lines; `merge.rs` is 479 lines).

The read-only worker audit selected the same bounded slice: direct-edge DFS
owns parent/order, lineage-aware strip owns original line preservation, and
the resolver closure is non-authoritative. No Cargo command or source edit was
delegated to the worker.

## Limits and next step

This correction does not implement the missing/foreign-lineage validation
described by the parent A0-2 brief; the product still has no production
consumer beyond parser attachment. It also does not select or implement A3
package admission. Once this card is committed and the pointer is synchronized,
A3 remains the next design-only worker slice.
