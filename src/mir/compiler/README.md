# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

### Generic G0 body-shape transport I0

`VerifiedResolvedSourceUnitV1::resolve_function` resolves the forest and the
resolver-owned body-shape inventories in one traversal.  The source unit
retains those inventories by `FunctionOwnerIdV1`; its root
`ResolvedFunctionLoweringInputV1` lends only the exact owner-matched body-shape
sibling.  Bare mechanical inputs and callable-module inputs remain explicitly
body-shape-free, so they cannot fabricate a Generic effect product.

`with_generic_g0_source_parent_v1` requires the borrowed sibling and checks
owner/body-root equality before issuing its existing demand/product cohort.
The accepted follow-up effect row now borrows the selected Generic structural
facts before moving the selection into demand/product, co-seals them with the
same body-shape/function/header cohort, and retains one private
`VerifiedGenericG0NoExternalEffectV1`.  That receipt records the exact bounded
two-local-write/one-tail-return shape without becoming an `EffectMask`; it
adds no Builder/session mutation, `ValueId`, CFG/SSA/PHI, lifecycle, Text,
route, fallback, retry, or production caller.

The Generic source parent now also retains one private result-ABI row from the
same selected observation before demand/product consumption.  The transport
checks owner/origin/source-kind and declaration-header ABI parity; it does not
classify a new ABI or open Completion, physical entry, or session effects.
It now lends the canonical `VerifiedFunctionCompletionV1` from the same input
after checking Generic tail, value-result, declared-`i64`, and empty-cleanup
parity.  Completion remains unconsumed and no physical/session effect opens.

### Generic G0 physical function-entry input I0

`generic_g0_physical_function_entry_input.rs` is the Generic-only,
caller-zero projection after the source parent has been co-sealed.  Its
private, non-`Clone` input keeps the parent attached while projecting a
receiver-prefix row (when the resolver says `DeclaredInstance`) followed by
dense ordinary `i64` rows.  The row records source binding/name/type evidence
and may borrow only the existing `ExistingCallableI64` carrier tag as a
mechanical transport detail.

This module does not reuse the S6C physical descriptor/header/signature,
allocate a `MirFunction`, reserve or publish `ValueId`s, issue an
`EffectMask`, consume Completion, or mutate Builder/session/CFG/SSA state.
The callback-scoped `consume` seam moves the parent and descriptors together;
the focused test proves the receiver/explicit order and that the loan cannot
escape.  The next stop is a Generic source-to-physical effect projection;
only after that issuer is accepted may a fresh skeleton be designed.  There
is no shortcut through S6C or `/N`/JSON counts.

## Typed ingress contract

### LLVM compile-target capability transport (I0)

The LLVM runner selects the single cataloged
`PinnedTextCompileTargetProfileV1` at the outer compile invocation and issues
one move-only `PinnedTextCompileTargetCapabilityV1`. `NormalCompileRequestV1`
transports that same invocation-branded capability through the normal
root-catalog lifecycle; the selected-normal close lends it only as a scoped
reference beside the existing physical-signature loan. The collector never
stores it, and JSON, MIR arity, host probing, C `TargetMachine`, and backend
code cannot reissue or infer it.

The capability remains transport-only on the Rust side, but the contract-bound
pure-first path now consumes its strict projection through one retained LLVM C
API 18 TargetMachine session. That session validates the realized triple/data
layout and emits the object with the same TargetMachine; it never consults
host/default/env target selection or falls back to external opt/llc. Uncontracted
modules retain the legacy path. The session does not derive a residence frame,
materialize pointers, or open lifecycle/MIR Text lowering.

### Pinned Text backend-frame Binder I0

The selected-normal close now co-seals one unpublished
`PinnedTextBackendFrameContractV1` from four scoped inputs: complete physical
callable lanes, the Residence-owned `ResidenceAbiLayoutV1`, the stamped
`PinnedTextAccessPlanTableV1` census, and the mandatory compile-invocation
target capability. The contract records only checked counts, revisions,
derived frame bounds, and invocation/owner stamps; it contains no `ValueId`,
runtime token, pointer, JSON-owned meaning, or host-layout observation.

The strict versioned transport projection, profile-schema consumer, module
census, and retained contract-bound TargetMachine object emitter are landed.
Contract-bearing modules use the same session for realization validation and
object emission with success-only temporary-file publication; uncontracted
modules retain the legacy external path. GEP/load, lifecycle CFG, session
residence adoption, route selection, fallback/retry, and production callers
remain closed.

