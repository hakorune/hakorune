# MIR Builder (`src/mir/builder/`)

Pointers:
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current selfhost bootstrap authority:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- MIR navigation root:
  - `src/mir/README.md`

このディレクトリは Rust 側の MIR 生成（AST → MIR）を担う。  
JoinIR の merge もここ（`control_flow/joinir/merge/`）が入口。

## Reading Order

1. `src/mir/README.md`
2. `src/mir/builder/README.md`
3. `src/mir/builder/control_flow/plan/`
4. `src/mir/contracts/README.md`

## 原則（SSOT / Box-First）

- **状態は Context が SSOT**: `MirBuilder` の状態は Context（箱）に分割され、二重管理をしない。
- **ValueId 発行は SSOT**: 関数内の ValueId は `MirBuilder::next_value_id()` を唯一入口にする。
- **境界は Fail-Fast**: JoinIR merge は `contract_checks.rs` で契約違反を早期検出する（debug-only）。

## Context 構成（責務マップ）

- `crates/hakorune_mir_builder/src/core_context.rs`
  - ID 生成器（ValueId/BlockId/BindingId 等）と最小の共通コア状態。
- `type_context.rs`
  - ValueId → 型/種別/起源（NewBox 由来など）の追跡。
- `scope_context.rs`
  - lexical scope / loop/if/try のスタックと、`current_function` / `current_block` の実行文脈。
- `crates/hakorune_mir_builder/src/binding_context.rs`
  - 変数名 ↔ BindingId の対応（shadowing の復元を含む）。
- `variable_context.rs`
  - 変数解決（variable_map 等）。
- `metadata_context.rs`
  - span/source_hint/region（観測）などのメタ情報。
- `compilation_context.rs`
  - コンパイル全体のレジストリ（Box/型レジストリ、reserved ids 等）。
- `crates/hakorune_mir_builder/src/context.rs`
  - 上記 Context を束ねる入れ物（`MirBuilder` はここを介して状態へアクセスする）。

## 主要エントリポイント

- ValueId/BlockId
  - `src/mir/builder/utils.rs`（`MirBuilder::next_value_id()` など）
- AST → MIR の基本道
  - `src/mir/builder/stmts.rs`
  - `src/mir/builder/exprs.rs`
- JoinIR merge（契約検証を含む）
  - `src/mir/builder/control_flow/joinir/merge/mod.rs`
  - `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`

## Top-Level Map

- `crates/hakorune_mir_builder/src/core_context.rs`: ID 生成器と最小の共通コア状態。
- `type_context.rs`: ValueId → 型/種別/起源の追跡。
- `scope_context.rs`: lexical scope / loop / if / try の実行文脈。
- `crates/hakorune_mir_builder/src/binding_context.rs`: 変数名 ↔ BindingId の対応。
- `variable_context.rs`: 変数解決（variable_map 等）。
- `metadata_context.rs`: span / source_hint / region の観測。
- `compilation_context.rs`: Box / 型レジストリと reserved ids。
- `crates/hakorune_mir_builder/src/context.rs`: 上記 Context を束ねる入れ物。

## 追加ルール（将来の変更者向け）

- 新しい状態を追加する場合は、まず「どの Context の責務か」を決めてから追加する（`MirBuilder` 直下に増やさない）。
- 変更後に最低限確認する:
  - `tools/smokes/v2/profiles/integration/apps/phase135_trim_mir_verify.sh`（MIR verify の回帰防止）

## P5 Crate Split Prep

`src/mir` の crate split を準備するとき、この subtree は `hakorune-mir-builder` 候補になる。
The first packaging slice has already landed in `crates/hakorune_mir_builder/`
with `core_context.rs` and `context.rs`; the remaining builder orchestration
stays here for now.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- public surface は `stmts.rs` / `exprs.rs` / `control_flow/` の入口に寄せる
- helper を増やす前に、Context の責務境界を README に書く
- split は docs-first で seam が固定されてから行う
