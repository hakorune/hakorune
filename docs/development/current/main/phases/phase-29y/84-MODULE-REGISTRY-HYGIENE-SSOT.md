# Phase 29y / Module Registry Hygiene SSOT

Status: Active
Decision: accepted
Last updated: 2026-02-19

## Purpose

`hako.toml` / `nyash.toml` の `[modules]` を肥大させないために、`modules.workspace + */hako_module.toml` を正本に固定し、top-level alias の残置境界を機械検証する。

## Contract

1. exact duplicate（`[modules]` と workspace export が同値）は常に `0`
2. intentional override は allowlist のみ
3. top-only alias は no-growth baseline（新規追加は fail-fast）
4. top-only alias の削減は許可（allowlist の追従更新は推奨）

## Guard / Inventories

- guard: `tools/checks/module_registry_hygiene_guard.sh`
- hako top-only allowlist: `tools/checks/module_registry_hako_top_only_allowlist.txt`
- nyash top-only allowlist: `tools/checks/module_registry_nyash_top_only_allowlist.txt`
- override allowlist: `tools/checks/module_registry_override_allowlist.txt`

## Snapshot (2026-02-19)

- `hako.toml`: `[modules]=143`, `top-only=143`, `override=0`, `duplicate=0`
- `nyash.toml`: `[modules]=158`, `top-only=158`, `override=0`, `duplicate=0`
- override allowlist: empty（`selfhost.vm.helpers.mini_map` withdrawn）

Prefix distribution:

- `hako.toml top-only`: `lang=53, hako=49, selfhost=25, apps=9, hakorune=3, nyash=2, sh_core=1, tools=1`
- `nyash.toml top-only`: `lang=68, hako=49, selfhost=25, apps=9, hakorune=3, nyash=2, sh_core=1, tools=1`

## Why top-only remains

1. `apps.*` / `tools.*`
- workspace module export の責務外。アプリ/ツール導線として top-level alias を維持する。

2. `hako.*` / `hakorune.*` / `nyash.*`
- 互換 alias（CLI/既存fixture/外部AI handoff）として維持する。

3. `lang.*` / `selfhost.*`
- 旧直参照 alias が残存。原則は workspace export へ移管し、段階的に削除する。

4. 差分キー
- `lang.shared.module_roots_priority_box`（hako-only）
- `lang.compiler.mirbuilder.*` の16キー（nyash-only）

## Override Withdrawal Rule (mini_map)

- 対象キー: `selfhost.vm.helpers.mini_map`
- 条件: workspace export（`lang/src/vm/hako_module.toml`）の正規化 path が実体（`boxes/mini_map_box.hako`）と一致した時点で、root `[modules]` override を両方（`hako.toml` / `nyash.toml`）から削除する。
- 同期要件: 同コミットで `tools/checks/module_registry_override_allowlist.txt` から対象キーを削除する。
- 検証: guard が `override=0` / `duplicate=0` を返すこと。stale alias が残る場合は guard fail で止める。

## Update Procedure

1. 新しい `.hako` box は `*/hako_module.toml` の `[exports]` に追加する
2. `hako.toml` / `nyash.toml` の `[modules]` へは新規追加しない（direct add は guard fail）
3. override/compat を追加する場合のみ allowlist を更新し、明示レビューで承認する
4. alias を移管・削除した場合は allowlist の追従更新を行い、guard を再実行する

## Acceptance

```bash
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh
bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh
```
