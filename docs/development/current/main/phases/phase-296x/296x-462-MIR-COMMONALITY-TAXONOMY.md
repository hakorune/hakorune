---
Status: Active
Date: 2026-06-06
Scope: Docs-only task split for escape / allowlist-gate / owner commonality.
Related:
  - docs/development/current/main/design/mir-commonality-taxonomy-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-461-VERIFIER-HYGIENE-TASK-SPLIT.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
---

# 296x-462 MIR Commonality Taxonomy

## Decision

Worker review accepted only thin commonality:

```text
escape:
  share cause classification through escape_barrier
  keep FastMemory no-escape policy FastMemory-owned

allowlist / gate:
  keep current mir/contracts and backend_capability entries
  do not add a generic AllowlistGate<T>

owner:
  keep AllocOwnerId / page owner / semantic owner separate
  add vocabulary only
```

This is BoxShape documentation. It does not add a new accepted FastMemory proof
shape and does not change lowering behavior.

## Delegation Order

Use this order when handing off to workers:

```text
1. ESCAPE-COMMON-001 worker
   Scope:
     src/mir/verification/fastmem.rs
   Task:
     replace raw used_values()-based ordinary-MIR escape scanning with
     classify_escape_uses() or a FastMemory-local wrapper over it
   Rule:
     preserve existing FastMemory error/report names

2. ESCAPE-COMMON-002 worker
   Scope:
     fastmem verifier tests only
   Task:
     add focused return/store/call/capture/debug/Phi escape fixtures
   Rule:
     single-input Phi passthrough and multi-input Phi barrier behavior must be
     explicit if covered

3. FMEM-TABLE proof worker
   Scope:
     VerifiedTableAccessProof / TableIndex bounds work
   Task:
     resume FastMemory proof payload work after escape commonality cleanup
   Rule:
     do not reuse DirectArray access payloads

4. AOT escape adapter worker
   Scope:
     future only
   Task:
     map EscapeBarrier causes to AOT escape_kind once a real AOT consumer opens
   Rule:
     do not create escape_kind global SSOT early
```

## Parked

```text
AllowlistGate framework:
  parked until two concrete domains need the same helper API

generic Owner abstraction:
  rejected for now

verifier walker helper:
  remains parked by 296x-461 until after the next FastMemory proof slice
```

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

No behavior changes are allowed in this docs-only card.
