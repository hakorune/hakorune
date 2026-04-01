---
Status: Active
Scope: Phase 29x の詳細タスク台帳（依存関係・受け入れ基準・証跡コマンド）。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
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
  - docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
---

# Phase 29x — Task Board (Detailed)

## 0. Current task tables

This section is the current docs-first cleanup queue. Historical X-series tasks remain below as landed ledger/history.

| Wave | Status | Goal | Owner doc |
| --- | --- | --- | --- |
| `W1 docs-first path-truth pass` | active | lock target buckets, names, and move order | `29x-99-structure-recut-wave-plan-ssot.md` |
| `W2 mixed-file split pass` | next | split owner-looking mixed files before behavior changes | `29x-99-structure-recut-wave-plan-ssot.md` |
| `W3 smoke/proof filesystem recut` | pending | move live proof and archive evidence into semantic homes | `29x-99-structure-recut-wave-plan-ssot.md` |
| `W4 Hako-side caller drain prep` | blocked-on-proof | exact root-first replacement proof for direct `.hako` callers | `29x-98-legacy-route-retirement-investigation-ssot.md` + `29x-99-structure-recut-wave-plan-ssot.md` |
| `W5 Rust compat receiver collapse` | pending-after-W4 | reduce legacy Rust receiver spread to one compat chokepoint | `29x-99-structure-recut-wave-plan-ssot.md` |
| `W6 final delete/archive sweep` | pending-after-W5 | delete helpers only after caller inventory reaches zero | `29x-98-legacy-route-retirement-investigation-ssot.md` + `29x-99-structure-recut-wave-plan-ssot.md` |

## 0.1 Current micro-task queue

| ID | Wave | Status | Task | Acceptance |
| --- | --- | --- | --- | --- |
| `99A` | W1 | landed | `phase2044` semantic bucket docs/manifest lock | llvmlite trio is the final live keep bucket |
| `99B` | W1 | landed | `phase2120` keep/historical docs + suite split lock | pure keep and pure historical are canonical |
| `99C` | W1 | landed | compat selfhost stack wording lock | `payload -> transport wrapper -> pack orchestrator` is fixed |
| `99D` | W1 | landed | direct caller vs wrapper inventory lock | `29x-98` separates direct callers from wrappers |
| `99E` | W1 | active | split-target inventory lock | target split homes are fixed for mixed owner/compat surfaces |
| `99F` | W1 | active | file-move / shim order lock | docs say move-first, shim-second, delete-last |
| `99E1` | W1 | active | lock `extern_provider.hako` split target | runtime owner and compat codegen stub are separated on paper |
| `99E2` | W1 | active | lock `llvm_codegen.rs` split target | thin tool boundary and legacy MIR front door are separated on paper |
| `99E3` | W1 | active | lock `LlvmBackendBox` split target | owner API and evidence adapter are separated on paper |
| `99E4` | W1 | active | lock compat proof-box homes | `CodegenBridgeBox` / `LLVMEmitBox` leave owner-looking paths on paper |
| `99F1` | W1 | active | move payload / wrapper paths first | compat payload and wrapper homes move before behavior changes |
| `99F2` | W1 | active | move proof boxes with re-export only | old paths keep thin shims only if required |
| `99F3` | W1 | active | keep behavior stable until references update | discovery / runner imports stay green before delete |
| `99F4` | W1 | active | delete old entrypoints last | only after caller inventory and archive evidence are explicit |
| `99G` | W2 | landed | split `extern_provider.hako` | runtime owner and compat codegen shim no longer share one file |
| `99H` | W2 | landed | split `llvm_codegen.rs` | thin tool boundary and legacy MIR front door no longer share one file |
| `99I` | W2 | landed | split `LlvmBackendBox` | owner API and evidence adapter no longer share one file |
| `99J` | W2 | active | move `CodegenBridgeBox` / `LLVMEmitBox` | compat/proof surfaces leave owner-looking paths |
| `99K` | W3 | pending | recut `phase2044` physically | semantic proof buckets get separate homes |
| `99L` | W3 | pending | recut `phase2120` physically | semantic proof/history buckets get separate homes |
| `99M` | W3 | pending | bundle archive proof surfaces semantically | archive replay evidence reads as one bundle |
| `99G1` | W3 | pending | suites / directory semantic recut | phase-number homes are replaced by semantic homes in the proof/archive tree |
| `99N-99P` | W4 | blocked-on-proof | exact root-first replacement proof and Hako-side caller drain prep | direct `.hako` callers can leave `CodegenBridgeBox.emit_object_args(...)` |
| `99Q-99S` | W5 | pending-after-W4 | Rust compat receiver collapse | receiver spread is reduced to one chokepoint |
| `99T-99V` | W6 | pending-after-W5 | final helper deletion and archive sweep | legacy helpers are deleted after zero callers |

