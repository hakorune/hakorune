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

### Raw invocation source transport classifier

`raw_invocation_source_transport.rs` owns exact path construction and temporal
source scopes. Its private
`raw_invocation_source_statement_classification.rs` child owns only the finite
statement located/compatibility classification. The split is behavior-neutral:
bare `FunctionCall` and `MethodCall` remain `CallObject` compatibility rows.
Future exact-site activation must change the classifier child explicitly and
must not reintroduce AST classification into the transport owner.

### Source-lineage witness for unlocated calls (P0)

When a located invocation is demoted to `UnlocatedCompatibility`,
`RawInvocationSourceContextV1` preserves an optional `expected_lineage` witness
from the same `RawInvocationRootLineageV1`. A source-backed context therefore
cannot silently become the compatibility `Unavailable` state after its exact
node site is lost; the later publication ingress must freeze it as source-loss
`Error`. A genuinely compatibility/test-created unlocated context carries no
witness and remains outside the source-bound publication contract. This carrier
does not issue a target, Recipe, Join, Call receipt, or publication, and it does
not widen MethodCall source admission.

### Exact callable bare-call location (P0)

For an installed, source-backed callable root (`Cataloged`, `TopLevel`, or
`InstanceConstructor`), a bare `FunctionCall` statement is carried as the
resolver-issued `FunctionBody -> Body(i)` site, and its argument descent uses
the existing `CallArgument(n)` path. This is transport only: the statement
classifier does not issue a Brand relation and the later consumer remains the
owner of missing/foreign/site-drift rejection. `MethodCall`, indirect `Call`,
explicit extern calls, raw `Main`/`ScriptRoot`, nested compatibility, and other
unlocated rows remain `CallObject`; no name, span, or lineage fallback is
introduced.

### Installed callable Brand consumer (I0)

Installed, source-backed callable roots now ask the resolver-issued callable
ledger for a `Constructor|NonBrand` disposition at the transported
`SourceNodeSiteV1` before direct `FunctionCall` preflight. The private query
port validates the call name and `CallArgument(0)` site, and the exact
constructor lowers its operand under the existing argument source scope. An
exact `NonBrand` route never re-probes `CompilationContext::is_brand_declared`;
it proceeds through the existing TypeOp, Math, FastMem, and ordinary routes.
Relationless Compatibility, Deferred, RawLegacy, nested/Main, and other
unlocated paths deliberately retain their compatibility behavior and are not
treated as exact consumers. MethodCall/unwrap, global legacy-map retirement,
and Script semantic consumption remain separate rows.

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

### Instance-constructor physical source transfer (P0)

The parser-issued `ConstructorSourceIdV1` carried by the installed normal
callable semantic package is the sole source identity for selected-normal
instance constructors. `VerifiedInstanceConstructorPhysicalSourceCohortV1`
validates the final Program Box ordinal/name/key against those package rows
before physical work is prepared. Immediate work and the Script-runtime
duplicate demand both carry that same opaque source ID; each demand still
creates its own physical admission. Sorted constructor-map keys and
`(statement, box, key)` coordinates are placement checks only, and the legacy
`CompilationContext::is_brand_declared` consumer remains intentionally
unchanged until the later constructor-consumer cutover.

### Instance-constructor physical demand manifest (I0)

The selected-normal work plan now issues an explicit role ticket for every
physical constructor demand. `ImmediateDeclaration` is required once for each
source row; non-app Script `Prefix` and `FullLifecycle` rows receive their
matching runtime ticket, while app and compatibility work receive no runtime
ticket. The manifest validates the complete immediate/runtime ticket set before
Builder effects, rejecting duplicate, foreign, swapped, or missing roles. It
does not select a constructor or replace the later semantic package loan; the
raw Brand consumer remains parked for that separate cutover.

### Instance-constructor semantic loan consumer (I0)

