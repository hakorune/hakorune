# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

## Typed ingress contract

- `LegacyModuleLoweringInputV1` owns a bare AST plus an explicit legacy origin.
- `LegacyWholeSourceCompileRequestV1` is the disconnected pre-selection owner
  for that input plus exactly one non-Clone
  `CompilerSuppliedStaticImportSnapshotV1`.
- The import snapshot distinguishes `None` from an explicitly supplied,
  sorted/deduplicated table. It can seal only a borrowed same-catalog alias
  view; it has no Builder installation or ambient lookup in the request row.
- `ResolvedModuleLoweringInputV1` can only borrow an opaque
  `VerifiedResolvedSourceUnitV1`.
- The verified unit is canonical syntax plus its sealed semantic owner forest;
  it is not a rewritten or resolved AST.
- A private request enum is matched once in `MirCompiler` and never reaches
  recursive Lower.
- Canonical failure never retries through the legacy route.
- The Stage-B request types have no production constructor caller until the
  later source-selection row. Existing Legacy alias mutation and route
  behavior remain unchanged meanwhile.
- `PreloopStageBWholeSourceProducerV1` is the disconnected sole owner of the
  bounded source-selection policy. Compatibility origins become explicit
  `Ordinary(ProfileExcluded)` before proof work; complete candidate
  cardinality becomes `Ordinary`, `Selected`, or a retained ambiguity
  rejection.
- Selection uses the same `seal_root` declaration surface as existing Builder
  lowering. One candidate is immediately co-sealed with its exact boxed
  catalog; prepared rows and construction-only catalog identity do not cross
  the carrier boundary.
- This selection row still has no production caller, Builder mutation,
  catalog/import installation, fallback, or retry.
- Production ingress is explicit by owner family. `resolve_function` owns the
  call-disabled body-only family. `VerifiedResolvedCallableProgramV1` owns all
  exact callable modules, including singleton self recursion. Neither retries
  through the other.

Exact child-site navigation belongs to B0-L2b. Function transaction cleanup
belongs to B0-L2c. BindingId adoption and production semantic activation belong
to atomic SA3-B.

## B0-L2b source projection boundary

`VerifiedSourceProjectionV1` is sealed beside the canonical syntax and owner
forest. It contains only structural owner locators; it never stores AST
pointers, Spans, names, traversal ordinals, or cloned nodes. A
`FunctionSourceViewV1` borrows one exact function/Lambda root through that
projection and is the only factory for `LocatedBodyV1`, `LocatedStmtV1`, and
`LocatedExprV1`.

Child navigation is parent-relative and immutable. Closed child-role enums
select an AST field and the existing `SourcePathSegmentV1` together. The
physical AST-field projection is implemented once in
`resolved_semantics/source_projection.rs`; compiler `source_projection.rs` is
only a thin typed-error consumer. Recursive Lower must consume located
carriers instead of rebuilding paths.

B0-L2b landed source views as disconnected transport. SA3-B now has exactly
one production consumer under `builder/resolved_lowering/`; Planner suffix
transport remains disconnected.

Forbidden identity sources are AST pointer, Span, name, traversal order,
producer path, and ProgramV0 reconstruction.

## B0-L4-S2 located suffix range boundary

`FunctionSourceViewV1` is also the sole factory and navigator for typed
`ConsumedSourceRangeV1` values. A range is owner/body/start exact, has a
`NonZeroU32` count, and is bounded against the borrowed canonical body before
publication. Suffix first/range/advance operations use checked `usize -> u32`
and checked end arithmetic. Empty, foreign, out-of-bounds, and body/start
mismatches (including a gap, overlap, or already-advanced suffix) fail with
typed navigation errors.

The compiler layer owns only exact syntax transport. It does not infer Loop
coverage, plan membership, effects, or MIR identity. The generic structural
coverage schema in `resolved_control_flow` verifies these sealed ranges without
reimplementing source-path navigation. S2′ leaves that schema disconnected from
all production control lowering.

## SA3-B first-family activation

`CanonicalLoweringPreflightV1` accepts exactly one non-main static/free
function owner with a straight-line closed grammar. It runs before a candidate
Builder is created. `CanonicalModuleLoweringSessionV1` discards that candidate
on any error and commits it only after compiler post-processing succeeds.

The function input is derived only from the verified unit. Recursive Lower is
owned by `CanonicalFunctionLowererV1` and receives only located carriers. Its
value environment is `BindingRefV1 -> ValueId`; names are diagnostic
cross-checks and never lookup keys. Declaration adoption, variable-use sites,
assignment-target sites, and Return sites must all finish coverage before the
unpublished draft may commit. The Builder's legacy BindingId allocator is
fallibly vetoed for the entire installed-owner interval.

