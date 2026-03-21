# Phase 29y: Lane Gate SSOT（quick/full single-entry replay）

Status: Ready (integration gate)
Scope: phase29y の ABI / RC insertion / observability / optional GC entry を quick/full 2段で再生する。

## 0. Goal

- phase29y の主要契約を single-entry で replay できる入口を固定する。
- 日常運用は quick、節目確認は full を使い分ける。
- docs-first 導線を gate で実動確認し、次フェーズへの handoff を短距離化する。

## 0.1 De-rust done handshake boundary

- この文書の quick/full gate は lane 運用の replay SSOT であり、de-rust transfer lane の done 判定そのものではない。
- de-rust done 判定は `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md` の
  `Done Criteria × Gate Coverage Matrix`（X32/X33/X34/X35）を正本とする。
- done 宣言時は次の 4 本を lane gate と別に replay する。
  1. `tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh`
  2. `tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh`
  3. `tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh`
  4. `tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh`
- 補助導線として `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1` を指定した quick gate は
  `phase29x_derust_done_matrix_vm.sh` を末尾で replay する（既定は OFF）。
- 上記 toggle は診断/節目確認の補助であり、quick gate の日常契約を変更しない。
- quick の既定運用は軽量維持を優先し、de-rust done matrix は既定では実行しない。

## 1. Fixed replay order

Quick profile:
1. direct-v0 retirement guard:
   - `tools/checks/phase29y_direct_v0_retirement_guard.sh`
2. ring1 pre-gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_ring1_gate_vm.sh`
3. Compiler pipeline parity gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
4. Binary-only run ported gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
5. MIR shape guard gate:
   - `tools/smokes/v2/profiles/integration/mir_shape/mir_shape_guard_vm.sh`
6. Direct v0 bridge guard gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`
7. Mainline no-compat gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
   - runtime stage-a no-compat probe only; bootstrap diagnostics are covered by the phase29bq contract smoke
8. Phase29y core contracts gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh`

Full profile:
1. `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
2. Optional GC lane entry preconditions:
   - `tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`

Core contracts gate（fixed order）:
1. `tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_borrowed_owned_vm.sh`
2. `tools/smokes/v2/profiles/integration/apps/phase29y_rc_insertion_entry_vm.sh`
3. `tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh`

## 2. Contract

1. phase29y lane gate quick は `direct-v0 retirement guard -> ring1 pre-gate -> compiler pipeline parity -> binary-only run ported gate -> mir-shape guard gate -> direct-v0 guard gate -> no-compat gate -> core contracts gate` の8段で実行する。
2. phase29y lane gate full は `quick gate -> optional GC entry gate` の2段で実行する。
3. ring1 pre-gate は `scope guard -> array guard -> array smoke -> map guard -> map smoke -> path guard -> path smoke -> console guard -> console smoke` を固定順で実行する。
4. ring1 scope guard は accepted/provisional 境界（`file,array,map,path,console=accepted`）を fail-fast で検証する。
5. ring1 array guard + smoke は `RING1-CORE-06` の min2/min3 契約（fixture出力と配線存在）を固定する。
6. ring1 map guard + smoke は `RING1-CORE-07` の min2/min3 契約（fixture出力と配線存在）を固定する。
7. ring1 path guard + smoke は `RING1-CORE-08` の min2/min3 契約（fixture出力と配線存在）を固定する。
8. ring1 console guard + smoke は `RING1-CORE-09` の min2/min3 契約（fixture出力と配線存在）を固定する。
9. compiler pipeline parity gate は stage1 using resolver の SSOT 委譲（`resolve_ssot_box`）契約を先頭で検証する。
10. binary-only run ported gate は repo外 `--hako-run` 成功契約（`lang/src/**` 読み依存不在）を no-compat gate より前に固定する。
11. mir-shape guard gate は selfhost-first emit MIR が collapse していないことを strict で固定し、collapsed fixture は必ず fail-fast させる。
12. direct-v0 guard gate は `--parser ny` 削除（CLI reject）と `NYASH_USE_NY_PARSER=1` no-op 化を固定する。
13. no-compat gate は stage-a runtime probe で compat lane 漏れを先に検知する。stage1 bootstrap diagnostics は phase29bq contract smoke に分離する。
14. optional GC lane は full profile でのみ実行し、quick profile の日常運用から分離する。
15. `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1` のときだけ quick profile 末尾で
    `phase29x_derust_done_matrix_vm.sh` を追加実行する（既定は OFF）。
16. RC insertion overwrite gate（`phase29y_rc_insertion_overwrite_release_vm.sh`）は lane gate の必須stepに含めない。
17. failure は先頭の失敗stepで fail-fast し、後段を実行しない。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29y_lane_gate_guard.sh`
- direct-v0 retirement guard:
  - `tools/checks/phase29y_direct_v0_retirement_guard.sh`
- Ring1 pre-gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_ring1_gate_vm.sh`
  - (`tools/checks/ring1_core_scope_guard.sh` / `tools/checks/ring1_array_provider_guard.sh` / `tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh` / `tools/checks/ring1_map_provider_guard.sh` / `tools/smokes/v2/profiles/integration/apps/ring1_map_provider_vm.sh` / `tools/checks/ring1_path_provider_guard.sh` / `tools/smokes/v2/profiles/integration/apps/ring1_path_provider_vm.sh` / `tools/checks/ring1_console_provider_guard.sh` / `tools/smokes/v2/profiles/integration/apps/ring1_console_provider_vm.sh`)
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- Quick gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
- Core guard:
  - `tools/checks/phase29y_core_contracts_guard.sh`
- Core gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh`
- No-compat gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
  - runtime stage-a no-compat probe only; bootstrap diagnostics are separate
- Compiler pipeline parity gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
- Binary-only run ported gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
- MIR shape guard gate:
  - `tools/smokes/v2/profiles/integration/mir_shape/mir_shape_guard_vm.sh`
- Direct v0 guard gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`

## 4. Evidence command

- `bash tools/checks/phase29y_lane_gate_guard.sh`
- `bash tools/checks/phase29y_direct_v0_retirement_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_ring1_gate_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
- `bash tools/smokes/v2/profiles/integration/mir_shape/mir_shape_guard_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
- `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`（診断補助、既定OFF）
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- `bash tools/checks/phase29y_core_contracts_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh`

## 5. Related

- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`
- `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
- `docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md`
- `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md`