Selected-normal constructor work now moves one non-Clone demand ticket through
the capture surface. The installed semantic package loans the matching
constructor forest by `ConstructorSourceIdV1` only, and the adapter installs a
request-local callable semantic scope around the existing raw body lowering.
Immediate and permitted Script-runtime roles may borrow the same immutable
forest, but each physical ticket is consumed exactly once and completion checks
manifest exhaustion. Compatibility, RawLegacy, bare/unlocated calls, and the
legacy `is_brand_declared` route remain outside this consumer row.

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
- normal-program declaration facts own the same neutral, AST-free Brand catalog
  consumed by Stage1. Duplicate effective Brand names reject before resolver or
  Builder effects; `CompilationContext.brand_decls` is only a temporary
  catalog-derived compatibility cache for the remaining raw call consumer.
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

## Current selected-Dynamic frontier (2026-08-11)

The selected `ParserScanLoopBox.skip_while/4` replacement is still
pre-cutover.  The target production path is:

```text
installed package loan
  -> A-prime exact-i64 physical demand
  -> fresh canonical function session
  -> site-keyed Completion claims
  -> DraftSeal prepare / Collector / atomic publish
```

The A-prime demand is currently Builder-free and has no production caller by
design.  The live production path remains the explicit migration edge below
(`source seed -> raw AST descent -> old JoinIR route`) until the named
cutover commit.  Do not add a second source observer, route planner, type
repair, fallback, or retry to bridge this gap.

### Generic G0 physical-entry adoption and retirement (2026-08-17)

The Generic entry facts are now consumed by the combined emitter admission and
the caller-zero `generic_g0_physical_emitter_session` preflight.  The session
transaction remains the sole owner of shell installation, lane adoption, and
discard; the common dispatcher is only borrowed after canonical entry/segment
receipts are ready.  The former detached skeleton, canary admission, and
entry-session modules were removed by `GENERIC-G0-ENTRY-CANARY-RETIREMENT-R0`.
No Generic Builder publication, Completion claim, CFG/PHI, lifecycle/Text,
route, fallback/retry, or production caller is implied by the caller-zero
probe.

## Legacy callable source ingress (pre-cutover edge)

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
common program owns operation/effect/continuation meaning. The semantic parent
now hands the assembler one source-free prepared-demand parent through a
one-shot consume method; no six-element tuple or independently re-pairable
semantic rows cross the compiler/Builder seam. It creates no Builder/session
effect or physical ID; the next bounded row is the caller-zero full physical
canary.

`normal_callable_dynamic_source.rs` owns the source-only co-seal for deliberate
untyped/dynamic callables. It combines the exact function root,
`VerifiedSourceProjectionV1`, and matching resolver forest once, then emits one
non-`Clone`, AST-free aggregate containing complete untyped-formal coverage and
exact formal-to-local-to-Loop-carrier relations. It never derives Dynamic from
`MirType::Unknown`, result requirements, names, or Builder state.

`normal_callable_dynamic_origin.rs` is the private P0 physical projection. The
normal callable loan hands the exact resolved input to one scoped lowering
state; the existing post-`setup_function_params` entry receipt installs formal
origins, and the local terminal's ordinal-keyed initializer/local receipt
propagates only exact source-authorized copies. Rebind invalidates the active
origin. This state is not a type owner, parameter publisher, GenericLoop/PHI
authority, or fallback route.

`normal_callable_loop_handoff.rs` owns the L0-R0 source-coverage projection.
It no longer treats `(condition reads, body reads, rebinds) = (1,1,1)` as a
semantic contract or discards exact relations into a count receipt. One
non-`Clone` schedule groups exact source sites by resolver `BindingRefV1` and
classifies the supported window as one pre-Loop carrier, variable read-only
operands, and Loop-declared iteration locals. The production
`ParserScanLoopBox.skip_while/4` source is the positive fixture: `i` alone is
the carrier, while `end`, `src`, and `pred_chars` are operands and `ch` is
iteration-local. This remains source-only; Dynamic operation results,
prepared representation, PHI, backend metadata, route selection, retry, and
fallback remain closed.

