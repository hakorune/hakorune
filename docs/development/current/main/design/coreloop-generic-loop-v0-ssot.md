---
Status: SSOT
Scope: CorePlan / FlowBox — generic structured loop v0 (decompose & compose)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
---

# CoreLoop generic structured loop v0 (SSOT)

## Goal

既知ルート（`scan_with_init` / `split_scan` / `loop_break` など。legacy labels are traceability-only）に一致しない “普通の while/loop” を、
CorePlan を汎用CFG命令セットへ肥大化させずに受理できる最小表現を固定する。

この文書は「未知ループは freeze」から「定義域内なら標準の CorePlan 合成で受理」へ移行するための SSOT。

## Non-goals

- 任意 goto / 任意ラベル分岐（禁止）
- AST/CFG 再解析による穴埋め（emit/merge は CorePlan 以外を見ない）
- irreducible/multi-entry loop の受理（Freeze: unstructured）
- v0 での nested loop 受理（v1 以降で “Loop を構造箱化” して対応）
- v0 での carrier 受理（non-loop-var 書き込みは v1 へ縮退）

## .hako migration policy (v0 is Rust-only)

- `.hako` 側の loop 受理は **v1-only**。v0 の語彙/契約は移植しない。
- v0 は **Rust 互換レイヤ専用**であり、移植が必要なら v1 の ShapeId + fixture + gate として再定義する。
- strict/dev + planner_required では v0 を抽出しない（v1-only で fail-fast）。
- v0 は v1 ShapeId に一致する形を **必ず reject** する（overlap policy SSOT）。

## Condition/Update canonicalization (analysis-only view)

generic_loop_v0 は “ループ変数が左辺に現れる” 等の形依存で候補生成が空になりやすい。
ただし raw 式を書き換える（例: `j+m<=n` → `j<=n-m`）のは、意味論（評価順/副作用/overflow 規約）リスクがあるため禁止する。

そこで Facts/Normalize 層で、実行コードを一切変更しない **解析用の正規形ビュー**を導入する（SSOT）。

### CondCanon (analysis-only)

- 目的: loop condition から `loop_var` を安全に抽出し、比較式の左右/向きを揃える（analysis-only）。
- 例の想定: `var + offset <= bound` / `bound >= var + offset` を同じ型として観測できる。
- 実装: 候補列挙（比較式の左右に現れる `Variable` / `Var ± ...`）は実装済み（`src/mir/builder/control_flow/plan/generic_loop/canon.rs`）。

### UpdateCanon (analysis-only)

- 目的: loop update の形揺れを吸収する（analysis-only）。
- 例の想定: `j=j+1`, `j+=1`, `j=1+j` を同一視して “step(delta=+1)” として観測できる。
- 実装: `UpdateCanon` と `canon_update_for_loop_var` を追加（`src/mir/builder/control_flow/plan/generic_loop/canon.rs`）。

### Safety contract (SSOT)

- **保守的（conservative）**にする: 少しでも怪しければ `None` を返す（正規化不能）。
- raw 式/文は保持し、実行/Lowering の意味論に介入しない（**no rewrite / no extra runtime work**）。
- 比較/算術の対象は最小スコープから開始する（例: 整数の `<,<=,>,>=` と `var ± const` のみ）。

## Conditional assignment / IfSelect (planned)

現状の CorePlan loop body contract（V12）は `IfEffect.then_effects` の空を禁止しているが、
“bind 更新だけの if（代入-only）” は leaf effect として表現されず、`then_effects` が空になりやすい。

その結果として `.hako` 側で `x = "" + x` のようなダミー leaf effect を入れて回避するルートが出やすい。
これは selfhost 移植（JoinIR/CorePlan を前提にした compiler 実装）で痛点になり得るため、CorePlan 側で解消する（予定）。

### Contract (SSOT)

- 方針: raw の制御構造を書き換えず、**データフローとしての条件付き更新**（Select/IfSelect 相当）へ正規化する。
- 対象: `if(cond){ x = <pure> } else { x = <pure> }` / else 省略（= else は x の現値）を最小単位で受理する。
- 純粋式のみ: `<pure>` は副作用を含まない式に限定（MethodCall/FunctionCall/ExternCall を除外）。
- 評価順: cond → then/else 値 → Select の順序を維持（pure 前提で then/else を両方評価しても意味論は不変）。
- 既定挙動: strict/dev + planner_required 限定で有効化し、release 既定は不変。
- no rewrite: AST を書き換えず、analysis-only view で Select を構成する。

### Goal

- `.hako` 側のダミー leaf effect を不要にし、meaningful な canonical form を SSOT として固定する。

## Next (v1 planned): Loop as a structural box

