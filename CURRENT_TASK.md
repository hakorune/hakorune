# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-02-28
Scope: Repo root の互換入口。詳細ログは `docs/development/current/main/` 側を正本にする。

## Purpose

- root から最短で current blocker と実行順へ到達するための入口。
- 長文の進捗履歴はここに蓄積しない。
- runtime lane の Next は `phase-29y/60-NEXT-TASK-PLAN.md` を単一正本に固定する。

## Current Blocker (SSOT)

- compiler lane: `phase-29bq / monitor-only`（active: failure-driven reopen only）
  - joinir migration task SSOT（lane A）:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
  - lane A mirror sync helper:
    - `bash tools/selfhost/sync_lane_a_state.sh`
  - done: `JIR-PORT-00`（Boundary Lock, docs-first）
  - done: `JIR-PORT-01`（Parity Probe）
  - done: `JIR-PORT-02`（if/merge minimal port）
  - done: `JIR-PORT-03`（loop minimal port）
  - done: `JIR-PORT-04`（PHI / Exit invariant lock）
  - done: `JIR-PORT-05`（promotion boundary lock）
  - done: `JIR-PORT-06`（monitor-only boundary lock）
  - done: `JIR-PORT-07`（expression parity seed lock: unary+compare+logic）
  - next: `none`（strict_nested accept-min1 done; failure-driven reopen only）
- runtime lane: `phase-29y / none`（current blocker: `none`。fixed order は `phase-29y/60-NEXT-TASK-PLAN.md` を正本とする）
  - commit boundary lock: `phase-29y/60-NEXT-TASK-PLAN.md` の `0.3 RVP Commit Boundary Lock (active rule)`
  - operation policy lock: `LLVM-first / vm-hako monitor-only`
  - policy SSOT: `docs/development/current/main/design/de-rust-lane-map-ssot.md` の `Runtime Operation Policy`
- config hygiene lane: `none`（monitor-only、SSOT: `phase-29y/84-MODULE-REGISTRY-HYGIENE-SSOT.md`）
- compiler pipeline lane: `hako-using-resolver-parity`（monitor-only: lane-B ternary debt decision fixed）
  - parity gate: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
  - active next: `none`（B-TERNARY-03 decision fixed）
  - task SSOT:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の `Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)`
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の `Binary-only --hako-run Contract (lane B)`
  - diagnostic pin（non-gating）:
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
  - binary-only contract SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- de-rust migration orchestration lane: `phase-29cc / plugin-wave-rollout`（RNR-05 complete; PLG-03 done; PLG-04-min1..min6 done; wave-1 complete; PLG-05-min1/min2/min3/min4/min5/min6/min7 done; PLG-06-min1/min2/min3/min4 done）
  - phase SSOT: `docs/development/current/main/phases/phase-29cc/README.md`
  - scope decision（L5 accepted）:
    - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
  - strict readiness（L4 done, 2026-02-25）:
    - `tools/selfhost/check_phase29x_x23_readiness.sh --strict` -> `status=READY`
  - done declaration（non-plugin accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md`
  - plugin lane bootstrap（docs-first, provisional）:
    - `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
  - plugin lane ABI lock（PLG-01 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
  - plugin lane gate pack lock（PLG-02 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
  - plugin lane wave-1 pilot lock（PLG-03 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min5 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`
  - plugin lane wave rollout lock（PLG-04-min6 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`
  - post-wave1 route lock（accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
  - plugin wave-2 entry lock（PLG-05-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min5 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min6 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`
  - plugin wave-2 rollout lock（PLG-05-min7 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`
  - plugin wave-3 entry lock（PLG-06-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`
  - plugin wave-3 rollout lock（PLG-06-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`
  - plugin wave-3 rollout lock（PLG-06-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`
  - plugin wave-3 rollout lock（PLG-06-min4 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`
  - plugin lane active next:
    - `none`（PLG-HM2-min1/min2/min3 done; monitor-only, failure-driven reopen）
  - plugin de-rust PLG-07 locks:
    - `docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md`（cutover order）
    - `docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md`（min1/min2）
    - `docs/development/current/main/phases/phase-29cc/29cc-180-plg07-min3-filebox-binary-hako-parity-lock-ssot.md`（min3）
    - `docs/development/current/main/phases/phase-29cc/29cc-181-plg07-min4-filebox-binary-dualrun-gate-lock-ssot.md`（min4）
    - `docs/development/current/main/phases/phase-29cc/29cc-182-plg07-min5-filebox-default-switch-lock-ssot.md`（min5）
    - `docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md`（min6）
    - `docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md`（min7）
  - plugin HM2 recovery line lock（PLG-HM2-min1 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md`
  - plugin HM2 route matrix lock（PLG-HM2-min2 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-211-plg-hm2-min2-core6-static-wave2-compat-ceiling-lock-ssot.md`
  - plugin HM2 route policy matrix lock（PLG-HM2-min3 done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-212-plg-hm2-min3-route-policy-matrix-lock-ssot.md`
  - plugin residue classification lock（29cc-213, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-213-plg-hm3-next-blocker-candidate-memo.md`
    - fixed:
      - `nyash-fixture-plugin` = test-only keep
      - `nyash-integer-plugin` = mainline keep（IntCellBox）
      - `nyash-math` = retire（legacy duplicate; `nyash-math-plugin` is active line）
  - runtime source-zero cutover lock（29cc-220, active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md`
    - static-link boundary lock（29cc-253, active）:
      - `docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md`
    - zero definition（fixed）:
      - long-term goal = source-zero（runtime/plugin の Rust実装撤去 + mainline/CI no-compat）
      - phase done = route-zero + stability（no-delete-first）
      - execution-path-zero は中間マイルストーンとして扱う
    - static-link policy（fixed）:
      - core runtime（host/kernel）は static-first（`libnyash_kernel.a` 正本）を維持
      - 外部 plugin は dynamic load を許可するが、mainline route の正本にはしない
    - latest cleanup（2026-02-28）:
      - `src/runtime/plugin_loader_v2/enabled/route_resolver.rs` を追加し、`ffi_bridge`/`instance_manager` の type/birth/method route 解決を共通化（compat fallback は fail-fast policy 下に限定）
      - `src/runtime/plugin_loader_v2/enabled/types.rs` の `drop/finalize/clone` 呼び出しを `PluginHandleInner` helper へ集約し、host_bridge route policy 呼び出し点を単一路線化
      - `src/runtime/plugin_loader_v2/enabled/loader/singletons.rs` の type_id/fini 解決重複を `route_resolver` へ移設し、singleton 生成経路の判断源を集約
      - `src/runtime/plugin_loader_v2/enabled/loader/metadata.rs` の type_id/fini 解決を `route_resolver` 経由へ統一し、metadata/construct_existing の重複判定を縮退
      - `src/runtime/plugin_loader_v2/enabled/loader/metadata.rs` の `type_id -> (lib,box)` 逆引きも `route_resolver` へ移設し、metadata 側の独自探索を撤去
      - `src/runtime/plugin_loader_v2/enabled/method_resolver.rs` の method_id/returns_result 解決を `route_resolver` へ統一し、`resolve_method_handle` も selected-lib route SSOT へ集約
      - `src/runtime/plugin_loader_v2/enabled/compat_method_resolver.rs` を新設し、legacy file fallback を compat専用モジュールへ隔離（mainline解決は method/route resolver に固定）
      - `src/runtime/plugin_loader_v2/enabled/compat_host_bridge.rs` を新設し、shim invoke fallback を compat専用へ隔離。`ffi_bridge` は non-zero invoke code を即時 `PluginError` で fail-fast
      - `src/runtime/plugin_loader_v2/enabled/loader/singletons.rs` の `RwLock unwrap` と invoke status 無視を撤去し、singleton prebirth 経路を `BidError::PluginError` fail-fast に統一
      - `src/runtime/plugin_loader_v2/enabled/instance_manager.rs` の `shutdown_singletons` を `BidResult<()>` 契約へ統一し、`globals` 経由 shutdown も lock poison を `PluginError` で fail-fast
      - `crates/nyash_kernel/src/plugin/compat_invoke_core.rs` を新設し、`invoke_core` の generic fallback route / legacy arg encode を compat専用へ分離（mainlineは handle route SSOT を維持）
      - `crates/nyash_kernel/src/encode.rs` に mainline用 `nyrt_encode_arg` を追加し、`plugin/invoke/by_id.rs`・`plugin/invoke/by_name.rs` を mainline encode + `fail_fast` 時2引数超rejectへ切替。compat時は `encode_legacy_vm_args_range(...)` で trailing payload を legacy VM slots から復元
      - `crates/nyash_kernel/src/plugin/future.rs` の spawn_instance3 payload encode を mainline `nyrt_encode_arg` 優先へ寄せ、`fail_fast` 時は 1payload 超を即時reject。compat時は `encode_legacy_vm_args_range(3..)` を使用して trailing payload を legacy VM slots から復元
      - `crates/nyash_kernel/src/plugin/mod.rs` に B3 public wiring contract test（`b3_public_wiring_contract_compiles`）を追加し、future/invoke entrypoint re-export drift を compile-time で fail-fast 監査
      - `tools/checks/dev_gate.sh runtime-exec-zero` に B3 wiring contract test を組み込み、entrypoint配線ドリフト監査を日常ゲート化
      - `tools/checks/phase29cc_kernel_b3_compat_isolation_guard.sh` を追加し、`compat_invoke_core` 呼び出しが `invoke_core.rs` 以外へ漏れないことを監査（runtime-exec-zero 組み込み済み）
      - `crates/nyash_kernel/src/plugin/invoke_core.rs` の compat encode helper を `fail_fast` ガード付き `encode_legacy_vm_args_range()` へ統一し、未使用 placeholder helper を撤去（mainline の誤経路侵入を構造的に遮断）
      - `29cc-242` / `29cc-220` に B3 closeout 実行証跡を同期し、kernel residue closeout -> route-zero + stability 判定導線を固定
  - runtime route residue relock（29cc-245, active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-245-runtime-route-residue-relock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-246-rz-array-min1-route-selector-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-247-rz-array-min2-route-policy-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-248-rz-loader-min1-route-contract-box-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-249-rz-loader-min2-ffi-host-route-contract-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-250-rz-loader-min3-compat-ffi-branch-isolation-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-251-rz-loader-min4-loader-types-route-reuse-lock-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-252-rz-loader-min4-closeout-route-zero-stability-lock-ssot.md`
    - fixed:
      - mainline 実行経路に残る Rust 依存（link / loader / kernel export）を 2026-02-28 時点で再棚卸し
      - 次境界を `RZ-LINK-min1 -> RZ-ARRAY-min1 -> RZ-LOADER-min1` の順で固定
      - 最適化と route cutover を分離（route先行、no-delete-first 維持）
      - `src/runner/modes/common_util/exec.rs` で `NYASH_LLVM_USE_HARNESS=1` 時のみ `verify_nyrt_dir` をスキップ（既定挙動は維持）
      - RuntimeDataBox array mono-route の symbol 選択を `select_runtime_data_call_spec()` に集約し、route切替境界を1箇所化（既定挙動は維持）
      - `NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY`（`array_mono|runtime_data_only`）を追加し、default=`array_mono` 固定で policy 切替境界を導入（invalid は fail-fast）
      - `route_resolver` に method/birth route contract 箱を追加し、`method_resolver` / `instance_manager` の route 解決重複を集約（挙動不変）
      - `route_resolver` に invoke route contract 箱を追加し、`ffi_bridge` / `instance_manager` の invoke route 解決を統一（挙動不変）
      - `ffi_bridge` の compat/dev trace/probe 分岐を `compat_ffi_bridge` へ隔離し、mainline invoke 責務を縮退（挙動不変）
      - `route_resolver` に `resolve_birth_contract_for_lib` を追加し、`loader/singletons` の type/birth/fini 判定を契約再利用へ統一
      - `instance_manager` / `ffi_bridge` / `types` の invoke_box route 解決を `resolve_invoke_route_contract` へ統一
    - active next:
      - `none`（RZ-LOADER-min4 closeout done; monitor-only, failure-driven reopen）
  - fullstack completion SSOT（meaning in `.hako`, host as minimal ABI）:
    - `docs/development/current/main/design/hako-fullstack-host-abi-completion-ssot.md`
  - Step-1 host ABI surface lock（docs-first）:
    - `docs/reference/abi/nyrt_host_surface_v0.md`
  - Step-2 host facade contract lock（docs-first）:
    - `docs/development/current/main/design/hako-host-facade-contract-ssot.md`
  - Step-2 host facade wiring（code-first, no-delete-first）:
    - `lang/src/runtime/host/host_facade_box.hako` を追加し、`extern_provider` / `extern_call_handler` / `mir_vm_s0` の direct `env.get`/console/extern invoke 経路を HostFacade 経由へ統一（2026-02-28）
    - `codegen_bridge` / `host_bridge` / `mir_io` と `extern_provider` C-ABI branch の direct hostbridge 呼びを HostFacade loader selector 経由へ統一（2026-02-28）
    - guard: `tools/checks/phase29cc_hostfacade_direct_call_guard.sh`（runtime/shared/vm で direct hostbridge 呼びの allowlist 監査）
    - latest: PASS（2026-02-28, residual direct hostbridge calls: 0）
  - runtime execution-path observability lock（29cc-215, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
  - runtime VM+AOT route lock（29cc-217, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh`
      - `tools/checks/dev_gate.sh runtime-exec-zero`
  - runtime V0 ABI slice lock（29cc-216, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
    - guard:
      - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
  - plugin method resolver fail-fast lock（29cc-218, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-218-plugin-method-resolver-failfast-lock-ssot.md`
    - fixed:
      - `resolve_method_id` の legacy file fallback は `NYASH_FAIL_FAST=0` 時のみ許可
      - default (`NYASH_FAIL_FAST=1`) は `InvalidMethod` で fail-fast
  - instance manager boundary lock（29cc-219, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-219-instance-manager-boundary-lock-ssot.md`
    - fixed:
      - `create_box` は resolve/invoke/build の3段責務へ分離
      - birth 契約解決は `config -> spec -> fail-fast`
  - runtime/plugin residue inventory lock（29cc-221, active）:
    - `docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md`
    - fixed:
      - runtime plugin loader residue と kernel plugin residue を責務単位で固定
      - 1 boundary = 1 commit で route cutover を先行（source削除は延期）
      - compat route は default-off を維持
  - final-wave non-target discovery lock（29cc-244, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-244-final-wave-nontarget-discovery-lock-ssot.md`
    - fixed:
      - Non-target 7 files の call/ABI/complexity を統合棚卸し
      - 7 commit slices（handle_helpers -> module_string_dispatch -> array -> map -> string -> intarray -> console）を固定
      - 7 commit slices を実装完了（`e8e9e2d79` / `ea54764df` / `a53c9a53d` / `5a575c503` / `ecd44c43d` / `ca0d82dd0` / `5f191ff25`）
  - runtime A1-min1 method_resolver route cutover lock（29cc-222, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-222-runtime-a1-min1-method-resolver-route-cutover-lock-ssot.md`
    - fixed:
      - `ffi_bridge` mainline は `resolve_method_id()` 依存を排除（selected lib config/spec/resolve_fn 限定）
      - `plugin_loader_unified::resolve_method` fallback は compat-only（`NYASH_FAIL_FAST=0`）
      - fail-fast 既定は `BidError::InvalidMethod` で停止
  - runtime A1-min2 instance_manager route cutover lock（29cc-223, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-223-runtime-a1-min2-instance-manager-route-cutover-lock-ssot.md`
    - fixed:
      - `instance_manager` birth contract は selected library key 優先
      - generic scan は compat-only（`NYASH_FAIL_FAST=0`）
      - fail-fast 既定は `BidError::InvalidType/InvalidMethod` で停止
  - runtime A2-min1 ffi_bridge route hardening lock（29cc-224, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-224-runtime-a2-min1-ffi-bridge-route-hardening-lock-ssot.md`
    - fixed:
      - `resolve_type_info` mainline fail-fast で generic box scan を禁止
      - config不在時の fallback は compat-only + lexical deterministic
  - runtime A2-min2 host_bridge route cutover lock（29cc-225, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-225-runtime-a2-min2-host-bridge-route-cutover-lock-ssot.md`
    - fixed:
      - `invoke_alloc_with_route` で `BoxInvokeFn` 優先を固定
      - fail-fast 既定（`NYASH_FAIL_FAST=1`）は missing route を `E_PLUGIN(-5)` で停止
      - shim fallback は compat-only（`NYASH_FAIL_FAST=0`）
  - runtime A3-min1 loader metadata route hardening lock（29cc-226, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-226-runtime-a3-min1-loader-metadata-route-hardening-lock-ssot.md`
    - fixed:
      - `box_invoke_fn_for_type_id` の generic `box_specs` scan は compat-only
      - fail-fast 既定は unresolved route を `None` で返し上位 fail-fast へ委譲
  - runtime A3-min2 types handle route cutover lock（29cc-227, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-227-runtime-a3-min2-types-handle-route-cutover-lock-ssot.md`
    - fixed:
      - `PluginHandleInner` に `invoke_box_fn` を保持
      - lifecycle/clone 呼び出しは `invoke_alloc_with_route` に統一
      - `type_id` からの route 解決を handle 構築時に固定
  - runtime A3-min3 globals/errors/extern fail-fast lock（29cc-228, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-228-runtime-a3-min3-globals-errors-extern-failfast-lock-ssot.md`
    - fixed:
      - `globals` の `RwLock` 取得失敗は panic せず `BidError::PluginError` で返す
      - `extern_call` unknown interface/method は fail-fast reject を固定
  - runtime A3-min4 PluginBoxMetadata route-aware lock（29cc-229, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-229-runtime-a3-min4-pluginboxmetadata-routeaware-lock-ssot.md`
    - fixed:
      - `PluginBoxMetadata` は `invoke_box_fn: Option<BoxInvokeFn>` を保持
      - metadata 生成時に `box_invoke_fn_for_type_id` を解決して保存
      - enabled/stub の型定義を同期
  - runtime A3 closeout lock（29cc-230, accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-230-runtime-a3-closeout-lock-ssot.md`
    - fixed:
      - A3-min1..min4 の done criteria を統合
      - next boundary を B1-min1（kernel invoke_core/birth）へ固定
  - wasm lane status SSOT（active next / latest lock / lock history）:
    - `docs/development/current/main/phases/phase-29cc/README.md`
  - wasm lane G2 task plan:
    - `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md`
  - wasm grammar/map SSOT:
    - `docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md`
  - wasm `.hako`-only output roadmap SSOT:
    - `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
  - wasm route governance（fixed 3 routes）:
    - `hako_native`（default）/ `rust_native`（parity）/ `legacy_bridge`（monitor-only）
    - SSOT: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md` の `Route Governance (fixed)`
  - wasm P7 hako-only locks（done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md`（min1）
    - `docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md`（min2）
    - `docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md`（min3）
    - `docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md`（min4）
  - wasm P8 compat bridge retire lock（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md`（min1）
  - wasm P9 non-native shrink locks（done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md`（min0）
    - `docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md`（min1）
  - wasm P9 bridge blockers（accepted-but-blocked done）:
    - `docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md`（min2）
    - `docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md`（min3）
    - `docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md`（min4）
  - wasm P10 locks（min1 accepted-but-blocked, min2/min3/min4/min5/min6/min7/min8/min9/min10 accepted）:
    - `docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md`（min1）
    - `docs/development/current/main/phases/phase-29cc/29cc-195-wsm-p10-min2-loop-extern-matcher-inventory-lock-ssot.md`（min2）
    - `docs/development/current/main/phases/phase-29cc/29cc-196-wsm-p10-min3-loop-extern-writer-section-lock-ssot.md`（min3）
    - `docs/development/current/main/phases/phase-29cc/29cc-197-wsm-p10-min4-single-fixture-native-promotion-lock-ssot.md`（min4）
    - `docs/development/current/main/phases/phase-29cc/29cc-198-wsm-p10-min5-expansion-inventory-lock-ssot.md`（min5）
    - `docs/development/current/main/phases/phase-29cc/29cc-199-wsm-p10-min6-warn-native-promotion-lock-ssot.md`（min6）
    - `docs/development/current/main/phases/phase-29cc/29cc-200-wsm-p10-min7-info-native-promotion-lock-ssot.md`（min7）
    - `docs/development/current/main/phases/phase-29cc/29cc-201-wsm-p10-min8-error-native-promotion-lock-ssot.md`（min8）
    - `docs/development/current/main/phases/phase-29cc/29cc-202-wsm-p10-min9-debug-native-promotion-lock-ssot.md`（min9）
    - `docs/development/current/main/phases/phase-29cc/29cc-203-wsm-p10-min10-native-promotion-closeout-lock-ssot.md`（min10）
  - wasm lane active next:
    - `none`（P10 closeout complete; monitor-only）
  - wasm migration fixed order（WSM-02+）:
    - `WSM-02a`: assignment/local path unblock（`Copy`/`ReleaseStrong` 最小対応）
    - `WSM-02b`: ExternCall coverage expansion（1 extern familyずつ）
    - `WSM-02c`: BoxCall coverage expansion（1 method familyずつ）
    - `WSM-02d`: boundary gates（supported/unsupported 両境界固定）
  - wasm demo goal pointer:
    - `docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md`
  - execution checklist（progress SSOT）: `docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md`
  - worker playbook: `docs/development/current/main/phases/phase-29cc/29cc-91-worker-parallel-playbook.md`
  - non-plugin residue task-set: `docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md`
  - RNR-05 shape SSOT: `docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md`
  - de-rust done judgement matrix SSOT:
    - `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`（X32/X33/X34/X35 replay）
