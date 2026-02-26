---
Status: Active (docs-first)
Decision: provisional
Date: 2026-02-25
Scope: plugin 実装を `.hako` へ移すための準備レーン（境界/順序/gate）を固定する。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-111-plg05-math-wave2-min6-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-113-plg06-pycompiler-wave3-min1-ssot.md
  - docs/reference/plugin-system/bid-ffi-v1-actual-specification.md
  - docs/reference/plugin-system/migration-guide.md
  - docs/reference/architecture/dynamic-plugin-flow.md
  - docs/reference/runtime/kernel-and-plugins.md
  - src/runtime/plugin_loader_unified.rs
  - src/runtime/plugin_loader_v2/enabled/loader/library.rs
  - src/config/nyash_toml_v2.rs
  - tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh
  - tools/plugin_v2_smoke.sh
  - tools/smoke_plugins.sh
  - tools/plugins/build-all.sh
  - tools/checks/windows_wsl_cmd_smoke.sh
---

# 29cc-95 Plugin Lane Bootstrap SSOT

## 0. Purpose

- non-plugin done 後の separate lane（plugin移植）を、実装前に docs-first で固定する。
- `.hako` plugin 実装を始める前に、対象範囲と受け入れ gate を一本化する。
- 「いきなり全面移植」を避け、1 blocker = 1 commit の運用に合わせる。

## 1. Scope Boundary

In scope（このレーンで扱う）:
1. plugin移植の順序（wave）固定
2. ABI/loader 境界（`nyash_plugin_invoke` + nyash.toml）固定
3. plugin lane 専用 gate pack（quick/milestone）固定

Out of scope（このレーンでは扱わない）:
1. non-plugin de-rust done 判定の再定義
2. 言語仕様拡張（`.hako` 文法の新機能追加）
3. fallback 常用で gate を通す運用

## 2. Inventory Snapshot (2026-02-25)

### 2.1 Wave-1 (core runtime boxes, first migration targets)
1. `plugins/nyash-string-plugin`
2. `plugins/nyash-integer-plugin`
3. `plugins/nyash-array-plugin`
4. `plugins/nyash-map-plugin`
5. `plugins/nyash-console-plugin`
6. `plugins/nyash-filebox-plugin`
7. `plugins/nyash-counter-plugin`

### 2.2 Wave-2 (utility / data / path/network helpers)
1. `plugins/nyash-json-plugin`
2. `plugins/nyash-regex-plugin`
3. `plugins/nyash-encoding-plugin`
4. `plugins/nyash-toml-plugin`
5. `plugins/nyash-math-plugin`
6. `plugins/nyash-path-plugin`
7. `plugins/nyash-net-plugin`

### 2.3 Wave-3 (bridge / high-complexity plugins)
1. `plugins/nyash-python-plugin`
2. `plugins/nyash-python-compiler-plugin`
3. `plugins/nyash-python-parser-plugin`
4. `plugins/nyash-egui-plugin`

### 2.4 Not-in-lane (workspace excluded placeholders)
1. `plugins/nyash-file`
2. `plugins/nyash-test-multibox`
3. `plugins/nyash-aot-plugin`
4. `plugins/nyash-mirjsonbuildermin-plugin`
5. `plugins/nyash-set-plugin`

## 3. Contract Lock (must keep)

1. plugin call ABI は単一エントリ `nyash_plugin_invoke` を維持する。
2. plugin metadata は nyash.toml（v2系）を唯一の設定入口として扱う。
3. loader の strict/best-effort 境界は既存挙動を崩さない（silent fallback禁止）。
4. path 解決は OS差分（`.so/.dylib/.dll`）を現行契約どおり維持する。

## 4. Fixed Order (docs-first, active)

1. `PLG-00` boundary lock（この文書 + 入口同期）
2. [done] `PLG-01` ABI/nyash.toml acceptance lock（fail-fast 条件の固定）
3. [done] `PLG-02` gate pack lock（quick/milestone lock, accepted）
4. [done] `PLG-03` wave-1 pilot（CounterBox, 1 plugin = 1 blocker = 1 commit）
5. `PLG-04` wave rollout（wave-1 -> wave-2 -> wave-3）

