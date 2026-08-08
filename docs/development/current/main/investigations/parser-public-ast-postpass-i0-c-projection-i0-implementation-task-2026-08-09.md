Status: closed
Date: 2026-08-09
Decision: implement the one-way I0-C projection from the shared decision set
Parent: `parser-public-ast-postpass-i0-c-projection-d0-design-task-2026-08-09.md`

# PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-I0

## Scope

Implement one parser-private projection walker for all postpass-visible AST
`BuildGate` rows. The walker consumes the already-issued
`PreparedBuildGateDecisionSetV1` by borrow, never calls
`eval_build_predicate`, and returns one aggregate containing the pruned AST,
validated top-level selection receipts, and optional v0 explain output.
Retained Box paths are derived from the prepared source seals immediately after
the receipt aggregate is validated; the projection never derives identity from
post-prune AST ordinals.

This is a parser postpass slice only. It does not open resolver, Recipe,
Builder, MIR, runtime, member-level gate semantics, grammar evidence, or
production cutover.

## Required implementation

Create dedicated parser modules (do not append to `src/parser/mod.rs` or
`src/parser/source_seal.rs`):

```text
src/parser/build_cfg/projection.rs
src/parser/build_cfg/projection_tests.rs
```

The projection cursor must verify every consumed row's:

```text
parser invocation brand
private preorder coordinate
predicate and span
optional parser-issued gate id/path
```

It must traverse selected and unselected branches for complete row coverage,
emit only the selected branch, and preserve the existing AST container shape.
The old `source_gate_prune::GateCursor`, generic `prune_build_gate_program`,
and `explain_build_gate_program` must not remain on the shared postpass path.

Strengthen `BuildGateSelectionReceiptV1` with predicate and coordinate
relation. Source-session validation must reject missing, duplicate, foreign, or
mismatched record/row/receipt combinations before retaining prepared seals.

Route the public explain API through the shared postpass coordinator with
`ExplainDemandV1::Capture` and add a consuming `CompletedParserPostpassV1`
projection for `(ASTNode, BuildGateExplainReport)`. The v0 counters count only
reachable rows; inactive rows remain available for coverage and diagnostics.

## Tests and evidence

Positive:

```text
top-level then/else/no-else gates
nested statement-level gates
mixed AST containers (function, loop, scope, lambda, try/catch)
explain parity with the existing v0 report
source-sealed ordinary Box path retention
compatibility cohort AST transport
```

Negative:

```text
unknown feature/unsupported key in an inactive subtree
row count mismatch
predicate/span mismatch
foreign brand
duplicate or missing source receipt
gate-id/path mismatch
second evaluator call or old postpass consumer path
```

The existing `parser_build_cfg_gate` 12-case gate must stay green. The known
nested member-gate source-path baseline remains isolated and must not be
repaired by weakening its signature rule. Add a static guard that the shared
projection path contains no `eval_build_predicate` call and no old generic
prune/explain call.

## Non-claims

```text
no member-level gate redesign
no grammar-evidence change
no resolver/source seal semantic expansion
no Recipe/Builder/MIR/runtime work
no fallback/retry/reparse/name reconstruction
no production selection or legacy deletion
```

## Closeout

Implementation receipt (2026-08-09): the shared projection is landed. One
borrowed `PreparedBuildGateDecisionSetV1` drives a single structural walker;
the walker emits the pruned AST, validated top-level source receipts, and the
reachable-row explain report. The public explain entry now consumes the same
postpass coordinator. `source_seal.rs` no longer calls the old source-gate
cursor or generic evaluator/prune path. The focused BuildCfg, source-seal,
postpass-envelope, and explain tests are green; the known nested member-gate
baseline remains separate. The static receipt guard is
`tools/checks/parser_public_ast_postpass_i0_c_projection_i0_guard.sh`.
The implementation is represented by `BuildGateProjectionOutputV1` and the
focused `build_cfg/projection_tests.rs` slice; the
projection contains no `eval_build_predicate` call and no old generic prune.

The implementation commit must update this task, the projection D0 card,
`src/parser/README.md`, `docs/reference/language/build-conditional-gate.md`,
`docs/reference/language/callable-contracts.md`, the postpass SSOT, CURRENT_STATE,
and the relevant guard/index in the same slice. Keep every touched Rust source
file below 800 lines; split before 760. Commit and push only with the focused
tests, BuildCfg regression gate, current-state guard, and parser postpass guard
green. Preserve and record the known parent-baseline red separately.