- perf lane: `phase-21.5 / llvm-aot-hotpath`（monitor-only）
  - scope: `numeric_mixed_medium` / `method_call_only` / `chip8_kernel_small` / `kilo_kernel_small`（C/AOT 比較） + VM monitor-only
  - task SSOT: `docs/private/roadmap/phases/phase-21.5/PLAN.md`
  - Step-2 acceptance lock (fixed):
    - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
    - `cargo test -p nyash_kernel box_from_i8_string_const_reuses_handle -- --nocapture`
    - `PERF_LADDER_AOT_SMALL=1 PERF_LADDER_AOT_MEDIUM=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_progressive_ladder_21_5.sh quick`（AOT行 `status=ok`）
  - WSL variance lock（single-run値で判定しない）:
    - canonical measure: `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`
    - latest (2026-02-28): `c_ms=77`, `py_ms=108`, `ny_vm_ms=979`, `ny_aot_ms=905`, `ratio_c_aot=0.09`, `aot_status=ok`
  - active next: `none`（monitor-only）
  - optimization resume policy（fixed）:
    - 脱Rustの経路固定（29cc-220/215/216/217 の guard 緑）が揃うまで最適化は monitor-only。
    - 最適化タスクは `Future Ideas` 扱いに固定し、runtime/plugin loader 経路の薄化完了後に再昇格する。

## Immediate Next (this round)

1. wasm lane: `WSM-P7..P10-min10` + `WSM-G4-min9/min10` lock（`29cc-184..203`, `29cc-206`, `29cc-207`）を維持し、default-only 契約を崩さない。
2. Rust WASM lifecycle は `Stop -> Freeze -> Retire` の固定順で運用する（SSOT: `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`）。
3. 現在は Stop 段として Rust WASM 新機能追加を停止し、`.hako` 主ラインで shape/gate を先行する。
4. wasm lane は monitor-only を維持し、failure-driven でのみ blocker を再起動する。
5. wasm route は `hako_native/rust_native/legacy_bridge` の 3 つに固定し、新規 route を増やさない。
6. Freeze 監査は `tools/checks/dev_gate.sh wasm-freeze-core` / `tools/checks/dev_gate.sh wasm-freeze-parity` を正本にする（min3: `rust_native` compile-wasm-only scope lock を含む）。
7. plugin de-rust HM2（min1/min2/min3）は done。plugin lane は monitor-only（`active next: none`）を維持し、failure-driven でのみ reopen する。
8. de-rust runtime は `29cc-220` を active lock とし、long-term は source-zero、現フェーズは route-zero + stability（no-delete-first）で C ABI cutover 順を固定する。
9. route drift 監査は `29cc-215`（observability）+ `29cc-217`（VM+AOT route）を正本にして運用する。
10. V0 ABI slice（3語彙）は `29cc-216` lock を正本にし、`string_len/array_get_i64/array_set_i64` 以外を混ぜない。
11. de-rust residue（plugin kernel + plugin_loader_v2）line は `29cc-221` 固定順（A1..A3/B1..B3）を維持し、実装真実ベースで closeout 再検証中。runtime lane は route-zero-sync closeout（29cc-243）方針を維持しつつ、compat入口の残件同期が終わるまで failure-driven で扱う。
12. B1-min1 lock（docs-first）:
  - `docs/development/current/main/phases/phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md`
13. B1-min1 closeout lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md`
14. B1-min2 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-233-kernel-b1-min2-runtime-state-route-lock-ssot.md`
15. B1-min3 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-234-kernel-b1-min3-instance-lifecycle-route-lock-ssot.md`
16. B1-closeout lock（accepted / implementation-truth recheck）:
  - `docs/development/current/main/phases/phase-29cc/29cc-235-kernel-b1-closeout-lock-ssot.md`
17. B2-min1 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-236-kernel-b2-min1-value-codec-encode-decode-route-lock-ssot.md`
18. B2-min2 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-237-kernel-b2-min2-borrowed-handle-route-lock-ssot.md`
19. B2-closeout lock（accepted / implementation-truth recheck）:
  - `docs/development/current/main/phases/phase-29cc/29cc-238-kernel-b2-closeout-lock-ssot.md`
20. B3-min1 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-239-kernel-b3-min1-future-route-lock-ssot.md`
21. B3-min2 lock（done）:
  - `docs/development/current/main/phases/phase-29cc/29cc-240-kernel-b3-min2-invoke-route-lock-ssot.md`
22. B3-closeout lock（accepted / implementation-truth recheck）:
  - `docs/development/current/main/phases/phase-29cc/29cc-241-kernel-b3-closeout-lock-ssot.md`
23. kernel-residue-closeout lock（accepted / implementation-truth recheck）:
  - `docs/development/current/main/phases/phase-29cc/29cc-242-kernel-residue-closeout-lock-ssot.md`
24. active next: 29bq-selfhost-hako-migration（mirbuilder first / parser later）
  - handoff lock:
    - `docs/development/current/main/phases/phase-29cc/29cc-243-runtime-route-zero-sync-closeout-lock-ssot.md`
  - execution order SSOT:
    - `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
    - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - latest quick evidence（2026-02-28, Stage2 fixed）:
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
    - `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4` PASS（`5/5`, `stageb_total_secs=19`, `avg_case_secs=3.80`）
    - `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` PASS
12. AOT/perf 最適化は route-zero + stability 達成後に再開可否を判断する（いまは monitor-only 維持）。

## Future Ideas (Not Active)

- Python AOT / HybridPy / Translation Validation / ReproBuild などの研究案は `future backlog` 扱いに固定し、Current blocker には含めない。
- optimization annotation（`@hint/@contract/@intrinsic_candidate`）は parser noop まで実装済みだが、本利用（verifier/registry/backend）は `not active` として扱う（SSOT: `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`）。
- public summary: `docs/development/current/main/30-Backlog.md`
- private canonical: `docs/private/development/current/main/30-Backlog.md`
- 運用ルール: backlog 案を採用する時だけ docs-first で lane/task に昇格する（それまでは monitor-only）。

## Quick Restart (After Reboot)

- 単一入口: `docs/development/current/main/05-Restart-Quick-Resume.md`
- 最短再開コマンド:
  - `cd /home/tomoaki/git/hakorune-selfhost`
  - `git status -sb`
  - `tools/checks/dev_gate.sh quick`
  - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
  - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
- 再開タスク: `phase-29y / fixed-order monitor`（selfhost/de-rust mainline guard）

## Read-First Navigation

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
4. `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
5. `docs/development/current/main/design/de-rust-lane-map-ssot.md`
6. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
7. `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
8. `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
9. `docs/tools/README.md`

## Daily Commands

