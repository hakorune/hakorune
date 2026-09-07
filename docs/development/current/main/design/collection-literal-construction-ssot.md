---
Status: Accepted design; staged implementation
Scope: Array literal construction-target preservation; selected LLVM C consumer
---

# Collection literal construction

## Current capsule

- Decision: preserve named versus intrinsic construction in the existing allocation products.
- Implementation: raw/typed-local/Core Array producers preserve IntrinsicArray; literal birth edges and duplicate Script runtime publication are retired.
- Next: preserve primitive runtime write Results, then close fallible allocation and checked ABI. Script handoff/frame/Invoke/cleanup/finishing are implemented; typed C remains stopped.
- Production stop: numeric typed Array literal locals reach the existing typed C capability Stop. Excluded typed source shapes and Loop retain Deferred.
- Retirement: Array literal birth callers/effects are removed; Map/Main remain. Wider Array execution is not complete.

The [language contract](../../../../reference/language/block-expressions-and-map-literals.md#4-collection-literal-construction-identity)
owns intrinsic literal semantics and named provider compatibility. This document
owns compiler representation and cutover, not another source resolver.

## Decision and ownership

Use one construction-target enum, conceptually Named(String) | IntrinsicArray,
in existing MirInstruction::NewBox and CoreEffectPlan::NewBox. Replace the
box_type field; do not retain a parallel string/optional receipt/side table.
No new allocation opcode or speculative IntrinsicMap variant is needed now.
Allocation destination, arguments, effect handling and remapping remain with their
existing owners. IntrinsicArray has zero constructor arguments.

An exact ArrayLiteral source arm selects IntrinsicArray before child descent.
The raw, typed-local and Core normalizer entries retain that closed syntax
distinction; they do not resolve a name. Core lowering copies the target into
MIR and Atomic Publish preserves it. Named New, Main synthesis and Map remain
Named in this series. Old name-only artifact inputs are Named, never inferred
to be literals.

This is construction identity preservation, not completion of AST-free collection
Facts/Recipe issuance. Existing source context/child relations remain with their
owners; the target enum issues no new ownership, Fault, ABI, lifecycle or Recipe
key authority. Array type/origin metadata can still serve its existing write/type
observers, but cannot certify builtin provider identity.

Do not supply a general intrinsic-to-"ArrayBox" name accessor to semantic readers.
Use explicit target matches: diagnostic display and representation type may show
Array, while name-resolution consumers can extract only Named.

## Choice and next execution gate

Keep the accepted intrinsic/Named split. Routing literals through Named would
allow provider lookup to change language-defined literal construction; mapping
all Named ArrayBox construction to intrinsic would erase explicit provider
semantics. One target enum preserves both without a second resolver or registry.

The read-only source audit confirms raw and typed-local callers share
`collection_literals.rs::build_array_literal_with_contract_and_port_v1`;
`normalizer/helpers_value/lower.rs` has the independent Core ArrayLiteral arm.
No new issuer decision is required for this construction-only change. This does
not establish the final AST-free Facts/Recipe pipeline.

| Order | Owner / concrete work | Acceptance and retirement |
| --- | --- | --- |
| 1 | Current consumer row: finish pending variant/view/frame/C changes and owner docs; verify and commit before source edits. | Allocation-only and nested physical rows, invalid/missing/duplicate/residual rejection, remap/Core13 and Named wire regressions. Physical evidence only. |
| 2 | Shared raw/typed helper and Core ArrayLiteral arm switch together. | Delete the helper's Array birth call and Core arm's birth MethodCall. Preserve Map/Main callers of the shared birth helper. |
| 3 | Same source-cutover slice: exercise natural source through selected C and independently linked OBJ/EXE. | Untyped empty/populated/nested/Loop execution, typed capability Stop, named-provider separation and failure/order preservation; no dummy Call to select an empty literal. |
| 4 | Close the same series: remove only exclusive obsolete branches/tests and synchronize docs. | Real switched callers plus deleted birth edges; then select the next family. No blanket shared-helper deletion. |

For Loop acceptance, observe the existing known-array write branch in
`effect_emission.rs`; a surviving fallback Method push is a cutover blocker,
not permission to add a new C name interpretation. If a source witness stops
at an earlier terminal, record dependency evidence and retain the open acceptance
item. Do not substitute a synthetic frame test or silently narrow the source set.

## Acceptance correction: typed Array capability

Decision: construction identity cutover preserves the existing typed Array Stop;
it does not activate a typed selected-C execution consumer. The original task's
combined typed/untyped EXE matrix conflicted with the existing capability owner.
`typed_array_backend_capability.rs::enforce_typed_array_backend_supported` admits
contract-bearing modules only to the reference interpreter. Selected OBJ/EXE
reject before transport with `typed_array_contract_backend_unsupported` and
`typed_array_exact_numeric_state_guard_v1`; the existing published array tests
already require that rejection. This is a task-premise correction, not a new
runtime acceptance or removal of the typed carrier.

Raw, typed-local and Core allocation implementations switch together. The
runner's materializer, not a manually chosen AST request, determines the source
entry. In source-backed Script, the relation connection below admits supported numeric
Array<T> literal locals through the existing Script source issuer. Other annotated
Local shapes retain Deferred before initializer traversal; no downstream acceptance
follows from those excluded source stops.
The successful AST-only typed capability test remains dependency evidence.
Untyped empty/populated/nested/Loop and unchanged same-name Box source must use
materialized source ingress and actual EXE/independently linked OBJ execution.

The original wider series remains open: the Script typed-local relation
connection below now reaches the existing typed backend Stop; a separately accepted
state-guard consumer must then open typed execution. Script Loop source admission
is also CutoverBlockerOpen: its unchanged materialized source reaches
Deferred/UnsupportedStatement(Loop) before Core. Earlier stops and AST-only
success do not close either original source-to-EXE requirement. No alternate
ingress, changed fixture or fallback may bridge the missing links.

### Script typed-local relation connection

Decision: replace Script lowering projection's local site-to-BindingRef payload
with the existing ResolvedInitializerRelationV1. Do not add a sibling semantic
receipt or annotation authority. Resolver shadow/stmt.rs observes initializers
before binding insertion, records declaration/binding/annotation/initializer, and
resolver canonicalization seals that relation. The existing Script pre-effect
issuer co-seals the parser invocation, root window, forest and source projection.

Boundary: runner-materialized Script -> ScriptLexicalCore Local -> exact resolver
relation -> same-source lowering projection -> binding materializer -> existing
local descent -> intrinsic allocation/claim/writes -> LocalSlot completion and
refreshed LocalInit row. Includes one binding, one literal initializer and all
seven supported numeric Array<T> annotations under one existing contract parser.
Untyped Local keeps its current acceptance, including absent initialization.
Excludes unrelated annotation admission, nonliteral typed initialization,
ScriptLambda, BlockExpr-prelude widening and Loop admission.

In traversal_profile.rs, only ScriptLexicalCore may admit this additional Local
family: one variable/slot, ArrayLiteral initializer and parse_annotation returning
Some(spec). Invalid/other annotations and missing/nonliteral typed initializers
retain their current Deferred before child effects. The bool gate does not issue
new diagnostics or retain a second contract; explicit legacy annotation errors
remain downstream dependency evidence. ScriptLambda and BlockExpr stay unchanged.

At normal_script_semantic_lowering_projection.rs sealing, require a bijection
between Local declarations and initializer relations, same owner/binding,
ordinal0 and exact optional LocalInitializer(0) membership in the same source
product. Keep annotation and initializer site in that relation, not in parallel
maps. No relation may be paired later by names or matching owner IDs alone.

normal_script_binding_materialization.rs passes the retained relation into an
explicit source-backed mode of the existing local input. Its syntax projection
uses relation-derived annotation; raw/callable compatibility retains its explicit
raw mode. Missing relation never selects raw mode. AST comparisons may reject
drift but cannot choose the annotation. Before allocation, compare the active
declaration and prepared initializer source with the relation coordinates.
Retire the selected Script RawLegacyLocalInputV1::new(input) decision edge.

Existing collection_literals.rs remains allocation -> one claim -> ordered child
writes; variable_stmt.rs completion registers that same preclaim ID with LocalSlot,
then refresh_source_rows preserves its LocalInit source. Do not substitute the
nonliteral path, which evaluates first and claims later. No new runtime schema,
source type inference, physical claim issuer or backend capability is authorized.

Acceptance: runner-materialized `local a: Array<i64> = [10, 20]; return 30`
produces verified intrinsic MIR, one preclaim, matching LocalInit and ordered
writes; all seven supported spellings share that evidence. Missing/duplicate or
foreign relation, binding/site drift reject before allocation. Child failure
retains allocation/claim and only preceding writes. Untyped locals and the
excluded source stops remain unchanged. OBJ/EXE must reach the existing typed
backend capability Stop; this does not close typed EXE acceptance.

Implementation order: preserve/co-seal the existing relation and connect the
selected input mode, then open the finite Script gate in the same slice. Focused
source/claim/drift tests, existing instance/canonical guards and owner/reference
updates close this row. Source owners are below760; no new file-count/guard layer
or additional worker census is required for this closed mapping.

Verification receipt: materialized numeric Array source covers all seven spellings,
one matching source claim/refreshed LocalInit carrier and pre-artifact OBJ/EXE Stop.
Relation-set missing/duplicate/foreign/missing-expression and input annotation/site
drift tests pass. Typed/untyped child-owner failure tests retain only preceding
writes; undefined-name natural source still stops earlier at resolution. Existing
Script19, pre-effect6, located3, view35 and Array/Pair EXE+OBJ30 regressions pass.
This is source connection evidence, not typed runtime execution or whole-suite green.

### Selected-C state-guard audit and prerequisite order

Decision: retain the existing backend capability Stop. Source failure/cleanup
issuance is a prerequisite to checked runtime ABI and C status lowering; a claim
export or backend whitelist alone cannot open execution. Existing final Fault
policy (cleanup, report/dispose, process70) remains owned by
[function exit and entry result](../../../../reference/language/function-exit-and-entry-result.md#target-process-exit-projection).
It does not prove that Array cleanup has been issued.

Historical pre-source-cutover census (superseded source/physical rows are resolved
by the Script physical mapping below; runtime/C rows remain open). Boundary:
materialized Script numeric Array literal LocalInit -> source
claim/element write -> published ordinary entry/C dispatch -> kernel Array handles
-> ArrayBox state; plus existing source failure and final-completion owners.
Includes ordinary/shared aliases, existing kernel integer/boxed/text mutation
routes and diagnostic alternative slot backends. Excludes general typed
ParameterEntry/ReturnExit/record boundaries, provider-owned external arrays,
Dynamic-specific cleanup, other backends and Loop source admission. This finite
inventory establishes the listed blockers, not full-kernel mutation parity or
concurrent clone/slice snapshot guarantees.

| Owner / product | Observed boundary and disposition |
| --- | --- |
| function/typed_array_contract.rs; type_contracts/typed_array.rs | Source claim/carrier preserves spec/state term and requires runtime checks, forbids proof elision. No failure continuation or Home cutpoint. CutoverBlockerOpen. |
| collection_literals.rs | Standalone allocation, claim, element evaluation/write. No Array failure co-seal/Invoke path. CutoverBlockerOpen. |
| resolved_control_flow/function_control.rs; resolved_semantics/home_new_prefix.rs | Existing Fault/Home issuer accepts direct-local New only; ArrayLiteral is source-not-new. It cannot supply Array obligations by default. |
| normal_callable_fault_state.rs; normal_callable_semantic_loan_port/main_root.rs | RootOwned frame is source-identity-checked App Main, not a general Script root authorization. |
| instruction/invoke.rs | Operations cover object lifecycle, not Array claim/write. Runtime frame availability does not authorize new semantic operations. |
| published_backend_view/c_transport.rs; runner/mir_json_emit | Ordinary write rows carry exact site/receiver/index/value. Claim JSON and carrier metadata exist, but no typed claim row or selected-C claim consumer. CutoverBlockerOpen. |
| published_mir_object.rs; C generic op dispatch | Existing capability rejects before transport. Removing it would expose unknown claim op; ordinary entry has no Array Fault/cleanup contract. |
| boxes/array/runtime_contract.rs; ArrayStateCell | Sole storage+contract under one RwLock. Claim audits existing elements before installation; identical claim is idempotent, conflicting claim rejects. Preserve this owner. |
| boxes/array/traits.rs | share_box aliases the same state; clone/slice copy contract into fresh state. No handle-keyed typed registry is needed. |
| array ops store/capacity/insert | Validation and commit share state lock; raw bool/-1/None results erase reason. Checked result preservation is required before a checked ABI. |
| array ops text/shared and surface_catalog | Text/mutable bypass rejects claimed state; surface paths retain mismatch diagnostics. Clear/remove/order-only changes preserve contract. |
| kernel array_runtime_aliases/array_handle_cache | Ordinary aliases reach ArrayBox; integer payload must not use ambiguous handle-or-integer decode. Existing write exports often flatten error to zero. |
| kernel plugin/array_slot_backend.rs | safe_rwlock uses ArrayBox; single_thread_exact copies handle-keyed Vec, direct_array_i64_exact uses separate storage. Explicit profile rejection or real state participation is required; default-env tests are insufficient. |
| kernel exports/fault.rs and fault_checked_object.rs | Existing caller-owned frame/status infrastructure is reusable physical input only. Array failure reasons/profile/wire spec and entry integration are undesigned. |

The historical first blocker was `TypedArraySourceFaultContinuationIssuerMissing`;
the source and physical Script slices below now discharge that issuer gap. The reference interpreter's
VMError and final exit70 do not supply a source cleanup continuation. No new
empty Home/Verified/Prepared receipt may be fabricated from metadata, EffectMask,
ArrayStateTerm or an emitted instruction sequence.

Ordered remaining tasks (not implementation permission):

1. **Source failure/cleanup co-seal design**, in existing source-exit/ownership and
   Script semantic source owners. Close allocation and preclaim, claim success,
   each element evaluation/write, LocalInit commit and final propagation as one
   finite lifecycle. Name the actual Array issuer, normal result and Fault
   continuation, uncommitted storage disposition and existing caller obligations.
   Do not extend direct-New by AST retagging or default empty cleanup.
2. **Checked runtime result/ABI contract**, after source failure meaning is fixed.
   Preserve one ArrayStateCell lock for check+mutation; expose existing claim and
   reason-preserving writes to kernel. Define explicit seven-spec wire mapping
   (never Rust enum discriminants), unambiguous value lanes, profile checks and
   Normal/Fault/InvalidContract mapping. Reject unsupported alternative storage
   before mutation. Existing raw wrappers may retain compatibility sentinels;
   selected typed consumers may not use those lossful results.
3. **Published input and selected-C consumer**, after both contracts are closed.
   Preserve the exact claim/spec/write/failure relation, use the existing runtime
   descriptor/session and final-entry policy where legitimately admitted, and
   retire selected typed reliance on metadata/name interpretation and raw-result
   writes. Keep unsupported entry/boundary families at the capability Stop; no
   blanket ny-llvmc exemption or claim-only completion.
4. **Execution and retirement**, same original cutover series: all seven specs,
   empty/populated and shared-alias writes; successful EXE and independently
   linked OBJ30. Range/negative-to-unsigned/type/claim-conflict failures must keep
   failed writes uncommitted, prior effects intact and later elements unexecuted,
   then perform exactly the issued cleanup -> report -> dispose ->70. Cover failed
   adoption without contract installation, idempotent claim, invalid handle/spec/
   output and alternative-profile rejection. Physical checks alone are dependency
   evidence; original typed execution remains CutoverBlockerOpen until real entry
   and obsolete selected edges are closed.

Step1 source cutpoints, Script frame/control and both finishing consumers are
implemented. Step2 now follows the checked-runtime audit below. Reuse the
runtime/C inventory while its named blockers remain open; this is not an Exhausted/zero-blocker cutover claim. No new guard,
sibling design document or speculative C entry is authorized.

### Accepted Script Array source lifecycle slice

Accepted premise correction: Array traversal previously retained child paths but
not body-shape edges. The existing shadow resolver issues passive
`ArrayLiteral { site, element_count }` and record each exact Element relation.
Existing body-shape seal verifies count, ordinal coverage, endpoint membership
and unique child/parent/role; zero is valid only with source-issued count0.
Do not reconstruct missing edges from path prefixes. Array-specific seal work
may be private within the same owner to respect the source line limit. This
replaces Array's coarse Other(kind) row and the draft consumer's empty-list
assumption. It adds no semantic Home/ABI authority or source family.

Decision: connect the existing
[intrinsic numeric acquisition law](../../../../reference/language/ownership.md#intrinsic-numeric-array-literal-acquisition)
to the current Script source continuation product. This chooses the existing
Home/acquisition law, not runtime handle ownership as source authority. No generic
container capability or new source syntax is decided here.

Sole issuance chain: VerifiedScriptSemanticSourceV1::seal_ast_with_forest ->
VerifiedScriptSourceContinuationV1::issue, while the same forest/window, exact
ResolvedInitializerRelationV1 and BodyShapeRelationV1 parent/Element ordinals are
available. Add private source-operation cutpoint rows within that product; do not
add an ABI sibling receipt, duplicate source resolver or second statement map.
Normal method-call continuation rows remain unchanged. Existing FunctionExit
remains the final Value/Unit authority. Direct-New and App Main-only products are
not borrowed or fabricated for Script.

| Source cutpoint | Responsibility / successor |
| --- | --- |
| Before allocation | Existing caller obligations; no nonexistent Array release. |
| Allocation acquired, before claim | Evaluation frame owns incomplete native Array, including failed wrapping/registration. |
| Claim succeeds | Same incomplete residence; no extra Home is created. |
| Child ordinal i evaluating | Incomplete Array plus exact child evaluation obligations; prior effects stay committed. |
| Child ready, before write | Child capability/transfer remains distinct from Array storage. |
| Write succeeds | Scalar stored; no element Home; advance once to ordinal i+1. |
| Claim/child/write Fault | Active child obligations -> incomplete residence -> caller exit; preserve first Fault and omit later elements. |
| LocalInit commits | Transfer one Home to exact destination and disarm incomplete cleanup. |
| Later outward exit | Exact committed Array Homes join the existing scope-exit order; aliases never add releases. |

ResolvedCleanupObligationsV1 holds crossed scopes and terminal bindings, not
in-flight acquisition. CallerNewHomePrefix is New-specific. The private cutpoint
state must represent this missing evaluation responsibility explicitly; an empty
list, ArrayStateTerm, EffectMask, handle or successful sample cannot stand for it.
There are no Recipe keys, ValueIds, blocks or physical runtime pointers here.

The implemented source row covers direct Script typed numeric Array literal LocalInit for
all seven specs, with source-proven Integer/Bool/Float primitive literal children.
Float kind proves no owned resource, not a particular value or range. Boolean and
floating child kinds do not prove numeric contract success. Source prefix
classification may include exact primitive scalar locals, prior typed Arrays
committed by this issuer, and ordinary borrowed aliases only when an exact
variable binding reaches a live committed Array with no intervening reassignment,
move or escape. It must inspect the source prefix, not assume the selected local
is the first statement. Reverse-order committed Home obligations come from that
finite scan; empty sets require positive coverage evidence.

Unclassified prior Home, opaque/effectful child, nested Array, arbitrary call
result, reassignment, branch/scope-crossing alias, String/Null/Void or unsupported
statement leaves its source lifecycle authority unavailable. Do not call these
language-invalid or infer Trivial from literal shape. Keep the original execution
obligation and explicit capability stop; no silent raw retry or fake cleanup.
The first row does not close those families, the Loop requirement or typed EXE.

Real handoff/consumer: existing Script A/C into_lowering_input ->
VerifiedScriptSemanticLoweringInputV1 -> ScriptSemanticLoweringState ->
normal_script_binding_materialization typed Local entry. Replace retention-only
`_continuation` use with exact initializer/destination/spec validation and one-shot
consumption of the selected source-operation row. Missing/foreign/reused rows must
not admit selected typed lowering. This retires the edge that lowers the selected
typed Local without checking its source lifecycle. Preserve explicit unavailable
capability handling for the excluded families; do not turn absence into fallback.

Acceptance: fixed materialized source and all seven specs co-seal the ordered
cutpoints and exact destination; prior scalar/committed Array/valid alias prefixes
produce the right reverse Home order; unknown or invalidated prefix/child capability
never yields an empty proof. Reject site/destination/spec drift and double consume.
Supported source still emits current verified MIR and reaches the existing typed
C Stop. Runtime RootOwned frame, Invoke lowering, native disposal and Fault70
execution are later consumers, not claims of this source-contract row.

After this source row, bind an explicit Script source root-frame/failure contract
and its physical operations before C activation. The accepted native contract
must be implemented with the sole ArrayBox state owner, checked status and profile
rejection described above. Do not accumulate unconsumed source products or open
another source family before this original series reaches execution/retirement.

### Script root terminal and final handoff order

Decision: before frame/Invoke activation, the existing Script continuation
must prove the whole root terminal and preserve its Array rows after Local
consumption. Local-prefix proof alone is not root cleanup proof. Extend existing
owners; do not add a parallel optional Script artifact receipt.

Actual source inputs are `shadow/stmt.rs::resolve_return` (Value relation,
ExplicitReturn and exact target function region), Script core owner/function
scope/function region/resolved exits, body-shape Return site/value, literal
source facts and the same verified root window. ScriptRootReturnExitAdmission
is only admission, not a result or cleanup issuer. App Main's function completion
product is not a Script completion product.

Implemented source row: co-seal explicit final Integer Return or bare Return
with its exact outward target, function scope, source body Sequence scope/region
and complete reverse committed-Home order. The body region is the return source,
not the target function region. The existing continuation is the sole issuer. Require that coverage
before selected typed Local lowering; Local start requires prior Homes completed,
and successful Local generation records completion while preserving the payload.
Script scope finish verifies terminal coverage and all Local completions against
the complete Home order, retaining source rows in a completed state.
Delete Available-to-unit-Consumed payload destruction and the finish path that
accepts consumed locals without inspecting the tail. No MIR/source re-resolution.

This row does not equate absent Return with Unit. Implicit Unit needs positive
whole-window normal-completion coverage and remains an explicit uncompleted
source obligation, as do unclassified result/tail, unknown/effectful child,
nested Array and Loop. These are capability-unavailable, not language-invalid;
no raw retry or changed original execution finish line.

Acceptance: real seven-spec source with empty/populated and prior Array/scalar/
borrowed-alias prefix; last Array included in terminal release order; exact
Integer/bare Return target/value/root scope; wrong target, missing value edge,
nonfinal Return, opaque tail, foreign owner, dropped/duplicate Home, double take
and incomplete Local reject. Source data remains in completed state. Selected
OBJ/EXE still reaches the typed capability Stop; no runtime frame claim.

The completed-root validation slot now retains Script source and emission
bindings in its own finite variant. The artifact outlet remains the mandatory
successor: into_artifact_parts -> FinalizedRootHandoffV1 -> normal published
pipeline -> borrowed view. Callable sealing still requires
FinishingChecked/App Main/Birth evidence; it cannot issue Script semantics.
Move the completed Script Array payload through this same outlet and reject
dropped or stale bindings. Do not recover meanings from the module or create
fake Birth entries to satisfy a callable variant.

### Completed-root validation handoff slice

Decision: move completed Script source and exact emission bindings into the
existing completed-root validation slot, and consume them in both diagnostic
and artifact finishing checks. This is the next retention/physical-validation
row, not permission to issue RootOwned/Invoke or activate typed C.

The actual path is program_root_lowering::lower_program_root_after_catalog_install_v1
-> with_script_semantic_source_v1 -> finish_normal_default_root_after_pre_effect_bind
-> finalize_module_with_root_validation -> CompletedNormalDefaultRootCatalogLifecycleV1
-> into_parts/into_artifact_parts finishing closures. Source scope closeout must
return its completed source with the normal result rather than discard it.

module_lifecycle::prepare_module_with_callable_main_policy creates the current
root function/entry. The caller captures that physical root anchor when installing
the same invocation's Script source. Local emission records bindings only under
that anchor. finalize_module_with_root_validation passes the exact finalized
function to its callback; bind its signature key there after matching the entry.
Later lookup uses this retained physical key. Never search for source identity
using the spelling main, a fixture value or an unrelated catalog entry.

Replace root_new_validation: Option<(String, Rc<OrdinaryNewClaimLedgerV1>)> in
normal_default_root_catalog_post_install and the completed-root owner with one
root-neutral finite slot: absent, existing ordinary-New validation, or completed
Script Array source plus physical bindings. Do not add a parallel script_validation
Option. Both finishing closures validate their selected variant.

Bind results at emission: initializer -> intrinsic allocation destination/site;
claim -> contract id/array/site; each exact child -> emitted value and ArrayWriteSiteId,
operands/site; LocalCommit -> BindingRef/value; root terminal -> Return operand/site.
Expose the write id already issued by emit_array_element_write instead of dropping
it. MIR scans may validate retained bindings, not discover source correspondence.

Delete source scope's successful payload-drop edge, result-only root return edge,
ordinary-New-only validation slot type and discarded selected emission results.
Acceptance runs real source compilation through both finishing closures, rejects
foreign root, duplicate bindings and deleted/swapped/changed allocation/claim/write/
Local/Return operands. Existing optimizer owners must update changed physical
coordinates; unresolved mapping remains a blocker. Keep typed OBJ/EXE capability
Stop and existing Array/Pair regressions.

The implementation retains the completed source, emitter-issued allocation dst,
claim ID, ArrayWriteSiteId, exact primitive definitions, LocalCommit binding/slot
and Return operand. Typed Local uses its existing initializer observation sink;
source Element order is the already-sealed continuation's order. Script runtime
Return arrives through DirectPortAwareExpression, so capture belongs at the raw
port expression ingress, not an unused statement-only hook. The exact finalizer
checks the physical root and both finishing closures check the retained bindings.
An ephemeral index locates only already-issued physical identities; it never
creates source correspondence. Missing/duplicate/foreign or changed operations,
carrier/slot drift, reordered operations and changed Return results reject.

Corrected physical-owner audit: default `[cli, plugins]` does not enable
`rc-insertion-minimal`; Legacy's RC call is a stats-only stub. Current CSE records
Const keys but has no Const replacement arm. The prior claim of active RC
insertion and Const-to-Copy rewriting was incorrect. Neither is an acceptance
requirement or an authorized feature expansion for this row. The draft constant
snapshot table/Copy acceptance was removed before cutover.

The active optimizer may remove dead Local Copy/unused constants and shift
instruction indexes. Validate live allocation/claim/write/primitive/Return IDs,
unchanged blocks and relative order; a missing Local Copy is permitted only when
its issued destination has no remaining uses. Tests run actual default plugins
optimization on/off and assert that DCE really removes the dead Local copies.
No alias chase, literal re-inference, unknown rename/reorder or optimizer skip.
RootCleanupBoundary remains only a bounded-validation design precedent.

### Root-neutral finalized artifact retention slice

Implementation: `mir/finalized_root_handoff.rs` owns `FinalizedRootHandoffV1`;
callable sealing imports it directly and pipeline/view use
`bind_finalized_root_handoff`. ScriptArray owns the moved Array source rows and
emitter bindings, with the original continuation owner and exact entry. The
consuming accessor requires Finished physical progress and preserves unissued
Array as absent. No map or complete lowering ledger is copied. The view borrows
the same product and checks exact root key/entry/source ownership; callable
projections return unavailable and constructor admission explicitly rejects
Script. The selected Script-to-None loss and universal constructor binding
assumption are removed; frame/Invoke/cleanup and typed C are still successors.

Decision: generalize the existing final root handoff enum and its one borrowed
view slot; move the validated Script Array source and emission payload into a
ScriptArray variant. Existing callable NoBirth/Births payloads retain their
contracts. This does not issue root ABI, storage profile, FaultFrame or Invoke.

Boundary: completed-root artifact finishing -> pipeline retained_root owner ->
published view callback -> existing typed backend capability Stop. Includes
selected Script Array and existing callable projections; excludes other Script
families, implicit completion, frame/Invoke/cleanup and runtime/C execution.
The selected Script-to-None information-loss edge is retired. Array-unissued
Script remains distinct; it must not become an
empty completed Array product or a constructor NoBirth variant.

Existing source continuation and actual emitters are the issuers; finishing
validates their completed products. A consuming accessor moves only their Array
source/emission payload into the existing handoff's private variant, without
copying maps or publishing a new semantic Prepared/Verified receipt. The pipeline
continues to own one final handoff and the view borrows it. Exact key/entry/owner
agreement is physical binding, not source reconstruction. Constructor-only
root_source/root_result/Birth ABI projections apply only to callable variants;
Script must not be admitted by returning an empty Birth list.

Acceptance: real source-backed typed Array remains present in the artifact
callback/view; foreign/missing/uncompleted bindings reject; constructor admission
rejects Script; existing Pair handoff and EXE/OBJ30 remain green. Array OBJ/EXE
still stops before output. Delete the Script-to-None drop, Birth-only artifact
slot type and universal constructor binding assumption in the same series.
The next consumers remain Script frame/Invoke/cleanup, checked runtime ABI and
C execution; retaining an artifact product does not complete them.

Removing post-allocation birth markers preserves failure handling structurally:
entry `emit_method_birth_mir_call` validates but emits no runtime operation;
nested birth is likewise a no-op. The allocation symbol and allocation-before-
children order stay unchanged. This is code-inspection evidence, not executed
OOM evidence; child failure/order has its own source tests.

## Script Array physical lifecycle mapping

Decision: consume the existing source cutpoints and terminal Home order in one
selected Script lifecycle lowering responsibility. Do not add a frame-only row:
a frame without Array Normal/Fault successors retires no execution edge and
cannot discharge the retained source obligations.

Source authority is the existing Script continuation's same-owner
ArraySourceLifecycleRows plus RootTerminal (exact source/body/function scope,
Return/result and terminal Homes). Recipe/control selection must co-seal those
products before physical emission. Physical identity, MIR lookup, object-store
metadata and a runtime handle are not source authorities.

The existing CallableFaultFrame materialization mechanics can be reused after
separating them from App Main selection permission. The Script consumer selects
RootOwned from its own source/root binding and materializes once. Its control
mapping must account for these finite transitions:

| Operation | Normal successor | Fault successor |
| --- | --- | --- |
| Array allocation | Normal-only result -> claim | prior Homes -> outward Fault; no nonexistent residence release |
| claim | first child or Local commit | incomplete residence -> prior Homes -> outward Fault |
| source-proven primitive child | exact result -> write | no fabricated source Fault for literal evaluation |
| write | next child or Local commit | incomplete residence -> prior Homes -> outward Fault |
| Local commit | exact binding gains one Home | no fabricated Fault edge for the existing nonfallible binding |
| explicit root Return | reverse terminal Home cleanup -> exact source Return | an existing first Fault remains outward Fault |

Primitive child capability is source-proven; its lack of child Homes is not a
default for arbitrary expressions. Aliases add no Home. Array incomplete and
committed ownership remain distinct even when native cleanup uses one primitive.

Physical premise correction: native `nyrt_handle_release_h`/`ny_release_strong`
return void and call host_handles::drop_handle. They do not call source fini or
modify FaultFrame. Do not fabricate a checked Normal/Fault status for this API,
or branch on a nonexistent source cleanup Fault. Registry retirement may be
deferred by call pins and surviving Arc/cache references; residence release is
not proof that every Array reference was freed. Nonpositive/missing handles are
not currently checked errors. Object HomeRelease/ReclaimUnpublished target the
typed-object store and are not Array cleanup. The same-named noop shim is not
native execution evidence.

Physical vocabulary and selected source emission/finishing implemented:

| Physical operation | Operands | Normal result |
| --- | --- | --- |
| InvokeOperation::IntrinsicArrayNew | none | one InvokeNormalResult Array value |
| InvokeOperation::ArrayStateContractClaim | contract_id, array | Unit; no projection |
| InvokeOperation::ArrayElementWrite | site_id, kind, producer, receiver, index, value | Unit; no embedded dst/projection |
| MirInstruction::ArrayResidenceRelease | value | no result, frame operand or source-Fault successor |

This emitter issues only LiteralAppend/Literal/index=None writes. Shared kind
vocabulary grants no additional source family. Allocation/claim/write carry the
existing Invoke frame and distinct Normal/Fault successors. Allocation result
exists only on Normal. Release is WRITE/non-pure and requires retained lifecycle
validation; it consumes one source-proven residence obligation. Incomplete
reclaim versus committed Home remains in the source/control binding, even though
both use the same physical release primitive. Passive DestroyOwned, alias-group
ReleaseStrong and ordinary-object cleanup retain their existing meanings.

Current connection: the pre-emission Recipe is issued at Script lowering-state
entry and moved through selected Local/Return inputs into retained bindings.
The Written-only late projection and selected standalone emission are retired.
`script_array_control_emission.rs` emits allocation/claim/write Invoke and exact
Fault cleanup; the Script Return consumer emits reverse Home cleanup. Dedicated
control validation runs in both finishing consumers and again on the borrowed
artifact. Only a validated source-selected Array root joins lifecycle coverage.
The existing typed host Stop runs before runtime session or artifact creation.

Source-to-physical owner chain:

1. ScriptSemanticLoweringState::new consumes the already co-sealed continuation
   and seals one private ScriptArrayLifecycleRecipeV1 from its complete Array
   cutpoints, prior Homes and exact RootTerminal. No source AST classifier,
   physical ID, Recipe key or default cleanup enters the Facts owner. The Recipe
   is the sole control selector; it never reissues source capability.
2. lower_script_local_v1 takes that exact Local recipe before emission and
   passes it through the existing Script-proven Local input. The typed literal
   descent consumes the recipe and existing exact Element child port. Existing
   Local preflight/slot/type-carrier/commit remains the sole Local owner; do not
   duplicate it in an Array wrapper. A selected recipe missing from a typed
   Script input rejects, never falls through to standalone compatibility.
3. The selected Array emitter creates allocation/claim/write Invoke sites,
   Normal projections and Fault cleanup directly. Reuse the current value/block
   and ArrayWriteSiteId issuers. Extend next_array_write_site_id to see Invoke
   writes as well as standalone writes; otherwise every new write can reuse0.
   No post-hoc standalone-MIR-to-Invoke semantic reconstruction.
4. The Script Return hook consumes the selected terminal recipe and current
   BindingRef values. It preserves the exact terminal result while emitting
   reverse Home releases before Return; it no longer lowers a raw direct Return
   for this selected family. Array-unissued Script keeps its separate ingress.
5. FunctionFaultFrameV1 in builder/function_fault_frame.rs owns the extracted
   physical materialization/validation mechanics. Existing callable lowering
   and retained Script/construction validation use that one owner. Source entry
   owners retain selection permission. The selected Script recipe selects RootOwned; its first Array
   consumer materializes once at entry. No standalone frame series or fake
   App Main selection is permitted.

Finishing retains source plus exact origin/Normal/Fault/result/release bindings
in the existing ScriptArray handoff. Replace the selected standalone binding
schema rather than keep competing validators. Each failure site has the source
specified release set and order: allocation failure excludes the new residence;
claim/write failure includes it, then prior Homes. A release never overwrites
an existing first Fault. Source Return and ReturnFault remain distinct terminals.

Physical audit: Script roots already enter validate_finished_array_root, not the
ordinary-New coverage validator. The Script validator must cover all emitted
frame/Invoke/release/ReturnFault sites itself. Prefer a direct Invoke chain with
no Array-internal Jump. DCE can shift instruction indices; SimplifyCFG can merge
nonempty single-predecessor Jump targets and rewrite InvokeNormalResult origins.
Where an emitted Jump exists, validate only contraction of that retained graph;
do not infer cleanup obligations from optimized MIR or skip the optimizer.

Finite physical consumer inventory for the same series:

- instruction/invoke and the existing release instruction visitors (effects,
  operands, rewrite, display/printer/query) consume the declared shapes;
- verification/invoke preserves one frame, exclusive Normal landing and result
  dominance; Array operations do not require an ordinary-object definition;
- type_contracts/typed_array::collect_claims and array_element_write::rebuild
  read exact Invoke operations, including terminators, under their current
  metadata owners; classify_state_term follows InvokeNormalResult to its exact
  IntrinsicArrayNew origin. Missing carriers are not compatibility evidence;
- DCE keeps non-pure release and existing Invoke result anchors; finishing
  validates any bounded SimplifyCFG coordinate change without optimizer skip;
- completed Script validation covers its own lifecycle sites without treating
  them as ordinary-New coverage; compiler finalization binds the same handoff;
- published Script lifecycle stays UnsupportedBeforeObject, bypasses object
  profile/constructor admission, and reaches the existing typed Array capability
  Stop before any host artifact/session. No CanonicalTyped promotion. Constructor
  physical program/JSON matches explicitly reject Array operations; nonselected
  interpreters/transports reject unsupported vocabulary, with no parity work.

The boundary is selected source scope/Recipe -> selected Local and Return
emission -> finishing/metadata refresh -> borrowed view -> typed Array host
Stop. Includes the above selected path and shared physical readers; excludes
runtime checked entries/C emission and all unselected source/backend families.
A missing mapping in this inventory is an implementation blocker, not permission
for a new semantic receipt, fixture bypass or generic fallback.

Exclusive same-series retirement: selected Script typed Array standalone
IntrinsicArray allocation, ArrayStateContractClaim, ArrayElementWrite and direct
Return without source Home cleanup. Preserve explicit compatibility and the
unselected backend roles. Gates cover one frame, each Normal/Fault landing,
result availability, cleanup order/exactly once, no allocation-failure release,
no alias double-release, first-Fault preservation, seven numeric specs,
empty/multiple literals and Integer/bare Return. Keep typed OBJ/EXE pre-output
Stop until checked allocation/claim/write ABI and selected C consumer are implemented.

Ordered successors: accepted physical mapping -> selected lowering and finished
validation cutover -> checked allocation/claim/write runtime ABI -> selected C execution plus independent
linked OBJ/EXE and caller-zero retirement. Abstract Invoke vocabulary does not
prove existing allocation exports report Normal/Fault or convert OOM to a
language Fault. Native abort is not successful cleanup or source Fault evidence.
Implicit completion, nested/opaque
children and Loop remain their existing unsupported obligations.

## Checked Array runtime premise audit and task order

Decision (2026-09-08, audited at `7be46e37e4`): first preserve structured
primitive write outcomes in the sole Array state owner. This is BoxShape with
live raw callers, not checked ABI activation. Two read-only workers audited
state outcomes and the independent allocation substrate; compiler/Fault
consumers were audited by the primary agent. No build or runtime proof is
claimed for this design audit.

Boundary: selected Script Array Recipe/finished handoff -> runtime physical
input -> native allocation/claim/primitive literal write/release -> root entry.
Includes i64/Bool/F64 children, seven numeric specs, Integer/Unit completion,
existing native aliases and selected storage/session agreement. Excludes new
source families, VM/WASM parity, boxed/text/RMW expansion and performance.
This is an open prerequisite inventory, not Exhausted or caller-zero closure.

| Owner | Observed fact / required work |
| --- | --- |
| `boxes/array/mod.rs` ArrayStateCell | One RwLock holds storage and element contract. Preserve that state identity; no handle-keyed contract table. |
| `boxes/array/ops/store.rs` primitive methods | i64/Bool/F64 each return bool. Negative/beyond-end index, InlineRecord storage and contract violation reject before mutation; i64 discards exact validator errors. Replace these three bodies with Result implementations and retain delegating raw bool wrappers. |
| `boxes/array/runtime_contract.rs` | Claim already returns StateConflict or indexed ExistingElementMismatch, audits before installing and is idempotent. It and its error type are crate-visible; a future kernel bridge must deliberately expose the existing owner. |
| kernel `array_compat`, `array_slot_store`, `array_slot_append`, safe `array_slot_backend` | Real primitive raw callers. Keep their legacy boolean/sentinel behavior through the sole Result implementation. Future checked callers consume Result directly. |
| `array_handle_cache::with_array_box_direct` | Reuse native Array access for checked entries; do not dispatch selected writes through environment-selected alternate stores. Other reachable alias mutation paths still require profile closure. |
| `exports/fault.rs`, `include/nyrt_fault_v1.h` | Reuse Normal=0/Fault=1/InvalidContract=2, caller-owned frame, allocation-free diagnostic recording and first-Fault retention. InvalidContract is never a cleanup successor. New Array wire/reason mapping is still required. |
| `storage.rs`, kernel `exports/env.rs`, `host_handles.rs` | Inner/outer Arc allocation, registry initialization/three-vector growth and element conversion/growth have no returned allocation-failure contract. Native release may grow its free-list. CutoverBlockerOpen. BoxBase creation is heap-free. |
| final Script handoff -> compiled-entry/physical input | Script retains source/result/control but callable RootI64/Birth-only input cannot consume it. Extend the existing final input with explicit root result, checked sites and Array contract/value representations; do not forge Births or object profiles. CutoverBlockerOpen. |
| C lifecycle V4 admission/emission | Admits exactly Pair's two functions/two-field layout; no Array operations, Float lanes or Unit root execution. A shared frame descriptor is not Array consumer admission. CutoverBlockerOpen. |
| runtime descriptor/session | Existing descriptor proves frame/entry layout; it does not prove new checked Array symbols/capability. Require archive agreement for the eventual Array operation ABI before output. |

Immediate slice: `MIR-ARRAY-PRIMITIVE-RESULT-PRESERVATION-I0`.
Source authority remains existing annotation/spec and source Recipe. Runtime
state methods validate supplied values against that installed contract; they
issue no source meaning. Use one owner-local runtime error enum for index,
unsupported storage and existing element-contract reason. Keep new Result APIs
crate-visible in this slice; public raw methods delegate `.is_ok()` and preserve
OOB observations, append-at-end, conversion and return values. Claim is reused
unchanged. Delete the three bool mutation bodies and their internal reason
loss; do not keep parallel implementations or precheck-then-mutate locks.

Acceptance: all three primitive overwrite/append/conversion paths; all seven
specs with endpoints/range/signed-to-unsigned failure; Bool/F64 type mismatch;
negative/beyond-end index and InlineRecord rejection; storage/length unchanged
on every returned rejection; alias shares the contract; existing raw/claim
behavior preserved. F64 uncontracted NaN/infinity acceptance stays unchanged.
Use the existing Array owner tests and corridor guard, plus the typed host Stop
regression. This slice does not turn allocation failures into returned errors.

Ordered remaining dependencies, all within the open cutover boundary:

1. Primitive Result BoxShape above, with live wrappers using the sole body.
2. Fallible shared-ownership Decision: installed Rust1.89 std Arc::try_new is
   gated by allocator_api; no existing fallible Arc owner was found. Choose an
   explicit toolchain policy or a separately designed shared substrate before
   implementing inner/outer Arc failure propagation. Neither allocator preflight,
   raw Arc layout fabrication nor catch_unwind closes this requirement.
3. Same host registry reserve -> commit: reserve slots/generations/call-lifetime
   and release free-list capacity under one lock before ID/generation/publication;
   initialize the same registry fallibly, preserve payload on failure, update TLS
   cache only after success. ID exhaustion is distinct from allocation failure.
   Concurrent allocations must preserve capacity for all required releases.
4. Connect Array construction/registration rollback and existing element storage
   fallible conversion/growth. Returned rejection must leave published state
   intact; memory exhaustion tests must target real allocation failure paths.
5. Checked kernel ABI: narrow cross-crate state bridge, explicit seven-spec and
   primitive-lane wire mapping, handle/profile validation, status/reason mapping,
   out-slot written only on Normal, allocation-free failure recording. Native
   void release keeps the original Fault and gains no invented status.
6. Existing final-input projection, Script root I64/Unit policy and physical
   diagnostic sites -> selected C status/control emission -> source EXE and
   independently linked OBJ. Retire selected lossy transport/dispatch edges and
   the selected pre-artifact Stop only with complete end-to-end evidence.

No checked ABI implementation is selected while its allocation substrate is
unclosed. OS kill/overcommit termination is distinct from a returned allocator
failure, and neither is evidence that source Fault cleanup ran. Global allocator
replacement, duplicate Array state and compatibility fallback are not fixes.

## Instance-prefix boundary repair

Decision: consume the existing `InstanceBoxSemanticOwner` transfer in
`normal_script_boundary_receipt_pack.rs::seal` using the same nonstatic
BoxDeclaration guard as the neutral-window and root-admission validators.
The parser-owned instance transfer cohort already proves invocation, source row,
methods and constructors; the receipt pack issues no new source meaning.
Previously this missing arm fell into `window-boundary/source mismatch` before
Array lowering. Preserve runtime source indices and default mismatch rejection.
The unchanged shadow source is the end-to-end witness; static/non-Box mismatches
remain negative cases. No baseline-red claim is made without parent execution.

## Instance declaration publication and completion

Decision: selected Script immediate declaration lifecycle is the sole physical
definition publisher; retained runtime instance declarations perform only their
existing Unit completion. Never skip by name or collector contents.

Boundary: selected-normal Script nonstatic Box -> exact transfer/source cohort
-> work-plan -> immediate publication -> retained runtime statement. Includes
plain/nonplain runtime arms; excludes raw compatibility, nested Box ingress and
App runtime-nonexecution. The former work-plan cloned each constructor batch
and issued immediate plus runtime demands. Both reached canonical publication.
Now one ImmediateDeclaration demand remains; runtime retains only Unit completion.

Immediate lowering already owns fields, weak fields, type ID, method slots,
property getters, metadata constants and constructor/ordinary-method definitions.
Neither runtime arm owns additional registration work; retain only `emit_void`.
Do not combine the earlier declaration-facts registration cleanup with this fix.

Implemented: both selected runtime arms use declaration Unit completion; runtime
source/constructor batch transport, demand tickets, the two exclusive runtime
roles and caller-zero prefix helper are removed. Preserve immediate/shared lifecycle, duplicate rejection and
unconsumed-source checks. No new semantic receipt, resolver or registry.

Acceptance: unchanged materialized shadow source through EXE/linked OBJ; plain
and nonplain source each publishes each constructor/method once and demands each
constructor once; field/weak/slot/getter metadata survives; declaration-only Script
retains Unit completion. The typed-local relation row now reaches the existing C
Stop; keep typed-C execution and Loop source blockers open. Materialized plain/generic shadow EXE and independently linked OBJ exit30,
parser-backed one-demand witnesses, metadata regression and declaration-only Unit
checks pass. Worker inspection and execution are separate evidence.

## Bounded inventory and treatment

Boundary: Array literal source entries -> Core/MIR construction products ->
target-preserving transforms -> published body/frame -> selected C allocation.
Includes empty, populated, nested and supported typed-local literals, plus the
existing Loop Core entry. Excludes Map cutover, Main synthetic arrays, named
constructor semantics, Invoke/Birth lifecycle and new VM/WASM feature parity.
The inventory names known semantic owners, not a claim that all 401 textual
NewBox search matches were independently audited. Structural conversion must
account for each touched match; unresolved reclassification blocks cutover.

| Owner / boundary | Required treatment |
| --- | --- |
| raw_expression_dispatch/mod.rs ArrayLiteral; collection_literals.rs helper | Intrinsic source issuer only at source cutover; retain child locators/order. |
| stmts/local_statement_descent.rs typed literal | Same target/helper; retain existing Array<T> claim and no PackedArray fallback. |
| control_flow/plan/facts/expr_generic_loop.rs; normalizer/helpers_value/lower.rs | Existing Array acceptance includes Core path; select target in ArrayLiteral arm. Do not leave Loop literals Named. |
| instruction.rs; control_flow/plan/effect.rs | One target vocabulary in both existing products; intrinsic args empty. |
| control_flow/plan/lowerer/effect_emission.rs | Copy target unchanged. Existing known-array push branch emits ArrayElementWrite; verify actual Loop source at cutover. |
| control_flow/verify/verifier/effect_validators.rs; normalizer/cond_lowering_freshen/remapper.rs | Validate target-specific shape; remap values, not identity. |
| builder/joinir_id_remapper.rs and joinir_id_remapper_values.rs | Preserve target and remap destination/args only. |
| optimizer_passes/normalize_core13_pure.rs | Preserve IntrinsicArray verbatim; only Named can become name Const + env.box.new. Existing strict consumers reject unsupported remaining ops. |
| array_element_write.rs; array_receiver_proof.rs | Recognize intrinsic allocation directly; do not promote Named Array compatibility evidence into builtin identity. |
| generic_method_route_facts; generic_method_route_plan origin_inference/flow_origin | Separate type observation from named authority. |
| user_box_method_route_plan target_collection/origin_inference/origin_route_flow; typed_object_plan; storage_inference; route_decision/typed_object | Intrinsic cannot re-enter user named target selection. |
| exact_numeric value/field facts; type_contracts/weak_field; ordered_map_origin_plan; record_helper_args; same_module body/fusion | Explicitly classify target-sensitive observations; preserve their existing scope. |
| array getset/string-store/rmw/window/indexof and userbox/global helper shape recognizers | Explicit Named/intrinsic applicability; no blanket name projection. |
| instruction_kinds metadata view | dst/args/effect-only observation needs no duplicate target storage. |
| builder/module_lifecycle.rs | Do not require named birth for intrinsic allocation. Printer/builder diagnostics show the distinction. |
| runner/mir_json_v0/module.rs; json_v0_bridge call_ops/globals | Old type string input/production remains Named; no implicit intrinsic promotion. |
| VM handler dispatch; WASM instruction dispatch | Mechanical enum handling must not route Intrinsic through named factory. Until separately selected, explicit unsupported rather than new backend parity. |

## Selected transport and allocation

The published view selects intrinsic allocation itself, including allocation-only
bodies. The C entry requires a nonempty session; an empty literal cannot rely on
an unrelated Print or mutation to become selected.

Extend the existing physical frame with kind 8, IntrinsicArrayNew, using its exact
function/block/instruction coordinate and required destination. Target symbol,
arity and unrelated array-write payload are zero/absent; flags contain only the
existing destination-present bit. This is a physical
projection of the issued target, not a second semantic registry. Keep the existing
row struct layout; Rust/C kind admission changes together and old libraries must
reject an unknown kind rather than interpret it.

The body writer at runner/mir_json_emit/emitters/mod.rs carries the explicit
intrinsic target tag. Named bodies retain their old type-string encoding. Reject
ambiguous target-plus-name input, unknown tags, nonempty constructor args,
missing/wrong/duplicate rows and residual unconsumed rows before object output.
Never recreate a required row from a body type name. Generic name-only ingress
keeps its existing policy; intrinsic input cannot silently fall back to it.

The four selected consumers requiring the same typed peek/take contract are:
- pure_compile_generic_lowering_prescan.inc (origin/declaration prepass);
- pure_compile_generic_lowering_op_dispatch.inc (entry allocation);
- same_module_prepass.inc (nested origin prepass);
- same_module_typed_object_emit.inc (nested allocation).

Prepass peeks without consumption; allocation emission takes exactly once.
Reuse existing Array birth physical emission and row exhaustion. Physical layout
selection remains subordinate to its existing contract; it is not source identity.

## Bounded implementation series

1. Named conversion (BoxShape, implemented): replace string target fields and adapt producers,
   structural readers/reissuers and legacy wire handling. Existing source producers
   emit Named only. The enum has only Named in this step, with no Deref/as_str/From adapter.
   Intrinsic variant and consumer preservation/rejection are added together in step2;
   no source acceptance change or birth deletion. Focused remap, old-wire and
   named-construction tests plus the existing canonical corridor guard.
2. Consumer preparation: intrinsic body/frame kind8, view selection, the four C
   consumers and preartifact validation. Synthetic physical tests are boundary
   evidence only. No intrinsic source producer until this step is verified.
3. Array source cutover: raw/typed-local/Core Array arms switch together, remove
   Array literal helper birth caller and Core Array birth effect. Retain Map and
   Main helper callers. Existing Core push lowering remains; no new Method
   interpretation. Verify natural-source empty/populated/nested/Loop arrays and typed capability Stop,
   same-name user/plugin noninterference, named-new separation, child order/failure,
   Core13 preservation and direct EXE/independently linked OBJ. No dummy Print
   solely to activate empty-array selection; earlier terminals are dependency
   evidence and cannot close the downstream row.
4. Closeout if needed: evidence and exclusive obsolete tests/branches, not a
   ceremonial extra commit. Only then continue Map and Task2 remaining dispositions.

Each step gates its successor. The series, not every intermediate commit, removes
the selected old edge. Named compatibility, whole Call R7 and other backend
coverage are not declared complete. Keep touched source below800; plan a >=760
owner split before editing. The helpers_value/lower.rs owner is already near that
threshold and must not grow unchecked.

Read-only root/worker audits support this design. Named conversion now passes
Cargo check including test targets and a fresh library-test build (peak8.60GiB),
60 existing focused tests, Pair/Bool host1 (14.19s), and the canonical corridor guard. The 795-line origin
owner moved its existing tests into its standard child module without changing
logical test paths; changed source
max777. That Named-only checkpoint did not prove intrinsic acceptance. Consumer preparation
now has a fresh quick test build, view35, old-wire12, remap/Core/source regressions18,
Pair/Bool host1 and C entry/nested/negative/residual acceptance. Production source
producers still issue Named; these physical tests do not establish source cutover.
Core push uses the existing
receiver-is-array-like/known-write branch, while actual Loop, Core13 and empty
source acceptance remain required implementation evidence; the Named regression
tests do not prove intrinsic consumer or source cutover.
