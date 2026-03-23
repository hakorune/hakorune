# Phase 29y (future, post self-host / docs-first): MIR lifecycle vocab freeze（RC/weak/ABI）

Status: Active (P0 complete, Y1/Y2 replayed, Y3 queue fixed, min1/min2/min3 done, RVP-3 complete, RVP-4 complete, RVP-5 complete; runtime pointer is `60-NEXT-TASK-PLAN.md`)
Scope: self-host 後に "脱Rustランタイム（NyRT/.hako）" を進める前提で、MIR の lifecycle/RC/weak を **どこまで語彙として固定**し、どこからを **runtime ABI（NyRT）**に委譲するかを SSOT 化する。

## Entry

- execution-lane parent SSOT: `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- 相談パケット（SSOT）: `docs/development/current/main/investigations/phase-29y-mir-lifecycle-vocab-consult.md`
- ~~次の指示書（P0, docs-only）~~ ✅ 完了: `docs/development/current/main/phases/phase-29y/P0-DOCS-FINALIZE-INSTRUCTIONS.md`
- 方針統合（GC + 実装順序）: `docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md`
- 次タスク予定表（Now/Next/Later）: `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- app-first 移植計画（Rust VM audit -> .hako VM port）:
  - `docs/development/current/main/phases/phase-29y/80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md`
  - `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
  - `docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md`
  - `docs/development/current/main/phases/phase-29y/83-VM-S0-REFACTOR-OUTSOURCE-INSTRUCTIONS.md`
  - `docs/development/current/main/phases/phase-29y/84-MODULE-REGISTRY-HYGIENE-SSOT.md`
- lane B binary-only 契約:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`（Binary-only `--hako-emit-mir-json` Contract）
- ring1 promotion template:
  - `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
  - dry-run task packs: `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`
- APP-1（Gate Log Summarizer）導線:
  - 指示書: `docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md`
  - app README: `apps/tools/gate_log_summarizer/README.md`
  - app smoke: `tools/smokes/v2/profiles/integration/apps/archive/gate_log_summarizer_vm.sh`
- APP-2（Controlflow Probe）導線:
  - app README: `apps/tools/controlflow_probe/README.md`
  - app smoke: `tools/smokes/v2/profiles/integration/apps/archive/controlflow_probe_vm.sh`
- APP-3（MIR Shape Guard）導線:
  - app README: `apps/tools/mir_shape_guard/README.md`
  - app smoke: `tools/smokes/v2/profiles/integration/mir_shape/mir_shape_guard_vm.sh`
  - stale-guard pin（non-gating）: `tools/smokes/v2/profiles/integration/apps/phase29y_continue_assignment_in_continue_stale_guard_vm.sh`
  - lane B blocked pin（non-gating）: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - lane B monitor pin（non-gating）: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
  - lane B monitor pin（non-gating）: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`

## Doc Roles (One-sheet)

- `10/20/30/40/50-*`:
  - phase29y core contract SSOT（ABI/RC/observability/optional-gc/lane-gate）。
- `60-NEXT-TASK-PLAN.md`:
  - runtime lane C の current/next fixed order 正本。
- `61-NEXT-TASK-HISTORY.md`:
  - 完了済み next-task の履歴アーカイブ（current blocker は持たない）。
- `70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md`:
  - APP 実装の外部委託/AI handoff 指示書。
- `80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md`:
  - capability 監査の方針SSOT。
- `81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`:
  - capability row（blocked/ported）の状態管理SSOT。
- `82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md`:
  - vm-hako `boxcall` method args 契約（subset/runtime共有）SSOT。
- `83-VM-S0-REFACTOR-OUTSOURCE-INSTRUCTIONS.md`:
  - `mir_vm_s0.hako` の BoxShape 分割を外部AIへ委託する固定指示書（挙動不変）。
- `84-MODULE-REGISTRY-HYGIENE-SSOT.md`:
  - `hako.toml` / `nyash.toml` の module registry 境界（top-only/override/duplicate）を固定する。
- `P0-DOCS-FINALIZE-INSTRUCTIONS.md`:
  - historical（完了済み）であり、current/next の正本ではない。

## Runtime Lane Next (SSOT pointer)

- runtime lane の current blocker / next fixed order は次を正本とする:
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- `CURRENT_TASK.md` には要約のみを残し、Next の重複転記はしない。
- この文書に current blocker / next task を重複記載しない（pointer only）。
- docs sync guard:
  - `tools/checks/phase29y_derust_blocker_sync_guard.sh`（`CURRENT_TASK.md` / `60-NEXT-TASK-PLAN.md` / `de-rust-lane-map-ssot.md` の blocker整合）
