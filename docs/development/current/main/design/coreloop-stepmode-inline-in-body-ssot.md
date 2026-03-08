---
Status: SSOT
Scope: StepPlacement / StepMode (InlineInBody) — no rewrite loop expressivity
Related:
- docs/development/current/main/phases/phase-29bq/README.md
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/design/coreloop-continue-target-slot-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# StepMode::InlineInBody (SSOT)

## Goal

step（loop increment / update）が loop body の末尾に無いループを、**no AST rewrite** を守ったまま strict/dev で受理する。

重要: これは最適化ではなく **表現力（lowering方式）の拡張**。raw AST の式変形・コード移動は行わない。

## Definitions

- `StepPlacement`: Facts が観測した “step の位置”。
  - validate して捨てない（Facts の成果物として保持し、Normalize/Lowerer が推測しない）。
- `StepMode`: step の lowering 方式。
  - `ExtractToStepBB`: 従来方式。安全に step を `step_bb` へ移動できる placement のみ。
  - `InlineInBody`: step を **body の元の位置**で lower する（no rewrite）。

## Scope / gating

- `InlineInBody` は **strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1`** 限定で解禁する。
- release 既定は不変（silent fallback なし）。

## Scope (design-only)

- StepMode::InlineInBody は strict/dev の例外吸収のみ
- release では既定挙動不変

## Adapter Plan

- Facts/Canon は StepPlacement を観測のみで保持
- Verifier は StepMode を契約で固定（受理は増やさない）

## Invariants

- AST rewrite 禁止
- 受理拡張なし
- Lower は VerifiedRecipe のみ参照

## Verification (design-only)

- `rg -n "InlineInBody" src/mir/builder/control_flow/plan` で直書き箇所を棚卸し

## Initial verifier contract (conservative)

`InlineInBody` は意味論事故（評価順/回数の変化、二重実行、join入力欠落）を避けるため、最初は次を要求する。

1. `step_bb` の effect 列は空（inline なのに extracted が残る “二重実行” を防ぐ）
2. `Continue(depth=1)` を禁止（continue が “明示 step” を踏む/踏まない混線を封じる）
3. step は body 内で **1回だけ**実行される（複製禁止）

この契約は strict/dev で fail-fast できる（局所検証のみ）。

注:
- ここでの `Continue` 禁止は「step を statement/effect として body に残す（= 明示 step が存在する）」ループに限る。
  - 例: `generic_loop_v1` の `StepPlacement::InBody` を `StepMode::InlineInBody` で受理するケース。
- `loop_cond_break_continue` のように “明示 step” を持たず、per-edge carrier join（`ContinueWithPhiArgs` → join PHI）で
  backedge を構成するループは対象外（continue が step をスキップする事故が起きない）。

## Implementation status (2026-02-10)

- [x] V10b（`InlineInBody` のとき `step_bb` effect 空）を verifier 実装済み
  - Code: `src/mir/builder/control_flow/plan/verifier/loop_validators.rs`
- [x] S1: `CoreLoopPlan.has_explicit_step` marker を追加（挙動不変）
  - Code: `src/mir/builder/control_flow/plan/core.rs`
  - 現在値: `ExtractToStepBb => true`, `InlineInBody => false`（保守的初期値）
- [x] V10c（明示 step を持つ `InlineInBody` で `Continue(depth=1)` 禁止）を verifier 実装
  - Code: `src/mir/builder/control_flow/plan/verifier/loop_validators.rs`
  - Scope: current loop scope only（nested loop 内の continue は対象外）
- [x] V10d（明示 step の backedge 契約: body→continue の normal wire 1本）を verifier 実装
  - Code: `src/mir/builder/control_flow/plan/verifier/loop_validators.rs`
  - Contract: branch/exit 経路で continue_target へ直結する形を禁止
- [x] S3: `InlineInBody && has_explicit_step=true` の実受理形を 1つ固定し、runtime fixture で pin
  - Source: `src/mir/builder/control_flow/plan/normalizer/simple_while_coreloop_builder.rs`
  - Runtime pin semantics: `loop_simple_while explicit-step`
  - legacy fixture pin token: see `joinir-legacy-fixture-pin-inventory-ssot.md` entry for `loop_simple_while explicit-step`
  - Gate case semantics: `loop_simple_while explicit-step`
  - legacy gate token: see `joinir-legacy-fixture-pin-inventory-ssot.md` entry for the `loop_simple_while explicit-step` gate pin
  - pin taxonomy SSOT: `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`
  - active本文では `loop_simple_while explicit-step` を主語にし、legacy pin token の詳細は inventory SSOT に置く。

実装順（次の1本）:
1. [x] `CoreLoopPlan` に explicit-step marker を追加（挙動不変）
2. [x] verifier に V10c/V10d を追加（strict/dev + planner_required 限定）
3. [x] verifier unit test + `phase29bq_fast_gate_vm.sh --only bq` で契約 pin
4. [x] 実受理形（`InlineInBody && has_explicit_step=true`）の source 固定 + runtime fixture pin

## Interaction with ContinueTarget

- `InlineInBody` の将来の continue 解禁は、LoopFrame の `ContinueTarget` を前提にする。
- 当面は「continue 禁止」で安全を最優先し、continue を許したくなった時点で
  `continue_target` と per-edge join（carrier merge）の SSOT を拡張する。

## Promotion trigger (avoid exception pileup)

同種の selfhost blocker が 2 回目に現れた場合、個別の受理拡張（例外パターン）で増やさず、`StepPlacement/StepMode` へ昇格する。

例:
- "step の後に control flow があるため extracted できない" を 2 回観測 → `InlineInBody` を導入して吸収

## Implementation inventory (棚卸し 2025-01)

### 出現箇所

```bash
rg -n "LoopStepMode::InlineInBody" src/mir/builder/control_flow/plan
```

| ファイル | 行 | 種別 | 到達条件 |
|---------|---|----|---------|
| verifier/loop_validators.rs | V10b | CHECK | InlineInBody 設定時のみ |
| features/loop_true_break_continue_pipeline.rs | 156 | SETTER | planner/build.rs:383-387 |
| features/loop_cond_continue_only_pipeline.rs | 152 | SETTER | loop_cond_continue_only/facts.rs:62-63 |
| features/loop_cond_continue_with_return_pipeline.rs | 160 | SETTER | loop_cond_continue_with_return/facts.rs:43-44 |
| features/loop_cond_break_continue_pipeline.rs | 244 | SETTER | loop_cond_break_continue/facts.rs:99-101 |
| features/loop_cond_return_in_body_pipeline.rs | 210 | SETTER | loop_cond_return_in_body/facts.rs:24-25 |

### ゲートパターン（全 SETTER 共通）

```rust
let planner_required = strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
if !planner_required {
    return Ok(None);  // または return;
}
```

### 結論

- **release 既定で `InlineInBody` をセットし得る経路は存在しない**
- 全 SETTER が `strict/dev + planner_required` でゲート済み
- SSOT（本ドキュメント）の "Scope / gating" 記述と実装は一致