Current active next:
- `PLG-06-min2`（wave-3 rollout）

Progress:
- `PLG-01` done（2026-02-25）:
  - `29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`
- `PLG-02` started（2026-02-25）:
  - `29cc-97-plugin-gate-pack-lock-ssot.md`（accepted）
  - `PLG-02-BFIX-01` done（Ring0 init panic 解消）
  - `PLG-02-BFIX-02` done（legacy nyash binary exit2 解消）
- `PLG-03` done（2026-02-25）:
  - `29cc-98-plg03-counterbox-wave1-pilot-ssot.md`（accepted）
- `PLG-04-min1` done（2026-02-25）:
  - `29cc-99-plg04-arraybox-wave1-min1-ssot.md`（accepted）
- `PLG-04-min2` done（2026-02-26）:
  - `29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`（accepted）
- `PLG-04-min3` done（2026-02-26）:
  - `29cc-101-plg04-mapbox-wave1-min3-ssot.md`（accepted）
- `PLG-04-min4` done（2026-02-26）:
  - `29cc-102-plg04-stringbox-wave1-min4-ssot.md`（accepted）
- `PLG-04-min5` done（2026-02-26）:
  - `29cc-103-plg04-consolebox-wave1-min5-ssot.md`（accepted）
- `PLG-04-min6` done（2026-02-26）:
  - `29cc-104-plg04-filebox-wave1-min6-ssot.md`（accepted）
- `PLG-05-min1` done（2026-02-26）:
  - `29cc-106-plg05-json-wave2-min1-ssot.md`（accepted）
  - active next: `PLG-05-min2`（wave-2 rollout）
- `PLG-05-min2` done（2026-02-26）:
  - `29cc-107-plg05-toml-wave2-min2-ssot.md`（accepted）
- `PLG-05-min3` done（2026-02-26）:
  - `29cc-108-plg05-regex-wave2-min3-ssot.md`（accepted）
- `PLG-05-min4` done（2026-02-26）:
  - `29cc-109-plg05-encoding-wave2-min4-ssot.md`（accepted）
- `PLG-05-min5` done（2026-02-26）:
  - `29cc-110-plg05-path-wave2-min5-ssot.md`（accepted）
- `PLG-05-min6` done（2026-02-26）:
  - `29cc-111-plg05-math-wave2-min6-ssot.md`（accepted）
- `PLG-05-min7` done（2026-02-26）:
  - `29cc-112-plg05-net-wave2-min7-ssot.md`（accepted）
- `PLG-06-min1` done（2026-02-26）:
  - `29cc-113-plg06-pycompiler-wave3-min1-ssot.md`（accepted）
- `PLG-06-min2` done（2026-02-26）:
  - `29cc-114-plg06-python-wave3-min2-ssot.md`（accepted）
- `PLG-06-min3` done（2026-02-26）:
  - `29cc-115-plg06-pyparser-wave3-min3-ssot.md`（accepted）
  - active next: `PLG-06-min4`（wave-3 rollout）

## 5. Gate Pack (locked)

Daily quick:
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh`

Milestone:
1. `bash tools/plugin_v2_smoke.sh`
2. `bash tools/smoke_plugins.sh`
3. `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`

## 6. Acceptance Rule

1. plugin lane の変更は non-plugin done 契約（29cc-94）を壊さない。
2. wave を跨ぐ変更はしない（1 commit = 1 wave 内 1 blocker）。
3. gate FAIL 状態で lane pointer を PROMOTE しない。

## 7. Reopen / Rollback

次で blocker を reopen する:
1. `phase134_plugin_best_effort_init` FAIL
2. `plugin_v2_smoke` FAIL
3. non-plugin lane gate が plugin変更で回帰した場合

rollback は次の順で固定:
1. docs pointer を `PLG-00` 状態に戻す
2. gate緑の直前コミットへ作業単位を戻す（履歴で可逆）
3. 1 blocker を再分解して再実装