`normal_callable_dynamic_operation_source.rs` owns the next source-only S0
co-seal. It combines the existing resolver ledger, source-backed Dynamic
callable product, and R0 binding schedule to issue one move-only exact
comparison/Add-rebind relation set. The production `skip_while/4` comparison
has an explicit Bool result, while its Add-by-exact-I64 result remains Dynamic
and targets the same carrier binding. The module does not import `MirType` or
`ValueId`, mutate Builder state, relabel I64 Recipe operations, or claim the
method-call/local/Return portions of the Loop.

`normal_callable_dynamic_loop_prepare.rs` owns the P0 pre-effect co-seal. It
consumes the exact Loop schedule and operation-source set, borrows the
existing current-origin owner, and returns one move-only prepared ingress.
Carrier and read-only entry bindings carry existing `ValueId`s plus opaque
source-backed Dynamic representation receipts; iteration locals wait for
their own materialization. The R0 extension also retains the selected
carrier's exact local declaration site, completed initializer/local ValueIds,
and Dynamic formal origin as one prepared Enter-definition row. This preserves
the existing local terminal's relation for later canonical SSA adoption; it
does not publish a value or declaration itself. The carrier also retains exact
Enter/Backedge expectations. The issuer has no Builder/CFG handle, no raw
Unknown representation arm, and no operation, PHI, backend, retry, fallback,
or route authority.

The P1 compiler-acceptance prerequisite is conservative at both common type
owners: ordinary Add completion and final BinOp re-propagation require exact
Integer evidence on both operands before publishing an Integer result.
`Unknown + Integer` stays physically unknown. A source-backed Dynamic result
is authorized only by the prepared operation relation; raw Unknown never
becomes Dynamic, and production `.hako` source is not rewritten to fit the
old inference shortcut.

`normal_callable_dynamic_loop_rebind.rs` is the private exact-once P1
operation/rebind canary. It consumes the complete prepared ingress, delegates
Compare/Const/Add insertion to existing writers, and uses a prepare plus
infallible commit to advance its bounded callable-current and Dynamic-origin
projections together. It does not yet prove canonical repeated-Loop or After
reaching-value semantics: the landed canary uses Enter as the operation input.
The next design must open the canonical Header current before operation
emission, correct this terminal to consume it, and close the PHI afterward
through the one canonical CFG/Binding SSA/`PhiTxn` session. Its move-only
output contains no second emitter, predecessor authority, PHI destination,
retry/fallback, or production route.

`dynamic_loop_phi.rs` owns the migration-private P2A Header-current opening
boundary. One `CanonicalSsaFunctionSessionV2` adopts the already-emitted local
Enter value through its exact resolver declaration site, allocates the bounded
Enter/Header/body/terminal-Backedge/After placement through canonical CFG,
and reads the still-unsealed Header through canonical Binding SSA. That read
creates the sole provisional PHI before Compare/Add emission. The opaque
result exposes no PHI token, raw predecessor vector, route-local writer, or
second SSA owner.

The corrected `normal_callable_dynamic_loop_rebind.rs` P1R terminal consumes
that whole open product. It derives its Header and terminal-Backedge blocks
from the sealed placement, emits both Compare and Add from the canonical
Header current, and retains the same open product with the source-backed
Dynamic Backedge receipt. It has no caller-chosen block/value seam and never
rewrites an emitted operand. It also does not consume the source assignment or
advance the legacy callable-current map. P2B must define the assignment once
through canonical Binding SSA, complete the real
Header-to-body-to-terminal-Backedge-to-Header path, and seal Enter, body,
terminal Backedge, then Header through canonical CFG. The Header witness is
exactly Enter plus terminal Backedge, and canonical Binding SSA uses it to
patch the existing provisional PHI to `(Enter, entry)` and `(Backedge, Add)`.
The resulting move-only close receipt contains no PHI token or predecessor
authority. Loop After, publication, DraftSeal, backend activation, retry, and
fallback remain closed.

`dynamic_loop_discard_tests.rs` closes the P2C atomicity proof without a
test branch in production code. Failures injected after open, after operation
emission, after duplicate canonical assignment definition, and after PHI
patch all discard the complete unpublished child and restore the exact caller
once. Two later fresh sessions reproduce the same semantic CFG/instruction
shape without comparing allocator IDs. Local PHI rollback is diagnostic
hygiene; whole-session discard remains the correctness owner.

