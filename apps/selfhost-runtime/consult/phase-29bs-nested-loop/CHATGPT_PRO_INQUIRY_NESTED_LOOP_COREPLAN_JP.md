---
Status: Active
Scope: ChatGPT Pro への設計相談（CorePlan で nested loop を“部品合成”として表現する）
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- docs/development/current/main/phases/phase-29br/README.md
---

# ChatGPT Pro 設計相談: CorePlan で nested loop を部品合成として表現する

目的: いまの JoinIR / CorePlan / planner-required の流れを壊さずに、「ネストしたループ」を CorePlan の構造ノード合成で表現できる形へ収束させたい。
最終的には selfhost 側（.hako）へ CorePlan を移植するため、低レベルな擬似Effect（ExitIf/IfEffect など）の増殖よりも、構造木としての表現を優先したい。

## 背景（現状）

- CorePlan には `Seq/If/Loop/Exit/Effect` がある。
- しかし Loop は次の制約があり、nested loop の表現が止まっている:
  - Verifier の V12: **Loop.body must be Effect-only**（Loop.body に `CorePlan::Loop` や `CorePlan::If` を置けない）
  - Lowerer は `flatten_body_effects()` により Loop.body を `CoreEffectPlan` にフラット化し、非Effectを見つけるとエラーにする
  - `Exit::Break/Continue` は loop context が無いと lowerer がエラーを返す（いまは ExitIf/IfEffect のような“疑似Effect”で回避している）

結果として、selfhost compiler（`compiler.hako`）が持つ `BundleResolver.resolve/4` のようなループ構造（内側に loop を含む）を、
planner-required（strict/dev）で通そうとすると `planner returned None` → freeze で落ちる（Phase 29br）。

## 相談したい設計ゴール

「loop だけの部品（外枠）」を持ち、body に別の CorePlan（If/Seq/Loop/Exit/Effect）を入れて合成できること。
これにより nested loop を “部品×部品” で表現し、.hako 側へも同じ構造で移植しやすくしたい。

## 制約（守ること）

- 既定挙動不変（release default unchanged）
- strict/dev では silent fallback 禁止（freeze/fail-fast で検出可能にする）
- by-name ハードコード禁止（構造条件と契約で解く）
- Gate SSOT を常に緑維持:
  - JoinIR regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - planner-required dev gate v4: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

## 具体的な質問（ChatGPT Pro へ）

1) **CorePlan の最小拡張案**
   - `CoreLoopPlan.body` を effect-only から `Vec<CorePlan>`（または `CorePlan` の木）へ拡張する場合、
     どの不変条件（Verifier）を追加/変更すべき？
   - nested loop のとき、break/continue のスコープ（どの loop frame に作用するか）をどう表すのが一番壊れにくい？

2) **Lowerer の設計**
   - 現状 `flatten_body_effects()` によるフラット化が前提になっている。
     これを廃止し、`LoopFrame { break_bb, continue_bb }` のスタックで再帰 lowering する場合の設計案を出してほしい。
   - `CoreExitPlan::Break/Continue` を “正規の CorePlan ノード” として許可するなら、
     lowerer の API（`lower(plan, ctx)`）に loop context をどう渡すべき？

3) **Planner / Facts / DomainPlan の影響**
   - nested loop を Planner が返す DomainPlan でどう表現するのが最小差分？
     例: DomainPlan に `NestedLoop { outer: ..., inner: ... }` を追加する / 既存の GenericLoop を木として返す 等。
   - Facts は “骨格＋特徴” を重視している。nested loop で Facts が肥大化しない設計（Skeleton+Featureの合成）を提案してほしい。

4) **段階導入の計画**
   - 一気にネスト loop を解禁すると影響が大きいので、段階導入の安全順（P0/P1/P2...）を提案してほしい。
   - 例: まず `Loop.body` に `If/Seq/Exit` を許可（Loop は禁止）→ 次に nested loop を許可、など。

5) **テスト/ゲート設計**
   - 新しく追加すべき最小 fixture と gate を提案してほしい。
   - “selfhost compiler の BundleResolver 由来”をターゲットにする場合、どの最小化が望ましい？

## 参考（関連ソース・現状の制約点）

- Verifier: `src/mir/builder/control_flow/plan/verifier.rs`（V12: Loop.body effect-only）
- Lowerer: `src/mir/builder/control_flow/plan/lowerer.rs`（flatten_body_effects / Break/Continue の扱い）
- Plan types: `src/mir/builder/control_flow/plan/mod.rs`
- Normalizer entry: `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- Generic loop v0: `src/mir/builder/control_flow/plan/generic_loop/facts.rs`, `src/mir/builder/control_flow/plan/generic_loop/normalizer.rs`
- planner-required contract: `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- BundleResolver (selfhost compiler 側の現実): `lang/src/compiler/entry/bundle_resolver.hako`

## 期待するアウトプット

- “最終形の設計” と “段階導入の計画” を分けて提示してほしい。
- 変更点は「CorePlan/Verifier/Lowerer/Planner」のどこに入るかを明示してほしい。
- 既存 gate（29ae / planner-required v4）を壊さないための注意点（互換維持）を列挙してほしい。

