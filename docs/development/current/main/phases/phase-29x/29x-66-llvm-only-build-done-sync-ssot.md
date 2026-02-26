---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X40 llvm-only build done 判定を docs/evidence/rollback で最終同期する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-64-llvm-only-daily-default-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-65-rust-lane-optin-isolation-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
  - tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh
  - tools/compat/phase29x_rust_lane_gate.sh
---

# Phase 29x X40: LLVM-Only Build Done Sync SSOT

## 0. Goal

Rust build retirement lane（X37-X40）の完了条件を
1枚で固定し、運用導線を迷わない状態で close する。

## 1. Done Criteria (X37-X40)

1. X37: LLVM+C ABI link 最小 gate が固定されている
2. X38: daily/milestone 既定が LLVM-only gate になっている
3. X39: Rust lane は `tools/compat` + explicit opt-in のみ
4. 証跡コマンド 3本が継続して PASS

証跡コマンド:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh`

## 2. Rollback / Escape Hatches

異常時の明示 rollback は次で固定する。

1. Rust lane 実行を一時再開:  
   `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`
2. strict/dev route rollback（X35/X36契約）:  
   `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` / `NYASH_VM_USE_FALLBACK=1` / `--backend vm-hako`

注記:
- rollback は「既定導線を戻す」のではなく、明示 opt-in で限定実施する。
- default daily/milestone は LLVM-only gate を維持する。

## 3. Remaining Rust Dependencies (explicit list)

LLVM-only 運用でも残る Rust 依存は次の通り。

1. `hakorune` バイナリ（frontend/runner）
2. `tools/build_llvm.sh` が呼ぶ Rustビルド（`nyash-rust`, `nyash_kernel`）
3. `tools/compat/phase29x_rust_lane_gate.sh`（互換検証専用、既定導線外）

この一覧以外の Rust lane 依存を default daily/milestone に再導入しない。

## 4. X40 Decision

Decision: accepted

- Rust build retirement lane（X37-X40）は docs + gate + evidence で同期完了。
- Phase 29x の完了条件（README Section 7）を満たした状態で close できる。