The P2 series remains an unpublished-session carrier proof. It cannot be
used as an executable `skip_while/4` route: that method's calls, iteration
local, inner If/early return, Loop After, final return, Completion, DraftSeal,
and collector are not yet consumed. The direct VM canary is `NoSafeSlice`.
The compiler now issues the complete AST-free source inventory for the exact
unchanged method. It must next add the route-disjoint source-bound Dynamic
member target, then close V2 Dynamic
Recipe/call relations, JoinSig-authorized If/Return transfers, set-aware
multi-return Completion, and the full callable. The Builder must not infer a
nominal receiver from method spelling. `ch` is iteration-local, not a carrier;
source Completion already owns both Return sites. Production source must not
be narrowed or rewritten. The exact task order is in
`docs/development/current/main/investigations/generic-loop-dynamic-full-body-closure-d0-task-2026-08-10.md`.

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
    caller ledgers, and MethodCall route splitting retain their own bounded
    rows. Exact static result publication is connected only through the
    module-installed `VerifiedStaticCallResultPublicationOwnerV1`.
  - `stmts/variable_assignment_completion.rs` is the source-neutral receipt
    sibling for the existing `build_assignment_from_value` authority. It calls
    that authority once and retains the exact target, RHS, and returned carrier
    without reading `variable_map` afterward.
  - `calls/method_call_terminal.rs` owns one source-neutral receipt-required
    static/global sibling. It shares `PreparedGlobalValueCallRequestV1` with
    the ordinary terminal and delegates to the existing generic physical Call
    receipt authority; it does not classify source results or publish facts.
    The source-bound sibling disables the legacy signature annotation because
    `PreparedStaticCallResultPublicationV1` is the sole selected-row result
    publisher after physical success.
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

## Complete Script source continuation I0

`VerifiedScriptSourceContinuationV1` is the narrow source-only continuation
product for a Complete Script root. The resolver's existing shadow seal issues
the `VerifiedBodyShape` and the Script product retains that same owner/shape
pair; the continuation sibling validates the canonical demand window and
projects only already-issued parent relations and terminal statements.
Transparent, transferred, and diagnostic root entries are explicit boundaries
and do not receive guessed body rows. `VerifiedScriptSemanticLoweringInputV1`
transports this continuation together with the existing lowering projection and
direct-static Facts bundle. The source/Facts-only
`VerifiedScriptDirectStaticResultPublicationOwnerV1` co-seals the existing
target/representation rows with the resolver-issued continuation and keeps the
ScriptRoot owner distinct from the callable-only publication owner. It issues
no Recipe key, Join signature, ValueId, MIR type, result-publication ABI,
physical block, fallback, or production switch; Recipe and physical result
publication remain later design rows.

## Shared Script MethodCall typeop policy I0

`src/mir/policies/source_method_typeop_route.rs` is the sole pure predicate for
the source-shaped `is`/`as` type-operation route. `calls/special_handlers.rs`
and `calls/build.rs` consume it as thin Builder adapters, while the Script
direct-static target inventory consumes the same disposition before admitting
a target. A method with exactly one direct string (or the existing
`StringBox` string shape) is a typeop noncandidate; other `is`/`as` arguments
remain ordinary static-call candidates. This slice changes no parser/source
admission, Recipe/Join, physical bridge, fallback, or production switch.

## Script direct-static Recipe I0

`VerifiedScriptDirectStaticRecipeV1` is a dedicated Script Recipe producer for
the co-sealed `VerifiedScriptDirectStaticResultPublicationOwnerV1`. It issues
an opaque Recipe-local key and retains the owner/site, ordered argument,
target, result representation, and continuation relations without re-reading
AST or re-resolving names. The first accepted shape is deliberately narrow:
the call must be the value of the final Sequence statement or the value of the
root Return statement. Local initializers, assignments, print/discarded calls,
control-flow/nested owners, Deferred, Compatibility, and RawLegacy remain
outside this Recipe and are rejected rather than represented by an empty row.