| ID | Lane | Task | Est. | Depends | Acceptance |
| --- | --- | --- | --- | --- | --- |
| X1 | docs | 29x docs bootstrap（README/checklist/board） | 30m | - | 3ファイル作成 + 入口リンク更新 |
| X2 | rc | explicit-drop 設計 + fixture | 2h | X1 | `apps/tests/phase29x_rc_explicit_drop_min.hako` で `x=null` 契約固定 |
| X3 | rc | explicit-drop 実装 + smoke | 2h | X2 | `phase29x_rc_explicit_drop_vm.sh` PASS |
| X4 | rc | scope-end release 設計 | 2h | X3 | X4-min1（Return 終端 cleanup）を docs で固定 |
| X5 | rc | scope-end release 実装 + smoke | 3h | X4 | `phase29x_rc_scope_end_release_vm.sh` PASS |
| X6 | ops | daily gate 契約同期 | 30m | X5 | 29x checklist と daily 3コマンド整合 |
| X7 | rc | early-exit cleanup 設計 | 4h | X6 | `29x-20-early-exit-cleanup-ssot.md` で順序契約固定 |
| X8 | rc | return path 実装 + smoke | 4h | X7 | `phase29x_rc_return_cleanup_vm.sh` PASS |
| X9 | rc | break path 実装 + smoke | 4h | X8 | `phase29x_rc_break_cleanup_vm.sh` PASS |
| X10 | rc | continue path 実装 + smoke | 4h | X9 | `phase29x_rc_continue_cleanup_vm.sh` PASS |
| X11 | rc | PHI/edge verifier 追加 | 4h | X10 | 矛盾時 fail-fast タグが出る |
| X12 | rc | RC 3規則包括 smoke | 2h | X11 | `phase29x_rc_three_rules_vm.sh` PASS |
| X13 | obs | observability 拡張設計 | 4h | X12 | 5カテゴリの観測契約が docs 一致 |
| X14 | obs | temps 実装 + smoke | 4h | X13 | `phase29x_observability_temps_vm.sh` PASS |
| X15 | obs | heap_fields 実装 + smoke | 4h | X14 | `phase29x_observability_heap_fields_vm.sh` PASS |
| X16 | obs | singletons 実装 + smoke | 3h | X15 | `phase29x_observability_singletons_vm.sh` PASS |
| X17 | obs | debug_root_summary 契約固定 | 3h | X16 | `phase29x_observability_summary_vm.sh` PASS |
| X18 | vm-route | VM route cutover 設計 | 4h | X17 | `vm`/`vm-hako`/compat 責務を明文化 |
| X19 | vm-route | route observability 追加 | 3h | X18 | `[vm-route/*]` 安定タグで分岐可視化 |
| X20 | vm-route | strict/dev で vm-hako 優先化 | 4h | X19 | strict/dev で暗黙 Rust fallback 0 |
| X21 | vm-route | non-strict compat 限定縮退 | 4h | X20 | compat は明示条件付きのみ |
| X22 | vm-route | 3日連続 gate evidence 採取 | 3d | X21 | daily gate 連続 PASS 記録 |
| X23 | docs | Rust-optional done 同期 | 2h | X22 | 判定基準 + 証跡リンク更新 |
| X24 | docs | thin-rust boundary lock | 2h | X23 | route/verifier/safety 責務を SSOT で固定 |
| X25 | vm-route | route orchestration 入口一本化 | 4h | X24 | 単一 orchestrator 以外の直配線が残らない |
| X26 | vm-route | route observability 契約固定 | 3h | X25 | `[vm-route/*]` タグで分岐理由が一意に観測できる |
| X27 | vm-route | compat bypass fail-fast 化 | 4h | X26 | strict/dev の暗黙 fallback が 0 |
| X28 | verifier | verifier gate 一本化 | 4h | X27 | backend ごとの verify 入口重複がない |
| X29 | safety | safety gate 一本化 | 4h | X28 | lifecycle/unsafe fail-fast 契約が 1 箇所に固定 |
| X30 | abi | thin-rust Core C ABI 最小面固定 | 4h | X29 | route/verify/safety 境界を C ABI docs/header で同期 |
| X31 | ops | thin-rust gate pack 固定 | 3h | X30 | thin-rust smoke pack + docs evidence が green |
| X32 | de-rust | `.hako` route orchestrator skeleton | 4h | X31 | dual-run で route 選択結果が一致 |
| X33 | de-rust | `.hako` verifier 経路導入 | 4h | X32 | verifier 不一致が fail-fast で止まる |
| X34 | de-rust | `.hako` safety 経路導入 | 4h | X33 | lifecycle 契約違反が fail-fast で止まる |
| X35 | de-rust | strict/dev 既定を `.hako` route へ切替 | 4h | X34 | strict/dev 既定で Rust thin 不要 |
| X36 | docs | de-rust done 同期 | 3h | X35 | done 判定 + rollback 条件 + 証跡リンク更新 |
| X37 | llvm-line | llvm+c-abi link gate 追加 | 4h | X36 | `.hako -> LLVM -> C ABI link` 最小ケースが smoke で固定 |
| X38 | ops | daily/milestone を llvm line 既定へ切替 | 3h | X37 | 日常コマンドに Rust runtime build 必須が残らない |
| X39 | ops | Rust lane を tools/compat 専用へ隔離 | 4h | X38 | 通常運用では Rust lane 非経由、明示時のみ許可 |
| X40 | docs | llvm-only build done 同期 | 3h | X39 | 証跡 + rollback 条件 + 残存 Rust 依存一覧を固定 |
| X41 | docs | post-29x closeout sync | 2h | X40 | X1-X40 完了判定を保持したまま X41-X46 導線を同期 |
| X42 | cache | cache key determinism（CB-1） | 4h | X41 | module/object/link key が同一入力で決定的 |
| X43 | cache | L1 MIR cache（CB-2） | 4h | X42 | module 単位 MIR/ABI 再利用が観測できる |
| X44 | cache | L2 object cache（CB-3） | 4h | X43 | object 再生成が必要差分時のみに限定される |
| X45 | cache | L3 link cache（CB-4） | 4h | X44 | link input 不変時は link 再実行を回避できる |
| X46 | ops | cache gate integration + done sync（CB-5） | 3h | X45 | daily/milestone に cache hit/miss 観測を固定 |
| X47 | docs | post-X46 runtime handoff bootstrap | 2h | X46 | X47-X53 の順序/非目標/入口を docs で固定 |
| X48 | vm-route | route pin inventory + guard | 4h | X47 | `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 固定点の増殖を guard で検出 |
| X49 | vm-hako | strict/dev replay gate 追加 | 4h | X48 | rust-vm pin なしの vm-hako replay が単独 gate で観測可能 |
| X50 | vm-hako | NewClosure contract lock | 4h | X49 | fail-fast維持 or parity実装を Decision 明記で固定 |
| X51 | abi | Core C ABI delegation inventory + guard | 4h | X50 | 非canonical ABI 呼び出し混入を guard で fail-fast 検出 |
| X52 | ops | handoff gate integration | 3h | X51 | X48-X51 を 1コマンドで再生可能に固定 |
| X53 | docs | post-X46 handoff done sync | 2h | X52 | 完了判定 + rollback 条件 + 残課題（GC optional）を同期 |
| X54 | docs | post-X53 runtime core extension bootstrap | 2h | X53 | X54-X66 の層順序/粒度/非目標を docs で固定 |
| X55 | vm-hako | S6 vocabulary inventory + guard | 4h | X54 | S6 語彙 inventory と drift guard を固定 |
| X56 | vm-hako | dual-run parity gate pack | 4h | X55 | success/reject parity を 1コマンドで再生可能に固定 |
| X57 | vm-hako | NewClosure runtime lane decision refresh | 4h | X56 | execute/fail-fast 境界を Decision 明記で固定 |
| X58 | vm-hako | S6 first vocabulary promotion | 4h | X57 | 1語彙追加を fixture+gate で固定 |
| X59 | abi | borrowed/owned conformance extension | 4h | X58 | ABI conformance matrix を gate で継続検証 |
| X60 | rc | RC insertion phase2 queue lock | 4h | X59 | loop/call/early-exit の挿入順序を SSOT 固定 |
| X61 | obs | observability drift guard | 3h | X60 | 5カテゴリ drift を guard で fail-fast 検出 |
| X62 | ops | runtime core integrated gate | 3h | X61 | ABI + RC + observability を single-entry で再生 |
| X63 | opt | optimization allowlist lock | 3h | X62 | const-fold/DCE/CFG 以外を deny で固定 |
| X64 | opt | optimization parity fixtures | 4h | X63 | pre/post parity + reject fixture を固定 |
| X65 | ops | optimization gate integration + rollback | 3h | X64 | optimization gate と rollback lock を固定 |
| X66 | gc | optional GC lane bootstrap (docs) | 2h | X65 | semantics 不変前提の GC lane 入口を固定 |

## 1. Why X18-X40 are mandatory

先行 18 タスク案は runtime 機能実装の粒度は良いが、VM 既定経路の主従切替が不足している。
X18-X23 で route 主従を固め、X24-X31 で薄いRust責務を一箇所化し、X32-X36 で `.hako` 側へ実移管する。

Post-core extension (X41-X46):
- X1-X40 完了後の build orchestration 拡張レーン。
- module/object/link 3層キャッシュを段階導入し、daily/milestone 運用へ統合する。
- SSOT: `29x-67-post29x-cache-lane-sequencing-ssot.md` + `hako-module-cache-build-ssot.md`

Post-X46 extension (X47-X53):
- cache lane 完了後の runtime handoff 拡張レーン。
- route pin inventory -> vm-hako replay -> C ABI delegation guard の順で戻りコストを抑える。
- SSOT: `29x-73-postx46-runtime-handoff-sequencing-ssot.md`

Post-X53 extension (X54-X66):
- runtime handoff lane 完了後の runtime core 拡張レーン。
- VM parity extension -> runtime core hardening -> optimization -> optional GC の層順序を固定。
- SSOT: `29x-80-postx53-runtime-core-sequencing-ssot.md`

## 2. Evidence commands (minimum)

Daily (lightweight):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`

