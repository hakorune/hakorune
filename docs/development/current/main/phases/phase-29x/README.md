---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: de-Rust runtime lane の実装タスクを、selfhost failure-driven と分離しつつ統合運用するための Phase 29x 実行計画。
Related:
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
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
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - docs/reference/language/lifecycle.md
---

# Phase 29x: De-Rust Runtime Integration (Active Plan)

## 0. Goal

Phase 29x の目的は次の 2 点を同時に満たすこと。

1. failure-driven 運用を維持しつつ、計画タスクを前進させる。
2. 既存 SSOT（ABI/RC/observability/GC policy）を崩さず、Rust VM 依存を段階縮退する。

## 0.5 Current Override (2026-03-27)

- current structure-first front is `backend owner cutover prep`
- canonical seam stays MIR; do not open `AST -> LLVM` direct lowering in this wave
- fixed order is:
  1. `backend-owner-cutover-ssot.md`
  2. `runtime-decl-manifest-v0.toml`
  3. `recipe-facts-v0`
  4. `.hako ll emitter` min v0
  5. explicit compare bridge
  6. boundary-only narrow owner flip
  7. archive/delete sweep
- current landed slice is subtraction-first:
  - `.hako ll emitter` min v0 is the daily owner for `ret const`, `bool phi/branch`, `Global print`, `StringBox.length`, `StringBox.indexOf`, `concat3 extern`, `RuntimeDataBox.length(StringBox)`, `RuntimeDataBox.length(ArrayBox)`, and `RuntimeDataBox.size(MapBox)`
  - compare bridge smoke stays `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_hako_ll_compare_min.sh`
  - daily owner smokes are `phase29x_backend_owner_daily_{ret_const,bool_phi_branch,print,string_length,string_indexof,concat3_extern,runtime_data_length,runtime_data_array_length,runtime_data_map_size,runtime_data_array_has,runtime_data_array_get,runtime_data_array_push,runtime_data_map_has,runtime_data_map_get}_min.sh`
  - compare wrapper app is `apps/tests/phase29x_backend_owner_hako_ll_compare_min.hako`
  - daily wrapper app is `apps/tests/phase29x_backend_owner_daily_min.hako`
  - archive/delete sweep wave 1 is landed:
    - flipped `phase29ck` locks now live in `tools/smokes/v2/suites/integration/phase29ck-boundary-legacy.txt`
    - default `phase29ck-boundary` no longer carries `ret const`, `bool phi/branch`, `Global print`, `StringBox.length`, `StringBox.indexOf`, `concat3 extern`, or the three `RuntimeData.length/size` observer locks
    - compare bridge assets remain explicit bridge-only and are not delete-ready yet
- legacy C `.inc` remains daily owner only for unflipped shapes, and demotion/archive tracking is now fixed in `29x-96-backend-owner-legacy-ledger-ssot.md`
- archive/delete sweep wave 1, code-side `legacy C daily demotion v1`, and the `hello_simple_llvm_native_probe_v1` owner flip are landed
  - the lookup family is landed; the `RuntimeData` mutator family is now landed for `runtime_data_array_push_min_v1`
  - remaining active owner-flip targets are 2 shapes: `indexof_line_pure_min_v1`, `substring_concat_loop_pure_min_v1`
- structural perf only:
  - attrs centralization
  - facts visibility
  - copy-transparency / bool-i1 cleanliness
  - compare ledger / verifier lane

## 1. Non-goals

- GC アルゴリズム本体（cycle collector/tracing）の新規実装。
- 言語仕様拡張（selfhost unblock のための仕様追加）。
- fallback の黙認（silent success）。

## 2. Baseline and Timebox

- `Current blocker: none` の通常日は、failure-driven の軽量ループ（quick/probe）を優先する。
- failure-driven は FAIL 発生時のみ実施し、日次最大 60 分までに制限する。
- 60 分超過時は `CURRENT_TASK.md` に詰まりメモを残し、Phase 29x の計画タスクへ戻る。
- 1 タスク = 1 受理形 = fixture+gate = 1 commit を維持する。

Milestone 3 commands（節目チェック）:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
3. `bash tools/checks/abi_lane_guard.sh`

Rust lane compatibility（opt-in only）:

1. `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`

## 3. Lanes (fixed)

### Lane A: Selfhost stability (failure-driven)

- 既存 blocker 対応だけを行う保守レーン。
- Green 時に新規 fixture を増やさない。

