---
Status: SSOT (design-only)
Scope: VerifiedRecipe の PortSig（出口署名）と wiring 契約（Recipe 合成 ⇒ 必ず lower 可能）
Related:
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/phi-input-strategy-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md
---

# VerifiedRecipe PortSig & Wiring Contract (SSOT)

## Summary (SSOT)

- 目的: VerifiedRecipe の PortSig/obligation を固定し、join/phi/loop-carried の silent wrong を防止する。
- 適用範囲: NoExit / IfJoin / LoopV0 / ExitAllowed / ExitOnly の VerifiedRecipeBlock。
- 検査対象: Fallthrough / Break / Continue / Return の obligation（Return は空なら許容）。
- strict/dev(+planner_required) でのみ contract freeze を出す（release 挙動は不変）。
- Verifier が PortSig を構成し、Parts は PortSig を検査するだけ（再判定しない）。
- then-only if の identity 許可は Step2 policy に従う（pre 値のみ）。
- Loop-carried は carrier 欠落を strict/dev(+planner_required) で freeze（Option A: 明示 carrier list）。

## Problem

Recipe-first で受理（Recipe 構築 + verify）できても、lowering の “配線規則” が欠けると **silent wrong** が起きる。
典型症状:

- `loop → if → loop` が **freeze せず**に実行されるが、出力がズレる（例: 期待10→実0）

これは「受理の再帰性」ではなく、**配線（join/phi/loop-carried）の不変条件不足**が原因になりやすい。

## Goal (SSOT)

**VerifiedRecipe = “配線できることが契約として保証された Recipe”** とする。

- Verifier が `RecipeBlock` を再帰的に検証し、`VerifiedRecipeBlock` を生成する
- `VerifiedRecipeBlock` は **PortSig（出口署名）** を持つ
- Lower/Parts は `VerifiedRecipeBlock` のみを入力として、PortSig に従って **Port を配線するだけ**の全域関数になる
- PortSig を満たせない場合は Verifier が VerifiedRecipe を作らず、strict/dev(+planner_required) で freeze（silent wrong 禁止）

## Recipe-first final form (SSOT)

- Facts は Recipe を組み立てるだけ（受理の真実は持たない）
- Verifier が「配線可能」の契約（PortSig/obligation）を保証する
- Lower/Parts はその契約に従って機械的に配線するだけ（再判定・例外処理はしない）

## Definitions

### PortType

Recipe を lower した結果の “出口” を型で表す。

- `Fallthrough`
- `Return`
- `Break(d)` / `Continue(d)`（ネスト深度 `d`）

### ObligationState (env obligation)

各出口で「その変数が正しく定義されているか」を 3 値で表す。

- `Defined`: 到達する全経路で定義済み（Lower は value を必ず持つ）
- `MaybeUndefined`: 未定義経路がある（strict/dev(+planner_required) では contract freeze）
- `OutOfScope`: その出口に持ち出せない（寿命/箱境界で死ぬ。strict/dev(+planner_required) では contract freeze）
  - MaybeUndefined は join_payload で freeze に直結する（pre/then/else 3-map 差分）

### PortSig

`PortType -> { var_name -> ObligationState }` の対応表。

- 値（ValueId）そのものは持たない（lowering の都合を Recipe payload に逆流させない）
- “配線可能性” だけを Verifier が保証する

## Core invariants (SSOT)

### I1. IfJoin completeness（join 入力完全性）

Join を持つ if（`IfContractKind::Join`）では、join に参加する各変数について:

- join に到達する predecessor **全て**が入力を持つ（PHI の基本制約）
- then-only if（else 無し）でも、join が必要なら **else 側入力を必ず定義**する

#### Policy (then-only if)

then-only if の “else 側入力” は、次のルールで SSOT 化する（曖昧さを残さない）。

- **pre で存在していた変数**: else 入力は identity（pre 値）を許可
- **then 側で初めて導入された変数**（local 宣言など）: else 経路で `OutOfScope` になり得るため freeze

Decision (Step 2):
この方針は **“pre 値流入の許可”**ではなく、「命令型意味論として未更新なら値が変わらない」を join で明示化する。

（これは “暗黙の pre 値流入” ではなく、「命令型意味論として未代入なら値は変わらない」を join で明示化する、という位置づけ）

### I2. Loop-carried completeness（キャリア入力完全性）

Loop header/backedge の wiring は、キャリア集合に対して入力が欠けないことを SSOT とする。

- header への入力は `preheader` と `backedge` が揃う
- `Fallthrough`（body 末尾）と `Continue(0)` は `backedge` と同扱い
- `Break(0)` は loop-after へ接続される

