Status: closed design; I0 implementation landed
Date: 2026-08-09
Decision: design the one-way I0-C projection from the parser-private decision set
Parent: `parser-public-ast-postpass-i0-c-design-task-2026-08-09.md`

# PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-D0

## Purpose

Define the next bounded projection before changing any BuildGate consumer.
`PreparedBuildGateDecisionSetV1` is now the parser-private decision authority;
this row only fixes how existing prune, top-level source-path survival, and
explain reporting consume it without re-evaluating predicates or rebuilding
source identity.

## Required design

The projection must specify, in one document and one future implementation
slice:

```text
decision-row identity and coverage
selected-branch projection for AST prune
reachable-row projection for the v0 explain counters
inactive-row retention for diagnostics
top-level SourceBuildGatePathV1 survival/rebase relation
predicate/brand/gate-id verification at every projection boundary
ownership and one-way consumption of the non-Clone decision set
```

The public `parse`, metadata, and explain wrappers must continue to consume
one `CompletedParserPostpassV1`. The projection may not add a second AST walk,
call `eval_build_predicate`, scan names/ordinals, or recreate a source path.

## Non-claims

```text
no production/public cutover in this design row
no member-level gate signature change
no grammar-evidence demand change
no resolver/Recipe/Builder/MIR/runtime work
no fallback/retry/reparse
no explain counter semantic change
```

## Acceptance criteria for the future I0-C projection slice

```text
one decision set enters the projection owner
all AST BuildGate rows are consumed exactly once
inactive rows remain available for coverage/diagnostics
unknown feature/unsupported predicate remains fail-fast
prune/source-path/explain outputs agree on branch decisions
existing 12-case BuildCfg gate remains green
known nested member-gate baseline remains isolated
all touched source files stay below 800 lines
reference/task/README/guard update lands in the same implementation commit
```

Until this design is implemented, the existing consumers remain unchanged and
the current work mode is `design_stop`.

Implementation receipt (2026-08-09): `PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-I0`
closed this boundary. The decision set is consumed by one projection walker;
prune, source-path survival, and explain capture now share its rows. The old
shared evaluator/cursor path is no longer used. The implementation keeps
member-level gates and grammar evidence outside this row.

## Accepted projection architecture

The decision set is the sole predicate truth, but it is not moved separately
into three consumers. `OpenParserPostpassProductV1` remains its non-Clone owner
and a private projection walker borrows its rows for one structural traversal.
The walker emits one aggregate:

```text
BuildGateProjectionOutputV1
  pruned AST
  top-level BuildGateSelectionReceiptV1[]
  optional v0 BuildGateExplainReport
  complete-consumption receipt
```

The aggregate is consumed by the existing source-session prune. Retained Box
paths are then read from the prepared source seals, which remain the source
identity authority; the projection never derives them from final AST ordinals.
The existing delegate/final-seal path then consumes the same prepared source
session. The decision set is dropped only after all
requested projections have succeeded; no consumer clones it or re-evaluates a
predicate.

### Single structural walker

`build_cfg/projection.rs` (new dedicated module) owns the future walker. It
visits AST `BuildGate` nodes in the same private preorder coordinate used by
the issuer and verifies, at every row:

```text
invocation brand
coordinate
predicate and span
optional parser-issued gate id/path
```

Both selected and unselected branches are traversed for row coverage. Only the
selected branch is emitted into the output AST. An inactive nested gate is
therefore consumed as a row without being evaluated or emitted. Non-gate AST
containers are rebuilt by the same walker; the current generic prune walk and
the source-gate `GateCursor` are not run afterward.

AST child ordinals may be used only as local traversal coordinates. They never
issue a source identity, gate id, or resolver identity. Top-level source
identity comes from the parser-issued `SourceBuildGatePathV1` and the existing
source-session records.

### Selection receipt strengthening

`BuildGateSelectionReceiptV1` must carry the decision row's predicate (and its
private coordinate) in addition to brand, gate id/path, and selected branch.
Source-session validation then checks:

```text
record ↔ decision row ↔ receipt
  same brand
  same gate id/path
  same predicate
  unique coordinate
```

The receipt is a projection receipt, not a new evaluator or source authority.
The existing `ParserSourceSessionV1::prepare_prune` remains the owner of
retaining prepared source seals; it receives validated receipts and does not
inspect AST names or ordinals.

### Explain compatibility projection

The v0 report is derived from reachable decision rows, not from the pruned AST:

```text
conditional_group_count = reachable gate rows
active_branch_count     = one for each reachable gate row
inactive_branch_count   = selected false/no-else, or the unselected else arm
```

Inactive rows remain in the decision set for complete coverage and diagnostics
but do not increment the v0 counters. The source predicate is never called by
the report builder.

The public explain wrapper must enter the same postpass coordinator as `parse`
and metadata, request `ExplainDemandV1::Capture`, and consume one completed
postpass product through a dedicated `into_ast_and_explain` projection. The
current direct `explain_build_gate_program` + `prune_build_gate_program` route
is retired by the implementation task. Grammar-evidence parsing remains a
separate nonclaim.

### Cohort and failure rules

Projection happens before ordinary/compatibility cohort admission so both arms
can retain the same branch decision and optional explain report. A projection
failure is `ParseError::BuildCfg`; there is no fallback to the old evaluator.
The member-level `BoxMemberState` gate selector remains outside this product.

## Ordered implementation task

The design is implemented by:

```text
PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-I0
```

in this order:

```text
1. private projection row/cursor and strengthened selection receipt
2. one walker producing pruned AST + source receipts + optional explain
3. source-session prune consumes only validated projection receipts
4. finish_total_s0 Capture arm and CompletedPostpass explain projection
5. switch public explain wrapper to the shared postpass entry
6. remove old evaluator calls from postpass consumers
7. focused parity/negative tests and same-slice reference/README/guard update
```

No part of this task opens resolver, Recipe, Builder, MIR, runtime, member-gate
semantics, or production selection.
