# Phase 67: MIR Var Identity Survey + Shadowing Probe

## 目的

- MIR 側の「ブロックスコープ/シャドウイング」が現状どう実現されているかを確定する。
- JoinIR/Normalized の Ownership/Relay を BindingId（束縛ID）化すべきか、MIR 側のスコープ機構で足りるか判断する。

## 調査対象（読んだ場所）

- MIR ビルダー
  - `src/mir/builder.rs`（`variable_map`, `build_variable_access`, `build_assignment`）
  - `src/mir/builder/stmts.rs`（`build_block`, `build_local_statement`）
  - `src/mir/builder/exprs.rs`（`ASTNode::Program` の lowering が `cf_block` に直結）
- JoinIR lowering
  - `src/mir/join_ir/lowering/scope_manager.rs`（名前ベース lookup）
- AST
  - `src/ast.rs`（`ASTNode::ScopeBox` の意図コメント）
- 仕様（doc）
  - `docs/reference/language/variables-and-scope.md`（block scope + shadowing + assignment rule）
  - `docs/quick-reference/syntax-cheatsheet.md`（未宣言代入はエラー、と書いてある）

## 観測: 現状の MIR ビルダーは「名前→ValueId」1枚

### 1) “束縛”は BindingId ではなく name で追跡される

- `MirBuilder` は `variable_map: BTreeMap<String, ValueId>` を持つ（`src/mir/builder.rs:90`）。
- `build_variable_access` は `variable_map.get(name)` のみで解決し、無ければ即 `Undefined variable: <name>` エラー（`src/mir/builder.rs:533`）。
- `build_assignment` は「既存束縛の有無」を見ず、`variable_map.insert(name, value_id)` を行う（`src/mir/builder.rs:574`）。

結論: MIR/SSA の識別子解決は “name ベース上書き” であり、現状は BindingId（束縛の同一性）を持っていない。

### 2) ブロックは変数束縛のスコープフレームを作っていない

- `ASTNode::Program` は `cf_block` に入り、そのまま `build_block` を呼ぶ（`src/mir/builder/exprs.rs:20`, `src/mir/builder/control_flow/mod.rs:59`）。
- `build_block` は statements を順に lowering するだけで、`variable_map` の退避/復元を行わない（`src/mir/builder/stmts.rs:168`）。
- `local` は `build_local_statement` が `variable_map.insert(var_name, var_id)` を直で行う（`src/mir/builder/stmts.rs:281`）。

結論: 現状の MIR では「ブロックスコープ」は存在せず、`local` の再宣言は outer を恒久的に上書きする。

### 3) `ASTNode::ScopeBox` は “意味不変” のラッパーであり、束縛スコープではない

- `ASTNode::ScopeBox` は “診断/マクロ可視性のための no-op スコープ” と明記されている（`src/ast.rs:593`）。

## 仕様（doc）の不一致

- `docs/reference/language/variables-and-scope.md` は:
  - `local` は block-scoped（`docs/reference/language/variables-and-scope.md:9`）
  - inner `local` は shadowing（`docs/reference/language/variables-and-scope.md:11`）
  - 未宣言代入は “現在スコープに新規束縛を作る”（`docs/reference/language/variables-and-scope.md:22`）
- `docs/quick-reference/syntax-cheatsheet.md` は:
  - “未宣言名への代入はエラー” と書いてある（`docs/quick-reference/syntax-cheatsheet.md:13`, `docs/quick-reference/syntax-cheatsheet.md:204`）

この Phase 67 では「実装現状」を SSOT 化し、どちらの仕様に合わせて修正するかは Phase 68 以降で決める（ただし、少なくとも
shadowing の有無は言語中枢なので放置できない）。

## プローブ（VM スモーク追加）

追加したスモーク:

- `tools/smokes/v2/profiles/quick/core/phase215/scope_shadow_vm.sh`
  - 観測（Phase 67 当時）: `local x=1 { local x=2 } return x` が `rc=2`（inner `local x` が outer を上書きして leak）
- `tools/smokes/v2/profiles/quick/core/phase215/scope_assign_creates_local_vm.sh`
  - 観測（Phase 67 当時）: `{ y = 42 } return y` が `rc=42`（束縛が block 外へ leak）

再現コマンド:

- `bash tools/smokes/v2/profiles/quick/core/phase215/scope_shadow_vm.sh`
- `bash tools/smokes/v2/profiles/quick/core/phase215/scope_assign_creates_local_vm.sh`

## 結論と選択肢（Phase 68 への分岐）

結論（現状）:

- MIR ビルダーはブロックスコープ/シャドウイングを実装していない（関数スコープの `variable_map` 1枚）。
- JoinIR lowering 側も `ScopeManager::lookup(name)` の名前ベースであり、BindingId を前提にしていない（`src/mir/join_ir/lowering/scope_manager.rs:58`）。

選択肢:

- A) MIR にスコープフレームを入れてシャドウイングを MIR で解決
  - Phase 67 のプローブが “仕様どおり” に通るようになるまで、MIR ビルダー側で `variable_map` をスコープスタック化するのが
    直接的。
- B) JoinIR/Ownership を BindingId（束縛ID）SSOT にして、名前は表示専用に降格
  - Ownership/Relay/ShapeGuard の解析精度は上がるが、言語全体の block scope は MIR 側で結局必要になる（JoinIR だけ直しても
    Stage-3 の一般コードは救えない）。

Phase 67 の観測（de facto 実装）から、Phase 68 はまず A（MIR スコープフレーム）に進むのが安全。
その上で、Ownership/Relay が shadowing を正確に扱う必要が出た箇所だけを Phase 69+ で BindingId 化するのが最小差分になる。

---

## Status update（Phase 68/69 反映）

この設計分岐は実際に Phase 68/69 で完了した。

- Phase 68: MIR 側で lexical scope を実装
  - `{...}`（Program）/ `ScopeBox` を lexical scope として扱い、`local` shadowing を正しく復元。
  - “未宣言名への代入はエラー” を SSOT（quick-reference/LANGUAGE_REFERENCE）に揃えて Fail-Fast 化。
  - プローブは仕様固定へ更新:
    - `scope_shadow_vm` の期待は `rc=1`（outer が保持される）
    - `scope_assign_creates_local_vm` は “未宣言代入はエラー” を検証
    - `scope_loop_body_local_vm` を追加（loop body local が外へ leak しない）
- Phase 69: Ownership/Relay 側で shadowing-aware 化
  - `AstOwnershipAnalyzer` を内部 `BindingId` で分離し、ネスト block local が loop carriers/relay に混線しないように修正。

次の焦点は、Phase 65 で定義した merge relay / 本番 multihop の実行導線（Phase 70+）に戻る。

関連:
- Phase 68（lexical scope 実装）: `src/mir/builder/vars/lexical_scope.rs`（commit `1fae4f16`）
- Phase 69（shadowing-aware ownership）: `src/mir/join_ir/ownership/ast_analyzer.rs`（commit `795d68ec`）
