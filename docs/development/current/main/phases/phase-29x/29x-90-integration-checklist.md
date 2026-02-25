---
Status: Active
Scope: Phase 29x の日次運用と週次マイルストーンを 1 枚で固定するチェックリスト。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-67-post29x-cache-lane-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-68-cache-key-determinism-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-69-l1-mir-cache-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-70-l2-object-cache-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-71-l3-link-cache-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-72-cache-gate-integration-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-74-postx46-runtime-handoff-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-79-runtime-handoff-gate-integration-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-88-observability-drift-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-89-runtime-core-integrated-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-93-optimization-parity-fixtures-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-94-optimization-gate-integration-rollback-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-95-optional-gc-lane-bootstrap-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
---

# Phase 29x — Integration Checklist

## 0) Daily start

- [ ] `git status -sb`
- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`

判定:
- [ ] daily で blocker が出なければ Phase 29x 計画タスクへ進む
- [ ] FAIL 時は 60 分以内で failure-driven FIX
- [ ] 60 分を超える場合は `CURRENT_TASK.md` に詰まりメモを追記して計画タスクへ戻る

Rust lane compatibility（opt-in only）:
- [ ] `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`

## 0.5) Milestone check（節目のみ）

- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
- [ ] `bash tools/checks/abi_lane_guard.sh`

## 1) Week 1 (2026-02-16 .. 2026-02-22)

- [x] X1 docs bootstrap（29x README/checklist/task board）
- [x] X2 explicit-drop 設計 + fixture（`apps/tests/phase29x_rc_explicit_drop_min.hako`）
- [x] X3 explicit-drop 実装 + smoke（`phase29x_rc_explicit_drop_vm.sh`）
- [x] X4 scope-end 設計（X4-min1: Return 終端 cleanup）
- [x] X5 scope-end 実装 + smoke（`phase29x_rc_scope_end_release_vm.sh`）
- [x] X6 daily gate 契約の同期

Week 1 done:
- [x] `phase29y_*` / `phase29x_rc_*` が green
- [x] `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh` PASS
- [x] `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh` PASS
- [ ] 節目 3 コマンド green 維持

## 2) Week 2 (2026-02-23 .. 2026-03-01)

- [x] X7 early-exit cleanup 設計
- [x] X8 return path 実装 + smoke
- [x] X9 break path 実装 + smoke
- [x] X10 continue path 実装 + smoke
- [x] X11 PHI/edge verifier
- [x] X12 RC 3規則包括 smoke

Week 2 done:
- [x] overwrite/explicit/scope-end がすべて smoke で固定
- [x] return path cleanup（X8）が smoke で固定
- [x] break path cleanup（X9）が smoke で固定
- [x] continue path cleanup（X10）が smoke で固定
- [x] verifier failure が fail-fast で観測可能

## 3) Week 3 (2026-03-02 .. 2026-03-08)

- [x] X13 observability 拡張設計同期
- [x] X14 temps 実装 + smoke
- [x] X15 heap_fields 実装 + smoke
- [x] X16 singletons 実装 + smoke
- [x] X17 debug_root_summary 契約固定

Week 3 done:
- [x] root categories 5点が docs と一致

## 4) Week 4 (2026-03-09 .. 2026-03-15)

- [x] X18 VM route cutover 設計（`vm`/`vm-hako`/compat）
- [x] X19 route observability（安定タグ）追加
- [x] X20 strict/dev: `vm-hako` 優先化（compat は明示時のみ）
- [x] X21 non-strict compat lane の限定縮退
- [x] X22 3日連続 gate green 証跡採取
- [x] X23 Rust-optional done docs 同期

Week 4 done:
- [x] Rust fallback は明示フラグ時のみ
- [x] X22 Day1/Day2/Day3 evidence を記録
- [x] 3日連続 gate green
- [x] GC optional（非意味論）を docs で再確認

## 5) Post-X23 fast lane（no-date）

開始前提:
- [x] X23 完了
- [x] X11 / X12 完了
- [ ] daily gate 安定（FAIL 時は failure-driven 60 分以内）

Thin-Rust hardening（X24-X31）:
- [x] X24 thin-rust boundary lock（route/verifier/safety SSOT）
- [x] X25 route orchestration 入口一本化
- [x] X26 route observability 契約固定
- [x] X27 compat bypass fail-fast 化
- [x] X28 verifier gate 一本化
- [x] X29 safety gate 一本化
- [x] X30 thin-rust Core C ABI 最小面固定
- [x] X31 thin-rust gate pack 固定

Thin-Rust done:
- [x] route/verifier/safety が一箇所化
- [x] strict/dev で暗黙 fallback なし
- [x] docs + smoke + evidence 同期

De-Rust transfer（X32-X36）:
- [x] X32 `.hako` route orchestrator skeleton
- [x] X33 `.hako` verifier 経路導入
- [x] X34 `.hako` safety 経路導入
- [x] X35 strict/dev 既定を `.hako` route へ切替
- [x] X36 de-rust done docs 同期

De-Rust done:
- [x] strict/dev 主経路が `.hako` route/verifier/safety で完結
- [x] Rust thin lane は明示フラグ時のみ
- [x] rollback 条件と証跡が docs で固定

Rust build retirement（X37-X40）:
- [x] X37 llvm+c-abi link gate 追加
- [x] X38 daily/milestone を llvm line 既定へ切替
- [x] X39 Rust lane を tools/compat 専用へ隔離
- [x] X40 llvm-only build done docs 同期

LLVM-only done:
- [x] 日常運用の build/gate が LLVM+C ABI line で完結
- [x] Rust build は明示互換モード時のみ
- [x] 残存 Rust 依存一覧と rollback 条件が docs で固定

Post-29x cache extension（X41-X46）:
- [x] X41 post-29x closeout sync（docs）
- [x] X42 cache key determinism（CB-1）
- [x] X43 L1 MIR cache（CB-2）
- [x] X44 L2 object cache（CB-3）
- [x] X45 L3 link cache（CB-4）
- [x] X46 cache gate integration + done sync（CB-5）

Cache lane done:
- [x] module/object/link key の決定規則が docs + implementation で一致
- [x] 2回目実行で cache hit が観測できる
- [x] runtime ABI 境界変更時に link cache が fail-safe に無効化される
- [x] daily/milestone で cache hit/miss が観測可能

Post-X46 runtime handoff（X47-X53）:
- [x] X47 post-X46 handoff bootstrap（docs）
- [x] X48 route pin inventory + guard（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 固定点）
- [x] X49 vm-hako strict/dev replay gate（rust-vm pin なし）
- [x] X50 NewClosure contract lock（fail-fast 維持 or parity 実装）
- [x] X51 Core C ABI delegation inventory + guard
- [x] X52 handoff gate integration
- [x] X53 done sync + rollback lock

Post-X46 handoff done:
- [x] route pin 増殖が guard で検出できる
- [x] vm-hako replay と既存 daily gate を分離観測できる
- [x] Core C ABI boundary 逸脱が fail-fast で検出できる

Post-X53 runtime core extension（X54-X66）:
- [x] X54 lane bootstrap（docs）
- [x] X55 vm-hako S6 vocabulary inventory + guard
- [x] X56 vm-hako dual-run parity gate pack
- [x] X57 NewClosure runtime lane decision refresh
- [x] X58 S6 first vocabulary promotion
- [x] X59 ABI borrowed/owned conformance extension
- [x] X60 RC insertion phase2 queue lock
- [x] X61 observability drift guard
- [x] X62 runtime core integrated gate
- [x] X63 optimization allowlist lock
- [x] X64 optimization parity fixtures/reject fixtures
- [x] X65 optimization gate integration + rollback lock
- [x] X66 optional GC lane bootstrap（docs）

Post-X53 runtime core done:
- [x] vm-hako S6 parity drift が guard で検出できる
- [x] runtime core（ABI/RC/observability）が single-entry gate で再生できる
- [x] optimization gate で pre/post parity を固定できる
- [x] optional GC は semantics 不変（意味論変更なし）を維持

## 6) Commit rule lock

- [ ] 1 タスク = 1 commit（BoxCount/BoxShape 混在禁止）
- [ ] fast gate FAIL のまま `cases.tsv` 追加禁止
- [ ] WIP は `git stash` で退避し、まず契約を直す