RC lane (X2-X3):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh`

RC lane (X4-X5):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh`

RC lane (X8):

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_return_cleanup_vm.sh`

RC lane (X9):

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_break_cleanup_vm.sh`

RC lane (X10):

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_continue_cleanup_vm.sh`

RC lane (X11):

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_phi_edge_verifier_vm.sh`

RC lane (X12):

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_three_rules_vm.sh`

Observability lane (X13):

1. `cat docs/development/current/main/phases/phase-29x/29x-30-observability-extension-ssot.md`

Observability lane (X14):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh`

Observability lane (X15):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh`

Observability lane (X16):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh`

Observability lane (X17):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh`

Latest evidence:
- 2026-02-13: `phase29x_rc_explicit_drop_vm.sh` PASS（main/module `release_strong=1`）
- 2026-02-13: `phase29x_rc_scope_end_release_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）
- 2026-02-13: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（5/5）
- 2026-02-13: `29x-20-early-exit-cleanup-ssot.md` 追加（X7 契約固定）
- 2026-02-13: `phase29x_rc_return_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）
- 2026-02-13: `phase29x_rc_break_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）
- 2026-02-13: `phase29x_rc_continue_cleanup_vm.sh` PASS（baseline + `rc_insertion_selfcheck`）
- 2026-02-13: `phase29x_rc_phi_edge_verifier_vm.sh` PASS（`[freeze:contract][rc_insertion/phi_edge_mismatch]` 観測）
- 2026-02-13: `phase29x_rc_three_rules_vm.sh` PASS（`[rc_three_rules]` marker + selfcheck PASS）
- 2026-02-13: `29x-30-observability-extension-ssot.md` 追加（X13 契約固定）
- 2026-02-13: `phase29x_observability_temps_vm.sh` PASS（`[lifecycle/leak]   temps: <n>` 観測）
- 2026-02-13: `phase29x_observability_heap_fields_vm.sh` PASS（`[lifecycle/leak]   heap_fields: <n>` 観測）
- 2026-02-13: `phase29x_observability_singletons_vm.sh` PASS（`[lifecycle/leak]   singletons: <n>` 観測）
- 2026-02-13: `phase29x_observability_summary_vm.sh` PASS（5カテゴリの一意 + 固定順を観測）
- 2026-02-13: `29x-40-vm-route-cutover-ssot.md` 追加（X18 契約固定）
- 2026-02-13: `phase29x_vm_route_observability_vm.sh` PASS（`[vm-route/select]` の3経路タグ観測、`lane=vm` は opt-out で固定）
- 2026-02-13: `29x-42-vm-route-strict-dev-priority-ssot.md` 追加（X20 契約固定）
- 2026-02-13: `phase29x_vm_route_strict_dev_priority_vm.sh` PASS（strict/dev 既定 `vm-hako` + compat 明示のみ）
- 2026-02-13: `29x-43-vm-route-non-strict-compat-boundary-ssot.md` 追加（X21 契約固定）
- 2026-02-13: `phase29x_vm_route_non_strict_compat_boundary_vm.sh` PASS（non-strict compat は明示のみ）
- 2026-02-13: `29x-44-vm-route-three-day-gate-evidence.md` 追加（X22 Day1 証跡台帳）
- 2026-02-13: X22 Day1 PASS（`cargo check -q --bin hakorune` + route3 smoke + `selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`）
- 2026-02-14: X22 Day2 PASS（`stageb_total_secs=15`, `avg_case_secs=3.00`）
- 2026-02-15: X22 Day3 PASS（`stageb_total_secs=15`, `avg_case_secs=3.00`）
- 2026-02-15: `tools/selfhost/check_phase29x_x22_evidence.sh --strict` PASS（X22 done）
- 2026-02-15: `tools/selfhost/check_phase29x_x23_readiness.sh --strict` PASS（X23 done sync ready）
- 2026-02-13: `29x-50-thin-rust-boundary-lock-ssot.md` 追加（X24 境界固定）
- 2026-02-13: `29x-51-route-orchestration-single-entry-ssot.md` 追加（X25 単一入口固定）
- 2026-02-13: `cargo test -q route_orchestrator -- --nocapture` PASS（8 tests）
- 2026-02-13: `29x-52-vm-route-observability-contract-ssot.md` 追加（X26 観測契約固定）
- 2026-02-13: route observability contract smoke 3本 PASS（pre-dispatch/select 語彙固定 + legacy tag 不在）
- 2026-02-13: `29x-53-vm-route-compat-bypass-failfast-ssot.md` 追加（X27 bypass fail-fast 固定）
- 2026-02-13: `vm_route_bypass_guard.sh` + `phase29x_vm_route_compat_bypass_guard_vm.sh` PASS
- 2026-02-13: `29x-54-vm-verifier-gate-single-entry-ssot.md` 追加（X28 verifier gate 一本化）
- 2026-02-13: `vm_verifier_gate_guard.sh` + `phase29x_vm_verifier_gate_single_entry_vm.sh` PASS
- 2026-02-13: `29x-55-vm-safety-gate-single-entry-ssot.md` 追加（X29 safety gate 一本化）
- 2026-02-13: `vm_safety_gate_guard.sh` + `phase29x_vm_safety_gate_single_entry_vm.sh` PASS
- 2026-02-13: `29x-56-thin-rust-core-cabi-min-surface-ssot.md` 追加（X30 Core C ABI 最小面固定）
- 2026-02-13: `nyrt_core_cabi_surface_guard.sh` + `phase29x_core_cabi_surface_guard_vm.sh` PASS
- 2026-02-13: `29x-57-thin-rust-gate-pack-ssot.md` 追加（X31 gate pack 固定）
- 2026-02-13: `phase29x_thin_rust_gate_vm.sh` PASS（X24-X30 contracts locked）
- 2026-02-13: `29x-58-derust-route-orchestrator-skeleton-ssot.md` 追加（X32 `.hako` route skeleton 固定）
- 2026-02-13: `phase29x_derust_route_dualrun_vm.sh` PASS（Rust/.hako route lane 4ケース一致）
- 2026-02-13: `29x-59-derust-verifier-path-ssot.md` 追加（X33 `.hako` verifier path 固定）
- 2026-02-13: `phase29x_derust_verifier_vm.sh` PASS（mismatch fail-fast + match pass）
- 2026-02-13: `29x-60-derust-safety-path-ssot.md` 追加（X34 `.hako` safety path 固定）
- 2026-02-13: `phase29x_derust_safety_vm.sh` PASS（lifecycle fail-fast + clean pass）
- 2026-02-13: `29x-61-derust-strict-default-route-cutover-ssot.md` 追加（X35 strict/dev 既定 route cutover 固定）
- 2026-02-13: `phase29x_derust_strict_default_route_vm.sh` PASS（strict/dev default hako route + rust-thin explicit）
- 2026-02-13: `29x-62-derust-done-sync-ssot.md` 追加（X36 done 判定 / rollback / evidence 同期）
- 2026-02-13: `29x-63-llvm-cabi-link-gate-ssot.md` 追加（X37 LLVM+C ABI link gate 固定）
- 2026-02-13: `phase29x_llvm_cabi_link_min.sh` PASS（core C ABI surface guard + `.hako -> LLVM link` + exe run）
- 2026-02-13: `29x-64-llvm-only-daily-default-ssot.md` 追加（X38 daily/milestone default を LLVM line へ切替）
- 2026-02-13: `phase29x_llvm_only_daily_gate.sh` PASS（`abi_lane_guard` + X37 gate を 1コマンド化）
- 2026-02-13: `29x-65-rust-lane-optin-isolation-ssot.md` 追加（X39 Rust lane opt-in 隔離）
- 2026-02-13: `phase29x_rust_lane_optin_only.sh` PASS（`tools/compat` 入口 + opt-in required 固定）
- 2026-02-13: `29x-66-llvm-only-build-done-sync-ssot.md` 追加（X40 done判定/rollback/残存Rust依存を同期）
- 2026-02-13: `29x-67-post29x-cache-lane-sequencing-ssot.md` 追加（X41 post-29x closeout sync）
- 2026-02-13: `29x-68-cache-key-determinism-ssot.md` 追加（X42 cache key determinism 固定）
- 2026-02-13: `phase29x_cache_key_determinism_vm.sh` PASS（Module/Object/Link key determinism + diff probe）
- 2026-02-13: `29x-69-l1-mir-cache-ssot.md` 追加（X43 L1 MIR cache 固定）
- 2026-02-13: `phase29x_l1_mir_cache_vm.sh` PASS（L1 cache miss->hit contract）
- 2026-02-13: `29x-70-l2-object-cache-ssot.md` 追加（X44 L2 object cache 固定）
- 2026-02-13: `phase29x_l2_object_cache_vm.sh` PASS（L2 cache miss->hit + ABI diff miss）
- 2026-02-13: `29x-71-l3-link-cache-ssot.md` 追加（X45 L3 link cache 固定）
- 2026-02-13: `phase29x_l3_link_cache_vm.sh` PASS（L3 cache miss->hit + runtime ABI diff miss）
- 2026-02-13: `29x-72-cache-gate-integration-done-sync-ssot.md` 追加（X46 cache gate integration 固定）
- 2026-02-13: `phase29x_cache_lane_gate_vm.sh` PASS（X42-X45 cache contracts single-entry replay）
- 2026-02-13: `phase29x_llvm_only_daily_gate.sh` PASS（llvm-only daily gate + cache lane integration）
- 2026-02-13: `phase29x_l2_object_cache_vm.sh` / `phase29x_l3_link_cache_vm.sh` / `phase29x_cache_lane_gate_vm.sh` に `can_run_llvm` guard を追加し、LLVM未対応ビルド時は SKIP 契約へ統一
- 2026-02-13: `phase29x_llvm_only_daily_gate.sh` PASS（`nyash 1.0 features:llvm` で cache lane full replay）
- 2026-02-13: `29x-73-postx46-runtime-handoff-sequencing-ssot.md` 追加（X47 handoff bootstrap 固定）
- 2026-02-13: `29x-75-vm-route-pin-inventory-guard-ssot.md` 追加（X48 route pin inventory 固定）
- 2026-02-13: `phase29x_vm_route_pin_guard.sh` + `phase29x_vm_route_pin_guard_vm.sh` PASS
- 2026-02-13: `29x-76-vm-hako-strict-dev-replay-gate-ssot.md` 追加（X49 strict/dev replay 契約固定）
- 2026-02-13: `phase29x_vm_hako_strict_dev_replay_vm.sh` PASS（vm-hako lane success/reject replay）
- 2026-02-13: `29x-77-newclosure-contract-lock-ssot.md` 追加（X50 fail-fast Decision 固定）
- 2026-02-13: `phase29x_vm_hako_newclosure_contract_vm.sh` PASS（allowlist + runtime fail-fast probe）
- 2026-02-13: `29x-78-core-cabi-delegation-inventory-guard-ssot.md` 追加（X51 delegation inventory 固定）
- 2026-02-13: `phase29x_core_cabi_delegation_guard.sh` + `phase29x_core_cabi_delegation_guard_vm.sh` PASS
- 2026-02-13: `29x-79-runtime-handoff-gate-integration-ssot.md` 追加（X52 handoff gate integration 固定）
- 2026-02-13: `phase29x_runtime_handoff_gate_guard.sh` + `phase29x_runtime_handoff_gate_vm.sh` PASS
- 2026-02-13: `29x-74-postx46-runtime-handoff-done-sync-ssot.md` を placeholder から Decision=`accepted` へ更新（X53 done sync + rollback lock 固定）
- 2026-02-13: `29x-80-postx53-runtime-core-sequencing-ssot.md` 追加（X54 post-X53 runtime core bootstrap 固定）
- 2026-02-13: `29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md` 追加（X55 S6 vocabulary inventory 固定）
- 2026-02-13: `phase29x_vm_hako_s6_vocab_guard.sh` + `phase29x_vm_hako_s6_vocab_guard_vm.sh` PASS
- 2026-02-13: `29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md` 追加（X56 dual-run parity gate pack 固定）
- 2026-02-13: `phase29x_vm_hako_s6_parity_gate_guard.sh` + `phase29x_vm_hako_s6_parity_gate_vm.sh` PASS
- 2026-02-13: `29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md` 追加（X57 NewClosure decision refresh 固定）
- 2026-02-13: `phase29x_vm_hako_newclosure_decision_guard.sh` + `phase29x_vm_hako_newclosure_decision_refresh_vm.sh` PASS
- 2026-02-13: `29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md` 追加（X58 `nop` vocabulary promotion 固定）
- 2026-02-13: `phase29x_vm_hako_s6_nop_promotion_guard.sh` + `phase29x_vm_hako_s6_nop_promotion_vm.sh` PASS
- 2026-02-13: `29x-86-abi-borrowed-owned-conformance-extension-ssot.md` 追加（X59 borrowed/owned matrix extension 固定）
- 2026-02-13: `phase29x_abi_borrowed_owned_matrix_guard.sh` + `phase29x_abi_borrowed_owned_conformance_vm.sh` PASS
- 2026-02-13: `29x-87-rc-insertion-phase2-queue-lock-ssot.md` 追加（X60 RC phase2 queue lock 固定）
- 2026-02-13: `phase29x_rc_phase2_queue_guard.sh` + `phase29x_rc_phase2_queue_lock_vm.sh` PASS
- 2026-02-13: `29x-88-observability-drift-guard-ssot.md` 追加（X61 observability drift guard 固定）
- 2026-02-13: `phase29x_observability_drift_guard.sh` + `phase29x_observability_drift_guard_vm.sh` PASS
- 2026-02-13: `29x-89-runtime-core-integrated-gate-ssot.md` 追加（X62 runtime core integrated gate 固定）
- 2026-02-13: `phase29x_runtime_core_gate_guard.sh` + `phase29x_runtime_core_gate_vm.sh` PASS
- 2026-02-13: `29x-92-optimization-allowlist-lock-ssot.md` 追加（X63 optimization allowlist lock 固定）
- 2026-02-13: `phase29x_optimization_allowlist_guard.sh` + `phase29x_optimization_allowlist_lock_vm.sh` PASS
- 2026-02-13: `29x-93-optimization-parity-fixtures-lock-ssot.md` 追加（X64 optimization parity fixtures/reject fixture 固定）
- 2026-02-13: `phase29x_optimization_parity_guard.sh` + `phase29x_optimization_parity_fixtures_vm.sh` PASS
- 2026-02-13: `29x-94-optimization-gate-integration-rollback-lock-ssot.md` 追加（X65 optimization gate integration + rollback lock 固定）
- 2026-02-13: `phase29x_optimization_gate_guard.sh` + `phase29x_optimization_gate_vm.sh` PASS
- 2026-02-13: `29x-95-optional-gc-lane-bootstrap-ssot.md` 追加（X66 optional GC lane bootstrap docs-only 固定）

