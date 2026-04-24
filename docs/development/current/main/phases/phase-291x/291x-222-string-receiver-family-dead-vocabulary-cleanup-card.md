---
Status: Landed
Date: 2026-04-25
Scope: Remove the dead MIR-call receiver-family `STRING` vocabulary after StringBox receiver-surface fallback was pruned.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-220-stringbox-receiver-surface-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-221-method-surface-dead-branch-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
---

# 291x-222 String Receiver Family Dead Vocabulary Cleanup Card

## Goal

Remove the dead `HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING` enum variant from
MIR-call route policy. `ORG_STRING` scan-origin remains in use for Array/String
observer and store promotion logic; this card only removes the unused receiver
family classification.

## Boundary

- Do not touch `ORG_STRING` scan-origin.
- Do not change `runtime_string` route-state bits; they are still used by
  metadata-first String len/substring/indexOf routes.
- Do not change receiver-surface fallback rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed `HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING`.
- Kept `ORG_STRING` scan-origin and `runtime_string` route-state bits intact.
- No no-growth row count changed; guard remains `classifiers=10 rows=10`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
