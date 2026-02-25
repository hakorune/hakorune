---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: CURRENT_TASK.md から runtime lane の長大マイルストーン履歴（5.x）を分離し、再開導線を軽量化する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
---

# 29x-84: CURRENT_TASK Runtime Milestone Archive

## 0. Intent

- `CURRENT_TASK.md` の Quick Entry 可読性を優先し、詳細履歴は本アーカイブへ退避する。
- runtime lane の詳細証跡は `29x-91-task-board.md` を正本とし、本ファイルは移行時点の保全スナップショットを保持する。

## 1. Snapshot (migrated from CURRENT_TASK.md)

```text
5. Latest milestone (runtime lane)
   `D6-min44` fixed: `ArraySet` を `vm-hako` subset/runner/loader に追加し、register-slot shim（index は shape-only）で `ArrayGet` と整合させた。`phase29z_vm_hako_s5_array_set_parity_vm.sh` で JSON route parity（`rust-vm=42`, `hako-runner=42`）を契約化し、backend frame は phase 自動解決（`vm_hako_phase.sh`）へ更新。
5.5 Latest cleanup milestone (runtime lane)
   `D6-clean-10` fixed: `new_closure` の現状挙動を cross-route probe smoke（`phase29z_vm_hako_s5_newclosure_probe_vm.sh`）で固定し、rust-vm は `unsupported op 'new_closure' in mir_json_v0 loader`、hako-runner は `[vm-hako/unimplemented op=new_closure]` で fail-fast、両routeとも `rc=1` を契約化。
5.6 Latest compiler-lane milestone
   `RDN-0` fixed: `MirInstruction::DebugLog/Nop` を enum から retire。`__mir__.log/__mir__.mark` は `Debug` 列へ canonicalize、`mir_json_v0` loader は `debug_log` を `Debug` 列へ吸収し `nop` は lower-away。`backend/contracts/docs` を同期し、`cargo test -q instruction_diet_ledger_counts_match_docs_ssot -- --nocapture`、`cargo test -q instruction_diet_ledger_counts_match_ssot -- --nocapture`、`cargo test -q mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture`、`cargo check --bin hakorune` を再確認。
5.7 Latest compiler-lane cleanup milestone
   `RDN-1` fixed: `vm_hako` subset から legacy `nop`/`debug_log` 受理を撤去し unknown-op fail-fast へ統一。`mir-instruction-diet-ledger-ssot.md` を `kept=28/removed=16` に同期し、wasm codegen の no-op は `nop` 文字列ではなく空命令列へ canonical 化。`src/tests/mir_pure_e2e_vm.rs` には ring0 初期化ヘルパーを追加し、`mir_debug_minimal_printer_and_verifier` 単独実行の再起動後失敗を防止。
5.8 Latest compiler-lane cleanup milestone
   `NCL-0` fixed: `callsite_canonicalize` で `Call(callee=Closure)` を `NewClosure` へ正規化し、backend contracts で `call-closure-not-canonical` fail-fast を固定（JSON/VM allowlist 同期）。
5.9 Latest compiler-lane cleanup milestone
   `NCL-1` fixed: `NewClosure` に `body_id` を追加し、closure body は `MirModule.metadata.closure_bodies` へ外出し。builder emit と callsite canonicalization の両方で `body=[] + body_id=Some(id)` の薄い形を標準化。
5.10 Latest compiler-lane cleanup milestone
   `NCL-2` fixed: `Call(callee=Closure...)` の shape 判定を `src/mir/ssot/closure_call.rs` へ集約し、canonical 形（`dst=Some` + `args=[]`）のみ `NewClosure` へ正規化。非canonical 形は `call-closure-missing-dst` / `call-closure-runtime-args` で fail-fast 理由を固定。
5.11 Latest selfhost gate unblock
   `phase29bq_hako_mirbuilder_phase0_pin` fixed: v0 call emit で `func=4294967295` を無条件出力しないように調整し、`callee=Global/Extern/...` は v0互換形（`externcall` または `call + callee`）へ正規化。`hako_mirbuilder phase0 pin` / `phase29bq_fast_gate_vm --only bq` / `tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` を再緑化。
5.12 Latest runtime-lane milestone
   `X2/X3` fixed: RC explicit-drop（`x = null`）の契約を `apps/tests/phase29x_rc_explicit_drop_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh` で固定。次タスクは `X4`（scope-end release 設計）。
5.13 Latest runtime-lane milestone
   `X4/X5` fixed: RC scope-end release を `X4-min1`（Return 終端 cleanup）として docs で固定し、`apps/tests/phase29x_rc_scope_end_release_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh` で契約化。次タスクは `X6`（daily gate 契約同期）。
5.14 Latest runtime-lane milestone
   `X6` fixed: daily gate 契約同期を `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（5/5）で証跡化。次タスクは `X7`（early-exit cleanup 設計）。
