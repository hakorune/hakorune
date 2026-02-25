---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X66 optional GC lane bootstrap（docs-only）として semantics unchanged 契約と非目標境界を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-94-optimization-gate-integration-rollback-lock-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - docs/reference/language/lifecycle.md
---

# 29x-95: Optional GC Lane Bootstrap (SSOT)

## 0. Goal

- X66 として optional GC lane の入口を docs-only で固定する。
- GC ON/OFF が意味論を変えない契約（semantics unchanged）を再確認し、runtime core lane の終端条件を明文化する。
- 実装順序（ABI -> RC insertion -> observability -> VM parity -> optimization -> optional GC）を崩さない。

## 1. Fixed policy (carry-over)

1. lifecycle 意味論は GC 必須ではない（`fini` は決定的、物理解放タイミングは意味論外）。
2. cycle collector は optional 機能であり、意味論要件にしない。
3. GC ON/OFF の差分は回収タイミング/リーク耐性に限定する。
4. optional GC は runtime core lane（X54-X65）完了後にのみ検討する。

## 2. Scope / Non-goals

In scope（X66）:
- optional GC lane の入口・非目標・順序を docs で固定する。
- Phase 29y SSOT（ABI/RC/observability）への導線を一本化する。

Out of scope（X66）:
- GC/finalizer の新規実装
- lifecycle 仕様の変更
- runtime 挙動のデフォルト変更

## 3. Handoff entry (next phase)

optional GC lane を実装フェーズへ渡すときは、次を SSOT 入口として使う。

1. `docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md`
2. `docs/development/current/main/phases/phase-29y/README.md`
3. `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`
4. `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
5. `docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md`
6. `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md`
7. `tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`
8. `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
9. `tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh`

## 4. Evidence command

- `cat docs/development/current/main/phases/phase-29x/29x-95-optional-gc-lane-bootstrap-ssot.md`

## 5. Next step

- Phase 29x runtime core extension lane（X54-X66）は完了。
- 次は Phase 29y / runtime-gc-policy SSOT を入口に、optional GC 実装計画（docs-first）へ進む。