### Lane B: Lifecycle core (ABI + RC insertion + observability)

- ABI は `args borrowed / return owned` を維持。
- RC insertion は 1 箇所 SSOT を維持。
- observability は 5 root categories 完成まで拡張。

### Lane C: VM route cutover (missing lane in prior plan)

- `vm-hako parity` 完了後に、実行経路の主従を段階切替する。
- いきなり compat 全撤去はしない。strict/dev 優先の段階縮退で進める。

### Lane G: Post-29x cache build extension (X41-X46)

- X1-X40 完了後の拡張レーン（性能/運用改善）として扱う。
- `hako.toml` 依存解決を前提に module/object/link 3層キャッシュを段階導入する。
- 言語仕様を変えず、build orchestration のみを改善する。

### Lane H: Post-X46 runtime handoff (X47-X53)

- cache lane 完了後に runtime de-rust handoff を再開する拡張レーン。
- 先に route drift 制御（pin inventory / guard）を固定し、次に vm-hako replay と C ABI delegation を統合する。
- GC/cycle collector はこのレーンの対象外（optional/last）。

### Lane I: Post-X53 runtime core extension (X54-X66)

- handoff lane（X47-X53）完了後の次拡張レーン。
- `VM parity extension -> runtime core hardening -> optimization -> optional GC` の順で固定。
- 1タスク=1コミットの粒度を維持し、各タスクで fixture+gate+docs を同期する。

## 4. Four-week schedule (fixed dates)

### Week 1: 2026-02-16 .. 2026-02-22

- X1: Phase 29x docs bootstrap（README/checklist/task board 作成）
- X2: RC explicit-drop (`x = null`) 設計と fixture 追加
  - fixture: `apps/tests/phase29x_rc_explicit_drop_min.hako`
- X3: RC explicit-drop 実装 + smoke 固定
  - smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh`
- X4: RC scope-end release 設計
- X5: RC scope-end release 実装 + smoke 固定
- X6: daily gate 手順を 29x チェックリストへ固定

Week 1 done:
- `phase29y_*` + `phase29x_rc_*` が green
- 節目 3 コマンド green 維持

Week 1 contract pin (X2/X3):
- baseline: `--backend vm apps/tests/phase29x_rc_explicit_drop_min.hako` は exit 0
- feature path: `--features rc-insertion-minimal --emit-mir-json` で `main` と module に `release_strong=1` を固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh` PASS

Week 1 contract pin (X4/X5):
- scope-end の最小定義（X4-min1）: `rc-insertion-minimal` では Return 終端の BeforeTerminator cleanup を scope-end release として扱う
- out-of-scope: nested lexical block 単位の cleanup timing はこのタスクでは固定しない（early-exit/loop lane で扱う）
- fixture: `apps/tests/phase29x_rc_scope_end_release_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）

Week 1 ops sync (X6):
- evidence (2026-02-13): `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（5/5）

### Week 2: 2026-02-23 .. 2026-03-01

- X7: early-exit cleanup（return/break/continue）設計
- X8: return path 実装 + smoke
- X9: break path 実装 + smoke
- X10: continue path 実装 + smoke
- X11: PHI/edge verifier 追加
- X12: RC 3規則（overwrite/explicit/scope-end）包括 smoke 固定

Week 2 contract pin (X7):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-20-early-exit-cleanup-ssot.md`
- early-exit cleanup 順序を `return/break/continue` で統一し、実装済み範囲（return）と未実装範囲（break/continue）を明示

Week 2 contract pin (X8):
- fixture: `apps/tests/phase29x_rc_return_cleanup_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_return_cleanup_vm.sh`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_return_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）

Week 2 contract pin (X9):
- fixture: `apps/tests/phase29x_rc_break_cleanup_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_break_cleanup_vm.sh`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_break_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）

Week 2 contract pin (X10):
- fixture: `apps/tests/phase29x_rc_continue_cleanup_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_continue_cleanup_vm.sh`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_continue_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）

Week 2 contract pin (X11):
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_phi_edge_verifier_vm.sh`
- fail-fast tag: `[freeze:contract][rc_insertion/phi_edge_mismatch]`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_phi_edge_verifier_vm.sh` PASS（baseline + fail-fast tag observed）

Week 2 contract pin (X12):
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_three_rules_vm.sh`
- marker: `[rc_three_rules] overwrite=ok explicit_drop=ok scope_end=ok`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_three_rules_vm.sh` PASS

Week 2 done:
- RC 3 規則がすべて fixture+gate で固定
- verifier failure が fail-fast で観測可能

### Week 3: 2026-03-02 .. 2026-03-08

- X13: observability 5カテゴリ拡張の設計同期
- X14: `temps` 実装 + smoke
- X15: `heap_fields` 実装 + smoke
- X16: `singletons` 実装 + smoke
- X17: `debug_root_summary` 契約固定

Week 3 contract pin (X13):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-30-observability-extension-ssot.md`
- root surface 語彙（`locals/temps/heap_fields/handles/singletons`）と X14-X17 の実装境界を固定