Milestone checkpoint (節目のみ):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
3. `bash tools/checks/abi_lane_guard.sh`
4. `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`（X39）

VM route lane (X18):

1. `cat docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md`

VM route lane (X19-X22):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29z_vm_hako_backend_frame_vm.sh`
7. `tools/selfhost/record_phase29x_x22_evidence.sh <day> [YYYY-MM-DD]`（X22 証跡行を生成）
8. `tools/selfhost/check_phase29x_x22_evidence.sh --strict`（X22 完了判定）
9. `tools/selfhost/check_phase29x_x23_readiness.sh --strict`（X23 着手前の機械判定）

Thin-rust lane (X24-X31):

1. `cat docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md`（X24）
2. `cat docs/development/current/main/phases/phase-29x/29x-51-route-orchestration-single-entry-ssot.md`（X25）
3. `cat docs/development/current/main/phases/phase-29x/29x-52-vm-route-observability-contract-ssot.md`（X26）
4. `cat docs/development/current/main/phases/phase-29x/29x-53-vm-route-compat-bypass-failfast-ssot.md`（X27）
5. `cat docs/development/current/main/phases/phase-29x/29x-54-vm-verifier-gate-single-entry-ssot.md`（X28）
6. `cat docs/development/current/main/phases/phase-29x/29x-55-vm-safety-gate-single-entry-ssot.md`（X29）
7. `cat docs/development/current/main/phases/phase-29x/29x-56-thin-rust-core-cabi-min-surface-ssot.md`（X30）
8. `cat docs/development/current/main/phases/phase-29x/29x-57-thin-rust-gate-pack-ssot.md`（X31）
9. `cargo test -q nyrt_shim -- --nocapture`（X30-X31）
10. `cargo test -q safety_gate -- --nocapture`（X29-X31）
11. `cargo test -q verifier_gate -- --nocapture`（X28-X31）
12. `cargo test -q route_orchestrator -- --nocapture`（X25-X31）
13. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`（X26）
14. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`（X26-X31）
15. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`（X26-X31）
16. `bash tools/checks/vm_route_bypass_guard.sh`（X27）
17. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_compat_bypass_guard_vm.sh`（X27）
18. `bash tools/checks/vm_verifier_gate_guard.sh`（X28）
19. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh`（X28）
20. `bash tools/checks/vm_safety_gate_guard.sh`（X29）
21. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_safety_gate_single_entry_vm.sh`（X29）
22. `bash tools/checks/nyrt_core_cabi_surface_guard.sh`（X30）
23. `bash tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_surface_guard_vm.sh`（X30）
24. `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
25. `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh`
26. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_thin_rust_gate_vm.sh`（X31）

De-rust transfer lane (X32-X36):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh`（X32）
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh`（X33）
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh`（X34）
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh`（X35）
5. `cat docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`（X36）

LLVM-only retirement lane (X37-X40):

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`（X37）
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`（X38）
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh`（X39）
4. `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`（X39）
5. `cat docs/development/current/main/phases/phase-29x/29x-66-llvm-only-build-done-sync-ssot.md`（X40）

