---
Status: Accepted design; staged implementation
Scope: Array literal construction-target preservation; selected LLVM C consumer
---

# Collection literal construction

## Current capsule

- Decision: preserve named versus intrinsic construction in the existing allocation products.
- Implementation: raw/typed-local/Core Array producers preserve IntrinsicArray; literal birth edges and duplicate Script runtime publication are retired.
- Next: Script typed-local source relation/admission design; Loop source admission and typed C state-guard execution remain open.
- Production stop: typed Local and Loop source admission remain Deferred; typed C state-guard capability remains unsupported.
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
entry. In source-backed Script, typed Local currently stops even earlier:
`shadow/traversal_profile.rs::ScriptLexicalCoreV1` excludes annotated Local before
initializer traversal; `NormalScriptPreEffectSourceObservationIssuerV1` produces
Deferred. No typed initializer/helper/backend acceptance follows from that Stop.
The successful AST-only typed capability test remains dependency evidence.
Untyped empty/populated/nested/Loop and unchanged same-name Box source must use
materialized source ingress and actual EXE/independently linked OBJ execution.

The original wider series remains open: the accepted Script typed-local relation
connection below must reach the existing typed backend Stop; a separately accepted
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

Outstanding task: typed Array selected-C state-guard consumer design, owned by
`typed_array_backend_capability.rs` and `function/typed_array_contract.rs`, with
`typed_array_exact_numeric_state_guard_v1` as its required capability. Execution
is unimplemented; reopen only with an accepted guard/representation/failure
consumer contract. This construction Decision grants no implementation permission
and does not claim the original broader execution requirement is complete.

Removing post-allocation birth markers preserves failure handling structurally:
entry `emit_method_birth_mir_call` validates but emits no runtime operation;
nested birth is likewise a no-op. The allocation symbol and allocation-before-
children order stay unchanged. This is code-inspection evidence, not executed
OOM evidence; child failure/order has its own source tests.

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
retains Unit completion. Keep the typed-local/source and typed-C blockers above
open. Materialized plain/generic shadow EXE and independently linked OBJ exit30,
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
