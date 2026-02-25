---
Status: SSOT
Scope: “Language completion” boundary for selfhost + `.hako` mirbuilder migration (surface syntax + AST/JSON v0 contract + fail-fast tags).
Related:
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md
  - docs/development/current/main/design/cond-block-view-prelude-ssot.md
  - docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/parser-extensions-param-implements-interface-generic-ssot.md
  - docs/reference/language/block-expressions-and-map-literals.md
  - docs/development/current/main/20-Decisions.md
---

# Selfhost Language v1 Freeze (SSOT)

## Goal

`.hako` 側の mirbuilder / JoinIR 移植を “揺れなく” 進めるために、先に **必要な言語機能の境界（v1）**を固定する。

ここでの “v1” は「Nyash 全体の最終仕様」ではなく、
**selfhost compiler（`lang/src/compiler/**`）が依存する言語サブセット**のこと。

## Non-goals

- 新しい表面構文の増殖（B3 sugar の実装は後回し）
- “便利だけど揺れる” 仕様（truthiness 拡張、曖昧な `{}` adjacency、silent fallback）
- BlockExpr を mini-CFG 化（exit を許す、型推論で吸う、など）

## v1 Definition (SSOT)

v1 とは、次を “互換破壊しない” と約束する範囲:

1. Rust parser と selfhost parser が同じ AST/JSON v0 形を生成できる
2. planner-required 経路（CorePlan/Parts）で lowering できる
3. 失敗は `[freeze:contract]` タグ付きで fail-fast できる

## v1 Required Surface/AST Features

### Structure

- `static box` / `box` / `method`
- `main()` entry（static box 内）

### Statements

- `local`（初期化あり/なし）
- assignment (`x = expr`, `obj.field = expr`, `arr[idx] = expr`)
- `if (...) { ... } else { ... }`（括弧は当面 permissive）
- `loop(cond) { ... }` と `loop(true) { ... }`（selfhost compiler が使用）
- `try { ... } catch (e) { ... } cleanup { ... }`（Stage‑3 exceptions; Bridge は Result‑mode で pin）
- `break` / `continue` / `return`
- `print(expr)`

### Expressions (minimum)

- literals: int/string/bool/null/void
- variable reference
- unary/binary ops（算術/比較/論理）
- method call / function call / generic call
- field access / index
- (if needed by compiler) `new` / array literal

### BlockExpr `{ ... }` (B2)

- AST: `ASTNode::BlockExpr { prelude_stmts: Vec<ASTNode>, tail_expr: Box<ASTNode> }`
- Contract (v1):
  - `tail_expr` is required
  - expression-position BlockExpr は **non-local exit を再帰的に禁止**
    - fail-fast tag: `[freeze:contract][blockexpr]`
  - JSON v0 bridge compatibility (selfhost Stage‑B):
    - Stage‑B JSON v0 が `tail` を “stmt JSON” として出す場合がある（例: `If`）。
    - 許可（保守的）: `If`（再帰）, `Local`, `Extern`, `Expr` のみ。lower は stmt を実行して `void` を返す。
    - それ以外の tail stmt は `[freeze:contract][json_v0][blockexpr]` で fail-fast。

### Condition Prelude via BlockExpr (B4)

- Condition entry view: `CondBlockView { prelude_stmts, tail_expr }`
- planner-required（CorePlan/Parts）でも prelude を lower してから `tail_expr` を評価できる
- Prelude statement vocabulary SSOT:
  - `src/mir/builder/control_flow/plan/policies/cond_prelude_vocab.rs`
  - fail-fast tag: `[freeze:contract][cond_prelude]`（語彙外/exit混入）

### MapLiteral `%{ ... }` (B1)

- Surface: `%{ "k" => expr, ... }`（v1: string key only）
- Legacy `{ "k": v }` map literal は fail-fast（migration 完了済み）

### Throw compatibility (bridge-only; not surface)

- surface parser 契約（Rust route）は `throw` を常時 reject する。
  - fail-fast tag: `[freeze:contract][parser/throw_reserved]`
- ただし Stage‑B/JSON v0 bridge の互換検証では `StmtV0::Throw` を扱う。
  - 目的: pre-selfhost stabilization（Result-mode lowering）を pin するため。
  - これは surface syntax の受理要件ではない。

## v1 Fail-fast Tags (must remain stable)

Minimum stable tags (grep distance short):

- `[freeze:contract][blockexpr]` (BlockExpr exit forbidden)
- `[freeze:contract][cond_prelude]` (cond prelude vocabulary / exit)
- `[freeze:contract][exit_depth]` (Break/Continue depth != 1)
- `[freeze:contract][recipe]` (RecipeBlock contract violations; dev/strict only by policy)

## v1 Exclusions (v2+)

これらは v1 freeze の外（実装してもよいが、selfhost/mirbuilder 移植を止めない）:

- B3 sugar（`if local x = ...; ...` などの parser sugar）
- BlockExpr 内での non-local exit（mini-CFG 化）
- MapLiteral の identifier key（`%{ k => v }`）
- truthiness の拡張（型変換の暗黙化）
- optimization annotations（`@hint` / `@contract` / `@intrinsic_candidate`）
  - 注釈導入方針は `optimization-hints-contracts-intrinsic-ssot.md` を正本とする。