Post-29x cache lane (X41-X46):

1. `cat docs/development/current/main/phases/phase-29x/29x-67-post29x-cache-lane-sequencing-ssot.md`（X41）
2. `cat docs/development/current/main/phases/phase-29x/29x-68-cache-key-determinism-ssot.md`（X42）
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh`（X42）
4. `cat docs/development/current/main/phases/phase-29x/29x-69-l1-mir-cache-ssot.md`（X43）
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh`（X43）
6. `cat docs/development/current/main/phases/phase-29x/29x-70-l2-object-cache-ssot.md`（X44）
7. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh`（X44）
8. `cat docs/development/current/main/phases/phase-29x/29x-71-l3-link-cache-ssot.md`（X45）
9. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh`（X45）
10. `cat docs/development/current/main/design/hako-module-cache-build-ssot.md`（X46）
11. `cat docs/development/current/main/phases/phase-29x/29x-72-cache-gate-integration-done-sync-ssot.md`（X46）
12. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh`（X46）
13. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`（X46）

Post-X46 runtime handoff lane (X47-X53):

1. `cat docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md`（X47）
2. `bash tools/checks/phase29x_vm_route_pin_guard.sh`（X48）
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh`（X48）
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`（X49）
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh`（X50）
6. `bash tools/checks/phase29x_core_cabi_delegation_guard.sh`（X51）
7. `bash tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh`（X51）
8. `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh`（X52）
9. `cat docs/development/current/main/phases/phase-29x/29x-74-postx46-runtime-handoff-done-sync-ssot.md`（X53）

Post-X53 runtime core extension lane (X54-X66):

1. `cat docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md`（X54）
2. `cat docs/development/current/main/phases/phase-29x/29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md`（X55）
3. `bash tools/checks/phase29x_vm_hako_s6_vocab_guard.sh`（X55）
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_vocab_guard_vm.sh`（X55）
5. `cat docs/development/current/main/phases/phase-29x/29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md`（X56）
6. `bash tools/checks/phase29x_vm_hako_s6_parity_gate_guard.sh`（X56）
7. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh`（X56）
8. `cat docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md`（X57）
9. `bash tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh`（X57）
10. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_decision_refresh_vm.sh`（X57）
11. `cat docs/development/current/main/phases/phase-29x/29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md`（X58）
12. `bash tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh`（X58）
13. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_nop_promotion_vm.sh`（X58）
14. `cat docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md`（X59）
15. `bash tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh`（X59）
16. `bash tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh`（X59）
17. `cat docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md`（X60）
18. `bash tools/checks/phase29x_rc_phase2_queue_guard.sh`（X60）
19. `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh`（X60）
20. `cat docs/development/current/main/phases/phase-29x/29x-88-observability-drift-guard-ssot.md`（X61）
21. `bash tools/checks/phase29x_observability_drift_guard.sh`（X61）
22. `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh`（X61）
23. `cat docs/development/current/main/phases/phase-29x/29x-89-runtime-core-integrated-gate-ssot.md`（X62）
24. `bash tools/checks/phase29x_runtime_core_gate_guard.sh`（X62）
25. `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh`（X62）
26. `cat docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md`（X63）
27. `bash tools/checks/phase29x_optimization_allowlist_guard.sh`（X63）
28. `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh`（X63）
29. `cat docs/development/current/main/phases/phase-29x/29x-93-optimization-parity-fixtures-lock-ssot.md`（X64）
30. `bash tools/checks/phase29x_optimization_parity_guard.sh`（X64）
31. `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh`（X64）
32. `cat docs/development/current/main/phases/phase-29x/29x-94-optimization-gate-integration-rollback-lock-ssot.md`（X65）
33. `bash tools/checks/phase29x_optimization_gate_guard.sh`（X65）
34. `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh`（X65）
35. `cat docs/development/current/main/phases/phase-29x/29x-95-optional-gc-lane-bootstrap-ssot.md`（X66）

Note:
- X47-X53 の evidence commands はすべて実体化済み（placeholder なし）。
- X53 で runtime handoff lane の done criteria / rollback lock / residual scope（GC optional）を docs 同期済み。
- X47-X66 の evidence commands はすべて実体化済み（placeholder なし）。

## 3. Dependency graph (condensed)

- X1 -> X2 -> X3 -> X4 -> X5 -> X6
- X6 -> X7 -> X8 -> X9 -> X10 -> X11 -> X12
- X12 -> X13 -> X14 -> X15 -> X16 -> X17
- X17 -> X18 -> X19 -> X20 -> X21 -> X22 -> X23
- X23 -> X24 -> X25 -> X26 -> X27 -> X28 -> X29 -> X30 -> X31
- X31 -> X32 -> X33 -> X34 -> X35 -> X36
- X36 -> X37 -> X38 -> X39 -> X40
- X40 -> X41 -> X42 -> X43 -> X44 -> X45 -> X46
- X46 -> X47 -> X48 -> X49 -> X50 -> X51 -> X52 -> X53
- X53 -> X54 -> X55 -> X56 -> X57 -> X58 -> X59 -> X60 -> X61 -> X62 -> X63 -> X64 -> X65 -> X66

## 4. Risk controls

- compat 全撤去はこのフェーズで行わない（SSOT上は段階縮退）。
- GC implementation は扱わない（GC optional policy を維持）。
- failure-driven を保守レーン化し、計画タスクを毎日 1 件進める。
- X32+ は X11/X12 未完のまま開始しない（RC 契約が先）。
- X37+ は X36 未完のまま開始しない（route/verifier/safety 移管が先）。
- X42+ は X41 未完のまま開始しない（導線同期が先）。
- X43+ は X42 未完のまま開始しない（key determinism 固定が先）。
- X44+ は X43 未完のまま開始しない（L1 miss->hit 固定が先）。
- X45+ は X44 未完のまま開始しない（L2 miss->hit 固定が先）。
- X46+ は X45 未完のまま開始しない（L3 miss->hit 固定が先）。
- X48+ は X47 未完のまま開始しない（harness scope lock が先）。
- X49+ は X48 未完のまま開始しない（route pin inventory が先）。
- X51+ は X50 未完のまま開始しない（NewClosure contract lock が先）。
- X55+ は X54 未完のまま開始しない（lane scope lock が先）。
- X59+ は X58 未完のまま開始しない（VM parity extension が先）。
- X63+ は X62 未完のまま開始しない（runtime core gate 固定が先）。