The producer is separate from `RawScriptBodyRecipeV1` and the Loop Recipe
vocabulary. The Recipe is transported through the existing Complete Script
semantic source/lowering input/state only; no JoinSig, ValueId, MIR type,
physical block, route selection, raw retirement, or production switch is
performed in this I0. The focused guard and empty Complete Script fixture cover
the source/Facts boundary; non-empty physical consumption is a later row.

## Script direct-static Recipe-to-result handoff I0

`VerifiedScriptDirectStaticJoinHandoffV1` is the source/Facts bridge between
the dedicated Script Recipe and its already-issued result-publication owner.
It verifies source identity, owner, key/site, target, representation, ordered
arguments, terminal destination, and parent relations one-to-one, then carries
one immutable row per Recipe key through `VerifiedScriptSemanticLoweringInputV1`
and `ScriptSemanticLoweringState`. It does not create a JoinSig or physical
value/block, reclassify AST, infer a destination, or retire the raw route. Empty
Recipe/owner pairs remain explicitly empty; foreign, duplicate, missing, or
drifted rows fail before physical work. The focused handoff tests and reusable
Script direct-static guard own this boundary.

## Script direct-static claim carrier I0

`ScriptDirectStaticClaimLedgerV1` is an operational, scope-local carrier over
the already-issued Bundle and Join. It validates source identity, owner,
cardinality, and exact site coverage once while constructing
`ScriptSemanticLoweringState`; it does not issue a new semantic fact. A
`take(site)` returns an unchanged `Absent` only for a site that has never been
issued by the Bundle. A pending row becomes a non-`Clone` claimed Join row;
an in-flight or completed site is a fail-fast `DuplicateClaim`. The claimed
row can only be completed by consuming its token; there is no rollback,
reinsertion, retry, or name-based fallback. The token exposes only read-only
target/argument/representation views for the future physical consumer;
it does not re-issue semantic facts. `finish()` consumes the ledger and
rejects pending or in-flight rows. The future physical bridge owns invoking
that finish around a real Call consumer; this carrier-only I0 deliberately
does not fabricate a consumer merely to force exhaustion.

Compatibility, Deferred, and RawLegacy paths do not acquire this ledger. The
claim carrier emits no Call, ExactI64 publication, Return/signature, canonical
transport, performance evidence, or production switch. The focused ledger
tests and `script_direct_static_target_guard.sh` enforce the operational-only
boundary and the 760-line split trigger.

## Script direct-static claim ingress P0

`recursive_child_lowering_port.rs` now owns the small recursive-child contract
and its default, non-consuming `script_direct_static_claim_ingress_v1` hook;
the former 794-line lowering owner is now a 708-line implementation-only
module. The raw
invocation overrides the hook only for an active ScriptRoot semantic scope,
while `RawStructuredChildScopePortV1` delegates to its child and legacy/test
ports retain `Unavailable`. `StaticReceiver` probes this capability after
route selection and before receiver/argument descent, then continues through
the existing route unchanged. This BoxShape P0 emits no Call, publication,
Return/signature, fallback, retry, canonical transport, or production claim.
When a semantic ledger is installed, missing source context, unlocated
compatibility context, and foreign lineage are fail-fast contract errors before
child effects; only a ledger-free port may report `Unavailable`. A located
ScriptRoot with no matching row remains the sole `Absent` case and preserves
the existing static route. The ingress validates the row before moving it to
`in_flight`, so validation failure cannot require rollback or reinsert.

## Static-result publication ingress fail-fast P0

`static_result_publication_ingress.rs` is the pre-descent capability boundary
for the existing Cataloged static-result owner. It keeps four outcomes
separate: `Unavailable` means that a compatibility/test port has no
source-bound publication capability; `Absent` means an exact Cataloged site
and owner are present but no row was issued; `Selected` consumes one existing
publication handoff; and owner-backed source loss or drift is a typed freeze
error. A Cataloged `expected_lineage` demoted to `UnlocatedCompatibility` is
therefore never treated as ordinary `None`.

