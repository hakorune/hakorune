# Tools Script Index (SSOT)

Status: Active  
Purpose: 主要な運用スクリプトの入口を一枚で追える状態にする。
Scope: `tools/` 配下の頻用導線。履歴・実験スクリプトは原則ここには載せない。

## 1. 日常導線（最優先）

- `tools/checks/dev_gate.sh`  
  - 日常ゲートの統合（quick/hotpath/portability/milestone）
- `tools/checks/dev_gate.sh quick`  
  - 最低限の毎日状態監視
- `tools/smokes/v2/run.sh --profile quick`  
  - 軽量スモーク（core）
- `tools/selfhost/run.sh --gate --planner-required 1`  
  - セルフホスト最小ゲート
- `tools/perf/run_kilo_hk_bench.sh strict 1 3`  
  - hk hotpath 計測

## 2. Build 導線（canonical）

- `tools/build/build_check.sh`
  - `cargo check --lib` を実行し、`build_errors.txt` へ保存
- `tools/build/build_llvm.sh`
  - LLVM feature で release build（Linux/WSL 開発向け）
- `tools/build/build_llvm_wsl.sh`
  - WSL から MinGW ターゲットで Windows 向け LLVM ビルド
- `tools/build/build_llvm_wsl_msvc.sh`
  - WSL から `cargo xwin` で MSVC ターゲット LLVM ビルド

## 3. Route / emit 導線

- `tools/smokes/v2/lib/emit_mir_route.sh`  
  - Program→MIR の SSOT 入口（`direct|hako-mainline|hako-helper`）
- `tools/checks/route_env_probe.sh`  
  - emit route と主要 ENV の現状を一発で表示
- `tools/checks/route_no_fallback_guard.sh`  
  - fallback/helper トグル混入を quick gate 前に fail-fast で検出
- `tools/dev/direct_loop_progression_sweep.sh`
  - direct route（`--emit-mir-json -> --mir-json-file`）の loop progression を profile 単位で監視
- `tools/dev/phase29ca_direct_verify_dominance_block_canary.sh`
  - phase29ca の direct route 回帰監視（emit/run 契約: `emit_rc=0`, `run_rc=4`）
- `tools/hako_check.sh`  
  - lint/解析導線
- `tools/selfhost/build_stage1.sh`  
  - stage1 直接実行（launch/exe 任意）
- `tools/selfhost/run.sh`  
  - selfhost の統合エントリ
- `tools/selfhost/run_stage1_cli.sh`  
  - Stage1 CLI 実行ヘルパ
- `tools/selfhost/stage1_mainline_smoke.sh`
  - current Stage1 mainline smoke

## 4. 監査 / ガード

- `tools/checks/module_registry_hygiene_guard.sh`
- `tools/checks/ring1_core_scope_guard.sh`
- `tools/checks/macos_portability_guard.sh`
- `tools/checks/windows_wsl_cmd_smoke.sh`
- `tools/checks/phase29x_optimization_gate_guard.sh`
- `tools/checks/env_dead_accessors_report.sh`

## 5. 更新ルール

- 新規導線追加はまず本書へ追記する。
- 実装を追加したら関連SSOT（`docs/tools/README.md` や `docs/tools/check-scripts-index.md`）へ同コミットで反映する。
- 破棄・統合したスクリプトは本書から明示的に外す。