The first capability also admits straight-line lexical BlockExpr. Each
expression consumes its exact sealed scope/region pair, retires only inner
BindingRefs, and returns the tail ValueId after balanced leave. The default
bare-AST route, ProgramV0, REPL, Main, instance methods, Lambda,
If/Loop/CorePlan, and Planner remain outside it. Canonical failure never
retries legacy.

## P0c callable Program ingress

`VerifiedResolvedCallableProgramV1` owns the function-only Program, complete
immutable callable catalog, canonical-keyed function map, and one single-root
forest/projection per declaration. The same authority handles singleton self
calls, sibling calls, acyclic graphs, and recursive SCCs. Finite direct-call
rows are resolved and sealed before Builder effects; raw call names never reach
Lower.

The materializer emits ordinary call-result `ValueId`s and conservative
barrier calls. Each calling function receives one VM-only direct-call
capability; recursive topology additionally receives one module capability.
There is no legacy callable resolver, one-entry facade, exact-one policy,
fallback, or ownership operation. The ordinary `compile_resolved` ingress
remains explicitly call-disabled.

## MP0-S0/R0 resolved callable module

`VerifiedResolvedCallableModuleV1` is the disconnected multi-function carrier.
It owns the exact CAT0 Program/catalog source unit once and indexes
`VerifiedResolvedFunctionUnitV1` rows only by `CanonicalCallableKeyV1`. Each
function row keeps its declaration site, single-root semantic owner forest,
and exact source projection together.

MP0-S0 provides only the carrier shape. MP0-R0 adds its sole constructor. That
constructor consumes the same CAT0 resolver continuation, reuses every
pre-reserved top-level owner, resolves self/forward/backward direct-call targets
against the complete immutable catalog, issues nested Lambda owners from the
same compilation brand, and seals one exact source projection per function.

The body-reading source view consumes the CAT0 source unit and keeps Program
syntax and catalog inseparable until resolution finishes. MP0-R0 creates no
Builder, MIR draft, backend capability, runtime effect, or module publication;
whole-module preflight and publication remain MP0-P0/TX0 responsibilities.

## MP0-P0/TX0 module transaction

`VerifiedCallableModulePreflightV1` seals one canonical-keyed plan for every
resolved top-level function before any Builder effect. MP0-TX0 then lowers each
plan through a restored function session which returns an unpublished
`MirFunction`. The complete draft set checks catalog key, physical symbol,
signature arity, and cardinality before one atomic candidate-module insertion.

No successful earlier function is visible while a later function is lowering.
Any lowering, verification, identity, or publication failure returns without a
partially published callable set.

## P0c-B1 exact sibling-call ingress

`VerifiedResolvedCallableProgramV1` owns the complete exact Program through
catalog and body resolution. Its borrowed lowering input is accepted only when
the module has exactly two static exact-`i64` functions and exactly one direct
call edge whose target differs from its caller. Zero calls, self-only calls,
multiple edges, mutual recursion, and wider function cardinality reject before
the candidate Builder session opens.

The caller header travels with each `ResolvedFunctionLoweringInputV1`; the
co-sealed direct-call row separately owns the target header. Lower checks the
current draft against the caller header and materializes the target only from
the verified row. It performs no raw-name or module-table lookup. All drafts
still publish through the MP0 atomic batch, and the first executable backend
remains the Rust MIR interpreter.

## P0c-F-DX0a/DX0b finite direct-call substrate

`verify_function_with_finite_direct_calls_v1` is a disconnected preflight
facade for one-or-more exact direct-call sites, including nested calls in
argument position. It reuses the existing function-owned Binding SSA and
co-sealed direct-call rows; it creates no second call, ABI, or value authority.

The production `verify_function` entry remains on its exact-one admission law.
No module ingress calls the finite facade through DX0a/DX0b.

DX0b derives capability need once from the sealed profile before expression
lowering. A call-free function receives zero rows; a calling function receives
exactly one. Each call emitter verifies that exact V1 row before instruction
emission and never mutates metadata. The capability type owns the only row
installation facade; missing, duplicate, preexisting, or schema-drifted rows
fail explicitly.

## P0c-MR-G0 shared topology inventory and P0c-F DAG proof

`VerifiedCallableGraphInventoryV1` is derived only from the complete resolved
callable module. It projects already-resolved callable identities through the
catalog reverse index once, preserves every function-relative call site, and
collapses only caller/target topology edges. The sealed inventory is non-Clone
and owns no acyclic/SCC proof, source-name lookup, call ABI,
argument/evaluation order, effect, symbol, MIR, draft, publication, or backend
policy.