The `StaticReceiver` route head and lowered static `me` route invoke this
ingress before receiver/argument effects. A selected row reuses the existing
ordered argument driver, generic Call receipt emitter, and
`PreparedStaticCallResultPublicationV1`; no second target resolver, Call
emitter, publication owner, AST matcher, or late terminal hook exists. Only
the exact no-row `Absent` state may continue through the ordinary terminal.
Ledger-free Compatibility, Deferred, RawLegacy, and non-Cataloged roots remain
`Unavailable`; an owner-backed missing or foreign source is a typed freeze and
is never promoted by this P0. The reusable
`script_static_result_publication_ingress_guard.sh` pins the complete outcome
table and the no-fallback boundary.

## ME-call arity fail-fast P0

`method_call_handlers.rs` prepares the `me` route from the existing header
observation before any argument descent. A `LoweredGlobal` row compares the
header-owned parameter count with source arguments (plus the explicit `me`
receiver for instance calls); a strict mismatch returns the stable
`[freeze:contract][me-call/arity]` error before effects or Call emission.
Strictness is ON when `NYASH_ME_CALL_ARITY_STRICT` is unset; only an explicit
`=0` keeps the documented compatibility timing. Inline, standard, fallback, and
missing-header routes are not reclassified. The finite state table and guard
are in `me-call-arity-failfast-d0-2026-08-21.md` and
`tools/checks/me_call_arity_failfast_guard.sh`.

## Script direct-static physical bridge I0

The selected-normal ScriptRoot bridge now consumes the claimed
`ScriptDirectStaticClaimedRowV1` only at the `StaticReceiver` route head, after
typeop/reserved routing and before receiver or argument effects. `Absent` keeps
the existing static route unchanged; a matching row is an atomic handoff whose
canonical target and ordered argument sites come from the Join row, not from
AST names or ordinals. The bridge reuses the existing ordered argument driver
once and accepts only `CompletedUnifiedValueCallEmissionV1` from the existing
receipt-required generic Call terminal.

`PreparedScriptDirectStaticResultPublicationV1` is a Script-only physical
sibling: it accepts the already-issued `ExactI64` representation and publishes
`MirType::Integer` to the receipt destination exactly once. It does not reuse
the callable publication owner, emit Return/signature, or infer completion from
`ValueId`/`MirType`. The claim is completed only after Call and publication
succeed; the enclosing semantic scope finishes once, and any later error
discards the isolated candidate without rollback, retry, or ordinary fallback.

This I0 is a bounded selected-normal BoxShape only. Compatibility, Deferred,
RawLegacy, `StaticThis`, canonical Script transport, raw retirement, and
production cutover remain explicitly closed. The physical bridge and its
publication sibling stay below the 760-line split trigger; the reusable
`script_direct_static_target_guard.sh` checks the single receipt/publication
path and the no-fallback boundary.

## Script direct-static canonical physical input I0

`VerifiedScriptDirectStaticPhysicalInputV1` is the narrow AST-free input for a
future canonical Script physical consumer. It is composed only from the
already-issued Join rows and a resolver-owned scalar operand Recipe; it does
not re-open source, infer a target from names/ordinals, allocate physical
identities, or publish a second semantic fact. Unsupported literals,
operators, calls, variables, fields, comparisons, and typed payloads fail
before physical work.

`direct_static_entry_kernel.rs` is a detached helper only. It lowers the
ordered scalar trees, invokes the existing receipt-required generic Call
emitter exactly once, projects the already-sealed `ExactI64` result through
the Script publication sibling, and hands the value terminal to
`OpenScriptPhysicalEntrySessionV1::complete_lowered_terminal_v1`. The session
remains the sole candidate, verifier, Return/signature, and finish owner.

This I0 is not connected to source admission or production routing. The
selected-normal bridge, canonical Script transport, compatibility/Deferred/
RawLegacy paths, raw retirement, and performance evidence remain separate
rows. The focused physical-input and detached-session tests plus
`script_direct_static_canonical_physical_input_guard.sh` own this boundary;
`builder.rs` remains pre-existing migration debt and is not grown.

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