キャリア集合の決め方は 2 案ある（BoxShape を混ぜないため、決定が必要）:

- Option A: VerifiedRecipe が明示で carrier list を持つ（推奨: 安定）
- Option B: PortSig 推論を固定点（単調な集合方程式）として収束させる（保守的 freeze を含む）

Decision (Step 3):
Option A（VerifiedRecipe が明示で carrier list を持つ）を採用する。

### I3. Depth shift（Break/Continue の深さ合成則）

`Loop` で包むと出口の深さが 1 段シフトする。

- 内側の `Break(d)` / `Continue(d)` は、外側から見ると `Break(d+1)` / `Continue(d+1)` に伝播
- `Break(0)` / `Continue(0)` はその loop が消費する（after/backedge へ接続）

## Composition rules (informal)

詳細な推論規則（Seq/If/Loop の PortSig 合成）はこの文書を SSOT とするが、まずは次の “局所規則” を守る:

- `Seq`: `A.Fallthrough` を `B.entry` に接続し、`A` の非 Fallthrough ports は上に伝播
- `If`: then/else の `Fallthrough` を join に集約し、join 反映は “許可された binding 更新集合” のみに限定
- `Loop`: header/backedge/after を接続し、carrier 入力が欠ける場合は freeze

## Task plan (ordered)

この設計を実装へ落とす順序を固定する（BoxShape → fixture pin の順）。

1) docs-only: 本 SSOT と、関連 SSOT のリンク整備
2) BoxShape: strict/dev(+planner_required) で “silent wrong” を contract freeze に変換（観測→Fail-Fast）
   - Step 1: plan/steps/join_payload.rs の build_join_payload で then-only + join payload 非空を strict/dev(+planner_required) freeze に変換
3) BoxShape: IfJoin completeness（then-only identity policy）を Parts に実装（必要なら Verifier に obligation を追加）
4) BoxShape: Loop-carried completeness（carrier の決定法を Decision して実装）
   - Step 4 entry: Loop-carried wiring の実装入口は `parts/loop_.rs` / `parts/dispatch/if_join.rs` / `steps/join_payload.rs`
   - Backedge 直前で carrier を variable_map に同期し、欠落は strict/dev(+planner_required) で freeze
   - Step 4 done: wiring unit test で join dst → backedge carrier の反映を固定（test_joinir_wiring_then_only_loop_uses_join_dst_for_carrier）
   - Verifier/Parts: branch/loop-local (local) は obligation の対象外（block-scoped）
   - Parts: branch-local の収集と join map フィルタは `parts/join_scope.rs` に集約
   - Verifier: LoopV0 の carrier が pre に無い場合は freeze（strict/dev+planner_required）
- Verifier: If/Loop の最小合成（fallthrough と carrier Defined）を固定
- Verifier: OutOfScope の伝播（If/Loop 合成）を実装済み
- PortSig の保証範囲は NoExit/IfJoin/LoopV0 の VerifiedRecipeBlock に限定（ExitAllowed/ExitOnly は段階導入）
- ExitAllowed/ExitOnly でも PortSig を付与（strict/dev+planner_required のみ検査、release は挙動不変）
5) BoxCount: `loop → if → loop` を fast gate fixture で pin（入口寄せが必要なため保留）

## Next design stub (docs-only)

- Verifier が ObligationState を返す設計（Defined / MaybeUndefined / OutOfScope）
- Verifier は PortSig 構成時に ObligationState を付与し、strict/dev(+planner_required) で freeze 判定に使う
  - Partial impl: branch-local (local) は join obligation の対象外（branch-scoped）
- OutOfScope は Verifier が付与し、strict/dev(+planner_required) で freeze
- Verifier の PortSig は Parts 側で fallthrough obligation をチェックし、Undefined を freeze
- 出口別 PortSig を導入し、Parts 側で freeze 検査（Break/Continue/Return は strict/dev(+planner_required) で freeze）
- Return obligation は strict/dev(+planner_required) で freeze に昇格（release は挙動不変）
- Return freeze の適用範囲は strict/dev(+planner_required) に限定（release は挙動不変）
- Return obligation は空なら許容、値がある場合のみ freeze
- exit obligation の unit test を追加
- Seq 合成: A.Fallthrough の obligations を B へ引き継ぐ
- If 合成: then/else の Fallthrough を join し、Undefined が残れば freeze
- Loop 合成: carrier 以外は Loop に持ち出さない（OutOfScope 扱い）