`VerifiedAcyclicCallableGraphV1` consumes and retains that inventory by value.
It adds only self-edge rejection and a deterministic canonical-key Kahn
witness. G0 preserves the existing P0c-F behavior exactly and adds no SCC
production consumer or recursion activation.

## P0c-MR-S0 disconnected SCC partition

`VerifiedCallableSccPartitionV1` consumes one graph inventory by value and
seals SCC membership, minimum-canonical-key component IDs, recursion class,
and the condensation DAG/order. Its deterministic Kosaraju implementation uses
explicit work stacks; declaration and discovery order never become identity.

S0 is disconnected. It adds no compiler ingress, call ABI/effect authority,
MIR, publication, backend capability, runtime discovery, or recursive
execution. Malformed private drafts verify membership completeness, stable IDs,
strong connectivity, and condensation acyclicity before a partition exists.

## P0c-MR-V0 disconnected recursive module plan

`VerifiedRecursiveCallableModulePlanV1` combines one verified SCC partition
with one existing finite trivial Binding-SSA plan per canonical callable key.
It admits only modules with at least two functions, at least one call site, and
at least one recursive component, then seals exact inventory/function/SCC
membership/typed-plan cardinality and per-function call-row correspondence.

V0 remains disconnected. It adds no compiler ingress, MIR, publication,
backend capability, runtime recursion, effect precision, termination claim, or
ownership operation.

## P0c-MR-C0 passive recursive backend capability

`CanonicalRecursiveCallableModuleCapabilityV1` is one module-level schema
marker stored as an `Option` in `ModuleMetadata`. The shared backend preflight
accepts the exact marker only for `mir-interpreter` and rejects every other
backend with a stable no-fallback tag.

C0 has no production marker producer. Missing, duplicate installation, and
schema drift are exercised only by synthetic fixtures; graph/SCC scanning never
infers the marker.

## P0c-MR-I1 explicit recursive module activation

`MirCompiler::compile_resolved_recursive_callable_module` is the sole explicit
recursive ingress. It consumes the V0 typed plan, lowers every function into an
unpublished draft, atomically inserts the complete draft set, installs exactly
one module marker, and commits the isolated candidate only after canonical
finish succeeds.

The ingress never probes the acyclic, one-function self-call, or legacy routes.
All call effects remain conservative, only `mir-interpreter` is admitted, and
the selected route emits no ownership operations.

MR-specific failure fixtures also prove that call-depth overflow and inner
parameter/return contract failures restore the caller frame and leave the same
interpreter reusable. The Rust reference interpreter's call-depth guard is a
host-stack-safe resource boundary, not a language recursion or termination
contract. Its diagnostic is optional when Ring0 is absent and must not
initialize global runtime state. Supporting materially deeper recursion in the
reference interpreter requires a separate iterative call-frame design.

## P0c-F-V0 typed acyclic module plan

`VerifiedAcyclicCallableModulePlanV1` is the disconnected pre-Builder witness
that combines the S0 topology proof with the existing finite direct-call
function preflight. It stores only a canonical-keyed map of
`CanonicalTrivialBindingSsaPlanV1` rows.

The seal requires at least two functions and one resolved call site, exact
graph/function/plan key and cardinality correspondence, and equality between
each function's graph-site count and its verified direct-call profile rows.
It does not own Builder, MIR, callable symbols, effects, draft publication, or
backend activation. Production callers remain zero through V0.

## P0c-F-I1 atomic acyclic-module ingress

`compile_resolved_callable_module` consumes
`VerifiedAcyclicCallableModulePlanV1` directly. The V0 typed plan is the sole
activation witness: the compiler does not re-match a generic function-plan
enum after Builder effects, and the retired exact-two-function B1 activation
witness is no longer a parallel authority.

The transaction lowers every canonical-keyed typed function plan to an
unpublished draft, verifies the complete draft set, and performs one atomic
module insertion. Repeated calls, nested/argument-position calls, multiple
targets, multi-hop DAGs, and calls in both fallthrough If arms use the same
co-sealed direct-call rows and exact-once consumption ledger. A calling
function owns exactly one VM-only direct-call capability row regardless of its
number of call sites.

P0c-F-I1 adds no ownership operation, raw-name lookup, fallback, incremental
publication, or backend widening. Self edges, mutual recursion, and SCC
authority remain rejected. The next callable task is the behavior-neutral
P0c-MR-G0 inventory extraction.