5.15 Latest runtime-lane milestone
   `X7` fixed: early-exit cleanup（return/break/continue）の順序契約を `docs/development/current/main/phases/phase-29x/29x-20-early-exit-cleanup-ssot.md` で固定。次タスクは `X8`（return path 実装 + smoke）。
5.16 Latest runtime-lane milestone
   `X8` fixed: return path cleanup を `apps/tests/phase29x_rc_return_cleanup_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29x_rc_return_cleanup_vm.sh` で固定（baseline + `rc_insertion_selfcheck`）。
5.17 Latest runtime-lane milestone
   `X9` fixed: break path cleanup を `apps/tests/phase29x_rc_break_cleanup_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29x_rc_break_cleanup_vm.sh` で固定（baseline + `rc_insertion_selfcheck`）。次タスクは `X10`（continue path 実装 + smoke）。
5.18 Latest runtime-lane milestone
   `X10` fixed: continue path cleanup を `apps/tests/phase29x_rc_continue_cleanup_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29x_rc_continue_cleanup_vm.sh` で固定（baseline + `rc_insertion_selfcheck`）。
5.19 Latest runtime-lane milestone
   `X11` fixed: PHI/edge verifier を `src/mir/passes/rc_insertion.rs` に追加し、矛盾時は `[freeze:contract][rc_insertion/phi_edge_mismatch]` で fail-fast 固定。`src/bin/rc_insertion_selfcheck.rs` に edge-args/phi-input の負例を追加し、`tools/smokes/v2/profiles/integration/apps/phase29x_rc_phi_edge_verifier_vm.sh` でタグ観測を契約化。次タスクは `X12`（RC 3規則包括 smoke）。
5.20 Latest runtime-lane milestone
   `X12` fixed: RC 3規則包括 smoke を `tools/smokes/v2/profiles/integration/apps/phase29x_rc_three_rules_vm.sh` で追加し、`rc_insertion_selfcheck` の安定マーカー `[rc_three_rules] overwrite=ok explicit_drop=ok scope_end=ok` を契約化。次タスクは `X13`（observability 拡張設計）。
5.21 Latest runtime-lane milestone
   `X13` fixed: observability 拡張設計を `docs/development/current/main/phases/phase-29x/29x-30-observability-extension-ssot.md` で固定し、5カテゴリ語彙（`locals/temps/heap_fields/handles/singletons`）と X14-X17 の実装境界をSSOT化。次タスクは `X14`（temps 実装 + smoke）。
5.22 Latest runtime-lane milestone
   `X14` fixed: VM 実行中の strong temp root 最大値を `src/runtime/leak_tracker.rs` の `temps` へ接続し、`[lifecycle/leak]   temps: <n>` を root categories 契約へ追加。`tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh` で `temps>0` と limitation 行の `singletons=0` 収束を固定。次タスクは `X15`（heap_fields 実装 + smoke）。
5.23 Latest runtime-lane milestone
   `X15` fixed: `src/backend/mir_interpreter/mod.rs` の `obj_fields` から strong root 最大値を観測し、`src/runtime/leak_tracker.rs` の `heap_fields` へ接続。`tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh` で `heap_fields>0` を固定。次タスクは `X16`（singletons 実装 + smoke）。
5.24 Latest runtime-lane milestone
   `X16` fixed: `src/runtime/leak_tracker.rs` の `singletons` を runtime module globals から実測し、root categories を 5カテゴリ実測（`handles/locals/temps/heap_fields/singletons`）へ更新。`tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh` で `singletons>0` と legacy limitation alias 不在を固定。次タスクは `X17`（debug_root_summary 契約固定）。
5.25 Latest runtime-lane milestone
   `X17` fixed: `src/runtime/leak_tracker.rs` に `debug_root_summary()` を公開し、5カテゴリ語彙の固定順（`handles -> locals -> temps -> heap_fields -> singletons`）を出力契約化。`tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh` でカテゴリ一意性・順序・limitation line不在（VM lane）を固定。次タスクは `X18`（VM route cutover 設計）。
5.26 Latest runtime-lane milestone
   `X18` fixed: VM route cutover 設計を `docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md` で固定し、`vm / vm-hako / compat` の責務分割・切替順序（X19-X23）・fail-fast境界を明文化。次タスクは `X19`（route observability 追加）。
5.27 Latest runtime-lane milestone
   `X19` fixed: `src/runner/dispatch.rs` に安定タグ `[vm-route/select]` を追加し、`backend=vm(default)` / `backend=vm+fallback` / `backend=vm-hako` の分岐理由を 1行観測化。`tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh` で3経路タグを契約化。次タスクは `X20`（strict/dev で vm-hako 優先化）。