The caller-zero `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BORROW-I0` row now
lends the co-sealed facts through a scoped, non-`Clone`/
non-`Copy` `PinnedTextBackendFrameBorrowV1<'_>`. The JSON metadata writer uses
that view only for the duration of its existing transport projection, so the
borrow adds no serialization authority and exposes no pointer, byte length,
runtime token, slot, generation, or `ValueId`. The C validator and retained
TargetMachine session remain consumers only; runtime residence, lifecycle
entry/finish, `PinnedTextOp`, GEP/load, and direct production lowering are
still outside this row.

### Common V2 canonical-session admission I0

`common_v2_session_admission.rs` owns the effect-free fan-in after the typed
BlockExpr expectation transport. Within one installed S6C HRTB loan it borrows
the selected resolved input, obtains the resolver singleton Loop site, issues
the existing Loop-owned outer-If residual, borrows the batch-owned typed
expectation, checks the common V2 envelope owner, and nests the actual
Completion borrow. The non-`Clone` admission is callback-scoped and contains
no `ValueId`, `BasicBlockId`, Builder/session mutation, Completion consumption,
Recipe/MIR rescan, legacy finalizer, fallback, or retry. A future physical
session may consume this one fan-in; it may not reacquire any sibling by key.

The caller-zero follow-up `LOOP-COMMON-V2-PHYSICAL-SESSION-I0` is a thin
session-open canary. It consumes that admission once, projects the typed
BlockExpr count only inside `CanonicalSsaFunctionSessionV2::new_common_v2`,
and creates one owned `ResolvedFunctionCompletionConsumptionV1` from the
installed parent's scoped Completion borrow. The semantic Completion remains
owned by the installed cohort. A callback-scoped wrapper retains the same
pre-session envelope beside the session, so no second Port loan can be
reacquired. This canary does not mutate Builder/CFG/SSA/PHI state or emit
operations, claims, Returns, DraftSeal, lifecycle, Text, route, fallback, or
production code; the first session-effects boundary remains a separate design
stop.

### Callable-first semantic-program co-seal I0

`callable_semantic_program.rs` consumes one complete
`VerifiedCallableSingleLoopRecipeProductV1` parent and keeps its
operation/effect product, initialized-local input, resolver context,
JoinSig-owned continuation, and Callable Prelude/Tail together until the
existing prepared-operation consumer. The issuer accepts no separately
supplied Core, context, or continuation, so matching owner values cannot
re-pair products from different source frames. The existing
`VerifiedLoopOperationPhysicalDemandV1::issue` remains a mechanical
projection inside this adapter rather than a second semantic authority.

This is a Callable-first caller-zero slice. It does not claim a generic G0
issuer, CFG/SSA/PHI, Builder/session effects, lifecycle, Text lowering, route
selection, fallback, retry, or production activation.

### Common V2 physical function-entry input I0

`common_v2_physical_function_entry_input.rs` is the transport-only consumer
of the accepted physical-entry BoxShape. Inside the same installed S6C HRTB
loan it borrows the catalog-backed storage header, physical-signature lane
row, and source-backed physical effects, then projects one non-semantic
`PhysicalCallableParameterDescriptorV1` per physical lane. The descriptor
records lane role/order, source `BindingRef` relation, diagnostic name, source
annotation evidence, and the checked carrier tag
(`ExistingCallableI64` or `U64BitsOnI64`). It never issues semantic meaning,
allocates a skeleton, creates a `ValueId`, or mutates Builder/CFG/SSA state.

Receiver order is an explicit prefix row (`me`) and is not an explicit source
formal. Ordinary formals use one existing `i64` lane; one ExactText formal
uses adjacent `[slot, generation]` lanes over the existing `i64` mechanical
carrier, with `U64BitsOnI64` preserving the wire meaning. Missing/duplicate
source names, foreign owners, lane gaps/swaps, non-adjacent ExactText pairs,
and incomplete header/effects relations reject before any Builder effect.
The aggregate owns the same callback-scoped loan, so its siblings cannot be
re-paired or stored as a second package authority. Skeleton reservation has
landed as a detached caller-zero shell; entry-lane adoption is the active
caller-zero canary. Lifecycle, Text lowering, route selection, fallback, and
production callers remain closed.

### Common V2 physical function skeleton I0

`common_v2_physical_function_skeleton.rs` consumes the same prepared entry
input and reserves one detached `MirFunction` shell with one existing
mechanical `i64` parameter lane per physical descriptor. The shell retains
the source-backed symbol, result/effects, source declaration metadata, and
descriptor carrier tags while keeping the installed loan alive. It uses a
local unpublished entry block id and is never installed in a `MirBuilder` or
module during this slice.

