---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X53 完了後の post-X53 runtime core extension（X54-X66）を、層順序と1コミット粒度で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-74-postx46-runtime-handoff-done-sync-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
---

# 29x-80: Post-X53 Runtime Core Sequencing (SSOT)

## 0. Goal

- X53 close 後の次レーンを「迷わず再開できる粒度」で固定する。
- 「VM parity -> runtime core hardening -> optimization -> optional GC」の順序を崩さない。
- 1タスク=1コミットで証跡化し、silent fallback を防ぐ。

## 1. Fixed Order (X54-X66)

### Layer 1: VM de-Rust extension (X54-X58)

1. X54: lane bootstrap（docs-only）
2. X55: vm-hako S6 vocabulary inventory + guard
3. X56: vm-hako dual-run parity gate pack（success/reject split）
4. X57: NewClosure runtime lane decision refresh（execute or fail-fast boundary pin）
5. X58: S6 first vocabulary promotion（BoxCount 1語彙）

### Layer 2: Runtime core hardening (X59-X62)

1. X59: ABI borrowed/owned conformance matrix extension
2. X60: RC insertion phase2 queue lock（loop/call/early-exit）
3. X61: observability drift guard（5 root categories）
4. X62: runtime core integrated gate（ABI + RC + observability）

### Layer 3: Optimization safe set (X63-X65)

1. X63: optimization allowlist lock（const-fold / DCE / CFG simplification）
2. X64: optimization parity fixtures + reject fixtures
3. X65: optimization gate integration + rollback lock

### Layer 4: Optional GC (X66)

1. X66: optional GC lane bootstrap（docs-only, semantics unchanged）

## 2. Granularity Contract

1. 1タスク = 1コミット = 1受理形（BoxCount）または1責務整理（BoxShape）
2. BoxCount と BoxShape を同一コミットで混ぜない
3. 各タスクで fixture+gate+docs の 3点同期を行う
4. strict/dev では未対応形を fail-fast で固定し、fallback で通さない

## 3. Acceptance (lane close)

1. vm-hako route drift は guard で検出できる
2. runtime core（ABI/RC/observability）は単一 gate で再生できる
3. optimization gate で pre/post parity が壊れていない
4. optional GC は semantics 不変（ON/OFFで意味論不変）を維持する

## 4. X54 Evidence

1. `cat docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md`
