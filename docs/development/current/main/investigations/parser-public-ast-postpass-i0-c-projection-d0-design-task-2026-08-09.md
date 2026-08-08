Status: accepted design boundary; implementation not opened
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