ExactText remains one logical `BindingRef` with adjacent `[slot,generation]`
descriptors; this I0 reserves the two `ValueId`s but does not publish either
lane to BindingSSA. No Loop CFG/PHI, Completion claim, DraftSeal, lifecycle,
Text lowering, route, fallback, retry, or production caller is opened here.

### Common V2 physical-session stamp retention I0

The prepared physical-entry skeleton already carries one mechanical
`PhysicalFunctionEntryCohortStampV1`. The consuming physical-entry/session seam
moves that stamp exactly once into `CanonicalSsaFunctionSessionV2` before the
Builder transaction or any callback-scoped physical consumer is exposed. The
common-session wrapper lends only a scoped borrow; it does not clone,
reconstruct, or re-pair the stamp. The stamp remains a cohort witness (owner,
selected key, callable signature identity, and physical lane count), not a
result, lifecycle, or semantic authority. Missing, foreign, drifted, duplicate,
or escaped stamps reject before effect; the physical condition result and all
later CFG/PHI/Text/route work remain closed.

### ExactText physical entry/session seam I0

`LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION-D0` and its caller-zero lane
canary are now consumed by `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM-I0`.
`PreparedPhysicalEntrySessionInputV1` is compiler-only, non-`Clone`, and keeps
the installed S6C loan, detached shell, descriptor rows, and one cohort stamp
together. `with_common_v2_physical_entry_session` issues the common-V2
admission from that retained loan, opens one fresh
`CanonicalFunctionLoweringSessionV1`, installs the source Binding authority and
physical shell, then adopts the canonical one-value
`BindingRef -> slot ValueId` plus the private generation sidecar exactly once.

The outer function session is the sole rollback owner: late adoption or
callback rejection calls `discard_unpublished` once and leaves no current
function, BindingSSA entry, sidecar, or module-visible state. No public
`into_parts` split, second Port loan, ordinary generation read, Completion
claim, DraftSeal, CFG/PHI, lifecycle, Text lowering, route, fallback, retry,
or production caller is opened. The V2-native physical-ID-free
layout/placement BoxShape is now accepted; the active caller-zero slice is
`LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0`, which lends typed loop/block/item
topology and JoinSig transfer bindings through the same operation/control
cohort. Const/read/CFG/PHI, Completion, lifecycle, Text, route, and
production remain closed until this transport is sealed.

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

### Normal callable runner materialization

The MIR and LLVM runner frontdoors share one parser/transform materialization
helper in `runner/modes/common_util/normal_callable.rs`. It issues the
parser-backed `SourceBacked` callable product once and routes the explicit
`Compatibility` cohort through the existing AST compile request with exactly
one compatibility-only `normalize_core_pass`. The source-backed branch goes
directly to its typed MIR request (`for_mir_mode_callable_source` or
`for_llvm_callable_source`); it never re-enters the AST normalizer. This is a
transport boundary only: it issues no new semantic receipt, fallback, retry,
or production backend route.

Exact child-site navigation belongs to B0-L2b. Function transaction cleanup
belongs to B0-L2c. BindingId adoption and production semantic activation belong
to atomic SA3-B.

## Dynamic full-body source inventory (I0)

`dynamic_full_body_source.rs` owns the first compiler-side acceptance repair
for the unchanged `ParserScanLoopBox.skip_while/4` method. Its sole issuer
consumes one canonical `ResolvedFunctionLoweringInputV1`, the exact resolver
Loop membership, and the existing function Completion product, then seals one
non-`Clone`, AST-free `VerifiedDynamicLoopFullBodySourceInventoryV1`.

The inventory contains exactly six source bindings and twenty-eight ordered
statement/expression roles covering the prelude, Loop, both Dynamic method-call
syntax sites, iteration-local `ch`, inner If/Return, induction rebind, and
outer Return. `ch` is source identity only: it is neither a carrier nor a
second Binding-SSA owner. Completion remains the unique authority for the
two Return sites and is moved into the inventory rather than recreated.

The local-scope R0 strengthens this same issuer rather than adding a sibling
product. `ch` must be declared in the resolver-sealed Loop-body scope, have
exactly the I7 argument read, and have no binding rebind or nested capture.
After the complete source/Recipe/envelope co-seal, callers may borrow the
neutral V10/I6/I7 relation; they cannot obtain Home, cleanup, or physical
authority from it.