5.28 Latest runtime-lane milestone
   `X20` fixed: `src/runner/dispatch.rs` の `backend=vm` 分岐を strict/dev 優先で `lane=vm-hako` へ切替え、compat は `NYASH_VM_USE_FALLBACK=1` 明示時のみ許可。`src/config/env/vm_backend_flags.rs` に `vm_hako_prefer_strict_dev()` を追加し、`tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh` で strict/dev 優先 + compat 明示の契約を固定。`phase29x_vm_route_observability_vm.sh` は `lane=vm` 観測時に `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を使う契約へ更新。次タスクは `X21`（non-strict compat lane の限定縮退）。
5.29 Latest runtime-lane milestone
   `X21` fixed: `src/runner/selfhost.rs` の Stage-A runtime route で compat lane を non-strict 既定OFFに縮退し、`NYASH_VM_USE_FALLBACK=1` 明示時のみ許可する境界を追加。明示なしは `[contract][runtime-route][expected=mir-json] ... non_strict_compat=disabled` で fail-fast。`tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh` で reject/accept 双方を契約化。次タスクは `X22`（3日連続 gate green 証跡採取）。
5.30 Latest runtime-lane milestone
   `X22-min1` fixed: 3日連続 gate 証跡台帳を `docs/development/current/main/phases/phase-29x/29x-44-vm-route-three-day-gate-evidence.md` に追加し、Day1（2026-02-13）を PASS で記録。`tools/selfhost/run_stageb_compiler_vm.sh` と `phase29bq_selfhost_planner_required_dev_gate_vm.sh` は strict/dev でも Stage-B gate を Rust VM core lane へ固定するため `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を明示し、`./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` を再緑化。次タスクは `X22` Day2 証跡採取。
5.31 Latest runtime-lane milestone
   `X22-min2` fixed: Day2/Day3 用に gate 一括実行ヘルパー `tools/selfhost/record_phase29x_x22_evidence.sh` を追加。`cargo check` + route3 smoke + 5-case selfhost gate を順次実行して、証跡台帳へ貼る Markdown 行を自動出力する。Day1 の実測値は同スクリプト再実行結果（`stageb_total_secs=18`, `avg_case_secs=3.60`）へ同期。次タスクは `X22` Day2 証跡採取。
5.32 Latest runtime-lane milestone
   `X22-min3` fixed: 証跡台帳の品質チェック用に `tools/selfhost/check_phase29x_x22_evidence.sh` を追加。通常実行は進捗表示、`--strict` は Day1-3 の PASS + 日付一意/昇順を機械判定し、X23 へ進む前提を fail-fast で確認できる。`29x-44`/`29x-91` に導線を追加。次タスクは `X22` Day2 証跡採取。
5.33 Latest runtime-lane milestone
   `X23-min1` fixed: Rust-optional done 同期の前提を `29x-45-rust-optional-done-sync-ssot.md` に固定し、`tools/selfhost/check_phase29x_x23_readiness.sh`（`--strict` 対応）を追加。X22 strict gate・GC optional docs・X23 SSOT 導線を機械判定できるようにした。次タスクは `X22` Day2 証跡採取（完了後 X23 strict sync）。
5.34 Latest runtime-lane milestone
   `X22/X23` fixed: `tools/selfhost/record_phase29x_x22_evidence.sh` で Day2/Day3 を採取し、`29x-44-vm-route-three-day-gate-evidence.md` を Day1-3 PASS に同期。`tools/selfhost/check_phase29x_x22_evidence.sh --strict` と `tools/selfhost/check_phase29x_x23_readiness.sh --strict` を PASS させ、`README/29x-90/29x-91` を X23 完了状態へ更新。次タスクは `X24`（thin-rust boundary lock）。
5.35 Latest runtime-lane milestone
   `X24` fixed: `29x-50-thin-rust-boundary-lock-ssot.md` を追加し、route orchestration / verifier / safety の責務境界を SSOT で固定。`README/29x-90/29x-91` を同期して X24 を完了状態に更新。次タスクは `X25`（route orchestration 入口一本化）。
5.36 Latest runtime-lane milestone
   `X25` fixed: `src/runner/route_orchestrator.rs` を新設し、`vm` / `vm-hako` / selfhost stage-a の route 判定入口を集約。`dispatch.rs` の vm系分岐と `selfhost.rs` の stage-a compat/strict判定を orchestrator 経由へ置換し、`phase29x_vm_route_*` 3 smoke + `cargo test -q route_orchestrator -- --nocapture` を PASS。`29x-51-route-orchestration-single-entry-ssot.md` / `README/29x-90/29x-91` を同期。次タスクは `X26`（route observability 契約固定）。