- `tools/checks/dev_gate.sh quick`（推奨: 日常の軽量セット）
- `tools/checks/dev_gate.sh hotpath`（perf/hotpath を触ったとき）
- `tools/checks/dev_gate.sh --list`（profile内容の確認）
- `cargo check --bin hakorune`
- `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`
- `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh`
- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- `bash tools/selfhost/run_lane_a_daily.sh`
- `bash tools/checks/ring1_core_scope_guard.sh`
- `bash tools/checks/module_registry_hygiene_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port01_parity_probe_vm.sh`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_vm_hako_caps_gate_vm.sh`
- `PYTHONPATH=src/llvm_py:. python -m unittest src/llvm_py/tests/test_ret_dominance.py`
- `PERF_SUBTRACT_STARTUP=1 bash tools/perf/bench_compare_c_vs_hako.sh box_create_destroy_small 1 3`
- `tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath`
- `tools/perf/run_progressive_ladder_21_5.sh quick`

## Milestone Commands

- `tools/checks/dev_gate.sh milestone-runtime`（節目: runtime/selfhost 側）
- `tools/checks/dev_gate.sh milestone-perf`（節目: perf 側）
- `tools/checks/dev_gate.sh milestone`（推奨: 統合セット）
- `tools/checks/dev_gate.sh portability`（週次: Windows/macOS portability preflight）
- `tools/checks/dev_gate.sh runtime-exec-zero`（execution-path-zero 観測 + VM+AOT route lock; source-zero まで中間ゲート）
- `bash tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh`（WSM-P7 default-only 監査）
- `bash tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh`（WSM-P8 compat bridge retire readiness 監査）
- `bash tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh`（WSM-P9 non-native shrink 監査）
- `bash tools/checks/phase29cc_wsm_p9_bridge_retire_refresh_guard.sh`（WSM-P9 bridge retire refresh 監査）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh`（WSM-P10 loop/extern native emit design lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh`（WSM-P10 loop/extern matcher inventory lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh`（WSM-P10 loop/extern writer section lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh`（WSM-P10 single fixture native promotion lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh`（WSM-P10 expansion inventory lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh`（WSM-P10 warn native promotion lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh`（WSM-P10 info native promotion lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_error_native_promotion_guard.sh`（WSM-P10 error native promotion lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh`（WSM-P10 debug native promotion lock 監査）
- `bash tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh`（WSM-P10 native promotion closeout lock 監査）
- `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`（WSL環境の週次Windows smoke）
- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `tools/perf/run_phase21_5_perf_gate_bundle.sh apps`
- `tools/perf/run_phase21_5_perf_gate_bundle.sh full`
- `PERF_STABILITY_INCLUDE_MEDIUM=1 PERF_STABILITY_INCLUDE_APPS=1 PERF_STABILITY_WRITE_BASELINE=1 tools/perf/record_baseline_stability_21_5.sh 2 1 1`
- `bash tools/checks/phase21_5_perf_regression_guard.sh`

## Runtime Diagnostic Pins (non-gating)

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_continue_assignment_in_continue_stale_guard_vm.sh`

## Compiler Diagnostic Pins (non-gating)

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh`

運用ルール:
- FAIL 時のみ failure-driven 保守へ切替（上限60分）。
- 60分で復旧しない場合は blocker メモを `CURRENT_TASK.md` と `phase-29y/60-NEXT-TASK-PLAN.md` に同期する。

## Runtime Debug Flow (Rust vs .hako)

- 詳細手順は `phase-29y/60-NEXT-TASK-PLAN.md` の `0.5 Debug Procedure Lock` を正本とする。
- lane 未確定のまま複数層へ同時パッチを入れない。
- known parity debt（non-blocking, lane B monitor item）:
  - expression lowering（nested ternary family）は Rust route 修正先行の既知差分候補として扱う。
  - probe fixture（`phase29y_hako_emit_mir_nested_ternary_probe_min.hako`）は strict parity lock 済み。未対応の ternary 形は引き続き fail-fast（`[builder/selfhost-first:unsupported:ternary_no_lower]`）で扱う。
  - Rust route green / `.hako` route NG を観測した時点で lane B blocker を再起動し、ported 昇格を凍結する。

## Quick Entry: Selfhost Migration

1. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
4. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`

## Quick Entry: De-Rust Lanes (A/B/C)

1. master task map（overall order / done contract）
   - `docs/development/current/main/design/de-rust-master-task-map-ssot.md`
2. scope decision（de-rust done boundary）
   - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
3. lane map（single source）
   - `docs/development/current/main/design/de-rust-lane-map-ssot.md`
4. lane A（compiler meaning）
   - `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md`
5. lane B（compiler pipeline）
   - `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
   - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
6. lane C（runtime port）
   - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
7. orchestration lane（M0-M4）
   - `docs/development/current/main/phases/phase-29cc/README.md`
   - `docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md`
   - `docs/development/current/main/phases/phase-29cc/29cc-91-worker-parallel-playbook.md`
   - `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
   - `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
   - `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
8. de-rust done judgement matrix（X32-X35）
   - `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`

## Curated SSOT Pointers

- Now index: `docs/development/current/main/10-Now.md`
- Backlog: `docs/development/current/main/30-Backlog.md`
- Dev tools quick entry: `docs/tools/README.md`
- De-rust master task map: `docs/development/current/main/design/de-rust-master-task-map-ssot.md`
- De-rust scope decision: `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
- De-rust lane map (A/B/C): `docs/development/current/main/design/de-rust-lane-map-ssot.md`
- De-rust plugin lane bootstrap: `docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md`
- De-rust plugin ABI lock (PLG-01): `docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
- De-rust plugin gate pack lock (PLG-02): `docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md`
- De-rust plugin wave-1 pilot lock (PLG-03): `docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min1): `docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min2): `docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min3): `docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min4): `docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min5): `docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md`
- De-rust plugin wave rollout lock (PLG-04-min6): `docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md`
- De-rust post-wave1 route lock: `docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md`
- De-rust plugin wave-2 entry lock (PLG-05-min1): `docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min2): `docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min3): `docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min4): `docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min5): `docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min6): `docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md`
- De-rust plugin wave-2 rollout lock (PLG-05-min7): `docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md`
- De-rust plugin wave-3 entry lock (PLG-06-min1): `docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min2): `docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min3): `docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md`
- De-rust plugin wave-3 rollout lock (PLG-06-min4): `docs/development/current/main/phases/phase-29cc/29cc-116-plg06-egui-wave3-min4-ssot.md`
- De-rust wasm lane lock (WSM-01): `docs/development/current/main/phases/phase-29cc/29cc-117-wsm01-wasm-unsupported-inventory-sync-ssot.md`
- De-rust wasm grammar/map lock: `docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md`
- De-rust wasm lane lock (WSM-02a): `docs/development/current/main/phases/phase-29cc/29cc-119-wsm02a-assignment-local-unblock-ssot.md`
- De-rust wasm demo-goal lock: `docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md`
- De-rust wasm lane lock (WSM-02b-min1): `docs/development/current/main/phases/phase-29cc/29cc-121-wsm02b-min1-console-warn-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min2): `docs/development/current/main/phases/phase-29cc/29cc-122-wsm02b-min2-console-error-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min3): `docs/development/current/main/phases/phase-29cc/29cc-123-wsm02b-min3-console-info-extern-ssot.md`
- De-rust wasm lane lock (WSM-02b-min4): `docs/development/current/main/phases/phase-29cc/29cc-124-wsm02b-min4-console-debug-extern-ssot.md`
- De-rust wasm lane lock (WSM-02c-min1): `docs/development/current/main/phases/phase-29cc/29cc-125-wsm02c-min1-boxcall-console-info-ssot.md`
- De-rust wasm lane lock (WSM-02c-min2): `docs/development/current/main/phases/phase-29cc/29cc-126-wsm02c-min2-boxcall-console-debug-ssot.md`
- De-rust wasm lane lock (WSM-02c-min3): `docs/development/current/main/phases/phase-29cc/29cc-127-wsm02c-min3-boxcall-console-warn-ssot.md`
- De-rust wasm lane lock (WSM-02c-min4): `docs/development/current/main/phases/phase-29cc/29cc-128-wsm02c-min4-boxcall-console-error-ssot.md`
- De-rust wasm lane lock (WSM-02d-min1): `docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md`
- De-rust wasm lane lock (WSM-02d-min2): `docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md`
- De-rust wasm lane lock (WSM-02d-min3): `docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md`
- De-rust wasm lane lock (WSM-02d-min4): `docs/development/current/main/phases/phase-29cc/29cc-132-wsm02d-min4-milestone-gate-promotion-lock-ssot.md`
- De-rust done judgement matrix (X32-X35): `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`
- Code retirement/history policy: `docs/development/current/main/design/code-retirement-history-policy-ssot.md`
- Compiler task order: `docs/development/current/main/design/compiler-task-map-ssot.md`
- Compiler pipeline: `docs/development/current/main/design/compiler-pipeline-ssot.md`
- Compiler de-rust roadmap: `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md`
- Runtime phase README: `docs/development/current/main/phases/phase-29y/README.md`
- Runtime timeline archive: `docs/development/current/main/phases/phase-29y/61-NEXT-TASK-HISTORY.md`

## Maintenance Rule

- このファイルに完了ログを積み上げない。
- 進捗は phase 文書へ記録し、ここは「入口リンク」と「current blocker」だけを更新する。

## Remaining Tasks (ordered; SSOT)

- smoke hygiene lane (new):
  1. [in-progress] strict VM lane contract: `HAKO_JOINIR_STRICT=1` スモークで rust-vm想定ケースの `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を段階適用（vm-hako専用ゲートは除外）
  2. [in-progress] LLVM lock contract: `tools/build_llvm.sh` の global lock + `llvm_exe_with_build_lock` 連携で並列衝突を封止
  3. [in-progress] inventory SSOT: `tools/checks/smoke_inventory_report.sh` で `integration/apps` の referenced/orphan candidate を可視化
  4. [in-progress] orphan candidate を `daily/quick/milestone/full` の入口基準で classify し、archive or gate 統合計画を phase docs に反映
  5. [done] alias-only wrapper 5本を削除（`*_block_vm.sh -> *_ported_vm.sh`）
  6. [in-progress] inventory baseline refresh: `integration/apps` 382（referenced 156 / orphan candidate 226）
  7. [done] triage SSOT 作成: `docs/development/current/main/investigations/smoke-inventory-triage-2026-02-26.md`
  8. [done] cargo output filename collision warning を抑制（`Cargo.toml` の `[lib] crate-type` を `rlib` 単独へ調整）

- config hygiene lane:
  1. [done] `[modules]` 直追加停止 + export追加導線固定
  2. [done] `selfhost.vm.helpers.mini_map` override withdrawn
- compiler pipeline lane:
  1. monitor-only（regression 時のみ failure-driven）
  2. diagnostic pin 3本を non-gating で維持
- runtime lane:
  1. `phase-29y/60-NEXT-TASK-PLAN.md` の fixed order（monitor-only）を維持