Week 3 contract pin (X14):
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh`
- contract: `[lifecycle/leak]   temps: <n>` を実測表示し、limitation 行に `singletons=0` を含める
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh` PASS

Week 3 contract pin (X15):
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh`
- contract: `[lifecycle/leak]   heap_fields: <n>` を実測表示し、limitation は `singletons=0` のみに縮退
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh` PASS

Week 3 contract pin (X16):
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh`
- contract: `[lifecycle/leak]   singletons: <n>` を実測表示し、legacy limitation alias（`singletons=0`）を撤去
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh` PASS

Week 3 contract pin (X17):
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh`
- contract: 5カテゴリ（`handles/locals/temps/heap_fields/singletons`）を一意・固定順で出力
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh` PASS

Week 3 progress:
- X13-X17（observability lane）完了

### Week 4: 2026-03-09 .. 2026-03-15

- X18: VM route cutover 設計（`vm`/`vm-hako`/compat の責務定義）
- X19: route observability 追加（選択理由タグ + fail-fast 理由）
- X20: strict/dev で `vm-hako` 優先経路を既定化（compat は明示時のみ）
- X21: non-strict compat lane を限定縮退（撤去ではなく境界固定）
- X22: 3 日連続 gate green 証跡採取
- X23: Rust-optional done 判定の docs 同期（GC optional 記述含む）

Week 4 contract pin (X18):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md`
- contract: `vm / vm-hako / compat` の責務・切替順序（X19-X23）・fail-fast 境界を固定
- evidence (2026-02-13): `cat docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md`

Week 4 contract pin (X19):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-41-vm-route-observability-ssot.md`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
- contract: `[vm-route/select]` タグで `vm(default opt-out)` / `compat-fallback` / `vm-hako` の分岐理由を一意観測

Week 4 contract pin (X20):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-42-vm-route-strict-dev-priority-ssot.md`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
- contract: strict/dev 既定で `backend=vm` は `lane=vm-hako`、compat は明示時のみ
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh` PASS