5.37 Latest runtime-lane milestone
   `X26` fixed: route observability 契約を `29x-52-vm-route-observability-contract-ssot.md` で固定し、`[vm-route/pre-dispatch]` + `[vm-route/select]` を許可語彙として統一（legacy `"[vm-route] pre-dispatch"` を撤去）。`src/runner/mod.rs` の pre-dispatch 出力は `route_orchestrator` 経由へ集約し、`phase29x_vm_route_observability_vm.sh` / `phase29x_vm_route_strict_dev_priority_vm.sh` / `phase29x_vm_route_non_strict_compat_boundary_vm.sh` を語彙固定チェック付きで再緑化。次タスクは `X27`（compat bypass fail-fast 化）。
5.38 Latest runtime-lane milestone
   `X27` fixed: compat fallback 実行入口を `route_orchestrator` 所有に固定し、`vm_fallback` 入口へ `enforce_vm_compat_fallback_guard_or_exit("vm-fallback")` を追加。`tools/checks/vm_route_bypass_guard.sh` で callsite 所有と guard hook を機械検証し、`phase29x_vm_route_compat_bypass_guard_vm.sh` を追加して smoke 契約化。`29x-53-vm-route-compat-bypass-failfast-ssot.md` / `README/29x-90/29x-91` を同期。次タスクは `X28`（verifier gate 一本化）。
5.39 Latest runtime-lane milestone
   `X28` fixed: verifier 入口を `src/runner/modes/common_util/verifier_gate.rs` に一本化し、`vm` / `vm-fallback` / `vm-hako` は `enforce_vm_verify_gate_or_exit` 経由のみで `NYASH_VM_VERIFY_MIR` を扱う契約へ更新。failure は `[freeze:contract][vm-route/verifier-gate]` で fail-fast 固定。`tools/checks/vm_verifier_gate_guard.sh` と `phase29x_vm_verifier_gate_single_entry_vm.sh` を追加し、`29x-54-vm-verifier-gate-single-entry-ssot.md` / `README/29x-90/29x-91` を同期。次タスクは `X29`（safety gate 一本化）。
5.40 Latest runtime-lane milestone
   `X29` fixed: safety 入口を `src/runner/modes/common_util/safety_gate.rs` に一本化し、`vm` / `vm-fallback` の source boundary と `vm` / `vm-fallback` / `vm-hako` の lifecycle boundary を共通 gate へ集約。`release_strong(values=[])` は `[freeze:contract][vm-route/safety-lifecycle]` で fail-fast 固定し、direct fail-fast 実装を mode から撤去。`tools/checks/vm_safety_gate_guard.sh` と `phase29x_vm_safety_gate_single_entry_vm.sh` を追加し、`29x-55-vm-safety-gate-single-entry-ssot.md` / `README/29x-90/29x-91` を同期。次タスクは `X30`（thin-rust Core C ABI 最小面固定）。
5.41 Latest runtime-lane milestone
   `X30` fixed: Core C ABI 最小面を `route/verifier/safety/lifecycle` の 6 symbol に固定し、`include/nyrt.h` / `src/abi/nyrt_shim.rs` / `docs/reference/abi/nyrt_c_abi_v0.md` / `docs/reference/abi/ABI_BOUNDARY_MATRIX.md` を同期。`nyrt_verify_mir_json` / `nyrt_safety_check_mir_json` を shim へ追加し、lifecycle retain/release contract（`retain_h(0)=0`, `release_h(0)` no-op）を tests で固定。`tools/checks/nyrt_core_cabi_surface_guard.sh` と `phase29x_core_cabi_surface_guard_vm.sh` を追加し、`29x-56-thin-rust-core-cabi-min-surface-ssot.md` / `README/29x-90/29x-91` を同期。次タスクは `X31`（thin-rust gate pack 固定）。
5.42 Latest runtime-lane milestone
   `X31` fixed: X24-X30 の contract evidence を `tools/smokes/v2/profiles/integration/apps/phase29x_thin_rust_gate_vm.sh` に集約し、route/verifier/safety/cabi guard+smoke を 1コマンドで再現可能に固定。`29x-57-thin-rust-gate-pack-ssot.md` を追加し、`README/29x-90/29x-91` と完了状態を同期。次タスクは `X32`（`.hako` route orchestrator skeleton）。
5.43 Latest runtime-lane milestone
   `X32` fixed: `.hako` route skeleton（`lang/src/vm/route_orchestrator_skeleton.hako`）を追加し、Rust `route_orchestrator` との dual-run smoke（`phase29x_derust_route_dualrun_vm.sh`）で lane 選択一致を 4ケース固定。`29x-58-derust-route-orchestrator-skeleton-ssot.md` を追加し、`README/29x-90/29x-91` と進捗を同期。次タスクは `X33`（`.hako` verifier 経路導入）。
