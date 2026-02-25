---
Status: SSOT
Scope: Phase 29bq+ の “compiler cleanliness / BoxShape-first” における、箱の責務と入口の最終形（north-star）。
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- docs/development/current/main/design/condition-observation-ssot.md
- docs/development/current/main/design/join-explicit-cfg-construction.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
---

# Compiler Pipeline (north-star) (SSOT)

目的: “層がどこまで何をするか” を SSOT で固定し、責務混線（特に PHI/SSA と call operand の境界）を BoxShape で収束させる。

## Pipeline (north-star)

```
Parser (観測; no rewrite)
  ↓
RecipeStore (SSOT)
  ↓
Observation Views (analysis-only; no rewrite)
  ↓
ShapeDecider (BoxShape only)
  ↓
Verifier (唯一の受理点; fail-fast)
  ↓
SSA/PHI Lower (Verified-only; φ/edge-copy で値を運ぶ)
  ↓
Codegen / VM
```

## 各段の責務（MUST / MUST NOT）

### 1) Parser（観測）

- MUST: ソース→AST を生成する（構文エラー位置の保持）。
- MUST NOT: AST rewrite（見かけ等価変形）を行わない。

### 2) RecipeStore（SSOT）

- MUST: AST を RecipeBody/RecipeBlock へ “参照” で束ねる（再構築しない）。
- MUST: “入口の SSOT” は RecipeTree を起点にする（`recipe-tree-and-parts-ssot.md`）。
- MUST NOT: SSA/PHI の真実を持たない（ValueId の伝播をここで直さない）。

### 3) Observation Views（analysis-only）

- MUST: 受理判定に必要な観測を view/canon に集約する（例: `CondBlockView`, `CondCanon`, `UpdateCanon`）。
- MUST NOT: rewrite で “形を合わせる” ことを禁止（観測だけを増やす）。

### 4) ShapeDecider（BoxShape only）

- MUST: 「どの箱の形か」を決める（skeleton/feature slot の選択）。
- MUST NOT: PHI/SSA の修復をしない（それは SSA/PHI Lower の責務）。
- NOTE: ShapeId は診断/coverage の hint であって、受理条件の真実にしない（`generic-loop-v1-acceptance-by-recipe-ssot.md`）。

### 5) Verifier（唯一の受理点; fail-fast）

- MUST: 受理条件を 1 箇所に固定し、契約違反は `[freeze:contract]` で止める。
- MUST: VerifiedRecipeBlock は “Lower が必ず下ろせる” を保証する（PortSig 等）。
- MUST NOT: silent fallback をしない（accept→lower での再判定も最小化）。

### 6) SSA/PHI Lower（Verified-only; φ/edge-copy で値を運ぶ）

- MUST: `VerifiedRecipeBlock` のみを入力に取り、CFG の “実 predecessor” に基づいて PHI/edge-copy を生成する。
- MUST: “def dominates use” を満たす形で ValueId を運ぶ。
- MUST NOT: entry 以前の層（JoinIR / Facts / ShapeDecider）で PHI 命令を emit して variable_map の真実を部分更新しない。

### Call operand / ValueId スコープ（重要）

- ValueId は **関数スコープ**（params + 関数内での dst 定義）に閉じる。
- call operand（recv/callee/args）に、関数スコープ外の ValueId が混入するのは禁止。
  - strict/dev(+planner_required) では fail-fast で止め、原因側へ寄せる:
    - `[freeze:contract][call/arg_out_of_function_scope]`（タグ/フィールドは debug contract SSOT）

### ValueId の定義完全性（ghost id 禁止）

- 「値を生む」ヘルパー（Const emission / CarrierInit / PHI fallback など）は、失敗を握りつぶして ValueId を返してはいけない。
  - 禁止例: `let dst = next_value_id(); let _ = emit_instruction(...); dst`（emit失敗で ghost id を作る）
- MUST: 値を生むヘルパーは `Result<ValueId, String>` を返し、`emit_instruction(...) ?` で必ず伝播する。
- ねらい: “未定義 ValueId が call operand に混入” を原因側で止め、VM/Verifier まで運ばない。
- 追加（今回の教訓）:
  - `alloc_typed()` / `next_value_id()` は **定義ではない**（ValueId の確保＋型メタ付与に過ぎない）。
  - variable_map の真実は “定義済み ValueId” のみを指してよい。PHI dst を先に確保する場合は、PHI 命令が emit されるまで
    plan 側の `phi_bindings` / `current_bindings` 等に閉じ込め、variable_map を先に更新しない（partial update 禁止）。

### 7) Codegen / VM

- MUST: MIR を実行/コード生成する。
- MUST NOT: “構造の正しさ” をここで直さない（Verifier/SSA Lower で止める）。

## JoinIR がやっていた「PHI の運び方」はどこへ行くか？

SSOT: JoinIR は “計画（spec）” を返すまで。

- JoinIR: `PhiSpec` / `JoinPayload` / snapshots を返す（plan-only; 命令 emit しない）。
- SSA/PHI Lower: CFG の実 predecessor を見て PHI（または edge-copy）を生成し、variable_map を確定する。
- Verifier: pred ラベル整合・dominance などを fail-fast で保証する。

これは “PHI の運び方（ownership）” の SSOT であり、混線禁止の根拠になる。

## Debug-driven convergence（推奨運用）

この最終形を “先に大改造で作る” のではなく、デバッグしながら収束させる。

- 失敗が出たら:
  1) 入口/責務の漏れを 1 箇所に絞る（BoxShape）
  2) strict/dev(+planner_required) で fail-fast（1行タグ）に寄せる
  3) SSOT（本書 + debug contract）に “禁止/責務/撤去条件” を固定
  4) 小コミットで原因側を潰す（1ブロッカー=1コミット）

デバッグが続くこと自体は異常ではないが、次を意味する:

- “境界がまだ穴あき” で、ValueId/PHI/call operand の責務が混線している可能性がある。
- それを **fail-fast + SSOT** で “原因側へ寄せる” のが BoxShape 作業。