Week 4 contract pin (X21):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-43-vm-route-non-strict-compat-boundary-ssot.md`
- smoke: `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- contract: non-strict Stage-A の compat lane は `NYASH_VM_USE_FALLBACK=1` 明示時のみ
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh` PASS

Week 4 contract pin (X22, Day3/3):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-44-vm-route-three-day-gate-evidence.md`
- gate set:
  - `cargo check -q --bin hakorune`
  - `phase29x_vm_route_observability_vm.sh`
  - `phase29x_vm_route_strict_dev_priority_vm.sh`
  - `phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- evidence (2026-02-13): Day1 PASS（`stageb_total_secs=18`, `avg_case_secs=3.60`）
- evidence (2026-02-14): Day2 PASS（`stageb_total_secs=15`, `avg_case_secs=3.00`）
- evidence (2026-02-15): Day3 PASS（`stageb_total_secs=15`, `avg_case_secs=3.00`）
- completion check: `tools/selfhost/check_phase29x_x22_evidence.sh --strict` PASS

X23 preflight:
- SSOT: `docs/development/current/main/phases/phase-29x/29x-45-rust-optional-done-sync-ssot.md`
- check:
  - `tools/selfhost/check_phase29x_x23_readiness.sh`
  - `tools/selfhost/check_phase29x_x23_readiness.sh --strict`
- evidence: `tools/selfhost/check_phase29x_x23_readiness.sh --strict` PASS

Week 4 progress:
- X18-X23 完了（X22 strict gate + X23 strict readiness を証跡化）

## 5. Post-X23 fast lane (no-date, execute ASAP)

`X24+` は日付固定せず、前提が満たれたら連続実行する。

Preconditions:
- X23 が完了している
- X11 / X12 が未完の場合は先に完了させる
- daily gate が安定（failure-driven は保守レーンで継続）

### Lane D: Thin-Rust hardening (X24-X31)

- X24: thin-rust boundary lock（route orchestration / verifier / safety の責務を SSOT で固定）
- X25: route orchestration 入口一本化（`vm` / `vm-hako` / selfhost）
- X26: route observability 契約固定（`[vm-route/*]` の選択理由タグを統一）
- X27: compat bypass fail-fast 化（暗黙 fallback の入口を遮断）
- X28: verifier gate 一本化（全 backend が同一 verify 入口を通る）
- X29: safety gate 一本化（lifecycle/unsafe 境界の fail-fast 契約を統一）
- X30: thin-rust Core C ABI 最小面固定（route / verify / safety の公開境界）
- X31: thin-rust gate pack 固定（smoke + docs + evidence）

Post-X23 contract pin (X24):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md`
- contract: route orchestration / verifier / safety の責務境界を 1 箇所化方針で固定
- evidence (2026-02-13): `cat docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md`

Post-X23 contract pin (X25):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-51-route-orchestration-single-entry-ssot.md`
- contract: `vm` / `vm-hako` / selfhost stage-a の route 判定入口を `route_orchestrator` へ一本化
- evidence (2026-02-13): `cargo test -q route_orchestrator -- --nocapture` PASS

Post-X23 contract pin (X26):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-52-vm-route-observability-contract-ssot.md`
- contract: `[vm-route/pre-dispatch]` + `[vm-route/select]` の語彙を固定し、legacy tag（`[vm-route] pre-dispatch`）を撤去
- evidence (2026-02-13): `phase29x_vm_route_observability_vm.sh` / `phase29x_vm_route_strict_dev_priority_vm.sh` / `phase29x_vm_route_non_strict_compat_boundary_vm.sh` PASS

Post-X23 contract pin (X27):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-53-vm-route-compat-bypass-failfast-ssot.md`
- contract: compat fallback 入口を `route_orchestrator` 所有に固定し、bypass は `[freeze:contract][vm-route/compat-bypass]` で fail-fast
- evidence (2026-02-13): `bash tools/checks/vm_route_bypass_guard.sh` / `phase29x_vm_route_compat_bypass_guard_vm.sh` PASS

Post-X23 contract pin (X28):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-54-vm-verifier-gate-single-entry-ssot.md`
- contract: verifier 入口を `common_util/verifier_gate` に一本化し、`NYASH_VM_VERIFY_MIR=1` failure は `[freeze:contract][vm-route/verifier-gate]` で fail-fast
- evidence (2026-02-13): `bash tools/checks/vm_verifier_gate_guard.sh` / `phase29x_vm_verifier_gate_single_entry_vm.sh` PASS

Post-X23 contract pin (X29):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-55-vm-safety-gate-single-entry-ssot.md`
- contract: safety 入口を `common_util/safety_gate` に一本化し、source/lifecycle 境界は `[freeze:contract][vm-route/safety-*]` で fail-fast
- evidence (2026-02-13): `bash tools/checks/vm_safety_gate_guard.sh` / `phase29x_vm_safety_gate_single_entry_vm.sh` PASS

Post-X23 contract pin (X30):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-56-thin-rust-core-cabi-min-surface-ssot.md`
- contract: Core C ABI 最小面（route/verifier/safety/lifecycle の6 symbol）を header/shim/doc で同期
- evidence (2026-02-13): `bash tools/checks/nyrt_core_cabi_surface_guard.sh` / `phase29x_core_cabi_surface_guard_vm.sh` PASS

Post-X23 contract pin (X31):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-57-thin-rust-gate-pack-ssot.md`
- contract: X24-X30 を `phase29x_thin_rust_gate_vm.sh` 1コマンドで再現し、gate pack を固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_thin_rust_gate_vm.sh` PASS

Lane D done:
- route / verifier / safety の責務が薄いRust層で一箇所化されている
- strict/dev で bypass なし、fail-fast 理由が観測できる

### Lane E: De-Rust transfer (X32-X36)

- X32: `.hako` route orchestrator skeleton 導入（Rust orchestrator と dual-run）
- X33: `.hako` verifier 実行経路導入（契約不一致は fail-fast）
- X34: `.hako` safety 実行経路導入（lifecycle 契約の runtime 境界を固定）
- X35: strict/dev 既定を `.hako` route へ切替（Rust thin は明示時のみ）
- X36: de-rust done 同期（証跡 + rollback 条件 + Rust optional 化の最終固定）

Post-X23 contract pin (X32):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-58-derust-route-orchestrator-skeleton-ssot.md`
- contract: `.hako` route skeleton (`lang/src/vm/route_orchestrator_skeleton.hako`) が Rust `route_orchestrator` と同じ lane 選択規則を持つ
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh` PASS（4ケース一致）

Post-X23 contract pin (X33):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-59-derust-verifier-path-ssot.md`
- contract: `.hako` verifier skeleton (`lang/src/vm/verifier_gate_skeleton.hako`) が Rust/Hako verifier 結果不一致を fail-fast で停止する
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh` PASS（mismatch fail-fast + match pass）

Post-X23 contract pin (X34):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-60-derust-safety-path-ssot.md`
- contract: `.hako` safety skeleton (`lang/src/vm/safety_gate_skeleton.hako`) が lifecycle 契約違反を fail-fast で停止する
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh` PASS（lifecycle fail-fast + clean pass）

Post-X23 contract pin (X35):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-61-derust-strict-default-route-cutover-ssot.md`
- contract: strict/dev 既定 route で `[derust-route/select] ... source=hako-skeleton` を観測し、Rust thin は明示時のみ `source=rust-thin-explicit`
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh` PASS

Post-X23 contract pin (X36):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`
- contract: de-rust done 判定（X32-X35）/ rollback 条件 / evidence 導線を 1 枚へ同期
- evidence (2026-02-13): `29x-62-derust-done-sync-ssot.md` 追加

Lane E progress:
- X32 完了（route 選択 parity を dual-run で固定）
- X33 完了（verifier mismatch fail-fast を `.hako` skeleton で固定）
- X34 完了（safety lifecycle fail-fast を `.hako` skeleton で固定）
- X35 完了（strict/dev 既定 route を `.hako` 契約へ切替）
- X36 完了（de-rust done sync 固定）

Lane E done:
- strict/dev 主経路が `.hako` route/verifier/safety で完結
- Rust thin lane は明示フラグ時のみ許可

### Lane F: Rust build retirement (X37-X40)

- X37: llvm+c-abi link gate 追加（`.hako -> LLVM -> C ABI link` の最小実行を固定）
- X38: daily/milestone から Rust runtime build 必須を外し、LLVM line gate を既定化
- X39: Rust lane を tools/compat 専用へ隔離（通常運用から分離）
- X40: llvm-only build done 同期（証跡 + rollback 条件 + 残存 Rust 依存一覧）

Lane F contract pin (X37):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md`
- contract: `phase29x_llvm_cabi_link_min.sh` で core C ABI surface guard + `.hako -> LLVM link` + linked exe run（exit 0, `42`）を同時固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh` PASS

Lane F contract pin (X38):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-64-llvm-only-daily-default-ssot.md`
- contract: `phase29x_llvm_only_daily_gate.sh` を daily/milestone の既定入口にし、`abi_lane_guard` + X37 gate を 1コマンドで固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh` PASS

Lane F contract pin (X39):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-65-rust-lane-optin-isolation-ssot.md`
- contract: Rust lane 入口を `tools/compat/phase29x_rust_lane_gate.sh` に隔離し、`PHASE29X_ALLOW_RUST_LANE=1` 明示時のみ実行を許可
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh` PASS

Lane F contract pin (X40):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-66-llvm-only-build-done-sync-ssot.md`
- contract: X37-X39 の done判定 / rollback条件 / 残存Rust依存一覧を 1枚で固定
- evidence (2026-02-13): `29x-66-llvm-only-build-done-sync-ssot.md` 追加

Lane F progress:
- X37 完了（LLVM+C ABI link の最小 gate を smoke で固定）
- X38 完了（daily/milestone 既定を LLVM-only gate へ切替）
- X39 完了（Rust lane を tools/compat + explicit opt-in へ隔離）
- X40 完了（llvm-only build done 判定を docs で同期）

Lane F done:
- 日常運用の build/gate が LLVM+C ABI line で完結
- Rust build は明示互換モード時のみ
- 「脱Rust一応完了（運用面）」を docs + evidence で固定

### Lane G: Post-29x cache build acceleration (X41-X46)

- X41: post-29x closeout sync（docs）
- X42: cache key determinism（CB-1）
- X43: L1 MIR cache（CB-2）
- X44: L2 object cache（CB-3）
- X45: L3 link cache（CB-4）
- X46: cache gate integration + done sync（CB-5）

Lane G contract pin (X41):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-67-post29x-cache-lane-sequencing-ssot.md`
- contract: X1-X40 の完了判定を保持したまま、X41-X46 導線を docs で一本化する

Lane G contract pin (X42):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-68-cache-key-determinism-ssot.md`
- contract: `phase29x_cache_keys.sh` の `ModuleCompileKey/ObjectKey/LinkKey` は同一入力で決定的、`profile` / `abi_boundary_digest` 差分で期待どおり変化する
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh` PASS

Lane G contract pin (X43):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-69-l1-mir-cache-ssot.md`
- contract: `phase29x_l1_mir_cache.sh` で module 単位 MIR/ABI artifact を保存し、`phase29x_l1_mir_cache_vm.sh` で `miss -> hit` を固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh` PASS

Lane G contract pin (X44):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-70-l2-object-cache-ssot.md`
- contract: `phase29x_l2_object_cache.sh` で object artifact を保存し、`phase29x_l2_object_cache_vm.sh` で `miss -> hit` と ABI差分 miss を固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh` PASS

Lane G contract pin (X45):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-71-l3-link-cache-ssot.md`
- contract: `phase29x_l3_link_cache.sh` で link artifact を保存し、`phase29x_l3_link_cache_vm.sh` で `miss -> hit` と runtime ABI 差分 miss を固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh` PASS

Lane G contract pin (X46):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-72-cache-gate-integration-done-sync-ssot.md`
- contract: `phase29x_llvm_only_daily_gate.sh` に `phase29x_cache_lane_gate_vm.sh` を統合し、X42-X45 cache contract を日次入口で再現可能に固定する
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh` PASS

Lane G progress:
- X41: done（post-29x closeout sync を docs で固定）
- X42: done（cache key determinism を guard+smoke で固定）
- X43: done（L1 MIR cache miss->hit 契約を guard+smoke で固定）
- X44: done（L2 object cache miss->hit + ABI差分 miss 契約を guard+smoke で固定）
- X45: done（L3 link cache miss->hit + runtime ABI差分 miss 契約を guard+smoke で固定）
- X46: done（daily/milestone に cache lane gate を統合し、done sync を docs で固定）

Lane G done:
- cache lane（X41-X46）は docs + guard + smoke で再現可能状態を固定
- daily/milestone 入口（`phase29x_llvm_only_daily_gate.sh`）で cache hit/miss 観測を常設化
- LLVM未対応ビルドでは cache lane（L2/L3/X46）は `can_run_llvm` で SKIP、LLVM対応ビルドでは full replay（miss->hit + ABI diff）を維持する契約へ統一

### Lane H: Post-X46 runtime handoff (X47-X53)

- X47: post-X46 handoff bootstrap（docs-only）
- X48: route pin inventory + guard（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0` の固定点管理）
- X49: vm-hako strict/dev replay gate 追加（rust-vm pin なしの観測レーン）
- X50: NewClosure contract lock（fail-fast 維持 or parity 実装を Decision 明記で固定）
- X51: Core C ABI delegation inventory + guard（非canonical 混入検出）
- X52: handoff gate integration（X48-X51 を 1コマンド化）
- X53: done sync + rollback lock（完了判定/戻し条件を docs 同期）

Lane H contract pin (X47):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md`
- contract: X47-X53 の順序、非目標（GC optional）、ABI境界（Core C ABI/TypeBox ABI v2）を固定

Lane H contract pin (X48):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md`
- contract: `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` の固定点を allowlist で管理し、guard/smoke で逸脱を fail-fast 固定
- evidence (2026-02-13): `bash tools/checks/phase29x_vm_route_pin_guard.sh` PASS

Lane H contract pin (X49):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md`
- contract: strict/dev + `--backend vm` を route pin override なしで replay し、`lane=vm-hako` の success/reject 2ケースを固定
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh` PASS

Lane H contract pin (X50):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md`
- contract: NewClosure は runtime fail-fast 維持（Decision: accepted）、compiler-side shape 契約のみ継続
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh` PASS

Lane H contract pin (X51):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md`
- contract: Core C ABI minimal 6 symbols の owner を `include/nyrt.h` / `src/abi/nyrt_shim.rs` に固定し、混入を fail-fast
- evidence (2026-02-13): `bash tools/checks/phase29x_core_cabi_delegation_guard.sh` PASS

Lane H contract pin (X52):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-79-runtime-handoff-gate-integration-ssot.md`
- contract: X48-X51 を `phase29x_runtime_handoff_gate_vm.sh` の 1コマンドへ統合し、wiring欠落を guard で fail-fast
- evidence (2026-02-13): `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh` PASS

Lane H contract pin (X53):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-74-postx46-runtime-handoff-done-sync-ssot.md`
- contract: done criteria（route pin / vm-hako replay / Core C ABI delegation / X52 single-entry replay）と rollback lock を固定
- evidence (2026-02-13): `bash tools/checks/phase29x_runtime_handoff_gate_guard.sh` PASS

Lane H progress:
- X47: done（post-X46 handoff sequencing docs を固定）
- X48: done（route pin inventory + guard を固定）
- X49: done（strict/dev vm-hako replay gate を固定）
- X50: done（NewClosure fail-fast contract を固定）
- X51: done（Core C ABI delegation inventory guard を固定）
- X52: done（runtime handoff gate integration を固定）
- X53: done（handoff done sync + rollback lock を固定）

### Lane I: Post-X53 runtime core extension (X54-X66)

- X54: lane bootstrap（docs-only）
- X55: vm-hako S6 vocabulary inventory + guard
- X56: vm-hako dual-run parity gate pack
- X57: NewClosure runtime lane decision refresh
- X58: S6 first vocabulary promotion
- X59: ABI borrowed/owned conformance extension
- X60: RC insertion phase2 queue lock
- X61: observability drift guard
- X62: runtime core integrated gate
- X63: optimization allowlist lock
- X64: optimization parity fixtures/reject fixtures
- X65: optimization gate integration + rollback lock
- X66: optional GC lane bootstrap（docs-only）

Lane I contract pin (X54):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md`
- contract: X54-X66 の固定順序、層境界、1コミット粒度を固定
- evidence (2026-02-13): `cat docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md`

Lane I contract pin (X55):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md`
- contract: vm-hako subset op inventory を allowlist+guard で固定し、語彙 drift を fail-fast 検出
- evidence (2026-02-13): `bash tools/checks/phase29x_vm_hako_s6_vocab_guard.sh` PASS

Lane I contract pin (X56):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md`
- contract: success/reject parity probe を `phase29x_vm_hako_s6_parity_gate_vm.sh` の single-entry に統合し、wiring 欠落を guard で fail-fast
- evidence (2026-02-13): `bash tools/checks/phase29x_vm_hako_s6_parity_gate_guard.sh` / `phase29x_vm_hako_s6_parity_gate_vm.sh` PASS

Lane I contract pin (X57):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md`
- contract: NewClosure runtime boundary は fail-fast 維持（Decision: accepted）とし、X56 parity gate を前提に decision drift を guard+gate で固定
- evidence (2026-02-13): `bash tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh` / `phase29x_vm_hako_newclosure_decision_refresh_vm.sh` PASS

Lane I contract pin (X58):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md`
- contract: S6 first vocabulary として `nop` を 1語彙だけ昇格し、X56 parity precondition + nop parity fixture を gate で固定
- evidence (2026-02-13): `bash tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh` / `phase29x_vm_hako_s6_nop_promotion_vm.sh` PASS

Lane I contract pin (X59):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md`
- contract: borrowed/owned matrix cases を case inventory + guard + single-entry gate で固定し、X51 Core C ABI delegation guard を前提実行する
- evidence (2026-02-13): `bash tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh` / `phase29x_abi_borrowed_owned_conformance_vm.sh` PASS

Lane I contract pin (X60):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md`
- contract: RC insertion phase2 queue（loop/call/early-exit）を case inventory + marker + guard + single-entry gate で固定し、X59 ABI gate を前提実行する
- evidence (2026-02-13): `bash tools/checks/phase29x_rc_phase2_queue_guard.sh` / `phase29x_rc_phase2_queue_lock_vm.sh` PASS

Lane I contract pin (X61):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-88-observability-drift-guard-ssot.md`
- contract: observability 5カテゴリ（handles/locals/temps/heap_fields/singletons）を inventory + guard + single-entry gate で固定し、X60 RC queue gate を前提実行する
- evidence (2026-02-13): `bash tools/checks/phase29x_observability_drift_guard.sh` / `phase29x_observability_drift_guard_vm.sh` PASS

Lane I contract pin (X62):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-89-runtime-core-integrated-gate-ssot.md`
- contract: runtime core hardening（X59 ABI + X60 RC + X61 observability）を `phase29x_runtime_core_gate_vm.sh` の single-entry gate へ統合し、wiring 欠落を guard で fail-fast 検出する
- evidence (2026-02-13): `bash tools/checks/phase29x_runtime_core_gate_guard.sh` / `phase29x_runtime_core_gate_vm.sh` PASS

Lane I contract pin (X63):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md`
- contract: optimization safe-set（`const_fold/dce/cfg_simplify`）を allowlist + guard + single-entry gate で固定し、X62 runtime core gate を前提実行する
- evidence (2026-02-13): `bash tools/checks/phase29x_optimization_allowlist_guard.sh` / `phase29x_optimization_allowlist_lock_vm.sh` PASS

Lane I contract pin (X64):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-93-optimization-parity-fixtures-lock-ssot.md`
- contract: optimization parity fixtures（const_fold/cfg）と reject fixture（division-by-zero）を inventory + guard + single-entry gate で固定し、X63 allowlist gate を前提実行する
- evidence (2026-02-13): `bash tools/checks/phase29x_optimization_parity_guard.sh` / `phase29x_optimization_parity_fixtures_vm.sh` PASS

Lane I contract pin (X65):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-94-optimization-gate-integration-rollback-lock-ssot.md`
- contract: optimization lane（X63 allowlist + X64 parity/reject）を `phase29x_optimization_gate_vm.sh` の single-entry gate へ統合し、`--no-optimize` rollback probe（expected rc=6）を lock する
- evidence (2026-02-13): `bash tools/checks/phase29x_optimization_gate_guard.sh` / `phase29x_optimization_gate_vm.sh` PASS

Lane I contract pin (X66):
- SSOT: `docs/development/current/main/phases/phase-29x/29x-95-optional-gc-lane-bootstrap-ssot.md`
- contract: optional GC lane は docs-only で bootstrap し、GC optional / semantics unchanged / non-goal（GC本体実装なし）を固定する
- evidence (2026-02-13): `cat docs/development/current/main/phases/phase-29x/29x-95-optional-gc-lane-bootstrap-ssot.md`

Lane I progress:
- X54: done（post-X53 extension sequencing docs を固定）
- X55: done（S6 vocabulary inventory + guard を固定）
- X56: done（dual-run parity gate pack を固定）
- X57: done（NewClosure runtime lane decision refresh を固定）
- X58: done（S6 first vocabulary promotion を固定）
- X59: done（ABI borrowed/owned conformance extension を固定）
- X60: done（RC insertion phase2 queue lock を固定）
- X61: done（observability drift guard を固定）
- X62: done（runtime core integrated gate を固定）
- X63: done（optimization allowlist lock を固定）
- X64: done（optimization parity fixtures/reject fixture を固定）
- X65: done（optimization gate integration + rollback lock を固定）
- X66: done（optional GC lane bootstrap（docs-only）を固定）

## 6. Why X18-X40 are mandatory

先行案（18 タスク）は runtime 機能追加に偏り、`vm` 既定ルートの主従切替が不足していた。
本フェーズでは VM route cutover を独立レーンとして管理する。

## 7. Acceptance (Phase 29x complete)

- RC insertion 規則 3 点が実装 + smoke で固定されている。
- observability 5カテゴリが実装 + smoke で固定されている。
- strict/dev で Rust fallback が暗黙発火しない。
- Rust-optional done 判定を docs + evidence で同期済み。
- thin-rust lane（X24-X31）が docs + smoke + evidence で固定されている。
- de-rust transfer lane（X32-X36）が docs + evidence で同期済み。
- Rust build retirement lane（X37-X40）が docs + gate + evidence で同期済み。
- X41-X46 は post-29x extension（cache lane）として別管理し、本 acceptance の必須条件には含めない。
- X47-X53 は post-X46 extension（runtime handoff lane）として別管理し、本 acceptance の必須条件には含めない。
- X54-X66 は post-X53 extension（runtime core lane）として別管理し、本 acceptance の必須条件には含めない。

## 8. Entry points

- Checklist: `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
- Task board: `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
- Post-X46 sequencing SSOT: `docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md`
- Post-X53 sequencing SSOT: `docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md`
