---
Status: Active
Date: 2026-06-06
Scope: ESCAPE-COMMON-001 implementation for FastMemory verifier escape checks.
Related:
  - docs/development/current/main/design/mir-commonality-taxonomy-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-462-MIR-COMMONALITY-TAXONOMY.md
  - src/mir/escape_barrier.rs
  - src/mir/verification/fastmem.rs
---

# 296x-463 FastMemory EscapeBarrier Classifier

## Decision

FastMemory verifier escape checks consume the shared MIR escape-barrier cause
classifier, but keep FastMemory no-escape policy local.

Accepted implementation shape:

```text
Shared:
  classify_escape_uses(inst)

FastMemory-owned:
  MemOp produced-value origin tracking
  MemOp-to-MemOp allowed flow
  ordinary MIR consumer rejection
  FastMemory error/report reason shape
```

## Implementation Boundary

`src/mir/verification/fastmem.rs` now:

```text
1. records MemOp-produced values
2. propagates origin through single-input Phi passthroughs
3. uses classify_escape_uses() for shared cause labels
4. keeps an ordinary_use fallback for unclassified ordinary MIR consumers
5. emits existing memop-value-escapes FastMemory violations
```

This is not a generic escape policy. It is a FastMemory verifier consumer of a
shared cause classifier.

## Tests

Focused verifier coverage:

```text
return:
  barrier=return

store value:
  barrier=store_like

call arg:
  barrier=call

debug observe:
  barrier=debug_observe

ordinary BinOp use:
  barrier=ordinary_use

single-input Phi:
  propagates MemOp origin and reports the later escape barrier

multi-input Phi:
  barrier=phi_merge
```

## Acceptance

```bash
cargo test -q verification::fastmem --lib
cargo test -q escape_barrier --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
ESCAPE-COMMON-002:
  optional test-only follow-up if closure capture / extra boundary fixtures are
  needed after review

FMEM-TABLE:
  resume VerifiedTableAccessProof / TableIndex bounds work
```
