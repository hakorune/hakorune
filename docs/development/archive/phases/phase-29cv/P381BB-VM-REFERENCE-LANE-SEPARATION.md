---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: separate VM/reference naming from LLVM-mainline quick-gate paths
Related:
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/checks/dev_gate.sh
  - tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_llvm.sh
  - tools/smokes/v2/profiles/integration/phase21_5/perf/chip8/phase21_5_perf_chip8_kernel_crosslang_contract.sh
---

# P381BB: VM Reference Lane Separation

## Problem

The architecture audit showed that LLVM EXE is already the practical mainline,
but quick-gate naming still blurred that truth:

- a pure LLVM IR contract smoke still carried a `_vm` suffix
- the chip8 cross-language smoke also carried a `_vm` suffix even though it
  monitors a mixed C/Python/VM/AOT contract rather than a VM-only lane
- bootstrap docs still read as if Program(JSON v0) had to bridge through VM on
  the mainline

That made the VM lane look more central than it is.

## Decision

Separate the mainline names from the compatibility names.

Implemented:

- renamed the LLVM IR quick-gate smoke to `*_llvm.sh`
- renamed the chip8 cross-language quick-gate smoke to a neutral
  `*_crosslang_contract.sh`
- kept the old `_vm.sh` file names only as thin compatibility wrappers for
  historical cards and old callers
- updated quick-gate wiring and current docs to use the new mainline names
- clarified bootstrap-route docs so Program(JSON v0) keeps MIR as the contract
  center while VM stays an optional reference/debug consumer

## Boundary

Allowed:

- renaming quick-gate and doc entrypoints so the LLVM EXE path is visibly the
  mainline
- keeping historical wrappers for compatibility while removing them from active
  docs/gates

Not allowed:

- changing runtime semantics, perf thresholds, or backend logic
- deleting historical wrappers in the same card
- treating VM as deleted; it remains a reference/debug/bootstrap lane

## Acceptance

```bash
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_llvm.sh
bash tools/smokes/v2/profiles/integration/phase21_5/perf/chip8/phase21_5_perf_chip8_kernel_crosslang_contract.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- quick gate now names the LLVM/mainline checks as LLVM/mainline checks
- old `_vm` smoke paths no longer define the active mainline naming
- docs are more honest that VM is a reference lane, not the daily mainline

Next:

1. keep pushing VM/reference lane cleanup in docs/gates where active mirrors
   still blur it
2. keep the architectural mainline work on the uniform multi-function MIR
   emitter separate from this naming cleanup