5.44 Latest runtime-lane milestone
   `X33` fixed: `.hako` verifier skeleton（`lang/src/vm/verifier_gate_skeleton.hako`）を追加し、Rust/Hako verifier 結果の不一致を fail-fast で停止する契約を `phase29x_derust_verifier_vm.sh` で固定。`29x-59-derust-verifier-path-ssot.md` を追加し、`README/29x-90/29x-91` と進捗を同期。次タスクは `X34`（`.hako` safety 経路導入）。
5.45 Latest runtime-lane milestone
   `X34` fixed: `.hako` safety skeleton（`lang/src/vm/safety_gate_skeleton.hako`）を追加し、lifecycle 契約違反 fail-fast（`reason=release_strong-empty-values`）を `phase29x_derust_safety_vm.sh` で固定。`29x-60-derust-safety-path-ssot.md` を追加し、`README/29x-90/29x-91` と進捗を同期。次タスクは `X35`（strict/dev 既定を `.hako` route へ切替）。
5.46 Latest runtime-lane milestone
   `X35` fixed: `route_orchestrator` に `[derust-route/select]` 観測タグを追加し、strict/dev 既定は `source=hako-skeleton`、Rust thin は `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 明示時のみ `source=rust-thin-explicit` を `phase29x_derust_strict_default_route_vm.sh` で固定。`29x-61-derust-strict-default-route-cutover-ssot.md` を追加し、`README/29x-90/29x-91` と進捗を同期。次タスクは `X36`（de-rust done 同期）。
5.47 Latest runtime-lane milestone
   `X36` fixed: de-rust done 同期を `29x-62-derust-done-sync-ssot.md` で固定し、完了判定（X32-X35）/ rollback 条件（`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`, `NYASH_VM_USE_FALLBACK=1`, `--backend vm-hako`）/ evidence 導線を `README/29x-90/29x-91` と一致させた。次タスクは `X37`（llvm+c-abi link gate 追加）。
5.48 Latest runtime-lane milestone
   `X37` fixed: LLVM+C ABI link gate を `29x-63-llvm-cabi-link-gate-ssot.md` で固定し、`tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh` を追加。最小 `.hako` fixture（`apps/tests/hello_simple_llvm.hako`）で `tools/build_llvm.sh` の link 成功 + linked exe 実行（`exit=0`, `42`）と `nyrt_core_cabi_surface_guard` PASS を同時契約化した。`README/29x-90/29x-91` を同期。次タスクは `X38`（daily/milestone を llvm line 既定へ切替）。
5.49 Latest runtime-lane milestone
   `X38` fixed: daily/milestone default を `29x-64-llvm-only-daily-default-ssot.md` で LLVM line 既定へ切替え、`tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh` を追加。`abi_lane_guard` + `phase29x_llvm_cabi_link_min` を 1コマンド実行へ集約し、`README/29x-90/29x-91` の daily/milestone 導線を同期した。次タスクは `X39`（Rust lane を tools/compat 専用へ隔離）。
5.50 Latest runtime-lane milestone
   `X39` fixed: Rust lane を `tools/compat/phase29x_rust_lane_gate.sh` へ隔離し、`PHASE29X_ALLOW_RUST_LANE=1` 明示時のみ実行可能に固定。`tools/compat/README.md` と `phase29x_rust_lane_optin_only.sh` を追加し、opt-in 必須（`[compat/optin-required]`）と明示時 dry-run PASS を契約化。`29x-65`/`README`/`29x-90`/`29x-91` を同期。次タスクは `X40`（llvm-only build done 同期）。
5.51 Latest runtime-lane milestone
   `X40` fixed: llvm-only build done 判定を `29x-66-llvm-only-build-done-sync-ssot.md` で最終同期し、X37-X39 の done criteria / rollback / 残存 Rust 依存一覧を 1枚で固定。`README`/`29x-90`/`29x-91` を X40 完了状態へ更新し、Phase 29x lane（X37-X40）を close 可能にした。次タスクは post-29x closeout（次フェーズ選定）。
5.52 Latest runtime-lane milestone
   `X41` fixed: post-29x closeout sync（docs）を完了し、`README/29x-90/29x-91/CURRENT_TASK` の導線を X41-X46（cache lane）順序へ同期。X1-X40 の完了判定は維持したまま、次タスクを `X42`（cache key determinism）へ固定。
5.53 Latest runtime-lane milestone
   `X42` fixed: cache key determinism（CB-1）を `tools/cache/phase29x_cache_keys.sh` と `tools/checks/phase29x_cache_key_determinism_guard.sh` で実装し、`phase29x_cache_key_determinism_vm.sh` で契約化。`Module/Object/Link` key の同一入力安定性と `profile/abi` 差分変化を固定。次タスクは `X43`（L1 MIR cache）。
5.54 Latest runtime-lane milestone
   `X43` fixed: L1 MIR cache（CB-2）を `tools/cache/phase29x_l1_mir_cache.sh` で実装し、`phase29x_l1_mir_cache_vm.sh` で `cache_status=miss -> hit` を契約化。module 単位 MIR/ABI artifact 保存（`target/hako-cache/v1/<profile>/<target>/mir|abi/<module-id>/...`）を固定。次タスクは `X44`（L2 object cache）。
5.55 Latest runtime-lane milestone
   `X44` fixed: L2 object cache（CB-3）を `tools/cache/phase29x_l2_object_cache.sh` で実装し、`phase29x_l2_object_cache_vm.sh` で `cache_status=miss -> hit` と ABI差分 miss を契約化。L2 は L1 前提（`l1_cache_status`）を保持しつつ object artifact を `target/hako-cache/v1/<profile>/<target>/obj/<module-id>/...` へ保存。次タスクは `X45`（L3 link cache）。
5.56 Latest runtime-lane milestone
   `X45` fixed: L3 link cache（CB-4）を `tools/cache/phase29x_l3_link_cache.sh` で実装し、`phase29x_l3_link_cache_vm.sh` で `cache_status=miss -> hit` と runtime ABI差分 miss を契約化。L3 は L2 前提（`l2_cache_status`）を保持しつつ link manifest/binary を `target/hako-cache/v1/<profile>/<target>/link|bin/<module-id>/<link-key>...` へ保存。次タスクは `X46`（cache gate integration + done sync）。
5.57 Latest runtime-lane milestone
   `X46` fixed: cache lane 統合 gate を `phase29x_cache_lane_gate_vm.sh` と `phase29x_cache_gate_integration_guard.sh` で追加し、`phase29x_llvm_only_daily_gate.sh` に X42-X45 の cache contract 再生を組み込んだ。`29x-72-cache-gate-integration-done-sync-ssot.md` を追加して `README/29x-90/29x-91/CURRENT_TASK` を X46 完了状態へ同期。次タスクは post-29x lane closeout 維持（daily/milestone 運用）。
5.58 Latest runtime-lane milestone
   `X46-min2 / D6-clean-12` fixed: `phase29x_l2_object_cache_vm.sh` / `phase29x_l3_link_cache_vm.sh` / `phase29x_cache_lane_gate_vm.sh` を `can_run_llvm` guard 付きへ統一し、LLVM未対応ビルドでは SKIP、対応ビルドでは cache contract replay を維持する契約へ修正。併せて `phase29bq_fast_gate_vm.sh` / `joinir_planner_first_gate.sh` / `vm_hako` driver・newclosure probe の実行経路を `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` で rust-vm lane に固定し、strict/dev route drift を抑止。`MirInstruction::NewClosure` は MIR JSON allowlist + emitter で canonical `mir_call(callee=Closure)` へ出力可能に同期。次タスクは post-29x lane closeout 維持（daily/milestone 運用）。
5.59 Latest runtime-lane milestone
   `X47/X48` fixed: post-X46 handoff bootstrap を `29x-73-postx46-runtime-handoff-sequencing-ssot.md` で固定し、route pin inventory を `29x-75-vm-route-pin-inventory-guard-ssot.md` + `tools/checks/phase29x_vm_route_pin_allowlist.txt` + `phase29x_vm_route_pin_guard.sh` で機械検証化。`phase29x_vm_route_pin_guard_vm.sh` を追加して smoke 契約を固定し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X48 完了状態へ同期。次タスクは `X49`（vm-hako strict/dev replay gate）。
5.60 Latest runtime-lane milestone
   `X49` fixed: `phase29x_vm_hako_strict_dev_replay_vm.sh` を追加し、strict/dev + `--backend vm` を route pin override なしで replay する gate を固定。supported fixture（`rc=42`）と reject fixture（`[vm-hako/unimplemented]` + non-zero）で first route tag を `lane=vm-hako reason=strict-dev-prefer` に pin。`29x-76-vm-hako-strict-dev-replay-gate-ssot.md` を追加し、`README/29x-90/29x-91/CURRENT_TASK` を X49 完了状態へ同期。次タスクは `X50`（NewClosure contract lock）。
5.61 Latest runtime-lane milestone
   `X50` fixed: `29x-77-newclosure-contract-lock-ssot.md` で Decision=`accepted`（runtime fail-fast 維持）を固定し、`phase29x_vm_hako_newclosure_contract_vm.sh` を追加。`cargo test -q mir_json_allowlist_accepts_new_closure` と `phase29z_vm_hako_s5_newclosure_probe_vm.sh` を 1 gate へ統合し、compiler-side shape 受理 + runtime fail-fast（rust-vm unsupported / hako-runner unimplemented）を同時契約化。`README/29x-90/29x-91/CURRENT_TASK` を X50 完了状態へ同期。次タスクは `X51`（Core C ABI delegation inventory + guard）。
5.62 Latest runtime-lane milestone
   `X51` fixed: Core C ABI delegation owner を `tools/checks/phase29x_core_cabi_delegation_allowlist.txt`（`include/nyrt.h`, `src/abi/nyrt_shim.rs`）へ固定し、`phase29x_core_cabi_delegation_guard.sh` を追加。`abi_lane_guard` / `nyrt_core_cabi_surface_guard` を前提に、minimal 6 symbols の非canonical owner 混入を fail-fast 検出する契約を `29x-78-core-cabi-delegation-inventory-guard-ssot.md` と `phase29x_core_cabi_delegation_guard_vm.sh` で固定。`README/29x-90/29x-91/CURRENT_TASK` を X51 完了状態へ同期。次タスクは `X52`（handoff gate integration）。
5.63 Latest runtime-lane milestone
   `X52` fixed: `29x-79-runtime-handoff-gate-integration-ssot.md` を追加し、`phase29x_runtime_handoff_gate_vm.sh`（X48-X51 連結）と `phase29x_runtime_handoff_gate_guard.sh`（wiring 欠落 fail-fast）を導入。`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X52 完了状態へ同期し、次タスクを `X53`（post-X46 handoff done sync + rollback lock）へ更新。