The spellings `substring` and `indexOf` are checked only as the bounded
unchanged-source shape for this I0. They do not classify receiver type,
result type, declaration target, provider, ABI, or physical route. Those
meanings remain closed until the route-disjoint source-bound Dynamic dispatch
contract is designed. This module owns no Recipe keys, Builder/MIR effects,
fallback, source rewrite, or narrowed fixture.

## Callable single-loop logical issuer (S0)

The accepted `CALLABLE-LOOP-PRODUCTION-LOGICAL-ISSUER-D0` is closed by the
bounded S0 promotion. The production-scope logical issuer consumes the
resolver/SourceMap product exactly once and delegates Recipe verification,
JoinSig, After binding, and source-bound Core sealing to their existing
owners. The common result is `VerifiedLoopRecipeCoSealV1`; callable Prelude
and Tail remain disjoint sibling contracts. The fixed profile Recipe shape is
owned by `callable_single_loop_recipe.rs`; the old
`callable_single_loop_recipe_shape.rs` is a test-only parity fixture wrapper.
No production selector or caller invokes the issuer yet.

This boundary owns no AST rematch, Builder/MIR/ValueId/BasicBlockId, ABI,
Completion, physicalizer, selector, retry, fallback, or production route. The
exact Tail statement site is carried by the source-map target; it is not
reconstructed from a name or ordinal. Physical preparation and production
selection remain closed until a later explicit row.

The callable initialized-local Prelude input now crosses the compiler boundary
as the neutral `VerifiedLoopInitializedLocalInputSourceSetV1` from
`loop_recipe_contract::input_source`. Callable currently supplies exactly one
row; all consumers consume the complete set, while Generic parameter inputs
remain a separate contract. This set verifies declaration/initializer,
binding, Recipe input, carrier, and Core binding correspondence before any
Builder effect.

The bounded `CALLABLE-SOURCE-SHAPE-THIN0` split keeps neutral syntax shapes in
`callable_single_loop_source_shapes.rs`; the neutral SyntaxFacts and
SourceMap issuers now compile in production scope, while fixture constructors,
mutation helpers, and observer/source-map tests remain test-only siblings.
The production issuer entry obtains the exact Loop membership from
`CallableSemanticSourceLedgerView::only_loop_site()` and reopens it only via
`FunctionSourceViewV1::stmt_at(membership)`; SourceMap parity remains a
source-only check and never creates a Bridge owner.
`CALLABLE-STATIC-PREFIX-S0` now adds a separate
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

`CALLABLE-STATIC-PREFIX-P0` is now closed as a positive pre-effect relation.
The prepare entry derives the caller result ABI from the completion
declaration and exact callable header, derives the callee result ABI from the
resolver-issued target header, and seals one `PreparedCallableLoopPhysicalizationV1`
for the `FreeStatic` fixture. The old externally supplied ABI argument is gone;
the MethodCall remains a typed `MissingPreludeTarget` negative. The common
physicalizer/session design stop is now closed; its Prelude argument receipt
prerequisite is recorded below.

## Callable Prelude argument receipt P0 (caller-zero)

`LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` is closed as a pre-effect source-to-boundary
receipt. `callable_single_loop_prelude_arguments.rs` issues one move-only,
AST-free `VerifiedCallablePreludeArgumentListV1` from the resolver-backed
`FreeStatic` call site. The first profile admits only direct local parameter
variables with exact `i64` representation; literals, nested expressions,
upvars, foreign bindings, arity mismatches, and unsupported ABI shapes remain
typed `NoSafeSlice` dispositions. The prepared Prelude owns the list exactly
once and exposes it only to the future outer materializer; the common Loop
physicalizer will receive only the resulting private entry receipt.

The focused prepare suite checks ordinal, binding owner, and ABI. This row does
not reread AST by name, open Builder/MIR/physical lowering, or add a selector,
retry, fallback, or production caller. The next bounded row is
`LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`.
Its four-block execution brief is kept in the active workstream card and the
common physical-demand SSOT; do not open a production caller from this README.

The topology/After-only canary is now landed in the test-only
`resolved_lowering::loop_recipe_physicalizer` module. It consumes the neutral
move-only boundary and a private `ReadyLoopEntryV1`, validates exact owner and
parent/preheader topology, and returns one recursive After receipt. It does
not publish a production physicalizer or emit operation MIR. Operation
lowering remains closed until the design-only
`LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0` row issues a neutral `LoopItemKey` plus
exact source-anchor effect product; name, ordinal, and profile matching are
forbidden. This is not an alternate profile route or a reason to reopen the
old scheduler.

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
green. The production source-parent row consumes this handoff only through
the selected Generic source cohort; it still has no physical, Builder/MIR,
retry, fallback, or legacy-retirement claim.