- perf lane:
  1. [done] `bench_compare_c_vs_hako.sh` を bench-profile（`NYASH_VM_FAST=1`）で固定
  2. [done] Rust VM: `new StringBox("const")` hot path を軽量化（`NYASH_VM_FAST` 配下）
  3. [done] C/Python/VM baseline を再計測して next hotspot を確定
  4. [done] LLVM/AOT Step-1: `StringBox.length/size` の extern fast-path（`NYASH_LLVM_FAST=1`）を実装
  5. [done] MIR shape contract smoke を2ベンチで固定（`phase21_5_perf_mir_shape_contract_vm.sh`）
  6. [done] small→medium→app の計測導線を固定（`run_progressive_ladder_21_5.sh` / `bench_apps_wallclock.sh`）
  7. [done] AOT 出力契約を固定（`bench_compare` / `record_baselines` に `status/reason/stage` 付与）
  8. [done] AOT bench（`PERF_AOT=1`）で C/VM/AOT 差分を固定（small/medium とも `status=ok`）
  9. [done] LLVM/AOT Step-2（optional）: `new StringBox("const")` の const-intern/global-data 経路を導入
  10. [done] LLVM/AOT Step-3: `method_call_only` medium の call dispatch hotspot を縮退（AOT 276ms→7ms, ratio 0.01→0.43）
  11. [done] LLVM/AOT Step-4a: branch-only compare を i1 直結化（`phase21_5_perf_loop_i1_branch_contract_vm.sh`）
  12. [done] LLVM/AOT Step-4b-1: const string boxer hoist（`phase21_5_perf_const_hoist_contract_vm.sh`）
  13. [done] LLVM/AOT Step-4b-2: direct emit dominance 崩れを fail-fast 契約で固定（`phase21_5_perf_direct_emit_dominance_block_vm.sh`）
  14. [done] LLVM/AOT Step-4b-3: loop integer hotspot（binop/safepoint周辺）の縮退（`phase21_5_perf_loop_integer_hotspot_contract_vm.sh`）
  15. [done] LLVM/AOT Step-5: medium/app regression lock（`run_progressive_ladder_21_5.sh` default guard / contract smoke / baseline 差分監視）
  16. [done] NYASH_VM_FAST Step-6a: `mir_shape_guard` scanner hotspot narrowing（`MIR_SHAPE_PROFILE=1` と scan budget 契約を固定）
  17. [done] NYASH_VM_FAST Step-6b: app compile/run split visibility（`bench_apps_wallclock` `timing_ms.*` + `phase21_5_perf_apps_compile_run_split_contract_vm.sh`）
  18. [done] NYASH_VM_FAST Step-6c: app prebuilt-MIR route fix + one-route prep（`--emit-mir-json`→`--mir-json-file` route contract を固定）
  19. [done] NYASH_VM_FAST Step-6d: app entry source/mir_shape_prebuilt delta contract（`bench_apps_wallclock` app entry mode + `phase21_5_perf_apps_entry_mode_contract_vm.sh`）
  20. [done] NYASH_VM_FAST Step-6e: app entry mode compare + delta contract（`bench_apps_entry_mode_compare.sh` + delta smoke）
  21. [done] NYASH_VM_FAST Step-6f: entry-mode significance tuning + gate threshold lock（`PERF_APPS_ENTRY_MODE_SIGNIFICANCE_MS` + significance smoke/gate）
  22. [done] NYASH_VM_FAST Step-6g: entry-mode sample spread + hotspot lock（`--json-lines` compare + spread/hotspot smoke/gate）
  23. [done] Step-5 ext: entry-mode baseline/regression lock（`record_baseline_stability_21_5.sh` + `phase21_5_perf_regression_guard.sh`）
  24. [done] perf lane: failure-driven monitor-only（monitor-only 運用を経て APP-PERF 系へ再起動）
  25. [done] PERF-TRUTH-01: `bench_compare_compile_run_split.sh` を prebuilt fail-fast 契約に固定（`emit_route` 明示）
  26. [done] PERF-TRUTH-02: `numeric_mixed_medium` の `--hako-emit-mir-json` -> `--mir-json-file` 実行不整合（phi pred mismatch）を修正
  27. [done] VM-HOT-02: `numeric_mixed_medium` の VM 実行ホットパス（mod/compare/binop）縮退
    - `NYASH_VM_FAST_REGFILE`（bench profile 既定ON）で dense ValueId slot を導入
    - arithmetic/phi/branch の hot path を alias-aware + slot-aware 化
    - phase21_5 perf gate 緑を維持したまま `numeric_mixed_medium` の VM 実行を縮退
  28. [done] VM-HOT-03: HashMap insert hotspot の残り（非算術 write path）を slot writer へ段階移行
    - `write_reg` を slot-first 化し、`fast-regfile` 時は copy-alias/caches を直通バイパス
    - `handle_copy` を `fast-regfile` 専用経路で単純 copy に縮退（alias map 更新を停止）
    - `numeric_mixed_medium` 実測: `NYASH_VM_FAST_REGFILE=0` で `ny_ms=511` → `=1` で `ny_ms=288`（`bench_compare ... 1 3`）
  29. [done] VM-HOT-04: `write_reg` ホットスポットの残り（slot path 分岐/resize 判定）を最小化
    - 関数開始時に `next_value_id` ベースで fast-regfile slot を先取り確保
    - hot loop（diagnostic OFF）で `Const/BinOp/Unary/Compare/Copy` を direct dispatch 化
    - `numeric_mixed_medium` 実測: `NYASH_VM_FAST_REGFILE=0` で `ny_ms=450` → `=1` で `ny_ms=253`（`bench_compare ... 1 3`）
  30. [done] VM-HOT-05: fast-regfile register contract lock（write/read/remove の単一路化）
    - `self.regs.insert/get/remove` の直参照を Interpreter 内部 helper に集約（`write_reg`/`reg_peek_resolved`/`take_reg`）
    - `boxes/memory/type_ops/weak/select/destination_helpers` を slot-aware write/read 契約へ統一
    - `release_strong_refs` を slot+map 両走査に拡張し、fast-regfile 時の解放漏れを封じた
    - gate 追加: `phase21_5_perf_fast_regfile_contract_vm.sh`（`method_call_only_small` / `box_create_destroy_small` preflight lock）
  31. [done] VM-HOT-06: `execute_instruction` 残命令（Load/Store/Call）の hot-loop 直ディスパッチ化を評価
    - hot loop（diagnostic OFF）で `Load/Store/Call` を direct dispatch 化
    - `phase21_5_perf_fast_regfile_contract_vm.sh` と `phase21_5_perf_gate_vm.sh` が継続で緑
    - 実測（`PERF_SUBTRACT_STARTUP=1 bench_compare ... 1 3`）:
      - `method_call_only_small`: `c_ms=3 ny_ms=1 ratio=3.00`
      - `box_create_destroy_small`: `c_ms=2 ny_ms=3 ratio=0.67`
      - `numeric_mixed_medium`: `c_ms=4 ny_ms=276 ratio=0.01`
  32. [done] VM-HOT-07: hot-loop fallback 命令（`TypeOp/Select/RefNew/WeakRef`）の直ディスパッチ化を failure-driven で評価
    - hot loop（diagnostic OFF）で `TypeOp/Select/WeakRef/RefNew` を direct dispatch 化
    - `phase21_5_perf_fast_regfile_contract_vm.sh` と `phase21_5_perf_gate_vm.sh` が継続で緑
    - 実測（`PERF_SUBTRACT_STARTUP=1 bench_compare ... 1 3`）:
      - `method_call_only_small`: `c_ms=2 ny_ms=2 ratio=1.00`
      - `box_create_destroy_small`: `c_ms=3 ny_ms=3 ratio=1.00`
      - `numeric_mixed_medium`: `c_ms=4 ny_ms=271 ratio=0.01`
  33. [done] LLVM-HOT-01: AOT helper build で `NYASH_LLVM_FAST_INT` を既定ON化（perf-only）
    - `tools/perf/lib/aot_helpers.sh` の AOT build 呼び出しで `NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}"` を追加
    - 契約固定: `phase21_5_perf_bench_env_contract_vm.sh` で `NYASH_LLVM_FAST` / `NYASH_LLVM_FAST_INT` pin を検証
    - 契約固定: `bench_compare_c_vs_hako.sh` の AOT EXE 出力を PID suffix 化（同時実行時の `perf_ny_<key>.exe` 競合を防止）
    - 参照更新: `benchmarks/README.md` に `NYASH_LLVM_FAST_INT` 導線を追加
    - 実測（`PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 PERF_SUBTRACT_STARTUP=0 bench_compare ...`）:
      - `numeric_mixed_medium (aot)`: `ny_aot_ms=8`（`FAST_INT=0/1` で有意差なし）
      - `method_call_only (aot)`: `ny_aot_ms=6`（`FAST_INT=0/1` で有意差なし）
  34. [done] LLVM-HOT-01b: AOT fast-link を non-PIE 契約で固定（perf-only）
    - `crates/nyash-llvm-compiler/src/main.rs` の link path で `NYASH_LLVM_FAST=1` かつ Linux 時に `-no-pie` を付与
    - 契約追加: `phase21_5_perf_aot_link_mode_contract_vm.sh`（`readelf -h` の `Type: EXEC` を検証）
    - gate 配線: `phase21_5_perf_gate_vm.sh` に `PERF_GATE_AOT_LINK_MODE_CHECK=1`（optional）を追加
    - 参照更新: `benchmarks/README.md` に AOT fast-link と gate toggle 導線を追加
  35. [done] LLVM-HOT-02a: trivial PHI alias 配線（copy-like）を導入して無駄PHIを縮退
    - `FunctionLowerContext` に `phi_trivial_aliases`（`(block_id,dst_vid)->src_vid`）を追加し、関数ローカルSSOT化
    - `phi_wiring/tagging.py` で trivial PHI 判定を追加
      - 全incoming同一src
    - trivial alias は placeholder/wiring をスキップし、`resolver.resolve_i64` で src resolve に直結
    - `block_lower` の snapshot へ alias dst を明示 materialize して、後続PHI/terminator 解決を安定化
    - safety fix: self-carry invariant alias は dominance 非自明で `numeric_mixed_medium` の loop bound を壊すため撤回（copy-like のみ維持）
    - 追加テスト: `src/llvm_py/tests/test_phi_trivial_alias.py`（self-carry は alias しない契約）
    - 検証:
      - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_phi_trivial_alias.py`
      - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
      - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_ret_dominance.py`
      - `PERF_GATE_AOT_LINK_MODE_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
      - `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 PERF_SUBTRACT_STARTUP=0 bash tools/perf/bench_compare_c_vs_hako.sh numeric_mixed_medium 2 11`
  36. [done] LLVM-HOT-02b: `%`/compare chain 契約を numeric ケースで固定（failure-driven guard）
    - 追加スモーク: `phase21_5_perf_numeric_compare_chain_contract_vm.sh`
      - `main` の先頭 loop guard が `< 0` に崩れないことを検証
      - `%` chain（31/97/89/53/17）と compare chain（10/20）の IR shape を固定
      - compare i1->i64 round-trip 不在を検証
    - gate optional 配線: `phase21_5_perf_gate_vm.sh` に `PERF_GATE_NUMERIC_COMPARE_CHAIN_CHECK=1`
    - smoke 側の `NYASH_LLVM_SKIP_BUILD` 既定を `1` に固定（重い再ビルドの常時発火を防止）
  37. [done] LLVM-HOT-02c: FAST IR pass トグル契約（ON/OFF）を固定
    - 追加スモーク: `phase21_5_perf_fast_ir_passes_contract_vm.sh`
      - `NYASH_LLVM_FAST_IR_PASSES` の default-on wiring（`llvm_builder.py`）を static contract で固定
      - `NYASH_LLVM_FAST_IR_PASSES=1/0` の両方で `bench_compare ... method_call_only_small` AOT `status=ok` を確認
    - gate optional 配線: `phase21_5_perf_gate_vm.sh` に `PERF_GATE_FAST_IR_PASSES_CHECK=1`
    - docs 導線: `benchmarks/README.md` Gate Toggles に `PERF_GATE_FAST_IR_PASSES_CHECK` を追加
  38. [done] LLVM-HOT-02d: `%`/compare hotspot trace 契約を追加（failure-driven 可観測化）
    - `NYASH_LLVM_HOT_TRACE=1` で `[llvm/hot]` の 1行 summary を導入（`binop_total/binop_mod/compare_* / resolve_*`）
    - `resolve_i64_strict(... hot_scope=binop|compare)` で fallback/local/global カウントを関数ローカルに集約
    - 追加スモーク: `phase21_5_perf_numeric_hot_trace_contract_vm.sh`
      - `numeric_mixed_medium (aot)` で `[llvm/hot] fn=main` と数値フィールド契約を固定
      - `compare_to_i64=0`（branch-only compare chain）を lock
    - gate optional 配線: `phase21_5_perf_gate_vm.sh` に `PERF_GATE_NUMERIC_HOT_TRACE_CHECK=1`
    - docs 更新: `benchmarks/README.md` と debug contract SSOT に trace 導線を追記
  39. [done] LLVM-HOT-03a: `%`/compare fallback 経路を strict resolver に統一 + ceiling 契約を固定
    - `compare.py` / `binop.py` の `resolver.resolve_i64(...)` 直呼びを `resolve_i64_strict(..., hot_scope=...)` へ寄せ、local/global 優先で fallback を最小化
    - `phase21_5_perf_numeric_hot_trace_contract_vm.sh` に fallback ceiling 契約を追加（既定: `resolve_fallback_binop<=0`, `resolve_fallback_compare<=0`）
      - override: `PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_BINOP`, `PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_COMPARE`
    - docs 更新: `benchmarks/README.md` に hot-trace backend/ceiling env 導線を追加
  40. [done] LLVM-HOT-04a: `method_call_only (aot)` の call path hot trace 観測を固定
    - hot summary に `call_total` / `resolve_*_call` フィールドを追加（`trace.py::HOT_SUMMARY_FIELDS`）
    - `mir_call` dispatcher で `call_total` をカウント
    - call 経路（`call.py`, `mir_call/method_call.py`, `mir_call/global_call.py`）を `resolve_i64_strict(..., hot_scope=\"call\")` に統一
    - 追加スモーク: `phase21_5_perf_method_call_hot_trace_contract_vm.sh`
      - `method_call_only (aot)` で `call_total>0` と `resolve_fallback_call<=0`（既定）を lock
    - gate optional 配線: `PERF_GATE_METHOD_CALL_HOT_TRACE_CHECK=1`
    - docs 更新: `benchmarks/README.md` に method-call hot trace env/toggle 導線を追記
  41. [done] LLVM-HOT-05: `mir_call` 残経路（constructor/value/extern/legacy）の resolve strict 化と call fallback coverage 拡張
    - `constructor/value/extern` + `mir_call_legacy` の `_resolve_arg` を `resolve_i64_strict(..., hot_scope=\"call\")` へ統一
    - `global_call` の broad `except` を狭義化し、`extern` pointer bridge fallback 例外も狭義化
    - hot trace contract docs を `trace.py::HOT_SUMMARY_FIELDS` と同期（`call_total` / `resolve_*_call`）
    - hot trace contract smoke 2本を共通 helper (`tools/smokes/v2/lib/perf_hot_trace_contract.sh`) に集約
  42. [done] LLVM-HOT-06: `closure_call` / `print_marshal` の resolve strict 化と fallback coverage 追加
    - `mir_call/closure_call.py` の `_resolve_arg` を `resolve_i64_strict(..., hot_scope=\"call\")` へ統一
    - `mir_call/print_marshal.py` に strict resolver 導線（`vmap/preds/block_end_values/bb_map` optional）を追加し、broad `except` を狭義化
    - `src/llvm_py/tests/test_mir_call_hot_fallback.py` に closure/print marshal の fallback counter 契約を追加
    - 検証: `test_mir_call_hot_fallback.py` + `phase21_5_perf_*hot_trace*` + optional perf gate 緑
  43. [done] LLVM-HOT-07: `numeric_mixed_medium (aot)` の copy/const hotspot を failure-driven で縮退
    - `copy.py` FAST lane で local `vmap` hit を resolver round-trip より優先（copy dense chain の Pythonオーバーヘッドを縮退）
    - `const.py` に i64 constant cache（`_I64_CONST_CACHE`）を追加し、hot const path の定数生成コストを縮退
    - 追加スモーク: `phase21_5_perf_copy_const_hotspot_contract_vm.sh`
      - static wiring（FAST copy gate / const cache）を固定
      - `numeric_mixed_medium (aot)` の `status=ok` と `ny_aot_ms` ceiling（default 40ms）を契約化
    - gate optional 配線: `PERF_GATE_COPY_CONST_HOTSPOT_CHECK=1`
    - docs 更新: `benchmarks/README.md` に ceiling env / gate toggle 導線を追加
  44. [done] LLVM-HOT-08: AOT FAST lane の native codegen tuning を導入し、契約を固定
    - `build_opts.py` に target-machine SSOT を追加（`create_target_machine_for_target`）
      - `NYASH_LLVM_FAST=1` 時のみ host CPU/features を適用
      - `NYASH_LLVM_FAST_NATIVE=0` で generic target へ fail-safe に戻せるよう固定
    - `llvm_builder.py` / `tools/llvmlite_harness.py` の target machine 生成を SSOT helper 経由に統一
    - 新規スモーク: `phase21_5_perf_fast_native_codegen_contract_vm.sh`
      - `NYASH_LLVM_FAST_NATIVE=1/0` 両方で AOT `status=ok` を固定
    - gate optional 配線: `PERF_GATE_FAST_NATIVE_CODEGEN_CHECK=1`
    - docs 更新:
      - `benchmarks/README.md`（toggle / gate 導線）
      - `docs/reference/environment-variables.md`（`NYASH_LLVM_FAST_NATIVE` 追加）
  45. [done] LLVM-HOT-09: `numeric_mixed_medium (aot)` arithmetic chain CSE（`i%31` 再利用）を FAST lane で固定
    - `FunctionLowerContext` / `Resolver` に function-local expr cache を追加
      - `binop_expr_cache`, `compare_expr_cache`（関数境界で自動初期化）
    - `binop.py` に i64 arithmetic expr-cache を追加（`NYASH_LLVM_FAST=1` 時のみ）
      - cache key: `(block_name, i64, op, lhs_value_id, rhs_value_id)`（可換演算は順序正規化）
      - hot counter: `binop_expr_cache_hit/miss`
    - `compare.py` に integer compare expr-cache を追加（`NYASH_LLVM_FAST=1` 時のみ）
      - keep_i1/i64 モードを key に含めて表現差を分離
      - hot counter: `compare_expr_cache_hit/miss`
    - `trace.py::HOT_SUMMARY_FIELDS` に expr-cache counters を追加（観測SSOT更新）
    - 新規スモーク: `phase21_5_perf_numeric_arith_cse_contract_vm.sh`
      - `numeric_mixed_medium (aot)` で `binop_expr_cache_hit>0` を固定
      - AOT `status=ok` を固定
    - gate optional 配線: `PERF_GATE_NUMERIC_ARITH_CSE_CHECK=1`
    - docs 更新: `benchmarks/README.md`（gate toggle 導線）
    - 実測（hot trace）: `binop_expr_cache_hit=1`（`numeric_mixed_medium (aot)`）
  46. [done] LLVM-HOT-10: AOT perf lane の `NYASH_LLVM_OPT_LEVEL` pin（2/3）契約を固定
    - 新規スモーク: `phase21_5_perf_opt_level_contract_vm.sh`
      - `NYASH_LLVM_OPT_LEVEL=2/3` の両方で `numeric_mixed_medium (aot)` `status=ok` を固定
    - static wiring lock:
      - `src/llvm_py/build_opts.py` の `_OPT_ENV_KEYS`（`NYASH_LLVM_OPT_LEVEL` / `HAKO_LLVM_OPT_LEVEL`）を検証
    - gate optional 配線: `PERF_GATE_OPT_LEVEL_CHECK=1`
    - docs 更新:
      - `benchmarks/README.md`（opt-level toggle / gate 導線）
      - `docs/reference/environment-variables.md`（`NYASH_LLVM_OPT_LEVEL=0..3` 追記）
    - 実測（`numeric_mixed_medium (aot)`, warmup=2/repeat=11）:
      - `opt=2`: `ny_aot_ms=8`
      - `opt=3`: `ny_aot_ms=8`
  47. [done] LLVM-HOT-11: `numeric_mixed_medium (aot)` ceiling を tighter lock（40ms→20ms）へ更新
    - `phase21_5_perf_copy_const_hotspot_contract_vm.sh` の既定 ceiling を `40`→`20` に更新
    - `benchmarks/README.md` の `PERF_COPY_CONST_HOTSPOT_MAX_AOT_MS` 既定値表記を `20` に同期
    - 検証: `phase21_5_perf_copy_const_hotspot_contract_vm.sh` と `phase21_5_perf_gate_vm.sh`（copy/const optional ON）緑
  48. [done] LLVM-HOT-12: `compare_expr_cache_hit` coverage を perf fixture で追加して固定
    - 追加ベンチ:
      - `benchmarks/bench_compare_reuse_small.hako`
      - `benchmarks/c/bench_compare_reuse_small.c`
    - 新規スモーク: `phase21_5_perf_compare_expr_cse_contract_vm.sh`
      - `compare_reuse_small (aot)` で `compare_expr_cache_hit>0` / `compare_expr_cache_miss>0` を固定
      - AOT `status=ok` を固定
    - gate optional 配線: `PERF_GATE_COMPARE_EXPR_CSE_CHECK=1`
    - docs 更新: `benchmarks/README.md`（gate toggle 導線）
    - 実測（warmup=1/repeat=1）:
      - `compare_reuse_small (aot)`: `ny_aot_ms=7`, `status=ok`
  49. [done] LLVM-HOT-13: `compare_reuse_small (aot)` の ceiling lock を追加し、compare CSE bench を regression guard 下に固定
    - 新規スモーク: `phase21_5_perf_compare_reuse_aot_ceiling_contract_vm.sh`
      - `compare_reuse_small (aot)` の `status=ok` と `ny_aot_ms` ceiling（default 20ms）を固定
    - gate optional 配線: `PERF_GATE_COMPARE_REUSE_AOT_CEILING_CHECK=1`
    - docs 更新: `benchmarks/README.md`（gate toggle + `PERF_COMPARE_REUSE_AOT_MAX_MS` 導線）
    - 実測（warmup=2/repeat=11）:
      - `compare_reuse_small (aot)`: `ny_aot_ms=6`, `status=ok`
  50. [done] LLVM-HOT-14: `compare_reuse_small` を progressive ladder の optional medium key に追加
    - 新規スモーク: `phase21_5_perf_ladder_extra_medium_key_contract_vm.sh`
      - `PERF_LADDER_EXTRA_MEDIUM_KEYS=compare_reuse_small` で ladder medium step と bench 出力を固定
      - default `MEDIUM_KEYS` は不変（`box_create_destroy` / `method_call_only`）を固定
    - gate optional 配線: `PERF_GATE_LADDER_EXTRA_MEDIUM_CHECK=1`
    - docs 更新: `benchmarks/README.md`（extra medium 実行例 + gate toggle 導線）
  51. [done] LLVM-HOT-15: perf gate optional toggles の preset 実行導線を追加
    - 新規ランナー: `tools/perf/run_phase21_5_perf_gate_bundle.sh`
      - `quick|hotpath|apps|full` で optional toggles を束ねて単一実行
      - default gate 挙動は不変（`quick` で core-only）
    - docs 更新:
      - `benchmarks/README.md`（Perf Gate preset wrapper 導線）
      - `docs/tools/README.md`（tools quick entry 導線）
  52. [done] APP-PERF-01: chip8 kernel の real-app crosslang baseline を追加
    - 目的: VM / LLVM-AOT / C / Python の4系統を同一ロジックで比較できる最小契約を固定
    - 指示書: `docs/development/current/main/phases/phase-21.5/APP-PERF-01-CHIP8-CROSSLANG-INSTRUCTIONS.md`
    - 実装:
      - `bench_chip8_kernel_small.{hako,c,py}`（同一ロジック）
      - `tools/perf/bench_compare_c_py_vs_hako.sh`（4-way summary）
      - `phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh`（契約スモーク）
    - 受け入れ:
      - `aot_status=ok`（chip8_kernel_small, warmup=1/repeat=1）
      - 4-way summary line（`[bench4]`）を固定
      - `phase21_5_perf_gate_vm.sh` 緑維持
    - 実測固定（2026-02-22, `NYASH_LLVM_SKIP_BUILD=1`, warmup=1/repeat=3）:
      - `chip8_kernel_small`: `c_ms=4`, `py_ms=73`, `ny_vm_ms=614`, `ny_aot_ms=6`
      - ratio: `ratio_c_vm=0.01`, `ratio_c_py=0.05`, `ratio_c_aot=0.67`, `aot_status=ok`
  53. [done] APP-PERF-02: kilo kernel の real-app crosslang baseline を追加
    - 目的: text-edit workload で VM/LLVM-AOT/C/Python の4系統比較を固定
    - 実装:
      - `bench_kilo_kernel_small.{hako,c,py}`（同一ロジック）
      - `phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh`（契約スモーク）
      - `phase21_5_perf_gate_optional_steps.tsv` / `run_phase21_5_perf_gate_bundle.sh` full へ optional 配線
    - 受け入れ:
      - `tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 1` 緑
      - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh` 緑
    - 実測固定（2026-02-22, `NYASH_LLVM_SKIP_BUILD=1`, warmup=1/repeat=3）:
      - `kilo_kernel_small`: `c_ms=73`, `py_ms=102`, `ny_vm_ms=970`, `ny_aot_ms=218`
      - ratio: `ratio_c_vm=0.08`, `ratio_c_py=0.72`, `ratio_c_aot=0.33`, `aot_status=ok`
  54. [done] LLVM-HOT-16: chip8 AOT の branch fanout 重複式を hoist/CSE で削減
    - 背景: `chip8_kernel_small` の IR で `(op*73+19)%65536` が分岐ごとに再計算されている
    - 狙い: 1 iteration あたりの `mul/srem` 回数を減らし、AOT 実行時間を縮退
    - 受け入れ:
      - `phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh` 緑
      - `tools/perf/bench_compare_c_py_vs_hako.sh chip8_kernel_small 1 3` の `aot_status=ok` 維持
      - `chip8` IR (`--emit-exe` dump) で重複 `mul/srem` の減少を確認
    - 実測メモ（NYASH_LLVM_SKIP_BUILD=1）:
      - `mul i64 .* 73`: `7 -> 1`
      - `srem i64 .* 65536`: `7 -> 1`
      - `srem i64 .* 16`: `6 -> 1`
  55. [done] LLVM-HOT-17: power-of-two 剰余（`%65536`, `%4096`）の軽量化
    - 狙い: 非負証明可能な経路で `%2^k` を `and` 化し、`srem` ホットパスを縮退
    - 実装: `src/llvm_py/instructions/binop.py` で `%2^k` を `and + signed adjust(select)` に変換（`srem` fallbackは非2^kのみ）
    - 併走整備:
      - `src/llvm_py/builders/function_lower.py`: conservative 非負VID解析を追加
      - `src/llvm_py/context/function_lower_context.py` / `src/llvm_py/resolver.py`: non-negative facts を context/resolver に保持
      - `src/llvm_py/cfg/utils.py`: dominator/reachable helper を追加（既存 cache 再利用の基盤）
    - 受け入れ:
      - `phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh` 緑
      - `tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath` 緑
      - chip8 IR で `srem i64 .*65536|4096` が 0、`and i64 .*65535|4095` が出現
  56. [done] APP-PERF-03: chip8/kilo の app-wallclock 比較導線を統合
    - 狙い: micro + real-app の 4系統比較を同一ハーネスで再生可能にする
    - 実装:
      - `tools/perf/bench_crosslang_apps_bundle.sh`（unified harness）
      - `phase21_5_perf_apps_crosslang_bundle_contract_vm.sh`（契約スモーク）
      - `PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK` optional 配線（gate/bundle）
      - 指示書: `docs/development/current/main/phases/phase-21.5/APP-PERF-03-CROSSLANG-APPS-BUNDLE-INSTRUCTIONS.md`
    - 受け入れ:
      - `tools/perf/bench_crosslang_apps_bundle.sh 1 1 1 1` 緑
      - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_apps_crosslang_bundle_contract_vm.sh` 緑
      - `PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK=1 ... phase21_5_perf_gate_vm.sh` 緑
    - 実測固定（2026-02-22, warmup/repeat=1）:
      - `chip8`: `ratio_c_aot=0.67`, `ny_aot_ms=6`, `aot_status=ok`
      - `kilo`: `ratio_c_aot=0.35`, `ny_aot_ms=213`, `aot_status=ok`
      - `apps`: `total_ms=389`, `hotspot=mir_shape_guard:314`
      - `entry_mode`: `source_total_ms=424`, `prebuilt_total_ms=78`, `delta_ms=-346`, `winner=mir_shape_prebuilt`
  57. [done] LLVM-HOT-18: kilo/text workload の AOT hotspot（substring/indexOf/concat）を縮退
    - 狙い: `kilo_kernel_small` の `ratio_c_aot` を段階的に引き上げる（0.35→0.50 目標）
    - 実装:
      - `nyash.string.substring_hii` の clone-heavy 経路と常時 trace 出力を削除（borrow + 1回 alloc）
      - `nyash.string.indexOf_hh` を追加し、LLVM `mir_call` の `indexOf/1` を direct export call 化
      - `nyash.string.concat_hh` に StringBox×StringBox の hot path を追加（`format!`/汎用 `to_string_box` を回避）
    - 実測（2026-02-23, `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
      - `c_ms=77`, `py_ms=108`, `ny_vm_ms=1012`, `ny_aot_ms=84`, `ratio_c_aot=0.92`, `aot_status=ok`
    - 契約スモーク:
      - `phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh` PASS
      - `phase21_5_perf_apps_crosslang_bundle_contract_vm.sh` PASS
  58. [done] LLVM-HOT-19: perf env hygiene（AOT safepoint default/validation/contract lock）
    - 狙い: perf lane の環境変数を「既定値・fallback・fail-fast」で一貫化し、計測の揺れと設定ミスを防ぐ
    - 実装:
      - `tools/perf/lib/bench_env.sh` に `perf_require_bool_01` / `perf_resolve_bool_01_env` / `perf_resolve_uint_env` / `perf_resolve_aot_timeout_sec` / `perf_resolve_aot_auto_safepoint` を追加
      - `tools/perf/lib/bench_compare_common.sh` を追加し、`bench_compare_c_vs_hako.sh` / `bench_compare_c_py_vs_hako.sh` の timing/preflight 実装を共通化
      - `tools/perf/bench_compare_c_py_vs_hako.sh` / `tools/perf/bench_compare_c_vs_hako.sh` を helper 経由の解決へ統一
      - `tools/perf/run_progressive_ladder_21_5.sh` の `PERF_LADDER_*` boolean を helper で fail-fast 検証
      - `tools/perf/lib/aot_helpers.sh`: `PERF_AOT_SKIP_BUILD` invalid 値を fail-fast へ統一（旧 fail-safe 0 を撤去）
      - 既定: `PERF_AOT_AUTO_SAFEPOINT=0`（未指定時 `NYASH_LLVM_AUTO_SAFEPOINT` fallback）
      - invalid 値は fail-fast（`0|1` boolean / numeric timeout を拒否）
      - 新規スモーク: `phase21_5_perf_aot_auto_safepoint_env_contract_vm.sh`
      - 新規スモーク: `phase21_5_perf_bench_compare_env_contract_vm.sh`
      - gate optional 追加: `PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK`
      - gate optional 追加: `PERF_GATE_BENCH_COMPARE_ENV_CHECK`
    - 受け入れ:
      - `phase21_5_perf_aot_auto_safepoint_env_contract_vm.sh` PASS
      - `PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_KILO_AOT_SAFEPOINT_TOGGLE_CHECK=1 ... phase21_5_perf_gate_vm.sh` PASS
  59. [next] LLVM-HOT-20: kilo/text workload の structural hotspot 仕分け（no-env）
    - 狙い: env トグル依存ではない恒久最適化候補を切り分ける（call/site density と runtime_data/string 境界）
    - 進捗（2026-02-24）:
      - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
        - `c_ms=74`, `py_ms=104`, `ny_vm_ms=974`, `ny_aot_ms=67`, `ratio_c_aot=1.10`, `aot_status=ok`
      - AOT microasm（`tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`）:
        - top: `array_get_by_index` 23.81%, `find_substr_byte_index` 15.92%, `memchr(avx2)` 11.68%, `host_handles::with_pair` 8.05%, `substring_hii` 6.50%
        - 補助観測: `safepoint_and_poll` は 0.32%（現在は支配的ではない）
      - 次アクション（固定）:
        - `array_get_by_index` / `with_pair` / `substring_hii` の read-path を優先順位Aで縮退し、同条件で microasm 再採取
      - cleanup-11（HOT-20 index decode borrow route, 2026-02-24）:
        - `src/runtime/host_handles.rs`: `with_handle()` を追加（single read-lock + borrowed ref）
        - `crates/nyash_kernel/src/plugin/value_codec/decode.rs`: `any_arg_to_index` を `handles::with_handle` 経由へ変更（Arc clone 回避、意味不変）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=74`, `py_ms=102`, `ny_vm_ms=956`, `ny_aot_ms=75`, `ratio_c_aot=0.99`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`: `c_ms=73`, `py_ms=104`, `ny_vm_ms=992`, `ny_aot_ms=69`, `ratio_c_aot=1.06`
      - cleanup-12（HOT-20 array read conversion fast route, 2026-02-24）:
        - `crates/nyash_kernel/src/plugin/value_codec/encode.rs`:
          - `runtime_i64_from_array_item_ref(value, drop_epoch)` を追加
          - array read 向けに `BorrowedHandleBox(source_handle + epoch一致)` を最短復元し、非一致時のみ既存 generic へフォールバック
        - `crates/nyash_kernel/src/plugin/array.rs`:
          - `array_get_by_index` の非整数 read path を `runtime_i64_from_array_item_ref` へ統一
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=75`, `py_ms=103`, `ny_vm_ms=986`, `ny_aot_ms=75`, `ratio_c_aot=1.00`
      - cleanup-13（HOT-20 host-handle/read micro cleanup, 2026-02-24）:
        - `src/runtime/host_handles.rs`:
          - `get_pair` / `with_pair` / `get3` の slot read を直線化（内部 closure を除去）
        - `crates/nyash_kernel/src/plugin/array.rs`:
          - `NYASH_CLI_VERBOSE` 判定を `cli_verbose_enabled()` で once-cache 化（hot path の getenv 連発を抑制）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=79`, `py_ms=108`, `ny_vm_ms=933`, `ny_aot_ms=68`, `ratio_c_aot=1.16`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 20.51%, `find_substr_byte_index` 15.46%, `with_pair` 5.84%, `substring_hii` 5.61%
      - cleanup-14b（HOT-20 borrowed-handle fast hint, 2026-02-24）:
        - `src/box_trait.rs`:
          - `borrowed_handle_source_fast()` を追加（default None）
        - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`:
          - `BorrowedHandleBox` で `borrowed_handle_source_fast` を実装
        - `crates/nyash_kernel/src/plugin/value_codec/encode.rs`:
          - `runtime_i64_from_array_item_ref` が trait hint を優先して handle 復元
          - `as_any/downcast` を通る前に borrowed-handle fast route で返す
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=80`, `py_ms=106`, `ny_vm_ms=985`, `ny_aot_ms=76`, `ratio_c_aot=1.05`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 19.83%（前回 25.34%）, `with_pair` 7.43%, `substring_hii` 4.68%
      - cleanup-15b（HOT-20 string fast-hint unification, 2026-02-24）:
        - `src/box_trait.rs`:
          - `as_str_fast()` を追加（default None）
        - `src/boxes/basic/string_box.rs`:
          - `StringBox` が `as_str_fast` を実装
        - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`:
          - `BorrowedHandleBox` が `as_str_fast` を inner 委譲
        - `crates/nyash_kernel/src/exports/string.rs`:
          - `with_string_pair_direct` / `concat3_hhh` direct path の string 判定を `as_any/downcast` から `as_str_fast` へ統一
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=76`, `py_ms=105`, `ny_vm_ms=984`, `ny_aot_ms=76`, `ratio_c_aot=1.00`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 21.28%, `with_pair` 7.55%, `substring_hii` 6.08%
      - cleanup-16（HOT-20 string pair/triple single-fetch route, 2026-02-24）:
        - `crates/nyash_kernel/src/exports/string.rs`:
          - `with_string_pair_direct` を `handles::with_pair` から `handles::get_pair` 単発取得へ変更
          - `concat3_hhh` の direct route を `get3` 単発取得へ統一（`with3` を撤去）
          - 同一取得結果を direct-string 判定と StringView fallback の両方で再利用
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=74`, `py_ms=104`, `ny_vm_ms=976`, `ny_aot_ms=72`, `ratio_c_aot=1.03`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 20.53%, `with_pair` 7.14%, `substring_hii` 5.89%
      - cleanup-17（HOT-20 substring full/empty early-return, 2026-02-24）:
        - `crates/nyash_kernel/src/exports/string.rs`:
          - `substring_hii` に full-range (`0..len`) の handle 直返却を追加
          - empty-range は shared empty handle を返す経路に統一（test時は従来どおり都度生成）
          - invalid boundary (`sub_opt.is_none()`) も shared empty handle へ統一
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel substring_hii_view_materialize_boundary_contract -- --nocapture`
          - `cargo test -p nyash_kernel substring_hii_fast_off_keeps_stringbox_contract -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=76`, `py_ms=111`, `ny_vm_ms=987`, `ny_aot_ms=69`, `ratio_c_aot=1.10`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 21.77%, `with_pair` 8.00%, `substring_hii` 5.65%
      - cleanup-18（HOT-20 array read helper specialization, 2026-02-24）:
        - `crates/nyash_kernel/src/plugin/handle_helpers.rs`:
          - `array_get_index_encoded_i64` を追加（array read 専用の non-generic helper）
          - `cached_array_entry` / `cache_store` / `encode_array_item_to_i64` を追加し、cache参照と encode経路を局所化
          - `with_*` 系の cache 書き込みを `cache_store` へ統一
        - `crates/nyash_kernel/src/plugin/array.rs`:
          - `array_get_by_index` を専用 helper 経由へ切替（`with_array_box` 汎用 closure 経路から分離）
        - `crates/nyash_kernel/src/plugin/value_codec/mod.rs` / `encode.rs`:
          - 未使用となった `runtime_i64_from_array_item_ref` を撤去（dead code cleanup）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=75`, `py_ms=103`, `ny_vm_ms=954`, `ny_aot_ms=72`, `ratio_c_aot=1.04`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`: `c_ms=73`, `py_ms=102`, `ny_vm_ms=950`, `ny_aot_ms=69`, `ratio_c_aot=1.06`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 25.35%, `with_pair` 6.83%, `substring_hii` 5.68%
      - cleanup-19（HOT-20 string borrow helper route, 2026-02-24）:
        - `src/runtime/host_handles.rs`:
          - `with_str_pair` / `with_str3` を追加（pair/triple を borrow で string 判定）
        - `crates/nyash_kernel/src/exports/string.rs`:
          - `with_string_pair_direct` を `with_str_pair` 経由へ切替
          - `concat3_hhh` direct route を `with_str3` 経由へ切替
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=75`, `py_ms=104`, `ny_vm_ms=971`, `ny_aot_ms=68`, `ratio_c_aot=1.10`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`: `c_ms=73`, `py_ms=102`, `ny_vm_ms=957`, `ny_aot_ms=67`, `ratio_c_aot=1.09`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `array_get_by_index` 21.94%, `with_pair` 7.19%, `substring_hii` 4.75%
      - cleanup-20（HOT-20 portability-first string pair span route, 2026-02-24）:
        - Status: adopted
        - 目的（Class A）:
          - `.hako` 移植後にも残る callshape/contract 最適化として、string pair read-path を span SSOT (`resolve_string_span_pair_from_handles`) へ一本化
          - `with_str_pair` 依存を string exports から撤去し、handle borrow helper 境界を縮退
        - 実装:
          - `crates/nyash_kernel/src/exports/string.rs`
            - `with_string_pair_direct` を撤去
            - `search_string_pair_hh` / `compare_string_pair_hh` / `concat_hh` を span-first へ統一
            - unused `concat_pair_to_owned` を削除
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=80`, `py_ms=109`, `ny_vm_ms=969`, `ny_aot_ms=70`, `ratio_c_aot=1.14`
          - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=75`, `py_ms=101`, `ny_vm_ms=1006`, `ny_aot_ms=89`, `ratio_c_aot=0.84`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `nyash.string.indexOf_hh` 30.40%, `nyash.array.get_hi` 12.25%, `host_handles::get` 11.64%
            - `host_handles::Registry::with_pair` は top から消失（span route 統一は反映）
      - cleanup-21（HOT-20 fast-str pair route + StringView fast hint, 2026-02-24）:
        - Status: adopted
        - 目的（Class A）:
          - string helper の pair read-path を `as_str_fast` 優先へ寄せ、`indexOf/concat/compare` の callshape を軽量化
        - 実装:
          - `crates/nyash_kernel/src/exports/string_view.rs`
            - `StringViewBox::as_str_fast()` を実装（base StringBox slice を直接返す）
          - `src/runtime/host_handles.rs`
            - `Registry::with_str_pair` / `Registry::with_str3` を直実装（borrow + single read-lock）
            - 公開 `with_str_pair/with_str3` は `reg()` 直経由へ統一
          - `crates/nyash_kernel/src/exports/string.rs`
            - `with_string_pair_fast_str` を追加（`with_str_pair` 経由）
            - `with_lossy_string_pair` / `concat_hh` の fast path を `fast-str -> span -> materialize` 順に整理
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=75`, `py_ms=107`, `ny_vm_ms=974`, `ny_aot_ms=78`, `ratio_c_aot=0.96`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `nyash.array.get_hi` 31.38%（次ボトルネック化）
            - `nyash.string.indexOf_hh` は 0.37% まで低下
      - cleanup-22（HOT-20 array read cache-hit direct path, 2026-02-24）:
        - Status: adopted
        - 目的（Class A）:
          - `array_get_hi` read-path の cache hit で Arc clone を避け、typed read を同一 TLS 参照内で完了させる
        - 実装:
          - `crates/nyash_kernel/src/plugin/handle_helpers.rs`
            - `array_get_index_encoded_i64` の cache-hit route を `HANDLE_CACHE.with` 内で直接 `ArrayBox` read + encode へ変更
            - miss 時のみ `handles::get + cache_store` を実行
        - 採用外（probe）:
          - Status: reverted (probe)
          - `with_handle` 借用ルート化は `Registry::with_handle` が 26% まで上昇し逆効果のため不採用（revert 済み）
          - cache 無効化（`array_get_index_encoded_i64` を常時 `handles::get` 直取得）は `ratio_c_aot=0.86` へ退行したため不採用（revert 済み）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測:
          - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`: `c_ms=80`, `py_ms=107`, `ny_vm_ms=964`, `ny_aot_ms=77`, `ratio_c_aot=1.04`
          - `tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
            - top: `std::thread::local::LocalKey::with` 19.02%, `find_substr_byte_index` 13.09%, `Registry::with_str_pair` 4.90%
            - `nyash.array.get_hi` は 1.37% まで低下
      - cleanup-23（HOT-20 LocalKey hotspot source split, 2026-02-24）:
        - Status: probe (partially adopted)
        - 目的:
          - `std::thread::local::LocalKey::with` 19% の主因を `string_span_cache` と `HANDLE_CACHE` で切り分ける
        - 実測:
          - baseline（policy既定）:
            - `PERF_AOT_SKIP_BUILD=0 ... bench_compare ...`: `c_ms=74`, `ny_aot_ms=74`, `ratio_c_aot=1.00`
            - microasm: `LocalKey::with` 19.09%
          - `NYASH_STRING_SPAN_CACHE_POLICY=off`:
            - `PERF_AOT_SKIP_BUILD=0 ... bench_compare ...`: `c_ms=73`, `ny_aot_ms=73`, `ratio_c_aot=1.00`
            - microasm: `LocalKey::with` 20.75%（改善せず）
        - 結論:
          - `LocalKey::with` 支配は string span cache 単体ではなく、`HANDLE_CACHE` 側寄与が主要と判断
        - 採用外（probe）:
          - `with_array_box` を direct `with_handle` 化する試行は `ratio_c_aot=0.99` と改善不足のため不採用（revert 済み）
      - cleanup-24（HOT-20 runtime_data borrow-route safety probe, 2026-02-24）:
        - Status: reverted (safety)
        - 試行:
          - `with_array_or_map` を `handles::with_handle` 借用ルートへ切替し、`runtime_data.*` 側の `HANDLE_CACHE` 参照削減を検証
        - 結果:
          - `runtime_data_dispatch` テストでハング（host-handle read lock 再入の疑い）
        - 対応:
          - 該当変更を即時 revert
          - `with_array_or_map` は `object_from_handle_cached` 経路を維持（安全優先）
    - 進捗（2026-02-23）:
      - `runtime_data -> ArrayBox` 境界で index boxing を外す fast path を追加
        - `src/boxes/array/mod.rs`: `get_index_i64` / `try_set_index_i64` / `set_index_i64` を追加
        - `crates/nyash_kernel/src/plugin/runtime_data.rs`: array get/set が新 fast path を使用
        - `crates/nyash_kernel/src/plugin/array.rs`: array plugin が新 fast path を使用
        - `src/runtime/host_api/host_array_ops.rs`: host dispatch が新 fast path を使用
      - 検証:
        - `cargo check --bin hakorune`
        - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
        - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
      - 実測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`）:
        - `c_ms=77`, `py_ms=108`, `ny_vm_ms=1002`, `ny_aot_ms=76`, `ratio_c_aot=1.01`, `aot_status=ok`
      - cleanup（contract lock, 2026-02-23）:
        - `runtime_data.get_hh(Array)` の負 index は即時 `0` を返す legacy 契約を復元（handle 返却ドリフトを防止）
        - `ArrayBox::has_index_i64` を追加し、`runtime_data.has_hh(Array)` の境界判定SSOTを集約
        - `nyash.array.set_h` の legacy 戻り値（常に `0`）をコメント+テストで固定
        - `plugin/handle_helpers.rs` を追加し、`runtime_data` / `array` / `map` の handle→downcast 重複を集約
        - `docs/reference/runtime/runtime-data-dispatch.md` に Array negative index 契約と legacy ABI note（`array.set_h`/`map.set_*` は completion code `0`）を追記
        - 追加テスト:
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `cargo test -p nyash_kernel array_set_h_legacy_return_code_contract -- --nocapture`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`）:
          - `c_ms=79`, `py_ms=110`, `ny_vm_ms=1007`, `ny_aot_ms=80`, `ratio_c_aot=0.99`, `aot_status=ok`
      - follow-up（HOT-20 text route lock, 2026-02-23）:
        - `src/llvm_py/instructions/mir_call/method_call.py`:
          - `indexOf` / `substring` / `lastIndexOf` で receiver を string-tag 伝播
          - `kilo` nested append (`current + "ln"`) の `set(...,0)` ドリフトを解消
        - 追加契約スモーク:
          - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - 検証: `concat_hh` 密度 (`>=3`) と `runtime_data.set_hhh(..., concat_hh_*)`
        - optional gate mapping:
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK`（`phase21_5_perf_gate_optional_steps.tsv`）
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=79`, `py_ms=109`, `ny_vm_ms=984`, `ny_aot_ms=78`, `ratio_c_aot=1.01`, `aot_status=ok`
      - cleanup-2（HOT-20 cleanliness, 2026-02-23）:
        - `MapBox` 参照経路を単発lookup化
          - `src/boxes/map_box.rs`: `get_opt` / `len` を追加
          - `crates/nyash_kernel/src/plugin/runtime_data.rs`: `get_hh(Map)` を `get_opt` 経由へ（`has+get` を撤去）
        - `Any.length_h` / `Any.is_empty_h` を `ArrayBox::len` / `MapBox::len` 直参照へ
          - `crates/nyash_kernel/src/exports/any.rs`
        - `llvm_py` string-tag 重複を helper 集約
          - `src/llvm_py/instructions/mir_call/method_call.py`: `_mark_receiver_stringish()`
        - `phase21_5_perf_kilo_text_concat_contract_vm.sh` を拡張
          - `[llvm/hot] fn=main` の `resolve_fallback_call=0` を追加固定
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=78`, `py_ms=112`, `ny_vm_ms=1011`, `ny_aot_ms=85`, `ratio_c_aot=0.92`, `aot_status=ok`
      - cleanup-3（HOT-20 string-read lookup, 2026-02-23）:
        - `src/runtime/host_handles.rs`:
          - `get_pair(a,b)` を追加（read-lock 1回で 2 handle を解決）
        - `crates/nyash_kernel/src/exports/string.rs`:
          - `nyash.string.concat_hh` hot path で `get_pair` を使用
          - `nyash.string.indexOf_hh` / `nyash.string.lastIndexOf_hh` で `get_pair` を使用
          - 目的: string helper の read-path lock/acquire 密度を下げる（二重lookup撤去）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=78`, `py_ms=112`, `ny_vm_ms=1017`, `ny_aot_ms=85`, `ratio_c_aot=0.92`, `aot_status=ok`
      - cleanup-4（HOT-20 intrinsic registry SSOT, 2026-02-23）:
        - 設計SSOTを追加:
          - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
          - `GeneralOptimizerBox` / `IntrinsicRegistryBox` / `BackendLayoutBox` の責務を固定
        - `llvm_py` の method 特別扱いリストを registry へ集約（挙動不変）:
          - `src/llvm_py/instructions/mir_call/intrinsic_registry.py` を新設
          - `src/llvm_py/instructions/mir_call/method_call.py`
            - `length/len/size` 判定を `is_length_like_method()` へ統一
            - receiver string-tag 付与判定を `requires_string_receiver_tag()` へ統一
            - string result 判定を `produces_string_result()` へ統一
          - `src/llvm_py/instructions/mir_call_legacy.py` の result-tag 判定も同 registry に統一
          - `src/llvm_py/tests/test_mir_call_intrinsic_registry.py` を追加（分類契約テスト）
        - 入口更新:
          - `docs/development/current/main/design/README.md` に SSOTリンク追加
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_strlen_fast.py`
          - `tools/checks/dev_gate.sh quick`
          - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=81`, `py_ms=113`, `ny_vm_ms=1039`, `ny_aot_ms=88`, `ratio_c_aot=0.92`, `aot_status=ok`
      - cleanup-5（HOT-20 autospecialize docs-first, 2026-02-23）:
        - docs-first（AutoSpecializeBox v0 契約を先に固定）:
          - `docs/development/current/main/design/auto-specialize-box-ssot.md` を追加
          - `docs/development/current/main/design/optimization-ssot-string-helper-density.md` に関連リンク追加
          - `docs/development/current/main/design/README.md` 入口リンク追加
        - 実装（v0 最小）:
          - `src/llvm_py/instructions/mir_call/auto_specialize.py` を追加
            - `receiver_is_stringish()`
            - `prefer_string_len_h_route()`
          - `src/llvm_py/instructions/mir_call/method_call.py`
            - length-like routeで stringish receiver は `nyash.string.len_h` を優先
            - 不成立時は既存 route（`nyrt_string_length` / `nyash.any.length_h`）へ戻す
          - テスト追加:
            - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
            - `src/llvm_py/tests/test_strlen_fast.py` に fast-off の size contract を追加
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `tools/checks/dev_gate.sh quick`
          - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=80`, `py_ms=114`, `ny_vm_ms=1032`, `ny_aot_ms=88`, `ratio_c_aot=0.91`, `aot_status=ok`
      - cleanup-6（HOT-20 concatN v0 docs-first, 2026-02-23）:
        - docs-first（concat3 fold 契約を先に固定）:
          - `docs/development/current/main/design/auto-specialize-box-ssot.md`
            - `AS-02` を追加（`concat_hh` chain -> `concat3_hhh`）
          - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
            - concatN v0 scope を固定（`concat3_hhh` のみ、lowering-only、AST rewrite 禁止）
        - 実装（v0 最小）:
          - `src/llvm_py/instructions/binop.py`
            - one-level chain fold:
              - `(concat_hh(a,b)+c)` -> `concat3_hhh(a,b,c)`
              - `(a+concat_hh(b,c))` -> `concat3_hhh(a,b,c)`
            - 不成立時は `concat_hh` へ即時フォールバック（意味不変）
          - `crates/nyash_kernel/src/exports/string.rs`
            - `nyash.string.concat3_hhh` export を追加
          - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
            - concat helper 密度判定を `concat_hh + concat3_hhh` 合算へ拡張
            - `runtime_data.set_hhh` が `concat_hh_*` または `concat3_hhh_*` を消費する契約に更新
          - テスト追加:
            - `src/llvm_py/tests/test_strlen_fast.py`
              - `test_binop_string_concat_chain_prefers_concat3_hhh`
            - `crates/nyash_kernel/src/tests.rs`
              - `string_concat3_hhh_contract`
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=79`, `py_ms=107`, `ny_vm_ms=1007`, `ny_aot_ms=86`, `ratio_c_aot=0.92`, `aot_status=ok`
      - cleanup-7（HOT-20 concat route cleanliness, 2026-02-23）:
        - `host_handles` に triple lookup helper を追加:
          - `src/runtime/host_handles.rs`: `get3(a,b,c)`（single read-lock）
        - `string export` の concat 実装を共通化:
          - `crates/nyash_kernel/src/exports/string.rs`
            - `string_handle_from_owned` / `concat_to_string_handle` / `to_owned_string_handle_arg`
            - `concat_hh` / `concat3_hhh` の重複 alloc/fallback を統一
            - `concat3_hhh` hot path を `get3` 経由へ
            - `eq_hh` / `lt_hh` も共通 helper 利用へ寄せて drift を低減
        - `llvm_py` concat write reason を実態に合わせて整理:
          - `src/llvm_py/instructions/binop.py`
          - `safe_vmap_write(..., "binop_concat")`
        - テスト強化:
          - `src/llvm_py/tests/test_strlen_fast.py`
            - `a + (b + c)` fold test を追加
            - non-chain `a + b` が `concat_hh` のみを使う fallback test を追加
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - run1: `c_ms=86`, `py_ms=107`, `ny_vm_ms=1041`, `ny_aot_ms=97`, `ratio_c_aot=0.89`, `aot_status=ok`
          - run2: `c_ms=81`, `py_ms=109`, `ny_vm_ms=1018`, `ny_aot_ms=81`, `ratio_c_aot=1.00`, `aot_status=ok`
      - cleanup-8（HOT-20 runtime_data mono-route, docs-first, 2026-02-23）:
        - docs-first:
          - `docs/development/current/main/design/auto-specialize-box-ssot.md`
            - `AS-03` 追加（RuntimeData array mono-route）
          - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
            - scope を `runtime_data` helper density まで拡張
          - `docs/reference/runtime/runtime-data-dispatch.md`
            - lowering 契約を更新（`runtime_data.*` / `array.*`）
        - 実装:
          - `crates/nyash_kernel/src/plugin/array.rs`
            - runtime_data互換 alias export 追加:
              - `nyash.array.get_hh`
              - `nyash.array.set_hhh`
              - `nyash.array.has_hh`
              - `nyash.array.push_hh`
          - `src/llvm_py/instructions/mir_call/auto_specialize.py`
            - array receiver 判定 + `prefer_runtime_data_array_route`
          - `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
            - AS-03 成立時に `nyash.array.*` route を選択
          - `src/llvm_py/instructions/mir_call/method_call.py`
          - `src/llvm_py/instructions/mir_call_legacy.py`
            - shared dispatch helper へ resolver/receiver/args context を渡す
          - array receiver fact 解析:
            - `src/llvm_py/cfg/utils.py`（`collect_arrayish_value_ids`）
            - `src/llvm_py/builders/function_lower.py`
            - `src/llvm_py/context/function_lower_context.py`
            - `src/llvm_py/resolver.py`
        - 契約/テスト:
          - 追加スモーク:
            - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
            - optional gate toggle: `PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK`
          - 更新スモーク:
            - `phase21_5_perf_kilo_text_concat_contract_vm.sh`（`runtime_data.set_hhh` / `array.set_hhh` 両許容）
            - `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`（dispatch symbol の許容範囲を更新）
          - 追加テスト:
            - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
            - `src/llvm_py/tests/test_strlen_fast.py`
            - `crates/nyash_kernel/src/tests.rs` (`array_runtime_data_route_hh_contract_roundtrip`)
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `cargo test -p nyash_kernel array_runtime_data_route_hh_contract_roundtrip -- --nocapture`
          - `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
          - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
        - 再計測（`bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`）:
          - `c_ms=74`, `py_ms=106`, `ny_vm_ms=1032`, `ny_aot_ms=71`, `ratio_c_aot=1.04`, `aot_status=ok`
      - cleanup-9（HOT-20 intrinsic registry contract gate, 2026-02-23）:
        - 目的:
          - `IntrinsicRegistryBox` を宣言テーブル化し、`method/arity/symbol/tags` の整合を fail-fast で固定
          - 注釈導入前（Phase-A, no grammar change）の契約検証導線を optional gate に追加
        - 実装:
          - `src/llvm_py/instructions/mir_call/intrinsic_registry.py`
            - `IntrinsicSpec` テーブル (`_INTRINSIC_SPECS`) を導入
            - `validate_intrinsic_specs` / `lookup_intrinsic_spec` / `iter_intrinsic_specs` / `get_registry_consistency_errors` を追加
            - import時に整合検証（契約破れは fail-fast）
            - 既存 API (`is_length_like_method` / `requires_string_receiver_tag` / `produces_string_result`) は互換維持
          - `src/llvm_py/tests/test_mir_call_intrinsic_registry.py`
            - duplicate `(method,arity)` 検知
            - `intrinsic-candidate` の `symbol/arity` 必須契約
            - lookup/registry consistency を固定
          - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_intrinsic_registry_contract_vm.sh`
            - registry構造 + validator 存在チェック
            - `test_mir_call_intrinsic_registry.py` / `test_mir_call_auto_specialize.py` を実行
          - optional gate mapping:
            - `PERF_GATE_INTRINSIC_REGISTRY_CHECK`
            - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
        - docs sync:
          - `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
            - Phase-A acceptance command を追加
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_intrinsic_registry_contract_vm.sh`
          - `PERF_GATE_INTRINSIC_REGISTRY_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-9b（HOT-20 any.length_h residual route narrowing, 2026-02-23）:
        - 目的:
          - `nyash.any.length_h` に残っていた active route 2 箇所を array/string 直ルートへ縮退
        - 実装:
          - `src/llvm_py/instructions/mir_call/auto_specialize.py`
            - `prefer_array_len_h_route` を追加
          - `src/llvm_py/instructions/mir_call/method_call.py`
            - length-like path: `string.len_h` / `array.len_h` 優先、fallback は `any.length_h`
          - `src/llvm_py/instructions/boxcall.py`
            - `size` path: `string.len_h` / `array.len_h` 優先、fallback は `any.length_h`
          - tests:
            - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
            - `src/llvm_py/tests/test_strlen_fast.py`
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-10（HOT-20 substring_hii view v0, docs-first, 2026-02-23）:
        - 目的:
          - `substring_hii` view 導入前に materialize 境界を SSOT で固定（実装前契約）
        - docs:
          - `docs/development/current/main/design/substring-view-materialize-boundary-ssot.md`（新規）
          - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`（正本リンク追記）
      - cleanup-10b（HOT-20 substring_hii view v0 runtime, 2026-02-23）:
        - 目的:
          - `substring_hii` を `StringView(base_handle+range)` へ切替し、read-only helper は view のまま通す
          - materialize 境界（map/array 永続格納, C ABI）を runtime SSOT に集約
        - 実装:
          - `crates/nyash_kernel/src/exports/string.rs`
            - `StringViewBox` + `resolve_string_span*` を追加
            - `NYASH_LLVM_FAST=1` 時に `substring_hii` は view handle を返却
            - `len_h/concat_hh/concat3_hhh/indexOf_hh/lastIndexOf_hh/charCodeAt_h` を view 対応
            - `StringViewBox.clone_box()` を materialize 境界として固定（persistent store対策）
          - `crates/nyash_kernel/src/exports/any.rs`
            - `any.length_h` / `any.is_empty_h` を view 対応
          - `crates/nyash_kernel/src/tests.rs`
            - `substring_hii_view_materialize_boundary_contract`
            - `substring_hii_fast_off_keeps_stringbox_contract`
        - 検証:
          - `cargo test -p nyash_kernel --lib -- --nocapture`
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=75`, `py_ms=103`, `ny_vm_ms=966`, `ny_aot_ms=67`, `ratio_c_aot=1.12`, `aot_status=ok`
      - cleanup-11（HOT-20 any.length_h residual elimination, 2026-02-23）:
        - 目的:
          - `kilo_kernel_small` main IR に残る `nyash.any.length_h` 2 箇所を pre-lowering 解析で縮退
          - `length` が `substring` より先に現れる順序でも stringish 事実を先に確定
        - 実装:
          - `src/llvm_py/cfg/utils.py`
            - `collect_stringish_value_ids(blocks)` を追加
            - seed: `const string` / `newbox StringBox` / `dst_type StringBox`
            - 伝播: `copy` / `phi` / `binop(+)`
            - use-based inference: `substring` / `indexOf` / `lastIndexOf` receiver を stringish 化
            - RuntimeData 要素推論:
              - `set/push` に stringish 値を書いた receiver を string-element container として記録
              - 同 receiver の `get` 結果を stringish 化
            - SCC closure を使って self-carry 由来の誤推論を抑制
          - `src/llvm_py/builders/function_lower.py`
            - `collect_stringish_value_ids` 結果を `context.resolver_string_ids` に適用
            - resolver 側 `builder.resolver.string_ids` を lowering 前に同期
          - `src/llvm_py/tests/test_strlen_fast.py`
            - `length` 先行 + `substring` 後続ケースで `any.length_h` 不在を固定
            - RuntimeData `set(string)->get->length` で `any.length_h` 不在を固定
          - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
            - main IR に `nyash.any.length_h` が残った場合は fail-fast
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=78`, `py_ms=105`, `ny_vm_ms=974`, `ny_aot_ms=70`, `ratio_c_aot=1.11`, `aot_status=ok`
      - cleanup-12（HOT-20 concat chain dead-call pruning, 2026-02-23）:
        - 目的:
          - `concat3_hhh` fold 時に残る中間 `concat_hh` デッドコールを削減して helper 密度を下げる
          - 特別扱いを増やさず `binop` 箱内の chain fold 完結で改善する
        - 実装:
          - `src/llvm_py/instructions/binop.py`
            - `_concat3_chain_args` を `(args, folded_call)` 返却へ拡張
            - `_value_has_users_in_function` / `_prune_dead_chain_call` を追加
            - `concat3_hhh` 採用時に folded `concat_hh` が未使用なら同ブロックから prune
          - `src/llvm_py/tests/test_strlen_fast.py`
            - chain fold テスト（left/right）の期待を更新:
              - `concat3_hhh` は存在
              - `concat_hh` は不在（中間デッドコール禁止）
          - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
            - concat helper 密度契約を `>=3` から `>=2` へ更新（dead-call 除去後の実体密度に合わせる）
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=79`, `py_ms=111`, `ny_vm_ms=1019`, `ny_aot_ms=60`, `ratio_c_aot=1.32`, `aot_status=ok`
      - cleanup-13（HOT-20 length bridge contraction, 2026-02-23）:
        - 目的:
          - stringish `length` fast path の `to_i8p_h -> nyrt_string_length` 2段呼び出しを 1 call に縮退
          - `StringView` 導入後の `length` を materialize不要の `string.len_h` に統一
        - 実装:
          - `src/llvm_py/instructions/stringbox.py`
            - `_emit_length` の stringish fast pathを `nyash.string.len_h` 優先へ変更
            - 後方互換トグル `NYASH_LEN_FORCE_BRIDGE=1` で旧 bridge 経路（`to_i8p_h` + `nyrt_string_length`）を強制可能
          - `src/llvm_py/tests/test_strlen_fast.py`
            - length fast path テスト契約を更新:
              - `ret i64 const` / `nyash.string.len_h` / `nyrt_string_length` のいずれかを許容
              - `phi self-carry` ケースも `len_h or nyrt` 契約へ拡張
        - main IR 観測（`kilo_kernel_small`）:
          - `nyash.string.to_i8p_h`: `2 -> 0`
          - `nyrt_string_length`: `2 -> 0`
          - `nyash.string.len_h`: `0 -> 2`
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=78`, `py_ms=110`, `ny_vm_ms=987`, `ny_aot_ms=64`, `ratio_c_aot=1.22`, `aot_status=ok`
      - cleanup-14a（HOT-20 runtime_data integer-key mono-route + SSOT, 2026-02-23）:
        - 目的:
          - `RuntimeDataBox` array mono-route の key decode 境界を縮退（`*_hh/*_hhh` -> `*_hi/*_hih`）
          - integer key 判定源を AutoSpecializeBox の 1 箇所へ集約（AS-03b）
        - 実装:
          - `src/llvm_py/cfg/utils.py`
            - `collect_integerish_value_ids(blocks)` を追加（const/copy/binop/select/phi のSCC closure）
          - `src/llvm_py/builders/function_lower.py`
            - `context.integerish_value_ids` / `resolver.integerish_ids` を lowering 前に同期
          - `src/llvm_py/context/function_lower_context.py`
          - `src/llvm_py/resolver.py`
            - function-local integerish fact storage を追加
          - `src/llvm_py/instructions/mir_call/auto_specialize.py`
            - `prefer_runtime_data_array_i64_key_route()` を追加（AS-03b）
          - `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
            - integer-key route table 追加:
              - `get -> nyash.array.get_hi`
              - `set -> nyash.array.set_hih`
              - `has -> nyash.array.has_hi`
          - `crates/nyash_kernel/src/plugin/array.rs`
            - index helper SSOT を抽出:
              - `array_get_by_index`
              - `array_set_by_index`
              - `array_has_by_index`
            - 新規 export:
              - `nyash.array.get_hi`
              - `nyash.array.set_hih`
              - `nyash.array.has_hi`
          - tests/smokes:
            - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
            - `src/llvm_py/tests/test_strlen_fast.py`
            - `crates/nyash_kernel/src/tests.rs` (`array_runtime_data_route_hi_contract_roundtrip`)
            - `tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
            - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - docs:
            - `docs/development/current/main/design/auto-specialize-box-ssot.md`（AS-03b）
            - `docs/reference/runtime/runtime-data-dispatch.md`（`get_hi/set_hih/has_hi` 契約）
        - main IR 観測（`kilo_kernel_small`）:
          - `nyash.array.get_hh`: `3 -> 0`
          - `nyash.array.set_hhh`: `2 -> 0`
          - `nyash.array.get_hi`: `0 -> 3`
          - `nyash.array.set_hih`: `0 -> 2`
        - 検証:
          - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py`
          - `cargo test -p nyash_kernel array_runtime_data_route_hh_contract_roundtrip -- --nocapture`
          - `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture`
          - `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
          - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
          - `tools/checks/dev_gate.sh quick`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=76`, `py_ms=106`, `ny_vm_ms=991`, `ny_aot_ms=286`, `ratio_c_aot=0.27`, `aot_status=ok`
      - cleanup-15a（HOT-20 poll-path getenv shrink, 2026-02-23）:
        - 背景（asm/probe切り分け）:
          - `cleanup-14a` 後の `kilo_kernel_small` AOT 退行を `objdump/perf stat` で調査
          - `runtime_data` i64-key route を一時的に `*_hh/*_hhh` へ戻すと `ny_aot_ms=59` まで復帰
          - `hi/hih` 維持時は `ny_aot_ms=271..286`
          - `perf stat`（standalone AOT exe）で `hi/hih` は `instructions=4.35B`、`hh/hhh` は `2.71B`
        - 実装:
          - `src/runtime/scheduler.rs`
            - `SingleThreadScheduler` に `poll_budget` / `trace_enabled` を追加
            - `new()` で env-derived knob を 1 回だけ capture
            - `poll()` の hot path から `sched_poll_budget()/sched_trace_enabled()` 呼び出しを除去
        - 実測（`hi/hih` route, 同一Result保持）:
          - before: `instructions=4,352,111,015`, `time=0.6507s`, `Result=179998`
          - after:  `instructions=2,953,781,451`, `time=0.5700s`, `Result=179998`
        - 検証:
          - `cargo build --release --bin hakorune`
          - `cargo build -p nyash_kernel --release`
          - `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=74`, `py_ms=105`, `ny_vm_ms=961`, `ny_aot_ms=273`, `ratio_c_aot=0.27`, `aot_status=ok`
      - cleanup-16（HOT-20 string helper env-read caching, 2026-02-23）:
        - 目的:
          - cleanup-15a 後に残った string helper 側の env lookup（`substring_hii` / `len_h`）を除去
        - 実装:
          - `crates/nyash_kernel/src/exports/string.rs`
            - `env_flag_cached` を追加
            - `substring_view_enabled()` を `OnceLock` 化（`NYASH_LLVM_FAST`）
            - `jit_trace_len_enabled()` を `OnceLock` 化（`NYASH_JIT_TRACE_LEN`）
            - hot path から per-call `std::env::var` を削除
        - 実測（`hi/hih`, 同一Result保持）:
          - before: `instructions=2,953,781,451`, `time=0.5700s`, `Result=179998`
          - after:  `instructions=2,847,467,807`, `time=0.4628s`, `Result=179998`
        - 検証:
          - `cargo build --release --bin hakorune`
          - `cargo build -p nyash_kernel --release`
          - `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=74`, `py_ms=104`, `ny_vm_ms=954`, `ny_aot_ms=274`, `ratio_c_aot=0.27`, `aot_status=ok`
      - cleanup-17（HOT-20 scheduler poll empty-fast-path, 2026-02-23）:
        - 目的:
          - `safepoint` ごとの `Scheduler::poll()` 空振り lock を削減
        - 実装:
          - `src/runtime/scheduler.rs`
            - `SingleThreadScheduler` に `pending_hint: AtomicUsize` を追加
            - `spawn/spawn_after` で pending hint を更新
            - `poll()` で `pending_hint == 0` の即 return fast path を追加
            - 実行済み task で pending hint を saturating decrement
        - 実測（`hi/hih`, 同一Result保持）:
          - before: `instructions=2,847,467,807`, `time=0.4628s`, `Result=179998`
          - after:  `instructions=2,646,673,527`, `time=0.3718s`, `Result=179998`
      - cleanup-18（HOT-20 host handle fast-hash map, 2026-02-23）:
        - 目的:
          - `host_handles::Registry::get/alloc` の hash/rehash コストを縮退
        - 実装:
          - `src/runtime/host_handles.rs`
            - `HashMap` -> `FxHashMap` へ置換
            - 初期 reserve(`8192`) を追加
        - 実測（`hi/hih`, 同一Result保持）:
          - before: `instructions=2,646,673,527`, `time=0.3718s`, `Result=179998`
          - after:  `instructions=2,089,510,745`, `time=0.3140s`, `Result=179998`
        - 検証:
          - `cargo build --release --bin hakorune`
          - `cargo build -p nyash_kernel --release`
          - `tools/checks/dev_gate.sh quick`
          - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=74`, `py_ms=102`, `ny_vm_ms=957`, `ny_aot_ms=241`, `ratio_c_aot=0.31`, `aot_status=ok`
      - cleanup-40（HOT-20 string export module split, 2026-02-24）:
        - 目的:
          - `string.rs` の ABI 層と `StringView/StringSpan` 実装層を分離し、責務を明確化
        - 実装:
          - `crates/nyash_kernel/src/exports/string_view.rs` を追加（view/span/cache 解決ロジック）
          - `crates/nyash_kernel/src/exports/string.rs` は C ABI 入口 + helper 呼び出しに縮退
          - `crates/nyash_kernel/src/exports/mod.rs` に module wire を追加
          - `crates/nyash_kernel/src/exports/README.md` を追加
        - 検証:
          - `cargo check -p nyash_kernel`
          - `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
      - cleanup-41（HOT-20 host handle borrowed triple route, 2026-02-24）:
        - 目的:
          - `concat3_hhh` hot path の Arc clone 密度を下げる
        - 実装:
          - `src/runtime/host_handles.rs`: `with3`（borrowed triple lookup）追加
          - `crates/nyash_kernel/src/exports/string.rs`: `nyash.string.concat3_hhh` を `with3` 優先へ
          - read-lock 中の handle alloc を避けるため、文字列連結と handle 化を分離
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-43（HOT-20 string helper fallback pipeline unification, 2026-02-24）:
        - 目的:
          - `indexOf/lastIndexOf/eq/lt` の pair/fallback 重複を削減し、契約 drift を防止
        - 実装:
          - `crates/nyash_kernel/src/exports/string.rs`:
            - `with_lossy_string_pair`
            - `search_string_pair_hh`
            - `compare_string_pair_hh`
            - `bool_to_i64` / empty-needle helper
          - 上記 helper へ `indexOf/lastIndexOf/eq/lt` を集約
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_compare_hh_contract_roundtrip -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_lastindexof_invalid_needle_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_lastindexof_multibyte_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-42（HOT-20 value codec profile split, 2026-02-24）:
        - 目的:
          - `value_codec` の decode/encode/borrowed-handle 責務を分離し、array fast policy を明示化
        - 実装:
          - `crates/nyash_kernel/src/plugin/value_codec/` へ分割:
            - `mod.rs`
            - `decode.rs`
            - `encode.rs`
            - `borrowed_handle.rs`
            - `tests.rs`
          - `CodecProfile::{Generic, ArrayFastBorrowString}` を導入
          - `array.rs` / `runtime_data.rs` は `any_arg_to_box_with_profile(..., ArrayFastBorrowString)` へ統一
          - `runtime_data.rs` に `resolve_array_index_key` を追加し、positive immediate key 契約を安定化
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-44（HOT-20 array get single-lock path, 2026-02-24）:
        - 目的:
          - `array_get_by_index` と `runtime_data.get_hh(Array)` の read-lock / fallback 再読込を縮退
        - 実装:
          - `crates/nyash_kernel/src/plugin/array.rs`: `array_get_by_index` を single read-lock 化
          - `crates/nyash_kernel/src/plugin/runtime_data.rs`: array get route を single read-lock 化
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
          - `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - cleanup-42/43 後: `c_ms=78`, `ny_aot_ms=75`, `ratio_c_aot=1.04`, `aot_status=ok`
            - cleanup-44 後: `c_ms=75`, `ny_aot_ms=80`, `ratio_c_aot=0.94`, `aot_status=ok`（WSL揺れあり）
      - cleanup-45（HOT-20 string span cache module split, 2026-02-24）:
        - 目的:
          - `string_view` の resolver と TLS cache 管理を分離し、責務境界を明確化
        - 実装:
          - `crates/nyash_kernel/src/exports/string_span_cache.rs` を追加
            - `string_span_cache_get`
            - `string_span_cache_get_pair`
            - `string_span_cache_put`
          - `crates/nyash_kernel/src/exports/string_view.rs`
            - cache実装本体を撤去し、cache module 呼び出しへ縮退
            - `StringSpan::span_bytes_len()` を追加（cache側上限判定で利用）
          - `crates/nyash_kernel/src/exports/mod.rs` に module wire を追加
          - `crates/nyash_kernel/src/exports/README.md` に module note 追記
        - 検証:
          - `cargo check -p nyash_kernel`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
      - cleanup-46（HOT-20 string direct pair fast-path extension, 2026-02-24）:
        - 目的:
          - `concat_hh/eq_hh/lt_hh` で `StringBox` 同士の direct route を優先し、resolver/TLS 経路の通過頻度を下げる
        - 実装:
          - `crates/nyash_kernel/src/exports/string.rs`
            - `concat_pair_to_owned` を追加
            - `nyash.string.concat_hh` に `with_string_pair_direct` fast path を追加
              - read-lock中は `String` 生成のみ
              - lock解除後に `string_handle_from_owned` で handle 化（deadlock回避）
            - `compare_string_pair_hh` に direct route を追加（`eq_hh/lt_hh` が利用）
        - 検証:
          - `cargo check --bin hakorune`
          - `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_compare_hh_contract_roundtrip -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract -- --nocapture`
          - `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
          - `tools/checks/dev_gate.sh quick`
          - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
            - `c_ms=74`, `py_ms=108`, `ny_vm_ms=963`, `ny_aot_ms=74`, `ratio_c_aot=1.00`, `aot_status=ok`
- compiler lane:
  1. [done] `JIR-PORT-00`（Boundary Lock, docs-first）
  2. [done] `JIR-PORT-01`（Parity Probe）
  3. [done] `JIR-PORT-02`（if/merge minimal port）
  4. [done] `JIR-PORT-03`（loop minimal port）
  5. [done] `JIR-PORT-04`（PHI / Exit invariant lock）
  6. [done] `JIR-PORT-05`（promotion boundary lock）
  7. [done] `JIR-PORT-06`（monitor-only boundary lock）
  8. [done] `JIR-PORT-07`（expression parity port: unary+compare+logic seed）
  9. [next] `none`（tail active）
  10. JoinIR 移植は `joinir-port-task-pack-ssot.md` の fixed order（JIR-PORT-00..07）で実施
