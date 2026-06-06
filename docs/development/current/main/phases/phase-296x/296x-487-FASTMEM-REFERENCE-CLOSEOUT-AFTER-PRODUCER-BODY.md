---
Status: Active
Date: 2026-06-06
Scope: FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001.
Related:
  - docs/development/current/main/phases/phase-296x/296x-486-MIR-FMEM-008E-PRODUCER-NEUTRAL-READINESS.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - tools/hako_check/README.md
---

# 296x-487 FastMemory Reference Closeout After Producer Body

## Decision

Close the post-producer reference sync after MIR-FMEM-008E.

Current reference wording now treats Python-template C as:

```text
role=explicit_diagnostic_baseline_only
semantic_ssot=0
hidden_runtime_fallback=0
product_activation=0
```

The current primary implementation direction is:

```text
.hako hako_alloc / fastmem / capability surface
  -> MIR FastMemRegion metadata + MemOp instructions
  -> verifier
  -> LLVM/object producer
```

## Updated

```text
docs/reference/language/low-level-capabilities.md:
  records the post-008E readiness gate and diagnostic-only baseline role.

docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md:
  updates hako_alloc / Python-template C / MIR-to-LLVM producer identity.

docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md:
  updates the producer transition plan and readiness report fields.

docs/development/current/main/workstreams/mimalloc-current.md:
  points the active implementation lane to MIM-PORT-FMEM-001.

tools/hako_check/README.md:
  clarifies `fastmem-producer-parity` as a diagnostic-baseline readiness gate,
  not a deletion or activation tool.
```

## Still Closed

```text
Python-template C diagnostic payload deletion/archive
MIR-to-C debug/diff artifact
TLS backing transfer
owner slot reuse as active owner transfer
AtomicRemoteHead lowering
same-owner / remote-owner routing policy
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
bash tools/hako_check/fastmem_producer_parity_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-001:
  migrate the first narrow hako_alloc owner/layout path using the already proven
  FastMemory substrate only.
```
