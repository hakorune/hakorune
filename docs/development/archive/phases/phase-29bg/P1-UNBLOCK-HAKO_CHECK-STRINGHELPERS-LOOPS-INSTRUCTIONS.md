---
Status: Ready
Scope: compiler change (CorePlan/JoinIR), docs-first
---

# Phase 29bg P1: Unblock hako_check — selfhost `StringHelpers` loop shapes

## Goal

`NYASH_DISABLE_PLUGINS=1` の `hako_check` 実行（`tools/hako_check/cli.hako`）が、JoinIR freeze で止まらずに gate を完走できる状態を **CorePlan 側の対応**で固定する。

本フェーズは「`.hako` 側のループを書き換えて逃げる」のではなく、**コンパイラ（CorePlan/Composer/Lowerer）を強くする**ことを目的とする。

## Non-goals

- `NYASH_DISABLE_PLUGINS=1` を外す（原則：解析導線は plugin 無しで動く）
- by-name 分岐やハードコードでの一時しのぎ（AGENTS.md 5.1）
- release 既定のログ増加／挙動変更

## Background (P0 first FAIL)

`./tools/hako_check/deadcode_smoke.sh` が Test1 で失敗する。

原因は `tools/hako_check/cli.hako` が `using selfhost.shared.common.string_helpers as Str` を読み込む際に、`StringHelpers` 内のループが JoinIR で未対応と判定され、以下で fail-fast するため:

- `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.`
- `Function: StringHelpers.*/*`

`NYASH_DISABLE_PLUGINS=1` は警告を出すが、失敗の直接原因は **compile-time の JoinIR freeze** である（plugin 有無では解決しない）。

## Target functions (minimum set)

まずは `tools/hako_check/cli.hako` を通すために、少なくとも以下のループ形状を CorePlan 側で受理できる必要がある（順不同）。

- `lang/src/shared/common/string_helpers.hako`
  - `int_to_str/1`（単純 while: `loop(v > 0)`、body は効果列＋step）
  - `read_digits/2`（無限ループ＋break：`loop(true)` 内で `break`）
  - `starts_with/3`（前方走査＋早期 return 形）
  - 付随：`to_i64/1`, `json_quote/1`, `trim/1`, `skip_ws/2`, `split_lines/1` など（P1で全網羅できなければ P2へ）

## Design (SSOT)

Related policy:
- `docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md`

### A. 方針

- 既存の「pattern名による入口分岐」を増やさず、**CorePlan の語彙＋Composer の合成**で吸う。
- ループの骨格は `CorePlan::Loop`（FlowBox/ports/ExitMap に整合）で表す。
- ループ内の `break/continue/return` は「一般CFG」ではなく **FlowBox の exit port** に落とす。

### B. 追加する最小語彙（提案）

`return-in-loop-minimal-ssot.md` に沿って、CoreLoop の body が “effect-only” である制約を壊さずに、以下のどちらかで吸う:

1. `CoreEffectPlan` に `ExitIfBreak` / `ExitIfContinue` を追加（`ExitIfReturn` と同型）
2. もしくは Loop.body 内に `CorePlan::If` と `CorePlan::Exit` を許可し、Verifier で「Exit は分岐末尾のみ」を強制

P1では (1) を優先（語彙が小さく、FlowBox 的に局所verifyしやすい）。

### C. Facts / Composer の受理境界（Fail-Fast）

受理できないループは:

- release: 従来どおり（既定挙動不変）
- strict/dev: `flowbox/freeze` に収束し、freeze code を SSOT に沿って固定（例：`unsupported_loop_shape` / `unstructured`）

## Implementation steps

### Step 1: Repro harness (SSOT)

`tools/hako_check/deadcode_smoke.sh` と同等の env で、`StringHelpers` の compile を単体で再現できる最小 fixture を用意する。

- `NYASH_DISABLE_PLUGINS=1`
- `NYASH_BOX_FACTORY_POLICY=builtin_first`
- `NYASH_FEATURES=stage3`

この fixture は「解析対象 `.hako`」ではなく「compiler が selfhost libs を compile できること」を固定する目的で作る。

### Step 2: Add minimal CorePlan support for loop exits

- `CoreEffectPlan` に `ExitIfBreak`（必要なら `ExitIfContinue`）を追加
- `PlanVerifier` に loop-body 内での許可位置（末尾のみ等）を SSOT どおりに固定
- `PlanLowerer` に最小 lowering（`if cond { jump after } else { next }`）を追加

### Step 3: Extend Facts/Composer to accept StringHelpers loops

最初は “selfhost tools only” の subset として:

- 条件：`var > 0` / `var >= 0` / `var < n` のいずれか（必要最小）
- step：`var = var + 1` / `var = var - 1` / `var = var / const`（必要最小）
- body：副作用無し（Pure/MUTのみ）＋ ExitIf 系（break/return）

### Step 4: Gate

P0 gate を再実行し、P1 の受け入れ基準を満たすこと。

## Acceptance criteria

- `cargo build --release`
- `./tools/hako_check/deadcode_smoke.sh` が PASS
- `NYASH_DISABLE_PLUGINS=1` は維持（plugin 依存回避）
- strict/dev でのみ `flowbox/*` タグが増える（release 出力は不変）

## Next

- P2: `StringHelpers` の残り loop 形状（`trim` / `skip_ws` / `split_lines` 等）を同じ方針で追加吸収し、`./tools/hako_check/run_tests.sh` まで緑にする。
