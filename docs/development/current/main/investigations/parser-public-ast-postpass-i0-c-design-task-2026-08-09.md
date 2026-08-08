---
Status: parked design
Date: 2026-08-09
Decision: design required before implementation
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-I0-C

## Scope

Unify the full BuildGate decision set used by prune, explain-report capture,
and top-level source-path rebase. I0-C must define one decision-set owner and
one consuming projection before any public explain edge is switched.

```text
parse once
  -> one typed BuildGate decision set
  -> prune / explain / source-path rebase
  -> one total postpass completion
```

## Design stop

Do not add a second parser invocation, AST/name rescan, or explain-only
fallback. Decide the exact receipt ownership, malformed/unknown predicate
diagnostics, fuel behavior, compatibility cohorts, and source-seal interaction
before implementation.

## Non-claims

```text
no implementation yet
no resolver source publication
no Recipe/Builder/MIR/runtime work
no fallback/retry/reparse
```

## Closeout

The accepted design must update the postpass SSOT, parser README, language
reference, task map, CURRENT_STATE, guard, and focused parity matrix before
the fast implementation row opens.
