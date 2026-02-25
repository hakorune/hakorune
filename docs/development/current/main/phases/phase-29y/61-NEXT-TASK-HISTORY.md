---
Status: Archive
Decision: informational
Date: 2026-02-19
Scope: phase-29y/60-NEXT-TASK-PLAN.md の完了履歴を退避し、current blocker 導線を短く保つ。
Related:
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
---

# Phase 29y Next Task History

## 2026-02-17 to 2026-02-18 Timeline

- queue complete を解除し、`RVP-0` を current blocker に設定。
- `RVP-0-min1` 完了（`src/cli/args.rs` / `src/runner/mod.rs` / `src/runner/emit.rs`）。
- `RVP-0-min2` 完了（stage1 minimum route liveliness smoke 2件を再検証）。
- `RVP-1-min1` 完了（`vm_hako_caps_filebox_newbox_vm.sh` PASS）。
- `RVP-1-min2` 完了（`vm_hako_caps_args_vm.sh` PASS）。
- `RVP-1-min3` 完了（`vm_hako_caps_file_error_vm.sh` PASS）。
- `RVP-3-min1` 完了（`vm_hako_caps_filebox_newbox_vm.sh` が ported 契約で PASS）。
- `RVP-3-min2` 完了（`vm_hako_caps_args_vm.sh` が ported 契約で PASS）。
- `RVP-3-min3` 完了（`vm_hako_caps_file_error_vm.sh` が ported 契約で PASS）。
- `RVP-4-min1` 完了（APP-1 residual capability rowization + blocked pin 5件を追加）。
- `RVP-4-min2` 完了（C04 `Select` emit contract を port、C04 smoke を success 契約へ昇格）。
- `RVP-4-min3` 完了（C05 `env.get/1` externcall を subset/runtime へ port、C05 smoke を success 契約へ昇格）。
- `RVP-4-min4` 完了（C06 `compare(!=)` を subset/runtime へ port、C06 smoke を success 契約へ昇格）。
- `RVP-4-min5` 完了（C07 `FileBox.read` を runtime へ port、C07 smoke を success 契約へ昇格）。
- `RVP-4-min6` 完了（C08 `FileBox.close` を runtime へ port、C08 smoke を success 契約へ昇格）。
- `RVP-5-min1` 完了（APP-1 vm-hako cutover blocker `const(void)` を fixture + smoke で blocked pin）。
- `RVP-5-min2` 完了（C09 `const(type:void)` を subset/runtime へ port、C09 smoke を success 契約へ昇格）。
- `RVP-5-min3` 完了（APP-1 vm-hako cutover blocker `compare(>=)` を fixture + smoke で blocked pin）。
- `RVP-5-min4` 完了（C10 `compare(>=)` を subset/runtime へ port、C10 smoke を success 契約へ昇格）。
- `RVP-5-min5` 完了（APP-1 vm-hako cutover blocker `boxcall(args>1)` を fixture + smoke で blocked pin）。
- `RVP-5-min6` 完了（C11 non-open `boxcall(args>1)` を subset/runtime へ port、stale subset blocker 不在を smoke で固定。次 blocker `boxcall-open-handle-missing` を blocked pin）。
- `RVP-5-min7` 完了（C12 `boxcall-open-handle-missing` を runtime へ port。`vm_hako_caps_open_handle_phi_ported_vm.sh` で stale tag 不在を固定し、次 blocker `app1-stack-overflow-after-open` を blocked pin）。
- `RVP-5-min8` 完了（C13 `app1-stack-overflow-after-open` を runtime へ port。`vm_hako_caps_app1_stack_overflow_after_open_ported_vm.sh` を success 契約へ昇格し、次 blocker `app1-summary-contract-mismatch-after-open` を固定）。
- `RVP-5-min9` 完了（C14 `app1-summary-contract-mismatch-after-open` を blocked pin で固定。`vm_hako_caps_app1_summary_contract_block_vm.sh` で Rust baseline 契約と vm-hako mismatch signature `0/0` を同期し、次は C14 port へ進行）。
- `RVP-5-min10` 完了（C14 `app1-summary-contract-mismatch-after-open` を runtime へ port。`vm_hako_caps_app1_summary_contract_ported_vm.sh` で SUMMARY/FAIL_LINES/FAIL順 parity を固定し、次 blocker `app1-summary-contract-full-fixture-time-budget` を row 化）。
- `RVP-5-min11` 完了（C15 `app1-summary-contract-full-fixture-time-budget` を blocked pin で固定。`vm_hako_caps_app1_summary_contract_block_vm.sh` で full fixture timeout（rc=124）と stale C12/C13 blocker 不在を同期し、次は C15 port へ進行）。
- `RVP-5-min12` 完了（C15 `app1-summary-contract-full-fixture-time-budget` を runtime へ port。`mir_vm_s0` の block-local instruction cache を導入し、`vm_hako_caps_app1_summary_contract_ported_vm.sh` を full fixture parity 契約へ昇格。`vm_hako_caps_app1_summary_contract_block_vm.sh` は stale-timeout guard へ更新）。
- `D-RVP-continue-assignment` 更新（continue 分岐内代入drop を compiler lane で修正。`generic_loop_v1` の conditional-update 受理境界を緩和し、`phase29y_continue_assignment_in_continue_stale_guard_vm.sh` を `FINAL=7` stale-guard 契約へ更新）。
- `lane B pre-emit I/O` 更新（stage1 module env snapshot cache を導入。`target/.cache/stage1_module_env.json` + `NYASH_STAGE1_MODULES_CACHE` override + workspace metadata signature の invalidation 契約を `phase-29y/60` に追記。計測ピンは cold `parent=9 + child=0` / hot `parent=0 + child=0` で固定）。
- `lane B monitor-only` 更新（`phase29y_hako_emit_mir_preemit_io_monitor_vm.sh` を追加。既定は non-gating 観測のみ、`--strict` 手動実行時のみ drift を fail-fast で検出）。
- `lane B binary-only` 更新（`selfhost-bootstrap-route-ssot.md` に Binary-only `--hako-emit-mir-json` 契約を追加し、`B01->B02->B03->B04` の固定順序を docs で先行確定）。
- `BINARY-ONLY-B01` 完了（`phase29y_hako_emit_mir_binary_only_block_vm.sh` を追加し、repo外 `./hakorune --hako-emit-mir-json` の blocked 契約を fail-fast marker で固定）。
- `BINARY-ONLY-B02` 完了（`src/runner/stage1_bridge/mod.rs` に `stage1_cli.hako` 埋め込み entry を導入し、default route の `lang/src/runner/stage1_cli.hako` ファイル依存を撤去。blocked smoke は internal stage1 timeout marker 契約へ更新）。
- `BINARY-ONLY-B03` 完了（`src/runner/stage1_bridge/modules.rs` に埋め込み module env snapshot を導入し、`src/using/resolver.rs` に stage1 child env-only fast path を追加。`--hako-emit-mir-json` 既定経路の TOML/module-manifest 収集依存を撤去、`strace -ff -e openat` で `hako.toml` / `*_module.toml` 不在を確認。`NYASH_STAGE1_MODULES_SOURCE=toml` で従来収集へ切替可能）。
- `BINARY-ONLY-B04` 完了（`src/runner/stage1_bridge/mod.rs` に repo外向け binary-only direct emit ルートを追加し、`phase29y_hako_emit_mir_binary_only_ported_vm.sh` を PASS 契約へ昇格。lane B は monitor-only 運用へ復帰）。

## 2026-02-19 Timeline

- `RING1-CORE-06-min3` 以降の昇格を継続し、`RING1-CORE-07/08/09`（`map/path/console`）を `min1/min2/min3` で完了。
- ring1 provisional domain は 0 件となり、`array/console/file/map/path` を accepted 固定へ移行。
- `BINARY-ONLY-RUN-01` 完了（repo外 `--hako-run` blocked pin を non-gating 契約として追加）。
- `BINARY-ONLY-RUN-02` 完了（stage1 run route に binary-only direct route を追加し、repo外 `--hako-run` の `lang/src/**` read 依存を撤去）。
- `BINARY-ONLY-RUN-03` 完了（`phase29y_hako_run_binary_only_ported_vm.sh` を追加し、repo外 `--hako-run` を success 契約へ昇格。`*_block_vm.sh` は legacy alias 化）。
- lane B/lane C は fixed backlog を持たない monitor-only 運用へ移行。
- `60-NEXT-TASK-PLAN.md` の done 履歴を縮退し、current/next 導線を pointer-only 化。