5.64 Latest runtime-lane milestone
   `X53` fixed: `29x-74-postx46-runtime-handoff-done-sync-ssot.md` を Decision=`accepted` で確定し、done criteria（route pin / vm-hako replay / Core C ABI delegation / X52 single-entry replay）と rollback lock（strict/dev pin override・compat opt-in・rust lane opt-in）を固定。`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X53 完了状態へ同期し、post-X46 runtime handoff lane を close。
5.65 Latest runtime-lane milestone
   `X54` fixed: `29x-80-postx53-runtime-core-sequencing-ssot.md` を追加し、post-X53 extension（X54-X66）の固定順序（VM parity extension -> runtime core hardening -> optimization -> optional GC）と1タスク1コミット粒度を lock。`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X54 完了状態へ同期し、次タスクを `X55`（S6 vocabulary inventory + guard）へ更新。
5.66 Latest runtime-lane milestone
   `X55` fixed: `tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt` と `phase29x_vm_hako_s6_vocab_guard.sh` を追加し、`check_vm_hako_subset_json` の op inventory 集合を allowlist と一致検証する契約を固定。`phase29x_vm_hako_s6_vocab_guard_vm.sh` と `29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X55 完了状態へ同期。次タスクは `X56`（dual-run parity gate pack）。
5.67 Latest runtime-lane milestone
   `X56` fixed: `phase29x_vm_hako_s6_parity_gate_vm.sh` と `phase29x_vm_hako_s6_parity_gate_guard.sh` を追加し、X55 inventory guard + S5 success/reject parity probes（array_get/array_set/await_non_future/backend_frame）を single-entry gate で再生する契約を固定。`29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X56 完了状態へ同期。次タスクは `X57`（NewClosure runtime lane decision refresh）。
