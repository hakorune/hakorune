# Developer Tools Quick Entry

Status: Active
Scope: 開発中に使うツールの単一導線（hako_check / smokes / bug origin triage）
Related:
- tools/hako_check/README.md
- docs/how-to/smokes.md
- docs/tools/cli-options.md
- docs/tools/check-scripts-index.md
- docs/tools/script-index.md

## 目的別の入口

### 0. 再起動直後の最短再開

単一入口:

- `docs/development/current/main/05-Restart-Quick-Resume.md`

最短コマンド:

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

### 1. バグ起点の切り分け（Rust か .hako か vm-hako か）

```bash
tools/dev/bug_origin_triage.sh <source.hako> --expect '<regex>' --expect-rc <n>
```

例:

```bash
tools/dev/bug_origin_triage.sh apps/tests/phase29y_loop_if_assignment_carry_min.hako \
  --expect '^FINAL=c$' --expect-rc 0 --timeout 10
```

判定の目安:
- `rust-vm=pass` かつ `stage1-route=fail`: `.hako compiler / stage1` 側
- `stage1-route=pass` かつ `rust-vm=fail`: Rust lane 側
- `vm-hako` に `[vm-hako/unimplemented]`: vm-hako 未実装ギャップ
- 複数 route 同時 fail: frontend/core 契約（SSOT）側を優先調査

## 2. `.hako` ルール検証（hako_check）

全ルール:

```bash
bash tools/hako_check/run_tests.sh
```

特定ケースのみ:

```bash
bash tools/hako_check/run_tests.sh tools/hako_check/tests/HC015_arity_mismatch
```

## 3. スモーク実行（回帰確認）

日常 quick:

```bash
tools/smokes/v2/run.sh --profile quick
```

integration:

```bash
tools/smokes/v2/run.sh --profile integration
tools/smokes/v2/run.sh --profile integration --suite presubmit
tools/smokes/v2/run.sh --profile integration --suite collection-core
```

詳細は `docs/how-to/smokes.md` を参照。

補足:
- `--profile` は coarse lane selector として維持
- `--suite` は `tools/smokes/v2/suites/<profile>/<suite>.txt` を読む curated allowlist
- `archive/lib/tmp/fixtures` は live discovery から除外済み

## 4. CLI オプション確認

CLI の現行オプションは `docs/tools/cli-options.md` を正本として参照。

## 5. Module Registry Hygiene

`hako.toml` / `nyash.toml` の `[modules]` 境界（top-only / override / duplicate）確認:

```bash
bash tools/checks/module_registry_hygiene_guard.sh
```

SSOT:
- `docs/development/current/main/phases/phase-29y/84-MODULE-REGISTRY-HYGIENE-SSOT.md`

## 6. Ring1 Provider Scope Guard

ring1 core provider の境界（`file/array/map/path/console=accepted`）確認:

```bash
bash tools/checks/ring1_core_scope_guard.sh
```

