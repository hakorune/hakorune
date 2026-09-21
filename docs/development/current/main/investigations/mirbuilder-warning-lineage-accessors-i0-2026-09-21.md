---
Status: closed__2026-09-21__WarningLineageAccessors__DeletedAndVerified
Task: MIRBUILDER-WARNING-LINEAGE-ACCESSORS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i114-2026-09-21.md
Implementation permission: true for the production-zero lineage accessor group
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I116
---

# Warning cleanup: redundant merged-lineage accessors

## Six-line brief

```text
Decision: remove MergedSourceLineageV1::root, segments, and edges accessors;
the existing pub(crate) fields remain the same data authority.
Source authority + canonical issuer: import_lineage.rs issue/root_only; the
lineage owner seals root, segment, and edge rows in one product.
Non-authority: warning suppression, cargo-fix, field visibility changes, or a
new lineage projection.
Fail-fast boundary: any production caller, focused lineage red, or warning
count mismatch rejects the row.
Smallest next slice: change finite tests to read existing fields, delete three
accessors, and run the lineage/source-hint focused gates.
Non-claims: no import merge semantics, diamond handling, parent relation,
source-line mapping, fallback, or LegacyCallV0 retirement.
```

## Census boundary

The bounded inventory is the three methods on `MergedSourceLineageV1`, their
unit tests in `import_lineage.rs`, and the four tracked Rust test callers in
`source_hint_normal_tests.rs`, `normal_default_root_catalog_merged_route_tests.rs`,
`normal_callable.rs`, and the lineage module tests. It excludes
`locate_global_line`, the lineage fields, merge construction, and all
source-path `segments()` methods.

## Caller census and acceptance

`rg` found no production call to `MergedSourceLineageV1::root`,
`.segments()`, or `.edges()` on a lineage value. The four tests use the
existing `pub(crate)` fields, preserving all count/order/edge assertions. The
focused gate is:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib runner::modes::common_util::resolve::strip
cargo test --profile quick --lib mir::builder::normal_default_root_catalog_merged_route_tests
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning refresh: lib **1,696 → 1,695** (the grouped accessor
warning) and lib-test **551 → 550**. No `#[allow]`, field authority, or
lineage semantics may change.

## Closeout evidence

The three redundant accessors were deleted. The four finite Rust test callers
now read the existing `pub(crate)` fields, preserving lineage counts, order,
edge, parent, and source-hint assertions. No production caller was found and
no lineage construction or semantics changed.

Focused and sequential validation:

```text
cargo test --profile quick --lib runner::modes::common_util::resolve::strip
  -> 5/5 passed
cargo test --profile quick --lib \
  mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_merged_route_tests
  -> 2/2 passed
cargo check --profile quick --lib -j4
  -> lib warnings 1,695
cargo test --profile quick --lib --no-run -j4
  -> lib-test warnings 550
cargo fmt --all -- --check                     -> pass
git diff --check                                -> pass
bash tools/checks/current_state_pointer_guard.sh -> pass
```

The old-edge lane remains `NoSafeSlice`; this warning deletion does not
authorize a legacy route or compatibility retirement. The next action is a
fresh warning baseline refresh before selecting one bounded cohort.
