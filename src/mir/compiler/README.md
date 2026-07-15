# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

## Typed ingress contract

- `LegacyModuleLoweringInputV1` owns a bare AST plus an explicit legacy origin.
- `ResolvedModuleLoweringInputV1` can only borrow an opaque
  `VerifiedResolvedSourceUnitV1`.
- The verified unit is canonical syntax plus its sealed semantic owner forest;
  it is not a rewritten or resolved AST.
- A private request enum is matched once in `MirCompiler` and never reaches
  recursive Lower.
- Canonical failure never retries through the legacy route.
- Production ingress is explicit by owner family. `resolve_function` owns the
  body-only family; `resolve_function_with_root_callable` owns the exact P0c
  current-owner self-call family. Both own one AST and co-seal its forest and
  exact source projection. Neither retries through the other.

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
physical AST-field projection is implemented once in `source_projection.rs`;
recursive Lower must consume located carriers instead of rebuilding paths.

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

## P0c-I1 exact direct-call ingress

`resolve_function_with_root_callable` first seals the root callable header and
one-entry `VerifiedCallableIndexV1`, then resolves the body against that same
index before any Builder effect. Production preflight accepts exactly one
current-owner `FunctionCall` with exact `i64` arguments/result. A co-sealed
direct-call row, not the raw call name, is the only Lower input.

The materializer emits one ordinary call-result `ValueId`, one conservative
barrier call, and one explicit VM-only direct-call capability. It performs no
legacy call resolution, name heuristic, fallback, or ownership operation.
Sibling calls and a multi-entry callable catalog remain outside this ingress.

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
