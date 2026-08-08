# MIR Builder (`src/mir/builder/`)

Pointers:
- final production pipeline north star:
  - `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
- active in-place replacement policy:
  - `docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md`
- active replacement task map:
  - `docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md`
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- parked clean-architecture consolidation task:
  - `docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md`
- current selfhost bootstrap authority:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- MIR navigation root:
  - `src/mir/README.md`

このディレクトリは Rust 側の MIR 生成（AST → canonical MIR emission）を担う。
`control_flow/plan` と JoinIR merge は物理的にはここにあるが、builder core
ではなく FlowPlanner / JoinIR glue として読む。

## Active replacement law

This directory remains the one live production MirBuilder. Do not build an
independent second Builder beside it.

The final authority flow is:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
-> Lower -> Seal -> Collect -> Atomic Publish
```

This is a responsibility flow, not a requirement to create one Rust file or
type per box. A replacement cell is useful only when it removes a competing
production authority and moves the live graph toward that flow.

For each cleanup cell:

```text
extract one responsibility
-> switch its existing production caller
-> delete the selected old branch/symbol
-> prove parity after the switch
```

Disconnected S0 code may survive at most one landed commit before its I0/R0.
An internal candidate connection with production callers at zero is not I0.
Stage-B-specific source routes must not be connected here; only their
source-neutral reusable parts may enter a named production replacement cell.

### Ordered Box-method compatibility edge (R5-S1)

The deferred non-Main static-Box Program path consumes the AST-owned
`BoxMethodInventoryV1` directly. It does not round-trip through
`HashMap<String, ASTNode>` or reconstruct an inventory at the Builder edge.
The historical alpha-before-beta execution order is retained only by the
explicitly named `into_compatibility_name_order()` projection in the
compatibility batch. This projection is not source-order authority and is
scheduled for removal only after its production callers reach zero.

### Ordered Box-method compatibility edge (R5-S2)

The connected static-`Main` compatibility child-port family now carries
`BoxMethodInventoryV1` directly through `RawBoxMethodChildPortV1` and its
normal/raw forwarding implementations. The compatibility leaf receives the
inventory without a legacy-map roundtrip and retains
`declaration_order::sorted_method_entries` only for the historical
helper-before-main execution order. Nested static `Main` remains a root-only
rejection before root effects. This edge does not promote name order to source
order or open resolver authority; the remaining legacy projections stay
explicitly parked until their own caller-zero receipts exist.

### Ordered Box-method compatibility closeout (R5-S3)

The production Builder census is now caller-zero for inventory-to-map
roundtrips. Remaining `sorted_method_entries` users are explicit compatibility
views for stable method-slot, lowering, scalar-fact, and callable-catalog
ordering; they are not source-order reconstruction and remain outside this
transport cleanup. Runtime `CoreBoxDecl` projection, legacy JSON, and test
fixture maps remain separately classified.

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
- `record_values.rs` owns record preflight, exactly-once field evaluation, and
  `RecordFieldContractCheck` / `RecordValuePublish` emission. Declaration and
  schema policy stays in `mir/type_contracts/record_value.rs`; VM, JSON, and
  backend capability policy must not move into the builder.

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

## Callable source ingress (current frontier)

`normal_callable_semantic_source.rs` owns the selected normal-callable source
loan. The S0 receipt retains the exact resolver ledger and
`ResolvedFunctionLoweringInputV1`; the S1 `PreparedCallableLoopIngressV1`
consumes that receipt together with one existing logical callable Loop product
and checks owner/origin/Loop frame/scope identity before any Builder effect.
This is still a Builder-free ingress: it does not allocate physical IDs, open a
session, emit MIR, select a route, or provide fallback. Full-demand preflight
is now closed by `normal_callable_prepared_operation.rs`. That assembler
consumes the ingress exactly once, issues the existing neutral operation/effect
demand, and calls `prepare_all` for the complete Recipe-order schedule. The
result retains only the callable source/input/Prelude/Tail transport while the
common program owns operation/effect/continuation meaning. It creates no
Builder/session effect or physical ID; the next bounded row is the caller-zero
full physical canary.

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
- recursive child-lowering boundary
  - `src/mir/builder/recursive_child_lowering.rs` owns one associated-input
    port across body, statement, and expression entries.
  - E0 selects one fresh raw port synchronously at each legacy facade. The
    port is never stored in `MirBuilder`, shared, cloned, or retried.
  - Existing helper recursion remains an explicit raw leaf. Located inputs,
    caller ledgers, MethodCall route splitting, and result publication remain
    disconnected until their later SITE0-R0 rows.
  - `stmts/variable_assignment_completion.rs` is the source-neutral receipt
    sibling for the existing `build_assignment_from_value` authority. It calls
    that authority once and retains the exact target, RHS, and returned carrier
    without reading `variable_map` afterward.
  - `calls/method_call_terminal.rs` owns one source-neutral receipt-required
    static/global sibling. It shares `PreparedGlobalValueCallRequestV1` with
    the ordinary terminal and delegates to the existing generic physical Call
    receipt authority; it does not classify source results or publish facts.