SSOT:
- `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
- `tools/checks/ring1_domains.tsv`（accepted/provisional の判定源）

## 7. Program→MIR Route Wrapper（SSOT）

日常の入口（smoke/check/perf/dev/cache 含む）は `emit_mir_route.sh` を使う:

```bash
tools/smokes/v2/lib/emit_mir_route.sh --route direct --timeout-secs 30 --out /tmp/out.mir.json --input apps/tests/fixture.hako
tools/smokes/v2/lib/emit_mir_route.sh --route hako-mainline --timeout-secs 30 --out /tmp/out.mir.json --input apps/tests/fixture.hako
tools/smokes/v2/lib/emit_mir_route.sh --route hako-helper --timeout-secs 30 --out /tmp/out.mir.json --input apps/tests/fixture.hako
```

route の意味:
- `direct`: `hakorune --emit-mir-json` 直経路
- `hako-mainline`: selfhost-first + no-delegate + mainline-only（fail-fast）
- `hako-helper`: helper 既定経路（診断/互換向け）

補足:
- `tools/hakorune_emit_mir_mainline.sh` / `tools/hakorune_emit_mir_compat.sh` は薄い互換ラッパとして維持。
- `tools/hakorune_emit_mir.sh` は内部実装であり、新規スクリプトからの直呼びは行わない。

## 8. Perf Gate Preset Runner (Phase 21.5)

phase21.5 perf gate の optional toggles をまとめて実行:

```bash
tools/perf/run_phase21_5_perf_gate_bundle.sh quick
tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
tools/perf/run_phase21_5_perf_gate_bundle.sh apps
tools/perf/run_phase21_5_perf_gate_bundle.sh full
```

補足:
- `quick`: core-only（既存 gate と同じ）
- `hotpath`: LLVM/AOT hotspot 契約
- `apps`: app entry/mode 契約
- `full`: hotpath + apps + regression helper 契約

## 9. Dev Gate Bundle (3-tier)

日常の「多すぎるコマンド」を 1 本にまとめた導線:

```bash
tools/checks/dev_gate.sh --list
tools/checks/dev_gate.sh quick
tools/checks/dev_gate.sh hotpath
tools/checks/dev_gate.sh portability
tools/checks/dev_gate.sh milestone
tools/checks/dev_gate.sh milestone-runtime
tools/checks/dev_gate.sh milestone-perf
```

プロファイル:
- `quick`: 毎コミット前の軽量セット（`route_no_fallback_guard` + `cargo check` + `strlen_fast` unittest + chip8 crosslang smoke）
- `hotpath`: `quick` + `phase21.5 perf gate bundle (hotpath)`
- `portability`: `windows_wsl_cmd_smoke`（既定は preflight）+ `macos_portability_guard`
- `milestone-runtime`: `hotpath` + `phase29y_no_compat_mainline_vm`
- `milestone-perf`: `hotpath` + `phase21.5 perf gate bundle (full)`
- `milestone`: `milestone-runtime` + `milestone-perf`（後方互換）

補足:
- 既定で `NYASH_LLVM_SKIP_BUILD=1` を使う（必要なら上書き可能）。
- 既存の個別コマンド実行も継続して利用可能（このスクリプトは導線整理用）。

## 10. Cross-Platform Maintenance Checks

Windows（WSL→Windows CMD smoke）:

```bash
# Preflight only
bash tools/checks/windows_wsl_cmd_smoke.sh

# Weekly recommended: build + cmd smoke
bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke
```

macOS（hardware がない間の契約ガード）:

```bash
bash tools/checks/macos_portability_guard.sh
```

## 11. Smoke Inventory (overgrowth triage)

任意の smoke subtree の過密状態を可視化して、orphan 候補を洗い出す:

```bash
bash tools/checks/smoke_inventory_report.sh
cat target/smoke_inventory/integration_apps_summary.txt

# 別 subtree を見る例
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/joinir \
  bash tools/checks/smoke_inventory_report.sh
```

出力:
- `target/smoke_inventory/integration_apps_inventory.tsv`
- `target/smoke_inventory/integration_apps_summary.txt`
- suite-aware summary は target subtree の scope に対して計算される
- profile root を target にすると profile-wide suite coverage を見やすい
- subtree を target にすると domain 別の orphan / coverage を見やすい

注意:
- `archive/lib/tmp/fixtures` は既定で live inventory から除外される
- それらを含めて見たいときだけ `SMOKE_INVENTORY_INCLUDE_PRUNED=1` を使う

## 12. Checks Script Index

`tools/checks/*.sh` の用途一覧（ENV棚卸し含む）:

```bash
cat docs/tools/check-scripts-index.md
tools/checks/env_dead_accessors_report.sh
```

## 13. Script Index (導線まとまり)

全 `.sh` 導線を SSOT で見たい場合は、まずこのインデックスに当たる:

```bash
cat docs/tools/script-index.md
tools/checks/route_env_probe.sh --route hako-mainline --source apps/tests/minimal.hako
tools/checks/route_env_probe.sh --route direct --require-no-fallback
tools/checks/route_no_fallback_guard.sh
tools/dev/direct_loop_progression_sweep.sh --profile default
tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe
tools/dev/phase29ca_direct_verify_dominance_block_canary.sh
```

`route_env_probe` は、`emit_mir_route.sh` 実行前の
`NYASH_MIR_CONCAT3_CANON` や fallback トグルを1回で確認する短絡入口。
`direct_loop_progression_sweep.sh` は direct route の loop progression 監視入口で、`phase29x-probe` は monitor-only（emit fail 許容）モード。
`phase29ca_direct_verify_dominance_block_canary.sh` は phase29ca の direct route 回帰（direct-verify/step-budget）を fail-fast で監視する。

## 推奨デバッグ順

1. 失敗 fixture を1つに固定
2. `bug_origin_triage.sh` で lane 分類
3. lane に応じて `hako_check` か Rust 側の最小ゲートを回す
4. 修正後に `tools/smokes/v2/run.sh --profile quick` で再確認
