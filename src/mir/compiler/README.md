# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

## Typed ingress contract

- `LegacyModuleLoweringInputV1` is a crate-internal Raw lifecycle carrier. It
  owns syntax only and is not a public `MirCompiler` admission authority.
- `ResolvedModuleLoweringInputV1` can only borrow an opaque
  `VerifiedResolvedSourceUnitV1`.
- The verified unit is canonical syntax plus its sealed semantic owner forest;
  it is not a rewritten or resolved AST.
- Public `compile*` methods seal a whole-file `Program` once and enter the
  typed normal lifecycle. Non-Program input fails before Builder effects.
- Canonical failure never retries through the legacy route.
- Production ingress is explicit by owner family. `resolve_function` owns the
  call-disabled body-only family. `VerifiedResolvedCallableProgramV1` owns all
  exact callable modules, including singleton self recursion. Neither retries
  through the other.

Exact child-site navigation belongs to B0-L2b. Function transaction cleanup
belongs to B0-L2c. BindingId adoption and production semantic activation belong
to atomic SA3-B.

## Callable single-loop Recipe co-seal (caller-zero)

`callable_single_loop_recipe_coseal.rs` is a `cfg(test)` implementation of the
closed `RECIPE-COSEAL-I0-R0` row. It consumes the resolver/MAP product exactly
once and delegates Recipe verification, JoinSig, and source-bound Core sealing
to their existing owners. The common result is
`VerifiedLoopRecipeCoSealV1`; callable Prelude and Tail remain disjoint sibling
contracts. `callable_single_loop_recipe_shape.rs` contains only the fixed
logical Recipe fixture so each source file stays below the 800-line lane cap.

This boundary owns no AST rematch, Builder/MIR/ValueId/BasicBlockId, ABI,
Completion, physicalizer, selector, retry, fallback, or production route. The
exact Tail statement site is carried by the source-map target; it is not
reconstructed from a name or ordinal. Physical preparation and production
selection remain closed until a later explicit row.

The bounded `CALLABLE-SOURCE-SHAPE-THIN0` split keeps neutral syntax shapes in
`callable_single_loop_source_shapes.rs` and keeps observer/source-map tests in
test-only sibling files. `CALLABLE-STATIC-PREFIX-S0` now adds a separate
top-level resolver/catalog fixture (`int_to_str` -> `to_i64`) and records
explicit `FreeStatic` shape plus direct-call ledger evidence without target
injection. `Method` remains the existing typed negative; neither shape issues
an ABI, Recipe key, or physical capability. The focused
shape/source-map/static-fixture suites remain caller-zero and all touched files
stay below the 800-line source limit. `CALLABLE-STATIC-PREFIX-MAP-S1` is now
closed as a source-only map relation: the resolver-issued `to_i64` target may
have a different owner when the compilation brand matches; a foreign brand is
a typed `ForeignOwner` rejection. The map retains the resolver target and
does not derive ABI or open a Prepared product. The next bounded cell is
`CALLABLE-STATIC-PREFIX-P0` for declaration-derived ABI/Prepared evidence.

## Generic G0 S0A source projector

`generic_g0_projection/` is the only AST-bearing source projector for the
Generic G0 S0A row. It consumes one natural `ResolvedFunctionLoweringInputV1`,
uses `FunctionSourceViewV1` and resolver-issued `BindingRefV1` lookup, and
passes an AST-free observation to
`loop_structural_facts::generic_g0::VerifiedGenericStructuralFactsG0`.
It also retains neutral condition/update operator syntax facts for the later
policy issuer, but owns no type/numeric/policy decision, Recipe, Builder/MIR
effect, retry, fallback, or production caller. The shared MirBuilder
caller-zero guard is the contract for this boundary; S0B adds source-type
inventory only after S0A is sealed.

## Generic G0 S0B source-type projector

S0B extends the same disconnected projector with one callable-header view.
It preserves owner-branded parameter/return annotation sites, resolver-issued
parameter `BindingRefV1` rows, raw declared type spelling, and the four S0A
literal role/context sites. The AST-free issuer lives only in
`resolved_semantics/generic_g0/`; the compiler projection is the sole place
that opens the source AST. It emits `VerifiedGenericSourceBundleG0` by moving
the already-sealed S0A product together with the S0B inventory. The bundle's
move-only owner is `loop_structural_facts::generic_g0`; this compiler module
does not retain a second aggregate authority.

Missing annotations are `Unresolved`; known non-`i64`, foreign, duplicate, or
non-integer source shapes are rejected. S0B does not infer a return type,
retag literals, choose numeric representation, issue Recipe keys, or enter
Builder/MIR/production. The shared replacement guard checks the recursive
semantic directory, source/test line cap, and caller-zero boundary.

## Generic G0 S0C numeric representation

S0C keeps `VerifiedGenericSourceBundleG0` as the single move-only source
product. The compiler-side `generic_g0_projection/numeric.rs` adapter builds an
AST-free scalar view and calls the sole issuer in
`numeric_substrate/generic_g0/`; the lower layer never imports compiler or
resolver products. The result is one `VerifiedGenericTypedSourceBundleG0`
containing the unchanged S0B bundle, one `VerifiedGenericNumericFactLeaseG0`,
and the existing `ExactTrivialReturnAbiV1` receipt.