- debug procedure lock:
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` の `0.5 Debug Procedure Lock`

## Runtime Operation Policy (LLVM-first / vm-hako monitor)

- phase-29y の日常運用は `LLVM-first` を固定する（runtime 実行系の主検証は LLVM）。
- execution-lane parent vocabulary では、operational default は `llvm-exe`、`vm-hako` は reference/debug/bootstrap-proof lane、`rust-vm` は bootstrap/recovery/compat lane と読む。
- `vm-hako` は monitor-only lane とし、`vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh` fail など blocker 発生時のみ failure-driven で修正する。
- `rust-vm` は recovery/compat keep として読み、runtime daily feature lane へ戻さない。
- policy の単一正本は `docs/development/current/main/design/de-rust-lane-map-ssot.md` の `Runtime Operation Policy` とする。

## Next-Task Docs Contract (sprawl lock)

- `phase-29y` の active next-task 文書は `60-NEXT-TASK-PLAN.md` の1本だけに固定する。
- `70/80/81` は実装補助SSOT（指示書/監査/feature matrix）であり、Next 順序の正本ではない。
- `P0-DOCS-FINALIZE-INSTRUCTIONS.md` は historical（完了済み）として扱う。
- 追加の next-task 文書を新規作成せず、Next の更新は `60-NEXT-TASK-PLAN.md` へ追記する。

## Ring1 Promotion Boundary (pointer-only)

- ring1 provisional domain の昇格は `min1/min2/min3` 境界を固定する。
- 境界定義と domain別 dry-run checklist は次を正本とする:
  - `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
  - `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`
- 現在の ring1 status/next は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を正本とし、この README は pointer-only を維持する。

## Non-goals (Phase 29y ではやらない)

- MIR 命令語彙の大改造（所有モデルを型に埋め込む等）
- self-host 前の NyRT の .hako 化（実装は別フェーズ）
- GC/finalizer の新規実装（境界の明文化まで）

## Policy lock (2026-02-13)

- 実行順序は `D4 docs同期 -> D5-min1(MIR-first) -> 29y.1(ABI->RC->observability) -> vm-hako parity -> optimization -> optional GC` に固定する。
- lifecycle 意味論は GC 必須ではない（`fini` は決定的、物理解放タイミングは意味論外）。
- cycle collector は意味論要件ではなく、診断/最適化寄りの optional 機能として扱う。

## Deliverables (最大 3 つに切る)

Phase 29y を “締める” 条件は実装ではなく、次フェーズへ切れること。

1. **ABI SSOT**: NyRT ABI（最小セット）+ 関数 ABI（args borrowed / return owned など）を docs に固定
2. **RC insertion SSOT**: retain/release/weak_drop の発火点を “1箇所” に寄せる設計（Frag 前後）を docs に固定
3. **Observability SSOT**: hidden root を追える観測点（root面の定義、診断API、smokeは exit code SSOT）を docs に固定

Docs（Phase 29y 内の SSOT - ✅ P0 complete, all Ready）:
- ABI SSOT: `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md` ✅ Ready
- RC insertion SSOT: `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md` ✅ Ready
- Observability SSOT: `docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md` ✅ Ready
- Optional GC lane entry SSOT: `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md` ✅ Ready
- Lane gate SSOT: `docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md` ✅ Ready

## Implementation Pilot (Phase 29y.1)

Phase 29y は docs-first が主目的だが、後続の実装フェーズへ迷わず切るために “最小の導線” を先に用意する。

- Task 1: NyRT handle ABI shim（lifecycle） ✅ done (2026-02-11)
  - `crates/nyash_kernel/src/ffi/lifecycle.rs`
  - `crates/nyash_kernel/src/ffi/mod.rs`
  - `crates/nyash_kernel/src/lib.rs`
  - fixture: `apps/tests/phase29y_handle_abi.hako`
  - smokes: `tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_{vm,llvm}.sh`
  - evidence:
    - `cargo test -p nyash_kernel handle_lifecycle_ -- --nocapture` PASS
    - `bash tools/smokes/v2/profiles/integration/apps/archive/phase29y_handle_abi_vm.sh` PASS
    - `phase29y_handle_abi_llvm.sh` は `can_run_llvm` 判定へ修正し、LLVM非対応ビルドで SKIP 固定
    - `bash tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_borrowed_owned_vm.sh` PASS（Task1b: borrowed/owned conformance）
