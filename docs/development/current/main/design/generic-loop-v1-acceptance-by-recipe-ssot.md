---
Status: SSOT
Scope: generic_loop_v1 acceptance (Recipe-first, recursive; ShapeId is hint-only)
Related:
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/coreplan-skeleton-feature-model.md`
- `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
- `docs/development/current/main/design/generic-loop-v1-shape-ssot.md`
- `docs/development/current/main/design/condition-observation-ssot.md`
- `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
---

# generic_loop_v1 Acceptance by Recipe (SSOT)

## Problem

- `RecipeBlock` は **再帰構造**（Seq/If/Loop/Exit/Stmt の合成）。
- 現状の `GenericLoopV1ShapeId` は **非再帰の列挙**になりやすく、ネスト（loop→if→loop…）が来るたびに
  “組み合わせ専用 ShapeId” を増やす運用へ落ちる。

これは “受理が合成で閉じない” ため、受理追加が作業化し、設計SSOT（RecipeTree + Parts）とも衝突する。

## Goal (SSOT)

generic_loop の受理を「ShapeId の列挙」ではなく、**Recipe の構造契約（Verifier）**で決める。

受理集合が `RecipeItem` の合成で閉じること（closure under composition）を SSOT とする。

### Loop unification target
- loop 受理は **最終的に generic_loop_v1 へ一本化**する（LoopCondBreak は特化箱として縮退・撤去する方針）
- 複数 Box が同じ loop で成立するのは “一時的な重なり” とみなし、最終形では guard を disjoint にして一意化する

## Non-goals

- AST rewrite（見かけ等価の式変形）
- silent fallback（strict/dev での “None で誤魔化す”）
- by-name/hardcode dispatch（関数名/文字列一致での分岐）
- Lower 側での再判定（Facts/Recipe の判断を Lower で再実装しない）

## Definitions

- **Recipe**: `RecipeBlock { items: Vec<RecipeItem> }`（構造SSOT）
- **VerifiedRecipe**: `VerifiedRecipeBlock`（Verifier を通った wrapper。Lower の唯一入力）
- **CondProfile**: 条件の parameterized skeleton（観測のみ。acceptance の真実は Verifier）
- **GenericLoopV1ShapeId**: body の “coverage label / diagnostic hint”。
  - 骨格や最適化のヒントとして保持してよいが、**受理の真実にしてはいけない**。

## Acceptance contract (SSOT)

### 0) Hole / Unverified の扱い

- Facts が観測できない箇所は “未構築” として扱う（Recipe に Hole を残さない）。
- VerifiedRecipe は **Hole を含まない**ことが条件（Hole が残るなら reject / freeze）。

### 1) planner_required + strict/dev の受理

generic_loop_v1 は次を満たすとき受理する:

1. Facts が `RecipeBlock(body)` を構築できる（観測のみ。no rewrite）。
2. Verifier が `RecipeBlock(body)` を検証し、`VerifiedRecipeBlock` を作れる。

`shape_id` の有無は受理条件ではない（hint-only）。

#### Shape overlap policy (safe-side)

- loop_cond_break_continue が先に観測されても、generic_loop_v1 の shape hint が取れる場合は v1 抽出を許可する。
- shape overlap 検知時は v1 優先しない（safe-side）。strict/dev では本体 extract の freeze で止める。

### 2) Freeze / reject taxonomy

タクソノミは `planfrag-freeze-taxonomy.md` に従う。

- Facts/Planner 段階で “対象っぽいが未対応”:
  - `[plan/freeze:unsupported] generic_loop_v1: cannot build recipe for body: <reason>`
- Recipe/Verifier 段階で契約違反:
  - `[freeze:contract][generic_loop_v1] <invariant>`

### 3) Lower boundary

- Lower/Parts は **VerifiedRecipeBlock のみ**を受け取る。
- Lower は `shape_id` を再検出しない（Facts の hint を参照しても “受理再判定” はしない）。

## ShapeId policy (SSOT)

### ShapeId is hint-only

- ShapeId は **coverage map**（fixture/gate の対応付け）として残してよい。
- ただし “ShapeId が無いから freeze” の設計は、ネスト合成を殺すため禁止。

### When ShapeId is allowed to grow

ShapeId を増やしてよいのは “真に Body skeleton が変わる” ときだけ:

- Loop skeleton の CFG/SSA に影響する新しい骨格（例: 新しい step placement モード）
- overlap policy を増やさずに一意化できる、診断上のラベル追加（挙動不変）

次は明示的に禁止:

- ネストの組み合わせ専用 ShapeId（`Loop(If(Loop(...)))` のような合成列挙）
- “selfhost blocker を通すためだけ” の ShapeId 増殖（根治は Recipe composability）

## Implementation order (SSOT)

1. **Recipe-first payloadization（generic_loop_v1）**
   - Facts が “body を RecipeBlock にする” 入口を持つ（既存の Recipe builders へ集約）。
   - 未観測は Hole ではなく、**未構築=unsupported** として fail-fast（strict/dev）。
2. **Verifier-only acceptance へ切替**
   - planner_required の “shape required” を廃止し、VerifiedRecipe を受理条件にする。
3. **ShapeId を hint に降格**
   - 形ラベルは coverage/診断に残しつつ、受理の真実から外す。
4. **観測SSOTの更新（漏れ防止）**
   - StepTree extractor / parity / count 系が再帰走査できることを同コミットで固定する。
5. **Gate/fixture の契約固定**
   - “ネスト合成で追加作業ゼロ” を示す最小 fixture を追加し、fast gate に pin する。

## Acceptance criteria (docs-first)

以下が満たされることを “完了条件” とする:

- generic_loop_v1 の受理追加が “ShapeId 増殖” ではなく “RecipeItem 合成” で行える。
- strict/dev + planner_required で、`shape_id=None` でも **VerifiedRecipe が作れれば**通る。
  - `body_exit_allowed.is_some()` で recipe が構築可能なら受理する（shape_id は hint-only）。
- 失敗時は taxonomy に従い、freeze タグが安定している（tests/gates の距離が短い）。

## Pinned fixtures (coverage)

- LoopCondBreak 側（非 generic_loop_v1）:
  - `apps/tests/phase29bq_generic_loop_v1_nested_if_min.hako`
  - planner_first: `LoopCondBreak`
- generic_loop_v1（Recipe-first）:
  - `apps/tests/phase29bq_generic_loop_v1_recipe_nested_if_min.hako`
  - planner_first: `LoopSimpleWhile`（legacy label: `Pattern1`）
  - NOTE: if 分岐内の Call は **意図的に未到達**（VM の closure 実行を避けるため）。

## Known gaps (未対応構造)

以下の構造は **JoinIR lower 側の BoxShape（wiring 契約）**として後日対応予定（受理追加ではない）：

- **loop → if → loop** （if でガードされた内側ループ）:
  - 現象: 期待出力 10 に対し実際の出力は 0（内側ループが実行されない）
  - 原因: IfJoin / Loop-carried の wiring 契約が不足している（silent wrong を防ぐ Fail-Fast が先）
- **loop → loop → if** （内側ループ内の if）:
  - 現象: `[joinir/freeze] Loop lowering failed`
  - 原因: 同上（wiring の受理範囲が未定義）

これらは「受理の問題」ではなく「lowering 実装の未完了」であるため、Backlog で BoxShape として対応する。
SSOT: `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`

## Status

Implemented
- Acceptance: recipe-only (`has_generic_loop_v1_recipe_hint`)
- ShapeId: hint-only (diagnostic/tag only, not acceptance)