Natural G0 accepts only plain contextual integer literals. Typed suffixes are
retained by S0B but rejected as out-of-profile by S0C; missing source context
is an S0B structural disposition, not a synthetic S0C fixture. The numeric
issuer owns exact target/range classification only; positivity and recurrence
progression remain the S1 policy row. No numeric wrapper Call, AST rewrite,
retry, Recipe, Builder/MIR effect, or production caller is introduced; the
policy handoff remains caller-zero.

The test-only `generic_g0_observation.rs` adapter is the S1 source-attempt
normalization boundary. It consumes S0A/S0B/S0C exactly once with an explicit
`NumericTarget`, maps typed projector errors to neutral C/D/U/R outcomes, and
does not call a selector, Recipe, Builder, MIR, retry, fallback, or production
route. Its 12 focused tests are part of the row guard; ambiguous source lookup
and binding evidence remains unresolved rather than being guessed.

## Generic G0 policy handoff I0/R0 (caller-zero implementation)

The compiler-side `generic_g0_projection::handoff` test adapter now issues the
sole source-projector co-seal `VerifiedGenericG0PolicyHandoffV1`. It retains
an opaque resolver/source brand borrowed from the canonical selector window,
the typed S0C bundle, exact role `BindingRef`s, numeric target, and post-loop
return relation as one AST-free move-only product. The handoff does not retain
a second window lease. Policy consumes and retains that product by value; it
does not downgrade to a bare bundle or reread source. The former
candidate-envelope witness remains cfg(test)-only evidence and is not wrapped
or paired after the fact.

Focused G0 observation/policy tests and the shared caller-zero guard are
green. This row still has no production caller, demand, Recipe, Builder/MIR,
retry, fallback, or legacy-retirement claim.

## Generic G0 demand S3 I0/R0 (caller-zero implementation)

The test-only selector-to-demand issuer consumes `Selected(Generic)` by value
and retains one canonical window lease, the borrowed handoff brand, the typed
source/numeric/return bundle, the post-loop return read, profile/mode/coverage,
and an opaque exact-role proof. Candidate evidence is checked against the
selector lease before the product is sealed. It does not issue Recipe keys or
touch Builder/MIR; the next boundary is the caller-zero S4 Recipe producer
`GENERIC-G0-RECIPE-S4-I0-R0`.

## Generic G0 Recipe S4 (design accepted)

S4 has one caller-zero Generic producer. It consumes the S3 demand once,
creates the private deterministic Recipe/key map, and delegates common Recipe
verification, JoinSig, After capability, and source-bound Core sealing to
their existing owners. The Generic producer alone issues `GenericG0`
provenance and the exact source/effect relation matrix. The After envelope
owns the logical tail and exact return ABI; P0 owns executable completion and
DraftSeal. No Builder/MIR, physical, retry/fallback, or production caller is
opened by this design.

The five family observers now consume their source attempts exactly once and
retain typed expected/observed identity, mode, and coverage evidence on every
C/D/U/R disposition. This R0 evidence-preservation change is caller-zero and
does not open the common admission assembler or any production lowering.

## NestedPredicate S1 source observation

`nested_predicate_observation.rs` is a `#![cfg(test)]` adapter from the
resolver/source-owned NestedPredicate projector to a neutral AST-free source
attempt. Forest lookup and invariant errors remain lossless at this boundary.
The policy observer is caller-zero and does not issue selection, Recipe/JoinSig,
Builder/MIR, retry/fallback, or production routing. Seven policy tests and
eight projection tests cover the bounded S1 matrix; LoopTrue is the next design
boundary.

## DirectAccum S1 source observation

`direct_accum_observation.rs` is a test-only adapter from the existing
resolver/source-owned DirectAccum projector to the neutral
`VerifiedDirectAccumSourceAttemptV1` transport. It maps compiler projection
rejects to typed Declined/Unresolved/Rejected reasons and is not compiled into
the production compiler path. The policy observer consumes only this
AST-free attempt; it does not receive AST, Builder/MIR state, route IDs,
Recipes, retry/fallback authority, or a production caller.

## LoopTrue S1 implementation receipt

`loop_true_break_continue_projection.rs` remains the sole syntax observer for
the bounded `loop(true)` plus explicit `break`/`continue` shape. The landed
`#![cfg(test)]` adapter maps typed source outcomes into the neutral structural
facts DTO and preserves lookup/navigation/missing-fact distinctions. The pure
policy observer owns the identity/mode/coverage recheck; it does not import the
legacy schedule policy. Nine policy tests and eight projection tests cover this
caller-zero boundary. No selector, Recipe/JoinSig, Builder/MIR, physical
route, retry/fallback, or production caller is authorized; the next boundary
is common five-family selection/admission design.

NestedPredicate S1 is already landed as a caller-zero source observation. Its
source authority remains
`nested_predicate_projection.rs::issue_nested_predicate_source_projection_v1`;
the adapter preserves forest lookup/invariant error distinctions and the policy
observer remains separate from the Nested Recipe producer and route selector.

## LoopCond S1 implementation receipt

`loop_cond_break_continue_projection.rs` is the sole syntax observer for the
bounded non-true loop with one explicit-else Break/Continue branch. It carries
only resolver-owned sites, typed exit origin/target evidence, and the sealed
owner/origin/kind/site/frame identity; it does not claim condition type/effect,
carrier/update, return, nested-loop, or physical semantics. The compiler
adapter is `#![cfg(test)]` and maps source outcomes into the neutral
Candidate/Declined/Unresolved/Rejected transport. Nine policy tests and five
projection tests are green. Selector, Recipe, Builder/MIR, retry/fallback, and
production callers remain closed.

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