5.68 Latest runtime-lane milestone
   `X57` fixed: `phase29x_vm_hako_newclosure_decision_guard.sh` と `phase29x_vm_hako_newclosure_decision_refresh_vm.sh` を追加し、X50/X57 Decision=`accepted`・S6 allowlist 非昇格（`new_closure` 非収載）・X56 parity 前提を 1 gate で再生する契約を固定。`29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X57 完了状態へ同期。次タスクは `X58`（S6 first vocabulary promotion）。
5.69 Latest runtime-lane milestone
   `X58` fixed: `nop` を S6 first vocabulary として昇格し、`check_vm_hako_subset_json` 受理 + allowlist 追加を `phase29x_vm_hako_s6_nop_promotion_guard.sh` で固定。`phase29x_vm_hako_s6_nop_promotion_vm.sh` を追加し、X56 parity precondition + `phase29z_vm_hako_s3_nop_parity_vm.sh`（route pin helper）を single-entry gate 化。`29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X58 完了状態へ同期。次タスクは `X59`（ABI borrowed/owned conformance extension）。
5.70 Latest runtime-lane milestone
   `X59` fixed: `args borrowed / return owned` の matrix を `tools/checks/phase29x_abi_borrowed_owned_matrix_cases.txt` に固定し、`crates/nyash_kernel/src/tests.rs` へ 3ケース（basic escape / multi-escape chain / invalid handle no-op）を同期。`phase29x_abi_borrowed_owned_matrix_guard.sh` と `phase29x_abi_borrowed_owned_conformance_vm.sh` を追加し、X51 Core C ABI delegation guard を前提に matrix cargo test（`handle_abi_borrowed_owned_`）を single-entry gate 化。`29x-86-abi-borrowed-owned-conformance-extension-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X59 完了状態へ同期。次タスクは `X60`（RC insertion phase2 queue lock）。
5.71 Latest runtime-lane milestone
   `X60` fixed: RC insertion phase2 queue（loop/call/early-exit）契約を `tools/checks/phase29x_rc_phase2_queue_cases.txt` と `src/bin/rc_insertion_selfcheck.rs` の安定 marker で固定。`phase29x_rc_phase2_queue_guard.sh` と `phase29x_rc_phase2_queue_lock_vm.sh` を追加し、X59 ABI gate（`phase29x_abi_borrowed_owned_conformance_vm.sh`）を前提 step にした single-entry replay を導入。`29x-87-rc-insertion-phase2-queue-lock-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X60 完了状態へ同期。次タスクは `X61`（observability drift guard）。