- legacy block descent boundary
  - `src/mir/builder/stmts/block_driver.rs` alone owns scope lifetime, the
    termination checks, last-value selection, and empty-block Void publication.
  - `LegacyBlockDescentPortV1` owns only statement count and exact
    one-statement lowering.  It has no suffix-view or optional routing
    capability; Loop routing belongs to the statement owner.
  - `block_stmt.rs` owns the selected `Vec<ASTNode>` port.  The driver may not
    import activation plans, caller ledgers, located carriers, or route policy.
- member call route selection / emission boundary
  - `src/mir/builder/calls/build.rs`
  - `src/mir/builder/calls/member_route.rs`
  - `src/mir/builder/calls/static_resolution.rs`
  - `src/mir/builder/calls/extern_calls.rs`
  - `src/mir/builder/calls/receiver_binding.rs`
- function-call preflight special gates
  - `src/mir/builder/calls/function_preflight.rs`
  - `src/mir/builder/calls/special_method_handlers.rs`
- function lowering transaction
  - `src/mir/builder/calls/function_session.rs` is the sole static/instance
    lifecycle owner: snapshot reentrant caller state, run one closure, restore,
    then publish the returned `MirFunction` draft. Existing
    `BoxCompilationContext` mode remains an explicit clear-only isolation
    policy rather than fabricated caller state.
  - `context_lifecycle.rs` owns that snapshot/isolation policy only. Call sites
    must not pair prepare/restore manually or pop FunctionRegion state.
  - error paths and panic unwinding restore the caller and publish no partial
    function. Explicit cleanup reports imbalances; Drop is only the panic
    backstop. B0-L2c itself is behavior-preserving; SA3-B reuses the same
    transaction through a separate resolved entry.
- resolved function lowering
  - `src/mir/builder/resolved_lowering/README.md` defines the first closed
    canonical family.
  - recursive lowering consumes exact located carriers and owns a
    `BindingRefV1 -> ValueId` environment. It never calls legacy AST dispatch
    for declaration, variable use, or assignment.
  - `vars/resolved_binding_state.rs` is only the structural veto gate for the
    legacy BindingId allocator; exact identity and coverage live under
    `resolved_lowering/`.
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
- function-session state は module truth / function-owned / observation / legacy compatibility のどれか一つに分類する。未分類の状態を snapshot/restore surface へ追加しない。
- 同じ semantic operation に completion policy を増やさない。既存入口が複数なら、入口別の修正ではなく共通 completion owner の task を開く。
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

## Loop PHI observer boundary (M6-B)

`LoopPhiMaterializerV1` under `control_flow/plan` is a caller-zero mechanical
observer, not a second Builder or production PHI/SSA owner. It consumes only a
verified Loop JoinSig and a sealed logical-to-physical edge/path map, then uses
the existing `PhiTxn` lifecycle. It must not read AST/routes/CorePlan,
recompute CFG, touch `variable_map`, infer Binding SSA, or add Retry/fallback.
Canonical CFG plus one function-owned Binding SSA remains the production
physicalization owner. The focused M6-B suite is 33/33 and the structural guard
is green; the structural P1b edge-path task is closed. The bounded resolved
DirectAccum bridge now seals `After` before reading carrier keys 0/1 through
`CanonicalDirectAccumBindingPort`, hands an owned
`DirectAccumFinalBindingReceiptV1` to the candidate helper, and then finishes
the existing Binding-SSA/PhiTxn lifecycle. The P4-S1 immutable candidate
snapshot is green for this singleton; it does not synthesize After PHIs or
become a second PHI/SSA owner. All-route physical parity remains separate.
