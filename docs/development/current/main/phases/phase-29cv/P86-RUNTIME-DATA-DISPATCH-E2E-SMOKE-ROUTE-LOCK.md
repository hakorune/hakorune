---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: repair the runtime_data dispatch e2e smoke contract.
Related:
  - tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
  - apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json
  - docs/development/current/main/phases/phase-29cv/P82-LOWERING-PLAN-ARRAYHAS-DIRECTABI-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P85-LOWERING-PLAN-MAPSET-COLDRUNTIME-CONSUME.md
---

# P86 RuntimeData Dispatch E2E Smoke Route Lock

## Goal

Make the active `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` smoke match
its prebuilt MIR fixture contract again.

## Decision

- Add the missing `RuntimeDataBox.has(ArrayBox)` route metadata to the e2e
  fixture.
- Keep this route on the explicit runtime-data facade helper:
  `nyash.runtime_data.has_hh`.
- Allow the smoke's `set` check to accept the fixture's declared map core
  route: `nyash.map.slot_store_hhh`.
- Do not change backend lowering code in this card.

## Non-goals

- no new LoweringPlan accepted shape
- no `.inc` matcher change
- no route promotion
- no expected rc change

## Acceptance

```bash
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