現状の v0 は「loop body は effect-only」を前提とし、loop を “構造部品” として積みにくい。
これを解消して CorePlan を整理するため、v1 では次を予定する（設計相談で合意済み、実装は別フェーズで段階投入する）:

- Implementation SSOT: `docs/development/current/main/phases/phase-29bs/README.md`
- `Loop.body` が `CorePlan`（木）を持てるようにする（`Seq/If/Loop` をネスト可能にする）
- `Break/Continue` は `depth`（最内=1）を持つ形へ解決する（将来のラベルは freeze で depth へ解決し、by-name を残さない）
- Lowerer は loop frame stack（break/continue の target をスタックで解決）で lowering する

## StepPlacement + StepMode (planned, strict/dev only)

`loop_increment`（step）が body の末尾ではないループは、step を step_bb に移すと意味論が変わり得る。
no-rewrite 方針のため、v0 はこの形を strict/dev で fail-fast してよい（selfhost canary で観測済み）。

そこで v1（strict/dev + planner_required）では、Facts が `StepPlacement` を **捨てずに保持**し、
Normalizer/Lowerer が step の扱いを `StepMode` として選択できるようにする。

- `StepMode::ExtractToStepBB`（現行）: 安全に移動できる placement のみ
- `StepMode::InlineInBody`（追加予定）: step を body の元の位置で lower（no rewrite）

### Verifier contract (initial, conservative)

`InlineInBody` は意味論事故を避けるため、最初は strict/dev で次を要求する:

- `Continue(depth=1)` を禁止（continue が step を踏む/踏まないの混線を封じる）
- `step_bb` の effect 列は空（inline なのに step_bb にも入る “二重実行” を防ぐ）

## Model

### Skeleton

- `Loop`（natural loop が一意に取れる範囲のみ）

### Body vocabulary

Loop body は “effect-only” を維持しつつ、制御は **ExitKind への脱出**だけ許可する。

許可:
- Leaf effects（既存 CoreEffectPlan の範囲: MethodCall/GlobalCall/BinOp/Compare/Const 等）
- ガード付き脱出（ExitIf）
- 非exitの最小条件分岐（IfEffect: then-only, leaf-effects only）
- IfEffect の then-body 末尾に限り `ExitIf(kind=Continue)` を 1個だけ許可（then-only）

禁止:
- 任意の分岐/ジャンプ（goto化）
- nested loop（v0では禁止。必要なら別subset）
- IfEffect の else / join / (Continue以外の) exit / ネスト（v0では禁止）

### Minimal control primitive

`ExitIf { cond, kind: ExitKind, payload }`

- `kind` は `Return/Break/Continue/(Unwind予約)` のみ
- normal へは飛べない（禁止: “次のブロックへ”）
- 発火したら、その箱の以降の effect は実行されない（箱の意味論）

注:
- `return-in-loop-minimal-ssot.md` の `ExitIfReturn` は、この一般形へ統合できる。

## Acceptance domain (v0)

generic loop v0 は次を満たす場合のみ受理する:

- loop condition が pure（副作用なし）
- body が “leaf effects + ExitIf + IfEffect(then-only, leaf-only)” のみで表現できる
- value expression は pure に限定し、`if` 式（`cond ? a : b` / `if cond { a } else { b }`）は **then/else が各1式かつ pure** の場合のみ許可
- break/continue/return が存在する場合は ExitIf で ports に落ちる（暗黙phiなし）
- in-body step は 1 回のみ許可（continue無し・loop_var再利用無し）で、末尾 step へ正規化する
  - strict/dev + planner_required では **step が body-local を参照する形**を許可（cond は pre-body map、step は post-body map で lower）

満たさない場合:
- release: 既定挙動を変えず（現状の fallback/Freeze 方針に従う）
- strict/dev: `flowbox/freeze` へ収束し、code を taxonomy SSOT に従って固定

## Why this is not "GeneralLoop CFG language"

- 制御は ExitKind への脱出に限定（任意 goto を禁止）
- join 表現は ports/payload の SSOT に固定（post-phi final form）
- emit/merge は再解析しない

## expr 判定 SSOT

generic_loop の expr 判定ロジックは `src/mir/builder/control_flow/plan/facts/expr/generic_loop_expr.rs` が SSOT:

- `is_supported_value_expr_for_generic_loop`: 値式検証
- `is_pure_value_expr_for_generic_loop`: 純粋値式検証
- `is_supported_bool_expr_for_generic_loop`: 論理式検証

## Idea: CanonicalLoopFacts redundancy reduction (SSOT)

- Rationale: Normalizer での再構築を減らし、Facts/Canon の重複を抑える。
- Status: design-only (no behavior change).
- Non-goal: change acceptance; refactor only.
