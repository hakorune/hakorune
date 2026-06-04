# MIR Builder (`src/mir/builder/`)

Pointers:
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current selfhost bootstrap authority:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- MIR navigation root:
  - `src/mir/README.md`

このディレクトリは Rust 側の MIR 生成（AST → canonical MIR emission）を担う。
`control_flow/plan` と JoinIR merge は物理的にはここにあるが、builder core
ではなく FlowPlanner / JoinIR glue として読む。

## Reading Order

1. `src/mir/README.md`
2. `src/mir/builder/README.md`
3. `src/mir/builder/control_flow/plan/ARCHITECTURE.md`
4. `src/mir/builder/control_flow/plan/REGISTRY.md`
5. `src/mir/contracts/README.md`

## Builder Core vs FlowPlanner

Builder core owns:

- AST node dispatch into canonical MIR emission.
- ValueId / BlockId issuance through `MirBuilder::next_value_id()` and related
  helpers.
- lexical scope / binding / local state through Context owners.
- source span / diagnostic provenance.
- actual MIR block assembly after a route has been selected.

FlowPlanner owns:

- control-flow shape facts.
- recipe contracts.
- CorePlan skeletons / features.
- planner-required fail-fast boundaries.
- plan lowering contracts.

Physical path today:

```text
src/mir/builder/control_flow/plan/
```

Conceptual owner name:

```text
FlowPlanner
```

Builder code should call the documented FlowPlanner / route-entry facades, not
reach into route-specific plan internals. The current boundary SSOT is
`docs/development/current/main/design/mir-builder-diet-flowplanner-boundary-ssot.md`.

## 原則（SSOT / Box-First）

- **状態は Context が SSOT**: `MirBuilder` の状態は Context（箱）に分割され、二重管理をしない。
- **ValueId 発行は SSOT**: 関数内の ValueId は `MirBuilder::next_value_id()` を唯一入口にする。
- **境界は Fail-Fast**: JoinIR merge は `contract_checks.rs` で契約違反を早期検出する（debug-only）。

## Context 構成（責務マップ）

- `crates/hakorune_mir_builder/src/core_context.rs`
  - ID 生成器（ValueId/BlockId/BindingId 等）と最小の共通コア状態。
- `crates/hakorune_mir_builder/src/type_context.rs`
  - ValueId → 型/種別/起源（NewBox 由来など）の追跡。
- `src/mir/builder/scope_context.rs`
  - lexical scope / loop/if/try のスタックと、`current_function` / `current_block` の実行文脈。
  - `MirFunction` と lexical-scope state がまだ同じ実行文脈にあるため、packaging は保留中。
- `crates/hakorune_mir_builder/src/binding_context.rs`
  - 変数名 ↔ BindingId の対応（shadowing の復元を含む）。
- `crates/hakorune_mir_builder/src/variable_context.rs`
  - 変数解決（variable_map 等）。
- `crates/hakorune_mir_builder/src/metadata_context.rs`
  - span/source_hint/region（観測）などのメタ情報。
- `src/mir/builder/compilation_context.rs`
  - コンパイル全体のレジストリ（Box/型レジストリ、reserved ids 等）。
  - `ASTNode` / `FunctionSlotRegistry` / `TypeRegistry` がまだ混在しているため packaging は保留中。
- `crates/hakorune_mir_builder/src/context.rs`
  - 上記 Context を束ねる入れ物（`MirBuilder` はここを介して状態へアクセスする）。

## 主要エントリポイント

- ValueId/BlockId
  - `src/mir/builder/utils.rs`（`MirBuilder::next_value_id()` など）
- AST → MIR の基本道
  - `src/mir/builder/stmts.rs`
  - `src/mir/builder/exprs.rs`
- member call route selection / emission boundary
  - `src/mir/builder/calls/build.rs`
  - `src/mir/builder/calls/member_route.rs`
  - `src/mir/builder/calls/static_resolution.rs`
  - `src/mir/builder/calls/extern_calls.rs`
  - `src/mir/builder/calls/receiver_binding.rs`
- function-call preflight special gates
  - `src/mir/builder/calls/function_preflight.rs`
  - `src/mir/builder/calls/special_method_handlers.rs`
- field/property receiver facts
  - `src/mir/builder/field_facts.rs` (observation only; no receiver AST re-lowering)
  - `src/mir/builder/fields.rs`
  - `src/mir/builder/property_reads.rs` (property getter lowering)
  - `src/mir/builder/properties.rs` (MIR-side property getter naming/registry)
- JoinIR merge（契約検証を含む）
  - `src/mir/builder/control_flow/joinir/merge/mod.rs`
  - `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`
- FlowPlanner public entry
  - `src/mir/builder/control_flow/joinir/route_entry/router.rs`
  - `src/mir/builder/control_flow/lower/planner_compat.rs`
  - `src/mir/builder/control_flow/plan/REGISTRY.md`

## Top-Level Map

- `crates/hakorune_mir_builder/src/core_context.rs`: ID 生成器と最小の共通コア状態。
- `crates/hakorune_mir_builder/src/type_context.rs`: ValueId → 型/種別/起源の追跡。
- `src/mir/builder/scope_context.rs`: lexical scope / loop / if / try の実行文脈。
  - packaging は `MirFunction` と lexical-scope state がさらに分かれてから。
- `crates/hakorune_mir_builder/src/binding_context.rs`: 変数名 ↔ BindingId の対応。
- `crates/hakorune_mir_builder/src/variable_context.rs`: 変数解決（variable_map 等）。
- `crates/hakorune_mir_builder/src/metadata_context.rs`: span / source_hint / region の観測。
- `src/mir/builder/compilation_context.rs`: Box / 型レジストリと reserved ids。AST node / function-slot / type-registry state が残るため packaging は保留中。
- `crates/hakorune_mir_builder/src/context.rs`: 上記 Context を束ねる入れ物。

## 追加ルール（将来の変更者向け）

- 新しい状態を追加する場合は、まず「どの Context の責務か」を決めてから追加する（`MirBuilder` 直下に増やさない）。
- 新しい control-flow shape / CorePlan rule は builder core ではなく
  FlowPlanner row として扱う。builder から route-specific plan internals を
  直接 import しない。
- 変更後に最低限確認する:
  - `tools/smokes/v2/profiles/integration/apps/phase135_trim_mir_verify.sh`（MIR verify の回帰防止）

## P5 Crate Split Prep

`src/mir` の crate split を準備するとき、この subtree は `hakorune-mir-builder` 候補になる。
The first packaging slice has already landed in `crates/hakorune_mir_builder/`
with `core_context.rs`, `context.rs`, `binding_context.rs`, `type_context.rs`,
`variable_context.rs`, and `metadata_context.rs`; the remaining builder
orchestration stays here for now.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- public entry は `stmts.rs` / `exprs.rs` / `control_flow/` の入口に寄せる
- helper を増やす前に、Context の責務境界を README に書く
- split は docs-first で境界が固定されてから行う
- member call は「route selection を 1 回、emit を 1 回」の順に保つ。
  static receiver / env method / this-me normalization は `calls/*` の classifier
  helper で決め、`build.rs` から重複判定しない
