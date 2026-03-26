---
Status: SSOT
Decision: provisional
Date: 2026-03-24
Scope: artifact/lane policy の detail owner。`llvm-exe` / current `vm-hako` / `rust-vm` の役割を固定し、owner proof と future interpreter 候補を混線させない。
Related:
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/reference/invariants.md
  - lang/README.md
---

# Artifact Policy (SSOT)

## Purpose

- artifact policy と owner proof を分けて読む。
- current `vm-hako` を co-mainline と誤読しない。
- future interpreter artifact 候補を current `vm-hako` promotion と混ぜない。

## 1. Active Artifact Roles

| artifact/lane | role | fixed reading |
| --- | --- | --- |
| `llvm-exe` | daily / CI / distribution artifact | 唯一の mainline |
| current `vm-hako` | semantic witness / debug / bootstrap-proof artifact | internal reference lane |
| `rust-vm` | bootstrap / recovery / compat artifact | explicit keep |

Operational rules:

- `llvm-exe` is the only production/mainline artifact.
- current `vm-hako` is not a co-mainline candidate in the current policy.
- current `vm-hako` acceptance is gate-backed; archived throughput/probe smokes do not change its role.
- current `vm-hako` LLVM/exe bridge proofs are manual monitor evidence only, not mainline acceptance.
- `rust-vm` may remain as bootstrap/recovery residue without affecting owner-migration reading.

## 2. Owner Proof Boundary

- artifact choice does not prove owner migration by itself.
- backend changes must preserve the cross-backend invariants in `docs/reference/invariants.md`.
- `.hako` artifact, `stage1` success, or `vm-hako` green do not automatically mean `.hako` owner proof.

## 3. Future Interpreter Reservation

- A future interpreter artifact is reserved as a possible usability lane for script / REPL / Python-like UX.
- It is **not** current `vm-hako` renamed or promoted.
- It is not an active lane, not a support-tier promise, and not a product target in this document.
- Any future interpreter lane must be reopened separately after entry contract, support tier, and test-matrix policy are explicit.

## 4. Promotion / Non-Goals

Current non-goals:

- promoting current `vm-hako` to co-mainline
- treating current `vm-hako` as a user-facing production artifact
- treating current `vm-hako` as the owner of LLVM backend / exe-build responsibilities
- defining future interpreter promotion criteria now
- changing raw CLI backend tokens or defaults in this doc

Promotion discussion for any future lane must first prove:

- independent entry contract
- independent test matrix
- no hidden dependence on another lane's route/fallback ownership