## Generic G0 demand S3 I0/R0 (caller-zero implementation)

The selector-to-demand issuer consumes `Selected(Generic)` by value and
retains one canonical window lease, the borrowed handoff brand, the typed
source/numeric/return bundle, the post-loop return read, profile/mode/coverage,
and an opaque exact-role proof. Candidate evidence is checked against the
selector lease before the product is sealed. The production
`generic_g0_source_parent` issuer invokes this demand and the S4 producer
inside one source-parent transaction, then validates the exact resolver input
and two source-parameter entry rows before lending a callback-scoped parent.
It does not issue physical demand or touch Builder/MIR.

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

## Generic G0 TopLevel declaration/header I0

`generic_g0_top_level_declaration_header.rs` is the only issuer for the
Generic G0 TopLevel declaration/header projection. It consumes the exact
`ResolvedFunctionLoweringInputV1::source()` view already held by the
source-parent transaction, validates the declared-function root, owner/source
kind, parameter name/`ParamDecl` coverage, return annotation, and empty
signature metadata, then stores one private non-`Clone`
`VerifiedGenericG0TopLevelDeclarationHeaderV1` in the same parent. The
callback lends that row; it does not re-scan AST or reconstruct a header from
`/N`, MIR parameter length, or S6C storage facts.

This I0 is a source projection only. Result ABI, receiver/lane layout,
function effect, Completion, skeleton/session, CFG/SSA/PHI, lifecycle, Text,
route, fallback, retry, and production caller remain closed by the active
physical-entry design stop. The source-only projection I0 is landed; the next
design stop is one same-cohort Generic physical-entry input issuer. No
skeleton, lane adoption, or session effect is authorized until that issuer is
named.

## Generic G0 storage/lane source projection I0

`generic_g0_storage_lane_source.rs` retains one parent-owned source row after
the declaration/header, result, effect, and Completion siblings have been
co-sealed.  It records the exact receiver policy and optional resolver
`BindingRef`, declaration metadata witnesses, dense explicit parameter rows,
and a local mechanical `ExistingCallableI64` carrier tag.  Receiver policy is
separate from explicit formal arity: `DeclaredInstance` contributes a
receiver prefix, while `Absent` contributes none; `/N`, MIR/JSON length, and
S6C physical rows are not consulted.

The row is non-`Clone` and callback-scoped through
`VerifiedGenericG0SourceParentV1`.  It owns no physical signature,
`EffectMask`, `ValueId`, `MirFunction`, Builder/session state, or lane
materialization.  Foreign owner/origin/frame, receiver-policy or BindingRef
drift, duplicate/missing rows, and type/ABI drift reject before any physical
effect.  Generic static narrowing, S6C descriptor reuse, and production
caller activation remain separate decisions.

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

The callable source-facts issuer uses two exact seams only: resolver
`CallableSemanticSourceLedgerView::only_loop_site()` proves one and only one
Loop membership (zero or multiple sites are typed rejects), and
`FunctionSourceViewV1::stmt_at(membership)` reopens only a statement present in
the sealed owner source-site inventory. The membership carries resolver
identity/frame provenance; neither seam accepts route ordinals, names,
raw AST path recovery, or source scans as identity. This S0 slice adds no
Recipe, Builder, physicalizer, selector, retry, or production caller edge.

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

## M8 S6A variable-accum recurrence (caller-zero)

`variable_accum_recurrence_projection.rs` is the sole source-view adapter for
the bounded `acc = acc + i; i = i + 1` family. It consumes the resolver ledger
and exact Loop membership, while `loop_structural_facts` owns the one atomic
AST-free Facts product. `variable_accum_recurrence_producer.rs` then projects
that Candidate into the existing Recipe/JoinSig/Core/input/effect owners.
No Recipe kind, route selector, Builder/MIR effect, or physical caller is
introduced. The normal `Main.main` resolver ingress and typed C/D/U/R
disposition are now closed for the bounded S6A ingress. Focused coverage
includes Candidate, Declined, incomplete-evidence Unresolved, and
foreign-owner Rejected. Duplicate binding-role and source-site coherence
negatives now map to typed Rejected outcomes; source identity remains a
resolver-owned rejection. The Facts owner stays below 800 lines through a
separate validation module. S6A is closed with no selector, physical caller,
or Builder/MIR effect.
