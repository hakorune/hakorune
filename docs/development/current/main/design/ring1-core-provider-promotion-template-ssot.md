# Ring1 Core Provider Promotion Template SSOT

Status: SSOT  
Date: 2026-02-18  
Owner: runtime lane (`phase-29y`)

## Purpose

- `ring1` provisional domain（`array/map/path/console`）を `accepted` へ昇格するときの最小手順を固定する。
- 「README だけ先行」「runtime配線だけ先行」の分断を防ぎ、1タスク完結で契約を固定する。

## Applies To

- `docs/development/current/main/design/ring1-core-provider-scope-ssot.md` の Decision Matrix で `provisional` と定義された domain。
- 現時点: `none`（`array` は `RING1-CORE-06`、`map` は `RING1-CORE-07`、`path` は `RING1-CORE-08`、`console` は `RING1-CORE-09` で `accepted`）。

## Promotion Contract (provisional -> accepted)

次の 5 点を **同一タスク** で満たした場合のみ `accepted` へ昇格する。

1. Runtime wiring
- `src/providers/ring1/mod.rs` に `pub mod <domain>;` を追加。
- runtime 側の配線SSOT（例: `provider_lock` / dispatch entry）を 1 箇所に固定。

2. Provider implementation
- `src/providers/ring1/<domain>/` に最小 provider 実装を追加。
- `ring2` 依存は禁止。意味決定ロジックの追加は禁止（薄い provider のみ）。

3. Contract tests
- fixture + smoke + guard を同一タスクで追加。
- lane gate へ preflight または step として組み込み、日次運用で検出可能にする。

4. Docs synchronization
- `ring1-core-provider-scope-ssot.md` の Decision Matrix を更新（`provisional -> accepted`）。
- `src/providers/ring1/<domain>/README.md` を placeholder から active 運用文書へ更新。
- `CURRENT_TASK.md` の done/next を更新。

5. Rollback path
- 失敗時は `pub mod <domain>;` と runtime wiring を戻せば復旧できるよう、差分を最小に保つ。

## One-Task Template

以下の順で 1 タスク（1 series）を実施する。

1. `RING1-CORE-XX-min1`: domain provider 実装 + wiring（まだ lane gate には入れない）
2. `RING1-CORE-XX-min2`: fixture + smoke + guard 追加
3. `RING1-CORE-XX-min3`: SSOT/README/CURRENT_TASK 同期 + lane gate 統合

## Commit Boundary Lock (min1/min2/min3)

`provisional -> accepted` 昇格では、次の境界を混ぜない。

- `min1`:
  - provider 実装 + runtime wiring だけ
  - smoke/guard/lane gate 配線はまだ触らない
- `min2`:
  - fixture + smoke + guard だけ
  - provider 実装ロジックは追加しない
- `min3`:
  - docs（scope/promotion/README/CURRENT_TASK）と lane gate 統合だけ
  - provider 実装は追加しない

## Domain Dry-Run Checklist

実昇格前に、対象 domain ごとに dry-run で「必要差分」を固定する。
実施結果（task pack）は次を正本とする:

- `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`

### `array` dry-run（historical, completed via `RING1-CORE-06`）

- min1:
  - `src/providers/ring1/mod.rs` に `pub mod array;` を追加する差分案を作る（未コミット可）
  - `src/providers/ring1/array/` へ最小 provider 骨格の差分案を作る
- min2:
  - `array` fixture/smoke/guard のファイル名と配置を確定する
  - lane gate へ入れる前提ステップ名を確定する
- min3:
  - `ring1-core-provider-scope-ssot.md` の `array` row 更新案を作る
  - `src/providers/ring1/array/README.md` の active 版文面を確定する

### `map` dry-run

- min1:
  - `src/providers/ring1/mod.rs` の export 追加案を作る
  - `src/providers/ring1/map/` provider 骨格差分案を作る
- min2:
  - `map` fixture/smoke/guard の命名と配置を固定する
  - lane gate step 連携案を固定する
- min3:
  - scope SSOT の `map` row 更新案を作る
  - `src/providers/ring1/map/README.md` active 版を作る

### `path` dry-run

- min1:
  - `src/providers/ring1/mod.rs` export 追加案を作る
  - `src/providers/ring1/path/` provider 骨格差分案を作る
- min2:
  - `path` fixture/smoke/guard の命名と配置を固定する
  - lane gate preflight or step の配置案を固定する
- min3:
  - scope SSOT の `path` row 更新案を作る
  - `src/providers/ring1/path/README.md` active 版を作る

### `console` dry-run（historical, completed via `RING1-CORE-09`）

- min1:
  - `src/providers/ring1/mod.rs` export 追加案を作る
  - `src/providers/ring1/console/` provider 骨格差分案を作る
- min2:
  - `console` fixture/smoke/guard の命名と配置を固定する
  - lane gate 連携の順序（preflight/step）を確定する
- min3:
  - scope SSOT の `console` row 更新案を作る
  - `src/providers/ring1/console/README.md` active 版を作る

## Phase-29y Docs Sync Targets

以下2箇所は promotion 実施時に必ず同期する。

- `docs/development/current/main/phases/phase-29y/README.md`
- `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`

受け入れコマンド（最低）

```bash
bash tools/checks/ring1_core_scope_guard.sh
bash tools/checks/phase29y_lane_gate_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh
```

## Non-Goals

- provisional domain を README 更新だけで `accepted` 扱いにすること。
- guard 未整備のまま lane gate へ runtime wiring を追加すること。