- Task 2: RC insertion pass 入口（skeleton, no-op） ✅ done (2026-02-11)
  - `src/mir/passes/rc_insertion.rs`
  - `src/mir/passes/mod.rs`
  - `src/mir/mod.rs`（compiler pipeline に接続）
  - fixture: `apps/tests/phase29y_rc_insertion_entry_noop_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_rc_insertion_entry_vm.sh`
  - evidence:
    - `bash tools/smokes/v2/profiles/integration/apps/phase29y_rc_insertion_entry_vm.sh` PASS
- Task 2b: RC insertion minimal（overwrite release 1-case） ✅ done (2026-02-11)
  - fixture: `apps/tests/phase29y_rc_insertion_overwrite_release_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29y_rc_insertion_overwrite_release_vm.sh`
  - evidence:
    - `bash tools/smokes/v2/profiles/integration/apps/archive/phase29y_rc_insertion_overwrite_release_vm.sh` PASS
  - stability note:
    - `phase29y_rc_insertion_overwrite_release_vm.sh` は現在 lane gate 必須stepからは切り離し、standalone diagnostic gate として維持する。
    - emit step は `NYASH_JOINIR_DEV=0` / `HAKO_JOINIR_STRICT=0` / `NYASH_USE_NY_COMPILER=0` を固定し、JoinIR dev strict の stack overflow ノイズを隔離する。
- Task 3: Observability root categories 拡張（handles + locals） ✅ done (2026-02-11)
  - `src/runtime/leak_tracker.rs`
  - smoke: `tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh`
  - evidence:
    - `bash tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh` PASS

## Current Recommendation (consultation summary)

- **実体**: RC の値と Alive/Dead/Freed 判定は runtime（NyRT）に置く
- **発火点**: retain/release/weak_drop は分散せず、CFG確定後の “1回だけ” の挿入パスで SSOT 化
- **関数ABI**: args borrowed / return owned（borrowed を保存/返す場合のみ retain）
- **weak identity**: alloc_id + generation token を SSOT 化（ログも token 表示）

## Next Steps（実装フェーズへ切るための最小タスク）

Phase 29y は docs-first を完了し、次フェーズ（Phase 29x/29z など）へ迷わず移れる状態になった。

以下の実装タスク（最大3つ）を次フェーズで進める:

1. **RC insertion pass の最小動作化** ✅ fixed（Task2b）
   - 結果: 上書き時 release（`x = <new>` 直前の旧値解放）を 1 ケースで pin
   - 契約: `phase29y_rc_insertion_overwrite_release_vm.sh` が `main` の `release_strong=1` を固定

2. **ABI borrowed/owned conformance smoke 追加** ✅ fixed（Task1b）
   - 結果: `phase29y_handle_abi_borrowed_owned_vm.sh` を追加し、`cargo test -p nyash_kernel handle_abi_borrowed_owned_conformance` を契約固定
   - 目的: 関数ABI契約（10-ABI-SSOT.md §3）の `args borrowed / return owned` を smoke 経由で継続検証

3. **Observability root categories 拡張** ✅ fixed
   - 結果: `leak_tracker.rs` の root summary を `handles + locals` へ拡張
   - 契約: limitation 文言を `temps/heap_fields/singletons` に縮小
   - smoke: `phase29y_observability_summary_vm.sh` で `handles + locals` 出力を pin

**受け入れ基準**:
- quick 154/154 PASS 維持
- integration smokes の phase29y_* が green 維持
- 恒常ログ増加なし

**Next Phase 候補**: Phase 29z（RC insertion minimal）または Phase 29x（De-Rust runtime）

## Y3 docs-first queue (fixed 2026-02-16, min1/min2/min3 done)

Y3（optional GC implementation queue docs-first 起票）は完了。実装は次の固定順序で 1 min task ずつ進める。

1. `min1`: GC mode boundary lock（wiring only, no behavior change）[done: 2026-02-16]
2. `min2`: Optional GC observability pin（dev/diagnostic only）[done: 2026-02-16]
3. `min3`: Optional GC pilot execution（guarded rollout）[done: 2026-02-16]

Non-negotiable:
- semantics invariance（`NYASH_GC_MODE=rc+cycle|off`）
- ABI fixed（`args borrowed / return owned`）
- RC insertion single-source

SSOT:
- `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md`（Section 8）

Optional GC lane entry:
- `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md`
- `bash tools/checks/phase29y_optional_gc_lane_entry_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`

Phase 29y core contracts gate:
- `bash tools/checks/phase29y_core_contracts_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh`

Phase 29y lane gate:
- `docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md`
- `bash tools/checks/phase29y_lane_gate_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## Related

- lifecycle semantics SSOT: `docs/reference/language/lifecycle.md`
- Phase 285（weak conformance / hidden root 根治）: `docs/development/current/main/phases/phase-285/README.md`