5.72 Latest runtime-lane milestone
   `X61` fixed: observability root categories（`handles/locals/temps/heap_fields/singletons`）の inventory を `tools/checks/phase29x_observability_categories.txt` に固定し、`phase29x_observability_summary_vm.sh` を inventory 参照へ更新。`phase29x_observability_drift_guard.sh` と `phase29x_observability_drift_guard_vm.sh` を追加し、X60 precondition + X14-X17 observability replay を single-entry gate 化。`29x-88-observability-drift-guard-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X61 完了状態へ同期。次タスクは `X62`（runtime core integrated gate）。
5.73 Latest runtime-lane milestone
   `X62` fixed: runtime core hardening（X59 ABI / X60 RC / X61 observability）の replay を `phase29x_runtime_core_gate_vm.sh` に統合し、`phase29x_runtime_core_gate_guard.sh` で dependency wiring を fail-fast 検証できる形へ固定。`29x-89-runtime-core-integrated-gate-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X62 完了状態へ同期。次タスクは `X63`（optimization allowlist lock）。
5.74 Latest runtime-lane milestone
   `X63` fixed: optimization safe-set vocabulary を `tools/checks/phase29x_optimization_allowlist.txt`（`const_fold/dce/cfg_simplify`）へ固定し、`src/mir/optimizer.rs` に `PHASE29X_OPT_SAFESET` と `mir_optimizer_phase29x_allowlist_lock` テストを追加。`phase29x_optimization_allowlist_guard.sh` と `phase29x_optimization_allowlist_lock_vm.sh` を追加し、X62 precondition + allowlist cargo test の single-entry gate を固定。`29x-92-optimization-allowlist-lock-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X63 完了状態へ同期。次タスクは `X64`（optimization parity fixtures/reject fixtures）。
5.75 Latest runtime-lane milestone
   `X64` fixed: optimization parity fixture inventory（`phase29x_optimization_parity_fixtures.txt`）と reject fixture inventory（`phase29x_optimization_reject_fixtures.txt`）を追加し、`phase29x_optimization_parity_guard.sh` で docs/gate/wiring drift を fail-fast 検証できる形に固定。`phase29x_optimization_parity_fixtures_vm.sh` を追加し、X63 precondition のうえで pre/post parity（`const_fold/cfg`）と reject parity（`division-by-zero` non-zero + failure text一致）を single-entry gate 化。`29x-93-optimization-parity-fixtures-lock-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X64 完了状態へ同期。次タスクは `X65`（optimization gate integration + rollback lock）。
5.76 Latest runtime-lane milestone
   `X65` fixed: optimization lane 統合 guard/gate（`phase29x_optimization_gate_guard.sh`, `phase29x_optimization_gate_vm.sh`）を追加し、X63 allowlist lock + X64 parity/reject fixture lock を single-entry replay に統合。`--no-optimize` rollback probe（`phase29x_optimization_parity_const_fold_min.hako` expected rc=6/stdout=6）を gate に固定して rollback lock を可観測化。`29x-94-optimization-gate-integration-rollback-lock-ssot.md` を追加し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X65 完了状態へ同期。次タスクは `X66`（optional GC lane bootstrap, docs-only）。
5.77 Latest runtime-lane milestone
   `X66` fixed: `29x-95-optional-gc-lane-bootstrap-ssot.md` を追加し、optional GC lane の docs-only 入口（GC optional / semantics unchanged / non-goal）を固定。Phase 29y SSOT（`README`, `10-ABI`, `20-RC-INSERTION`, `30-OBSERVABILITY`）と runtime-gc-policy SSOT への handoff 導線を一本化し、`README/29x-90/29x-91/10-Now/CURRENT_TASK` を X66 完了状態へ同期。次タスクは Phase 29y docs-first handoff（optional GC implementation planning）。
6. Runtime lane milestone archive
   `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md` の `D6 progress` を正本として参照（CURRENT_TASK への重複転記は停止）

```

## 2. Canonical ongoing log

- Runtime lane ongoing evidence: `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
- Quick restart pointer: `CURRENT_TASK.md`