## Concurrency / Async (out of v1; pinned elsewhere)

`nowait` / `await` は既存構文だが、selfhost compiler（`lang/src/compiler/**`）の v1 サブセット要件には含めない。

- 理由: selfhost compiler の実利用は現時点で `nowait/await` を要求しない（棚卸しは下の drift checks で再現可能）。
- 重要: “v1 に入れない” は “消す/無視する” ではない。
  - 意味論と VM+LLVM の整合は pre-selfhost stabilization として別 SSOT で pin する。
  - SSOT: `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Gates / Acceptance

v1 freeze の継続条件:

- Fast gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` が green
- Pinned fixtures:
  - `apps/tests/phase29bq_blockexpr_basic_min.hako`
  - `apps/tests/phase29bq_cond_prelude_planner_required_min.hako`
  - `apps/tests/phase29bq_map_literal_percent_min.hako`

## Selfhost Compiler Syntax Inventory (lang/src/compiler)

目的: “v1 が何を支えるべきか” を推測ではなく **実データ（selfhost compiler の使用実績）**で固定する。

### Scope (SSOT)

- 対象: `lang/src/compiler/**/*.hako`
- 注意: ここは “現状の利用実績” の棚卸しであり、v1 互換の可否とは独立に記録する（ギャップは下に明記）。

### Repro / Drift checks

棚卸しは次のコマンドで再現する（数字は目安。重要なのは “存在する/しない” の事実）:

- `rg -l "static box" lang/src/compiler | wc -l`
- `rg -l "\\blocal\\b" lang/src/compiler | wc -l`
- `rg -l "\\bif\\b" lang/src/compiler | wc -l`
- `rg -l "loop\\(" lang/src/compiler | wc -l`
- `rg -l "\\bnowait\\b" lang/src/compiler | wc -l`
- `rg -l "\\bawait\\b" lang/src/compiler | wc -l`
- `rg -l "\\btry\\b" lang/src/compiler | wc -l`
- `rg -l "\\bbreak\\b" lang/src/compiler | wc -l`
- `rg -l "\\bcontinue\\b" lang/src/compiler | wc -l`
- `rg -l "\\breturn\\b" lang/src/compiler | wc -l`
- `rg -l "\\busing\\b" lang/src/compiler | wc -l`
- `rg -l "%\\{" lang/src/compiler | wc -l`

BlockExpr prelude を “condition の中で” 使っているか（B4 の実利用）:

- `rg -n "if \\(\\{" lang/src/compiler`

legacy map/object literal（`{ key: value }` / `{ \"k\": v }`）が selfhost compiler code に残っていないこと（migration 完了確認）:

- broad grep（コメント/文字列も拾う。棚卸し用）:
  - `rg -n --glob '*.hako' '\\{\\s*[A-Za-z_][A-Za-z0-9_]*\\s*:' lang/src/compiler`
  - `rg -n --glob '*.hako' '\\{\\s*\"[^\"\\n]+\"\\s*:' lang/src/compiler`
- code-only（enforcement 用。0件が期待値）:
  - `rg --pcre2 -n --glob '*.hako' '^(?!\\s*//)[^\\\"]*\\{\\s*[A-Za-z_][A-Za-z0-9_]*\\s*:' lang/src/compiler`
  - `rg --pcre2 -n --glob '*.hako' '^(?!\\s*//)[^\\\"]*\\{\\s*\"[^\"\\n]+\"\\s*:' lang/src/compiler`

### Observed constructs (high signal)

- 構造: `static box` が広範に使用されている
- 制御: `if/else`, `loop(cond)`, `try/catch/finally`, `break/continue/return` が使用されている
- import 系: `using` が広範に使用されている
- MapLiteral: `%{ ... }` が使用されている（例: `lang/src/compiler/pipeline_v2/compare_extract_box.hako:23`）

### Critical gap: brace + colon “object/record literal”

`lang/src/compiler` では `{ ident: expr, ... }` 形式の “object/record literal” が実データとして存在する（例: `lang/src/compiler/entry/compiler.hako:36`）。

ただし v1 の表面仕様（B2）では `{ ... }` は BlockExpr であり、`{ ident: expr }` を literal として受理しない。

このギャップは `.hako` mirbuilder 移植の前提を崩すため、v1 freeze の範囲として次を固定する:

- v1 で “リテラルの map/object” を表す SSOT は `%{ "k" => expr }` のみ（string key only）
- selfhost compiler を v1 互換にするために、`{ ident: expr }` は **migration が必要**
  - 例: `{ emit: 0 }` → `%{ "emit" => 0 }`
  - 方針SSOT: `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`

## Change Policy (v1)

v1 の範囲に入る仕様変更は、必ず次の順で行う:

1. `docs/reference/language/*` を provisional/accepted として更新
2. `docs/development/current/main/20-Decisions.md` に Decision を追記（理由・非互換性・migration）
3. 実装
4. fixture + fast gate pin
