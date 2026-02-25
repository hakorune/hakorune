# Phase 29ag: JoinIR merge SSOT unification (post-29af)

Goal: Phase 29af で固めた boundary/layout/contract を前提に、merge 内の order SSOT をさらに一箇所へ寄せて回帰余地を減らす（仕様不変）。

## Status

- P0: ✅ COMPLETE
- P1: ✅ COMPLETE

## Why now

- Phase 29af で carrier order SSOT（`BoundaryCarrierLayout`）と整合契約（P4）まで揃った
- 次は “同じ order を別ソースで再計算する箇所” を減らし、未来の refactor でズレない構造にする

## P0: Coordinator param remap uses BoundaryCarrierLayout

- ✅ COMPLETE
- 指示書: `docs/development/current/main/phases/phase-29ag/P0-COORDINATOR-USES-BOUNDARY-CARRIER-LAYOUT-INSTRUCTIONS.md`

## P1: Eliminate ValueId(i) fallback; remap via boundary.join_inputs

- ✅ COMPLETE
- 指示書: `docs/development/current/main/phases/phase-29ag/P1-COORDINATOR-REMAPPERS-USE-JOIN_INPUTS-INSTRUCTIONS.md`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
