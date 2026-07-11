# APP-PERF-03 指示書 — chip8/kilo + app wallclock 統合ハーネス

Status: Implemented  
Scope: Phase 21.5 perf lane（micro + app integration）

## Context

目的は、以下 2 系統の計測導線を単一ハーネスで再生可能にすること。

1. micro cross-language baseline（APP-PERF-01/02）
  - `chip8_kernel_small`
  - `kilo_kernel_small_hk`（dataset alias: `kilo_kernel_small`）
2. app wallclock（apps/tools）
  - `bench_apps_wallclock.sh`
  - `bench_apps_entry_mode_compare.sh`

このタスクは **比較導線の統合のみ** を扱い、既存デフォルト挙動は変更しない（optional gate 配線のみ）。

## Deliverables

1. `tools/perf/bench_crosslang_apps_bundle.sh`（新規）
2. `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_apps_crosslang_bundle_contract_vm.sh`（新規）
3. `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`（更新）
4. `tools/perf/run_phase21_5_perf_gate_bundle.sh`（更新、full preset toggle追加）
5. `benchmarks/README.md`（更新）

## Output Contract

`bench_crosslang_apps_bundle.sh` は 1 行で以下を出力:

```text
[bench4-app] chip8_aot_status=<ok|skip|fail> chip8_ratio_c_aot=<r> chip8_ny_aot_ms=<n> \
             kilo_aot_status=<ok|skip|fail> kilo_ratio_c_aot=<r> kilo_ny_aot_ms=<n> \
             kilo_mode=<strict|diagnostic> kilo_result_parity=<ok|skip> \
             kilo_fallback_guard=<strict-no-fallback|...> kilo_vm_engine=<rust-vm|hako-vm|unknown> \
             apps_total_ms=<n> apps_hotspot_case=<name> apps_hotspot_ms=<n> \
             entry_source_total_ms=<n> entry_prebuilt_total_ms=<n> entry_delta_ms=<n> entry_winner=<name>
```

補足:

- `PERF_BUNDLE_KILO_MODE=strict`（既定）は `kilo_kernel_small_hk` の parity を必須化する。
- parity mismatch を調査中に計測だけ回したい場合は `PERF_BUNDLE_KILO_MODE=diagnostic` を使う。

## Rules（必須）

- `src/**` の Rust/LLVM 実装は変更しない
- fallback の新規追加は禁止
- 既存 gate の既定負荷を増やさない（`PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK` は optional）
- 1タスク1目的（統合ハーネス追加のみ）

## Acceptance Commands

```bash
# 1) unified harness
PERF_BUNDLE_KILO_MODE=diagnostic \
tools/perf/bench_crosslang_apps_bundle.sh 1 1 1 1

# 2) contract smoke
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_apps_crosslang_bundle_contract_vm.sh

# 3) optional gate wiring check
PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK=1 NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

## Non-goals（APP-PERF-03ではやらない）

- app case レジストリ（`apps_wallclock_cases.sh`）の既定ケース追加
- regression guard しきい値の再調整
- VM/LLVM 最適化ロジックの追加
