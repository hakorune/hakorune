---
Status: Active contract; retained Script Array execution and bounded retirement verified
Scope: intrinsic literal identity, retained Script numeric Array lifecycle and selected native LLVM C execution.
---

# Collection literal construction

## Current capsule

- **Current decision:** preserve Named versus intrinsic Array/Map identity and source-owned lifecycle/Recipe; physical representation cannot issue ownership.
- **Current implementation status:** raw/typed-local/Core literals preserve intrinsic identity; selected retained Script Arrays execute through the checked native ABI and bound V4 OBJ/EXE path. Literal birth and selected duplicate/preparation/projection/Stop edges are retired.
- **Next ordered task:** close callable Map source capability/consumer selection after the static V2 host/raw/Core checkpoint; retain the owning-slot target and distinguish compatibility from final coverage.
- **Production stop line:** exact retained Script LocalInit/claim coverage and a compatible explicit runtime session are required. Module-only/unselected typed families and Script Loop keep their Stops.
- **Retirement finish line:** the admitted numeric Script execution series is closed; Map/Main/named/compatibility callers remain live. Wider Array and Call R7 completion are not claimed.

The [language contract](../../../../reference/language/block-expressions-and-map-literals.md#4-collection-literal-construction-identity)
owns literal semantics and named provider compatibility. This document owns
compiler representation, runtime integration and the finite cutover boundary.
Current scheduling is selected only by `CURRENT_STATE.toml` and the rolling card.

## Decision and ownership

Use one construction-target enum, conceptually Named(String) | IntrinsicArray,
in existing MirInstruction::NewBox and CoreEffectPlan::NewBox. Replace the
box_type field; do not retain a parallel string/optional receipt/side table.
The selected Array series added no allocation opcode. Map identity is accepted
by the language contract. Its product/consumer cutover is the accepted series
below; IntrinsicMap must land with its MIR/view/transport contract, not as an
unowned enum addition.
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

## Source entry and initializer relation

Both actual MIR/LLVM artifact callers use `prepare_normal_source_with_imports`.
Selected prelude/main/final normalization preserves local declarations and their
annotations. Normal discovery and merge rediscovery traverse explicit using
relations only; implicit VM OperatorBox injection remains compatibility-owned.
Nested dependencies/aliases and explicit imports remain real source. No Array
spelling, file extension, environment toggle or fixture substitution selects a
special source path.

ScriptLexicalCore admits the existing one-binding Local with a literal Array
initializer and one of seven supported numeric annotations. The resolver's
`ResolvedInitializerRelationV1` retains declaration/binding/annotation/initializer
before binding insertion and is co-sealed with the same invocation/window/forest.
The lowering projection requires a bijection with Local declarations, ordinal0
and exact optional LocalInitializer membership. No parallel annotation map or
later owner/name pairing is allowed.

Binding materialization passes this relation through explicit source-backed
Local mode. Missing relation cannot choose raw mode. Before allocation, compare
active declaration and prepared initializer with retained coordinates; AST may
reject drift but cannot select the annotation. The existing Local owner alone
owns preflight, slot/type carrier and commit. Raw/callable compatibility keeps
its explicit mode, not a retry after selected failure.

Other annotations, missing/nonliteral typed initializers, ScriptLambda and
BlockExpr-prelude widening retain their prior Deferred boundaries. Untyped
locals retain their existing behavior. Materialized Script Loop still stops
before Core; Core's intrinsic-target support does not establish source Loop
admission. Earlier-terminal or AST-only evidence cannot close that requirement.

## Source lifecycle and terminal ownership

The existing shadow resolver records ArrayLiteral cardinality and exact Element
relations; body-shape sealing verifies count, ordinals, endpoints and unique
parent/child roles. Zero elements require source-issued count0. Path prefixes,
Other(kind), successful examples and empty default lists cannot reconstruct
missing relations.

`VerifiedScriptSemanticSourceV1` and its existing continuation co-seal the same
root/source forest/window, initializer relation and body-shape cutpoints.
The existing [intrinsic acquisition law](../../../../reference/language/ownership.md#intrinsic-numeric-array-literal-acquisition)
owns Home semantics; a runtime handle or AppMain's New-only ledger does not.
No ABI receipt, Recipe key, ValueId, block or runtime pointer enters source Facts.

| Cutpoint | Obligation and continuation |
| --- | --- |
| Before allocation | Prior caller Homes only; never release a nonexistent Array. |
| Allocation acquired / claim | Evaluation frame owns one incomplete native residence; successful claim creates no extra Home. |
| Primitive child ready / append | Retain exact child ordinal/value obligation; successful scalar store adds no element Home. |
| Claim/child/write returned Fault | Active child obligations, then incomplete residence, then prior Homes; retain first Fault and skip later children. |
| LocalInit commit | Transfer one Home to the exact destination and disarm incomplete cleanup. |
| Outward root exit | Reverse committed-Home order; borrowed aliases add no release. |

The admitted source is direct Script numeric Array literal LocalInit with
Integer/Bool/Float primitive literal children. Bool/Float kind proves no child
Home, not successful numeric validation. Prefixes may contain exact primitive
locals, prior Arrays committed by this issuer and borrowed aliases to a live
Array with no reassignment/move/escape. Complete prefix coverage proves the Home
set; an unknown prefix cannot be treated as empty.

The same continuation retains explicit final Integer Return or bare Return,
matching resolved outward function target, body Sequence/source scope and full
reverse Home order. Body region is distinct from target function region.
Local start requires prior Homes complete; completion preserves source rows.
Scope finish checks all Local completions including the last Array. Missing/
foreign/duplicate/reused/dropped rows, nonfinal Return, missing value edge or
wrong target/scope rejects before artifact. Missing Return is not implicit Unit.

Implicit completion, unclassified result/tail, opaque/effectful child, nested
Array, arbitrary call result, reassignment, scope-crossing alias, String/Null/
Void and Loop remain explicit unavailable obligations. They are not language-
invalid, retired by a smaller fixture or supplied with fabricated cleanup.

### Root-neutral finalized artifact retention slice

One completed-root validation slot distinguishes absent, ordinary-New and
completed Script Array ownership. Script scope moves its completed source and
exact emitted bindings through both diagnostic and artifact finishing into
`FinalizedRootHandoffV1::ScriptArray`. There is no parallel optional Script
receipt, copied lowering ledger, source-to-None drop or fake NoBirth handoff.

The root anchor comes from the actual module creation/entry and is bound during
finalization. Lookup by the retained physical key validates that same root;
spelling main and another catalog cannot recover source identity. Bind emitted
allocation/claim/write/primitive/Local/Return identities at their original
emitters; MIR lookup validates retained correspondence and never discovers it.

Both finishing consumers validate exact frame/control/value/source and operation
order, then require Finished before artifact handoff. DCE may remove a dead
Local Copy only when its issued destination has no remaining uses; indices can
shift while retained live identities/order remain exact. No unrestricted alias
chase, rename/reorder, constant re-inference or optimizer skip is allowed.
The normal finalization invocation owns the product; view borrows it. Callable
root-source/result/Birth projections remain unavailable for Script. Its own
input/admission branch consumes the retained Script cohort.

## Script Array physical lifecycle mapping

`ScriptSemanticLoweringState` issues one private `ScriptArrayLifecycleRecipeV1`
from already co-sealed cutpoints, prior Homes and RootTerminal before emission.
Recipe alone selects control; the source owner receives no Recipe keys/physical
IDs. Selected Local and Return inputs move the exact recipe to their existing
emitters. Missing selected recipe rejects instead of standalone compatibility.
`FunctionFaultFrameV1` supplies shared physical mechanics; the Script recipe
owns RootOwned selection and initializes one frame, without fake AppMain identity.

| Emitted operation | Normal | Returned Fault |
| --- | --- | --- |
| IntrinsicArrayNew Invoke | Exact Normal-only result, then claim | Prior Homes only. |
| ArrayStateContractClaim Invoke | First child or Local commit | Incomplete residence, then prior Homes. |
| Primitive literal / ArrayElementWrite Invoke | Exact value, then next child or commit | No invented literal-evaluation Fault; failed write releases incomplete residence, then prior Homes. |
| Local commit | One destination Home | No fabricated Fault for a nonfallible binding. |
| Root Return | Reverse issued Home release, exact saved result | Preserve an existing first Fault; ReturnFault remains distinct. |

Selected writes are LiteralAppend/Literal/index=None, with Unit result and no
embedded dst/projection. Shared write vocabulary does not admit other families.
Allocation alone produces InvokeNormalResult, tied to its exact Normal landing.
Write-site issuance includes Invoke terminators so successive writes cannot
reuse a standalone-only counter. No standalone-MIR-to-Invoke reconstruction.

`ArrayResidenceRelease` is WRITE/non-pure with one value and no frame/result/
source-Fault successor. Native `nyrt_handle_release_h` returns void, does not run
source fini and cannot overwrite the first Fault. Incomplete residence and
committed Home stay distinct in source/control even though they use this one
primitive. Object HomeRelease/Reclaim, DestroyOwned and alias-group ReleaseStrong
retain their own meanings. Pins/Arc/cache references may defer final destruction;
residence release is not proof that every reference was freed. A noop shim is
not native runtime evidence.

Visitors, Invoke verifier, claim/write metadata refresh and DCE consume the same
operations. Normal dominance/frame rules remain mandatory. The dedicated Script
validator covers its complete frame/Invoke/release/ReturnFault inventory instead
of borrowing ordinary-New coverage. Any permitted SimplifyCFG contraction must
match its retained graph; it cannot infer obligations from optimized MIR.
Selected standalone allocation/claim/write/direct Return and their competing
binding schema are retired. Other backend/compatibility readers keep explicit
unsupported behavior; no parity work is implied.

## Checked Array runtime premise audit and task order

`ArrayStateCell` owns storage and the installed contract under one RwLock.
Shared aliases use the same state; clone/slice follow their fresh-state contract.
Claim audits all existing elements before installing, is idempotent for the same
spec and rejects conflict without replacing the old contract. No handle-keyed
contract registry is added.

Primitive i64/Bool/F64 Result store/append operations share one mutation body
under that state lock. Append obtains length, validates, commits and returns the
committed length under one lock. The four kernel len/read/indexed-store sequences
are retired; concurrent append cannot overwrite another append's element.
Raw bool/length/zero APIs only project these Results for compatibility.
Returned rejection preserves storage/length and exact reason; boxed/text/RMW
owners and unsupported alternate stores are not silently promoted.

### Native failure policy consultation

The user-accepted native policy is stable Rust with existing Arc/registry/storage
and fatal allocator OOM distinct from returned Fault. Mandatory fallible-Arc/
registry/storage work is withdrawn as a cutover prerequisite. Stronger allocator
recovery requires an explicit new requirement. Returned-Fault cleanup remains
mandatory; native abort or OS kill is neither successful cleanup nor a source
Fault witness. The normative
[runtime failure policy](../../../../reference/runtime/runtime-data-dispatch.md#selected-native-array-failure-policy)
owns that distinction. No destructive OOM test is required or claimed.

### Checked ABI and session

Existing `nyash.array.checked_{new,claim,append_i64,append_bool,append_f64}_v1`
exports consume the same Array owner through `with_array_box_direct`, bypassing
environment-selected alternate slot stores. New/claim/append return u32
Normal0/Fault1/InvalidContract2. New alone writes an out handle, only on Normal;
nonnull pointers retain the caller's valid/aligned/live/exclusive/nonoverlap
promise. Validation is not a proof that an arbitrary pointer is valid.

Wire tags are explicit `1=i8,2=i16,3=i32,4=i64,5=u8,6=u16,7=u32`, never Rust
enum discriminants. i64, Bool u32(0/1) and f64 are distinct lanes, without
handle-or-integer decoding. Exact mapping and declarations remain in the
[ABI reference](../../../../reference/abi/nyrt_c_abi_v0.md).

| Returned condition | Runtime treatment |
| --- | --- |
| Successful allocation/claim/idempotent claim/append | Normal; existing primary Fault unchanged. |
| Claim conflict | Fault200, details requested tag/0; do not reread a guessed old spec. |
| Existing element mismatch | Fault201, details checked failing index/subtype. Claim adoption of InlineRecord/noninteger storage remains index0/type mismatch. |
| Append element mismatch | Fault202, details subtype/0. Subtypes1 type mismatch,2 negative-to-unsigned,3 out-of-range. |
| Invalid/non-Array handle, malformed frame/out, unknown tag/kind, Bool outside0/1, append InvalidIndex/UnsupportedStorage, unrepresentable diagnostic | InvalidContract before mutation; no fallback or source-Fault successor. |
| Fatal allocator failure | No returned-result or cleanup guarantee under the accepted policy. |

Allocation-free record_static retains first/suppressed/overflow Fault semantics.
Do not use append's UnsupportedStorage rule to change claim adoption behavior.
Checked New currently returns Normal or InvalidContract; an injected returned
allocation Fault tests generated control, not recoverable native OOM.

NativeArray requires exact definitions of the five checked exports plus
`nyrt_handle_release_h` in the selected archive. The existing descriptor/session
owner reads one checked `nm` inventory; absent, undefined-only, ambiguous/local/
data/prefix matches reject. Pair has no Array-symbol requirement. Frame/entry
layout compatibility alone cannot imply these symbol capabilities.

## Final input and native C execution

The existing physical input **contains** `CompiledEntryContractV1` and borrows
retained Script source/emission/Return correspondence. Normalize result to I64/
Unit once; retain exact claims, primitive lanes, supplied successors and cleanup.
Script has NativeArray requirements and no object profile/layout/Births. Pair's
callable branch retains its own required formals/layout/profile checks. Never
forge a constructor profile or empty semantic receipt for Script.

Float transport preserves u64 payload bits, signed zero and NaN payloads; C uses
bitcast without arithmetic. Bool remains distinct and narrows only at its checked
ABI. No Bool/Float root result or unrelated callable opcode is admitted.
`LifecycleInvocationInputV1` owns the completed physical input and borrows the
selected runtime session, validating agreement before serialization/artifacts.
The CAPI wrapper derives JSON and target from that same owner; EXE links its
returned archive and OBJ requires an explicit runtime session at its real caller.

The shared V4 parser/consumer checks exact physical revision/keys, one I64/Unit
root, ABI1, no object layout/profile/Birth, seven tags, representations, frames,
Normal origins and complete liveness through Copies/CFG. New creates a residence
only on Normal; claim precedes append; release consumes it; terminal has no live
residence. Merges agree. Double release, use-after-release, release on allocation
Fault, wrong-handle alias and residual operations reject. C validates issued
control and never chooses a new successor or Home order.

V4 parses once, validates target/session, emits LLVM text, invokes llc and
atomically publishes the object. Malformed input leaves existing output intact
and cleans temporary files. Root Unit disposes then exits0; I64 uses the existing
[0..255/range-Fault70 policy](../../../../reference/language/function-exit-and-entry-result.md#target-process-exit-projection)
after cleanup/report/dispose. Generic typed Stops,
other numeric/write checks and their ordering remain; only exact retained-input
identity plus complete carrier-to-claim coverage admits this Script cohort.
No generic JSON/name interpretation retries a selected failure.

### Accepted runtime-to-C task order

The bounded series is **closed** in its stated boundary:
selected source preparation/Recipe/finished handoff -> native state/checked ABI
-> bound final input -> real OBJ/EXE caller -> V4/runtime terminal. Includes all
seven specs, primitive children, empty/multiple Arrays, borrowed aliases,
Integer/bare Return and returned Fault. Excludes fatal termination, arbitrary
later alias mutations, module-only ingress, other source/backend families and
shared generic compatibility. Broader source obligations below remain open.

| Completed responsibility | Receipt / evidence |
| --- | --- |
| Sole atomic append | `23757dfeb1`: four kernel sequences replaced; state/kernel and concurrent append evidence. |
| Checked native ABI | `32ed589930`: exported ABI8/Fault9/kernel Array12/root Array52; independent header and returned-state/error tests. Runtime dependency alone did not activate C. |
| One final input/session | `c14cc1e140`: retained Script -> compiled entry -> physical input; same bound invocation, exact symbols and primitive/cleanup projections. |
| Real source/host switch | `feaaa5d5e8`: both real callers preserve declarations/explicit dependencies and consume native input/session; selected stripping/implicit observer/typed Stops retired. |
| Final execution and retirement | `b61aef93ec`:39 host EXE/independent OBJ cases,8 injected returned-allocation Fault cases,29 malformed rejects, ordered release/report/dispose; finite inventory exhausted with0 open/reopened blockers within this boundary. |

Runtime observations prove skipped later children, retained prior effects,
uncommitted failed writes, exact reverse releases and report/dispose ordering.
I64 result0/255/256/max and optimizer on/off are covered. Allocation failure at
attempt1/2 covers I64/Unit roots, with no claim/append/release for nonexistent
residence. Actual CLI EXE/OBJ/link, missing-runtime pre-artifact rejection and
Pair/untyped regressions have separate recorded evidence in the rolling card.
Exit70 alone was never accepted as cleanup proof.

Retired selected edges include Local stripping at prelude/main/final normalization,
implicit VM observer injection at discovery/merge, Script handoff loss, callable-
only projection, typed host Stop and native C pending restriction. No additional
caller-zero exclusive asset remained in the finite worker audit. Shared append/
dispatch, generic module-only fences and compatibility preparation have callers
and are not deletion targets. This documentation task adds no runtime evidence.

## Declaration and untyped identity boundaries

The existing nonstatic InstanceBox transfer is consumed in the Script boundary
receipt pack with its same source guard. Selected immediate declaration lifecycle
is the sole publisher of fields/weak fields/slots/getters/constants/constructors/
methods; runtime declaration arms only complete Unit. Duplicate runtime demands,
constructor-batch transport and the exclusive prefix helper are removed. Never
skip publication by name or collector contents. Static/non-Box mismatch,
duplicate and unconsumed-source checks remain.

`278b519d1f` records unchanged materialized plain/generic shadow EXE/linked OBJ30,
one-demand and metadata/declaration-only Unit evidence. Known prior work-plan/
raw-runtime-ingress baseline tests at parent `95f400280d` remain classified in
the rolling card; no source gate was relaxed or test ignored to erase them.

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
| raw_expression_dispatch/mod.rs ArrayLiteral; collection_literals.rs helper | Intrinsic source issuer; retain child locators/order. |
| stmts/local_statement_descent.rs typed literal | Same target/helper; retain existing Array<T> claim and no PackedArray fallback. |
| control_flow/plan/facts/expr_generic_loop.rs; normalizer/helpers_value/lower.rs | Existing Array acceptance includes Core path; select target in ArrayLiteral arm. Do not leave Loop literals Named. |
| instruction.rs; control_flow/plan/effect.rs | One target vocabulary in both existing products; intrinsic args empty. |
| control_flow/plan/lowerer/effect_emission.rs | Copy target unchanged. Existing known-array push branch emits ArrayElementWrite; materialized Script Loop remains a separate source Stop. |
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

Untyped intrinsic allocation remains a live separate published-static transport:
kind8 IntrinsicArrayNew names exact function/block/instruction and destination;
target symbol/arity/other write payload are absent and only dst-present is set.
The body retains the explicit intrinsic tag; Named retains legacy type-string
encoding. Reject ambiguous target+name, unknown tag, nonempty args, malformed/
duplicate/missing/residual rows before object output. Body names cannot repair
required rows. Allocation-only/empty literals need no dummy Print to select C.

Existing entry/nested prepasses peek; entry/nested allocation emitters take once:
`pure_compile_generic_lowering_prescan`, `pure_compile_generic_lowering_op_dispatch`,
`same_module_prepass`, `same_module_typed_object_emit`. Their shared static-call
frame is not a retired lifecycle V2 frame and remains required. Physical layout
selection is subordinate to its contract, not a source identity issuer.

## Map literal selected-construction design

Decision: resume the Map follow-up already ordered at `83d7553ceb` after Array.
The residual published audit at `dee0d91fd3` found two concrete literal issuers:
`builder/collection_literals.rs::build_map_literal_with_port_v1` and the Map arm
of `control_flow/plan/normalizer/helpers_value/lower.rs`. Both still emit
Named(MapBox) allocation and birth. This is CutoverBlockerOpen, not completion
of Task2 or a reason to delete generic Method compatibility.

### Closed source and physical facts

- MapLiteral itself selects builtin Map. Raw/normal Script already has exact
  `MapEntryValue(index)` child-demand/consume through its existing scoped port.
  Keys are ordered String data, not arbitrary key expressions. Preserve
  allocate -> key Const -> value once -> set, duplicate keys and iteration policy.
- Core retains the same child paths and copies `NewBox.target` into MIR. Its
  normalizer is AST-bearing legacy Core, not an AST-free portable Recipe.
- Both C walkers can allocate with `nyash.map.birth_h` and establish
  `ORG_MAP_BIRTH`. An explicit Map allocation row can select the published
  route by itself; empty Map must not require dummy Print or another call.
- This is insufficient for populated Map: its canonical `Call(Method set)`
  has no matching `generic_method_route_plan/write_routes.rs` plan, whose
  match currently requires LegacyCallV0. C Map set/get depend on those plans.
  Generic-entry size and same-module length also have different admission.
  Runtime symbols and retained type observations do not issue a canonical plan.

### MAP-LITERAL-SELECTED-CONSTRUCTION-D1: retained operation decision

Decision: add `IntrinsicMap` to the existing construction target and one
`MapLiteralEntryWrite { receiver, key, value }` to existing MIR/Core operation
enums when this series is executable. The write has no result, kind/producer
flag, optional receipt or second source-site table. Both literal producers
currently discard set's result. Canonical call vocabulary has no existing
intrinsic Map put target; a named Method or runtime Extern is not a replacement.

Source authority + issuer: the existing raw/normal Script literal owner and Core
Map arm retain construction, key String and exact MapEntryValue child. Keep
allocation -> key Const -> child evaluation -> write for every entry. Preserve
all existing child acceptance; no Integer/Bool-only source classifier.
Core copies this explicit operation into MIR. This remains an identity/write
cutover through current AST-bearing lowering, not portable Recipe completion.

Non-authority: box names, Call schema, legacy plans, destination completion and
unrelated Print cannot issue intrinsic semantics. The operation's fixed effect
must conservatively cover current raw MUT and Core Io ordering; do not add a
mutable effect field or weaken it to gain optimization.

Physical authority: the existing published input/frame owner must check and
project exact operand representation **before** the shared lossy JSON emitter.
Use private physical data, validated definition/ABI products and exact site/SSA
identity; no new semantic Verified/Prepared receipt. C consumes that projection,
validates the physical contract and performs boxing; it cannot classify source
expressions, default an unknown value to i64 or search handles to infer a kind.

### Physical representation obligation — implementation still closed

This retained anchor records the original loss boundary. The completed design
and current implementation order are the versioned-frame decision below; the
old stop does not override that accepted series.

The read-only D1 audit at `ca2dba4724` closes the operation choice above, but
found no existing general exact-value owner usable for the full Map domain:

- `runner/mir_json_emit/emitters/basic.rs` emits Rust Bool as i64(0/1), and
  Null/Void as void(0). C body-only reconstruction cannot recover Bool.
- `same_module_value_metadata` and `pure_value_type_metadata` collapse unknown
  handle classes and Void to T_I64. Missing origin is not integer proof.
- `ValueRepresentationFact` contains only BoxedSumHandle. Its refresh is not
  a general scalar/handle authority. Native Array's exact Const projection is
  limited to retained Script Array; Dynamic's ledger belongs to another session.
- `nyash.map.slot_store_hhh` / `map_slot_store_str_any` use
  MapValueBorrowString: live handles decode as values, otherwise bits become
  Integer. Passing raw scalar bits (including0/1) would permit handle collision
  or type loss. Its1/0 result reports applied/not-applied, not lifecycle Fault.

### Selected source premise and runtime treatment

Boundary of the completed source premise audit: MapLiteral child -> selected
Script source seal/current raw and Core child lowering -> published route.
Includes every ScriptLexicalCore expression gate; excludes runtime projection,
transferred child execution and unselected backends. This is not full cutover
Exhausted evidence. The complete gate in `shadow/traversal_profile.rs` admits
Literal/Variable, Unary/Binary/Await/Check/GroupedAssignment, Array/Map/Record/
EnumMatch, plus Script-only Lambda/MethodCall/FromCall. BlockExpr requires its
recursive pure predicate (Literal, Unary, Binary, Await, Check, BlockExpr), with
optional Print around a pure prelude expression. All other arms decline;
`is_catalog_brand_expression` is the separate existing catalog-brand exception.
Gate acceptance still requires lexical/schema/continuation and child-owner
admission. Transparent/Transferred/Diagnostic rows are not Resolved body rows.

Ordinary FunctionCall/Call/If reach existing source `ObservationDeferred` before
MIR effects. Map value MethodCalls now retain exact EntryValue relations and
reach explicit `UnsupportedMapEntryCall`, wrapped as source `IntegrityInvalid`.
The former accidental MissingParent boundary is retired; natural direct and
bound-receiver counterexamples are covered by continuation tests. This preserves
execution admission. Core value-if's separate pure-branch checks
and then-side type are not a generic mixed-result ABI authority. Thus preserving
Map's child port does not require promoting these unselected families. Float
has no source Stop: do not invent one to dismiss its physical dependency.

Decision: the selected write will pass an explicit kind and 64-bit payload to
one Map-specific runtime store, which materializes the value at the write.
It will not register temporary scalar handles and then run the Any decoder.
This local physical representation is not a whole-program tagged calling ABI.
The [planned runtime ABI](../../../../reference/abi/nyrt_c_abi_v0.md#selected-map-literal-store-v1) fixes five named tags,
OK0/InvalidContract2 and the length-aware String entry. Compiler input projection
and formal-domain ABI remain gated; the runtime exports are implemented.

| Proven representation | Runtime treatment at the write |
| --- | --- |
| I64 | Bit-preserving payload -> IntegerBox; a live handle with the same numeric bits is irrelevant. |
| Bool | Require exactly0/1, then BoolBox; Integer0/1 remains I64. |
| F64 | Preserve all64 bits -> FloatBox via `f64::from_bits`, including negative zero and NaN payload. |
| Handle | Require a live handle, then the existing MapValueBorrowString live-object branch; missing handle is InvalidContract, never Integer. |
| Boxed sum | Only an already validated boxed-handle ABI qualifies as Handle; unboxed variants require their own producer contract. |
| Void including Null | Require canonical zero payload and materialize VoidBox. Both are runtime Void under the language contract, distinct from Integer0. |

The runtime owner is `plugin/map_slot_store.rs` plus the existing live-object
branch of `value_codec/decode.rs`; extract that branch for compatibility reuse
without calling the permissive Any decoder from the selected store. Validate
map, strict String key and value before `MapBox::insert_key_str`. Decode/clone
outside the Map write lock. Preserve String/StringView borrow/materialization
and non-String `clone_box()` behavior; unconditional Arc sharing changes nested
Map semantics. InvalidContract must leave the Map unchanged. Existing fatal
allocator/lock behavior is not a returned source Fault and needs no fabricated
Array FaultFrame. The consumer must take its contract-failure terminal on any nonzero status,
without treating reserved status1 as a source Fault.

Length-aware String materialization is a mandatory dependency, for literal
keys and selected String values alike. Current same-module globals/boxing,
const hoisting and string-const helpers use `strlen`, so fixing only the store
would still truncate embedded NUL. Retain exact JSON byte length through the
existing C string-constant owner and LLVM byte emission, then use a length-aware
UTF-8 runtime entry. That entry can reuse
`exports/box_helpers.rs::string_literal_handle_from_text(&str)` and its existing
content-keyed cache; no second cache or pointer-based identity is needed. The
reference fixes nonnull readable bytes, target-size range and invalid UTF-8
return0; arbitrary pointer validity remains a caller precondition.
Do not reject NUL or change literal-key acceptance to avoid this work.

Copy/PHI/Select must retain the projected representation on the exact supplied
SSA edges. Prefer private tag/payload lanes over boxing on predecessor edges:
boxing early would move allocation before the Map/key/child evaluation sequence.
Mixed representations need those explicit lanes or an existing validated ABI;
raw-bit tests, absent origins and unseeded cycles are not evidence. Polymorphic
operation names alone do not establish their result ABI.

### Published operand projection inventory

Boundary: Map value operand -> reachable published SSA producer and selected
call ABI -> tag/payload consumed by C. Includes the producer categories below;
excludes source type inference, compatibility method re-resolution, V4 and
nonselected backend expansion. This is a finite audited inventory, not an
Exhausted claim over every MIR instruction or an implementation authorization.

| Producer | Projection or remaining contract |
| --- | --- |
| Scalar/String/Null/Void Const | Exact Rust ConstValue before JSON; Float payload is `to_bits()`, String uses exact byte length. |
| Intrinsic allocation | Construction target plus admitted allocation ABI yields Handle. |
| Named allocation | Only the selected physical allocation/typed-object plan proves Handle; retain `unsupported_newbox_type` for unsupported allocation. |
| Copy | Retain the source projection through alias rewriting. |
| CopyOwned | Demand traversal retains the ownership operation; selected C keeps the existing ownership capability Stop, never emits it as Copy. |
| PHI/Select | Retain all exact incoming values/edges or condition/arms, selecting both lanes; no unknown-to-known default. |
| Integer Add/Sub/Mul/Div/Mod | Require exact integer operands and the admitted integer opcode contract. |
| String Add | Require an admitted String operation's handle-return contract, not origin heuristics alone. |
| Compare/logical Not | Bool result only after input operation validity is established; normalize payload to0/1. |
| Other unary/binary | Preserve admitted consumer contracts and `unsupported_unop_kind` / `unsupported_binop_kind`; never use the generic integer path for Float arithmetic. |
| Selected Static/Free call result | Exact canonical key/definition and existing checked Integer return ABI only; not generic Call or unknown signatures. |
| Print | No value result. |
| Function formal | CutoverBlockerOpen: `SelectedPublishedFormalValueDomain`, below. |
| Boxed sum | Existing valid ABI plan and emitted boxed handle only. |
| Compatibility/mixed return or another reachable producer | Needs its own already admitted exact ABI or an authorized terminal; neither origin absence nor legacy success proves representation. |

Physical owner reuse for the planner: Named allocation needs the existing selected
allocation plan (`typed_object_plans` plus its validation and C allocation
consumer), not Named spelling alone. Compare/Not require exact operand domains
that match the existing operation consumer; generic i64 truthiness is not input
admission. String Add needs exact String producers and concat ABI, not a generic
Handle tag. Boxed sums use `build_function_boxed_sum_site_plan_map` and the
matching emitted VariantMake site, not `ValueRepresentationFact` alone.
No general Named-allocation or Compare/Not input-admission API was established
by the bounded physical-owner audit. These mappings must remain explicit in
Step1; the borrowed `map_body_index` dependency closure does not admit leaves
or supply their missing physical contracts.

Decision: the existing value row also carries a finite physical operation
selection for demanded I64 binary, I64/Bool/String comparison, String concat,
and I64/Bool Not. Result representation alone cannot select the consumer:
String and Integer comparison both yield Bool. The planner validates exact
operand domains and the admitted opcode before issuing this selection. C
checks and consumes it against the same body instruction; no operand copies,
String subtype on every value, source classifier or second graph are added.
Operation actions require their original producer and keep existing non-Map
uses. Float/mixed/unknown domains cannot default to an Integer/String mode.
This corrects the unpublished v2 schema before planner/consumer cutover.

Physical audit evidence: `hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc`
calls `emit_dynamic_string_or_icmp` for Eq/Ne. That helper probes raw values as
String handles, so distinct Integer payloads coinciding with equal-content
String handles can select String equality. This is a static counterexample,
not a reproduced runtime failure. Demanded exact Integer comparisons must use
direct integer comparison in both walkers, bypassing that helper. Bool needs
correct i1/i64 normalization; exact String comparison/concat uses its existing
String consumer. Not's raw zero test only establishes the Integer/Bool case;
String, Float and Void need their own contract and are not admitted by it.
`ownership_backend_capability::enforce` already rejects CopyOwned/DestroyOwned
for `ny-llvmc-obj` with `backend-missing-capability:owned-value-lifecycle-v1`.
Demand propagation tests do not remove that Stop or prove identity-preserving
execution. Named StringBox's arg0 alias is likewise not evidence that arbitrary
arg0 is a String handle; Named/typed-object admission remains plan-specific.

The Float issue is concrete: generic prescan registers every non-String Const
using `yyjson_get_sint` and `publish_plain_i64_value`; generic dispatch skips
non-i64 non-String constants. Same-module prepass does not admit Float. Adding
an F64 tag to that existing register is wrong. The private projection must emit
exact Rust Float bits directly and preserve them through Copy/PHI/Select; source
Float admission is unchanged. This does not authorize Float arithmetic or claim
that existing generic Float execution already has a sound unsupported terminal.

`SelectedPublishedFormalValueDomain` is a real Map cutover obligation. The
natural source counterexample (static reachability audit, not executed) is:

```hako
static box Helpers {
  stash(x) {
    local m = %{"v": x}
    return 7
  }
}
Helpers.stash(1)
Helpers.stash(true)
```

The result solver records local `m` as KnownNonI64 but proves Return7 ExactI64
with no required Integer arguments. `script_direct_static` and normal Script A
retain this as ExactI64Empty. The actual caller
`calls/script_direct_static_physical_bridge.rs` consumes that proof, lowers all
actuals through `AssociatedMethodCallArgumentsV1::lower_all` and emits the
canonical static Global. The definition's Outside route retains selected source
transport into raw lowering and cataloged definition commit. This does not claim
that today's Map C execution works; it proves that rejecting Bool as an
unselected source family would be wrong.

Formal identity/ordinal belongs to the declaration and parameter binding owner;
actual kinds belong to their existing literal/operation producers. Canonical
key + arity + ordinal supplies actual/formal correspondence. Unannotated formal
is MirType::Unknown in `prepare_static_method_signature`; Integer return proof
constrains only its required return arguments, not this Map operand. C's uniform
i64 signature supplies width, not a value-kind authority.

Decision: the frame's borrowed index computes a finite physical representation
closure before choosing Operation rows. Integer/Bool/F64/Void/String/Handle are
distinct observations of issued producers; Unresolved is not the empty set.
Copy, all PHI inputs, both Select values and every exact incoming actual union
their observations. First solve to a fixed point, then mark still-empty cycles
Unresolved and propagate again. A seeded PHI/Copy or recursive formal cycle is
not rejected merely because its first visit had no observation. Select keeps
both value arms even when its condition is currently constant; this pass does
not choose control flow or specialize the formal ABI.

Operation selection uses the internally completed map and requires its input
domain to support every observed combination. Compare/Not's provisional Bool
result is not input admission: all demanded operations are checked before the
selection map returns. String and arbitrary Handle remain separate, and an
unknown input cannot disappear beside a known Integer. This is physical
validation of already issued operations, not source type inference or a new
semantic proof. The existing internal-target/all-ingress cutover obligations
still apply; exact-call inventory alone does not close external callers.

`map_value_domains.rs` implements this bounded closure/operation selection.
An empty operation map does not prove that all leaf producers are admitted.
The complete projection map consumes the shared leaf mapping and existing Named
query binding; ownership capability and actual C materialization remain separate.
A Rust TypedObjectPlan match cannot choose the C NewBox consumer: generic builtin
arms precede typed plans, StringBox may alias arg0, and the same-module consumer
has a different accepted subset. The retained C query supplies this conditional
choice. Full frame/C consumption remains open; neither copying a Named-spelling
classifier into Rust nor rejecting all Named inputs closes that obligation.

### Named allocation emission selection — shared physical owner

Decision: retain allocation selection in its existing C physical owner. The
two actual Named emitters use one side-effect-free selector in
`hako_llvmc_ffi_named_allocation_select.inc`, consuming the active walker, target/dst/arg0, array-store choice and
existing borrowed typed-plan facts. It performs no getenv, source/provider
lookup, register publication, materialization or emission. Preserve lazy typed
plan validation: a builtin or successful alias must not be rejected by an
irrelevant invalid typed plan. Keep emitter effects and failure handling in
their existing consumers; do not add a public query ABI in this BoxShape.

Boundary: existing generic/same-module Named emission -> selected consumer and
current success/error terminal. Includes both actual emitters below; excludes
source admission, intrinsic allocation, prescan changes, Rust leaf/frame
binding, C query exports and Map production switch.

| Existing choice | Selector disposition / preservation requirement |
| --- | --- |
| DirectArrayI64 / ArrayBox | Existing birth consumer; ArrayBox's exact-store choice is an explicit input. |
| MapBox | Existing Map birth consumer. |
| FileBox | Generic env.box.new consumer only; same-module keeps its prior typed-plan fallback/unsupported behavior. |
| StringBox with dst and arg0 | Generic AliasOperand(0), not an issued String or Handle kind. Keep materialization, alias and origin side effects outside selection. |
| StringBox without successful alias | Preserve generic typed-plan fallback; same-module retains its existing own selection. |
| Remaining Named + typed plan | Only reached after the prior arms; preserve plan identity, type_id/field_count checks, and consumer-specific invalid/missing terminal. |

Retired within this extraction: duplicated Named priority/selection branches in
`hako_llvmc_ffi_pure_compile_generic_newbox_emit.inc` and
`hako_llvmc_ffi_same_module_typed_object_emit.inc`. Acceptance: full selected-C
build plus focused selector/emitter evidence for both walkers, all above arms,
builtin/invalid-plan collisions, alias success/failure and missing/invalid plan;
preserve return codes and selection-side-effect ordering. Reuse existing guards.
This closes two emission selections, not allocation-wide caller-zero.

Prescan is explicit remaining observer debt. Generic StringBox prescan does not
fall through to a typed plan when alias is absent, unlike emission; same-module
prepass also observes typed-plan presence before the emitter's full validation.
Changing either during this BoxShape would mix behavior repair into extraction.

### Allocation preflight ownership and ordered implementation

Decision: the eventual selected static invocation uses one C-owned, parse-once
session. It owns the `yyjson_doc`, borrowed program view, existing definition
plan and fixed allocation settings. Preflight and compile consume this same
session; reopening a pathname or repeating selection is not input binding.
The Rust host retains the original published view, serialized body and loaded
library until the session closes. It runs the MIR-side frame planner between
C calls, without a C-to-Rust callback or MIR dependency on dlopen/host providers.
This is physical invocation ownership, not a new semantic receipt.

Boundary: selected published host invocation -> C program/definition role ->
Named allocation consumer. Includes entry, dedicated Dynamic launch, ordinary
same-module definitions, numeric leaf eligibility, exact site disposition and
config lifetime. Excludes source admission, unrelated backend/config cleanup
and runtime store semantics. The following is the finite role inventory, not
a claim that all preflight consumers are implemented:

| Existing owner | Role rule to preserve |
| --- | --- |
| `pure_compile.inc::hako_llvmc_read_generic_pure_program_view` | Ordinary entry selection; selected Dynamic identity selects the helper as generic entry. |
| `selected_launch_emit.inc` | Dynamic launch explicitly uses the same-module pipeline as `ny_main`, independent of ordinary definition-plan membership. |
| `physical_definition_plan.inc` | Production caller supplies entry metadata; retain registered order and both planned sets. |
| `same_module_function_definition_emit.inc` | Same-module eligibility excludes an already emitted definition and a numeric body also planned as leaf; missing target/entry is skipped at its existing stage. |
| `module_leaf_function_emit.inc` | Existing numeric-body validation and leaf membership determine leaf emission; leaf-only nonnumeric bodies do not become same-module automatically. |

The existing planned leaf/same-module storage, metadata reader and membership
operations now reside in `hako_llvmc_ffi_physical_definition_plan.inc`, one
private physical definition plan with explicit arguments. Production membership, iteration and declaration
consumers use that owner directly; no preflight-only duplicate planner. Keep
separate leaf capacity256 and same-module capacity1024, registration order,
deduplication, duplicate-row return count, partial progress on malformed input,
null/missing metadata result0 and malformed result-1. The same symbol may be
present in both sets: one exclusive role enum would change this contract.
Emitted registries remain separate progress state. Numeric-body validation,
eligibility/skip timing and Dynamic launch stay in their existing consumers.

Retired in this extraction: the independent planned arrays/counts and add/membership
implementations in `module_leaf_function_emit.inc` and
`same_module_function_plan.inc`, plus direct array iteration in
`same_module_function_definition_emit.inc`. Preserve overflow diagnostics
(including the existing leaf/same-module difference), not just success values.
No query export, session activation, prescan repair or Map source switch belongs
in this extraction. Acceptance: full C build and existing published/corridor
proofs; parent comparison for both-set numeric/non-numeric bodies, duplicate and
missing targets, invalid/null metadata, capacities and Dynamic helper/launch.
Do not replace actual role coverage with a guessed role supplied by a test.

Implemented BoxShape: the selected pure-first document boundary is
single-owned before adding an opaque session ABI. The existing file wrapper
parses once, calls a borrowed `compile_doc_compat_pure` core, then frees once
on every return. Reuse one private borrowed validator implementation from the
existing file validator ABI and compile core; do not add a public validator
export merely to share code. Pinned census and exact-seed readers borrow the
same root, as do rune logging, Dynamic signature/C1 and generic compilation.

Preserve the current order: route/path/output gates -> validator -> pinned
census -> rune log -> exact-seed reader -> Dynamic signature/C1 -> generic
program/emission. Program-view selection stays in its existing stage; the
physical definition plan is still read in generic prescan after the earlier
indexof pattern opportunity. Do not eagerly validate that plan at document
creation. Missing or malformed JSON retains the existing file error wording.
No new semantic receipt, source admission, query export or ownership flag in
individual emitters belongs in this extraction.

This census covers selected published pure-first file ingress -> all compile
returns; includes validator/census/route readers and the generic include graph;
excludes downstream legacy exact-seed dispatch/replay and other ABI entrypoints.
The latter retain a pathname and may reopen it; selected typed rows already
reject those paths. A generic-only doc-taking extraction would not establish
parse-once for the selected boundary and is not the selected next slice.

Retired delete-set: repeated document reads/ownership in validator, pinned
census, rune logging, exact-seed reader, Dynamic signature and generic body;
all71 textual `yyjson_doc_free(d)` sites in the audited compile include graph.
The finite include inventory (common `hako_llvmc_ffi_` prefix, `.inc` suffix) is:
`pure_compile`, `pure_compile_generic_lowering`,
`pure_compile_generic_lowering_prescan`, `pure_compile_ir_open`,
`pure_compile_generic_newbox_emit`, `pure_compile_generic_lowering_op_dispatch`,
`pure_compile_generic_lowering_op_dispatch_calls`,
`pure_compile_generic_active_walk`, `pure_compile_variant_dispatch`,
`pinned_text_selected_dispatch`, `pinned_text_provenance_block_dispatch`,
`selected_launch_emit`, `selected_dynamic_entry_header`.
The signature/census/exact-seed documents and validator's document are additional
owners to retire; the source count is an audit observation, not a new guard quota.

All included consumers borrow the wrapper's document. Pattern success, early
error, normal return, GEN_ABORT and GEN_END return to that wrapper before its
one free. Retain FILE, owned string, PTFB session and PTFC draft/bytes cleanup;
do not combine their pre-existing cleanup differences with document ownership.
Acceptance: selected C build/published/corridor proofs; Named60, role22 and
Dynamic parent comparison; instrument parse/free ownership on normal success,
malformed JSON/schema, exact-seed and Dynamic rejection, generic abort and
pattern success. Prove one successful parse/one free and no internal document
free, including rejection paths. Use available memory checking for lifetime
failures; an early preflight rejection alone cannot validate emitter cleanup.

Verification: selected C build, published rows and corridor pass. Named60 and
role22 match the previous plan-owner baseline; selected Dynamic LLVM, origins,
machine code and relocations match. ASan plus document counters pass eight
cases including source-issued Dynamic and an early pattern return with a
malformed later definition plan. Reproduction is in `lang/c-abi/README.md`.
These observations close document extraction only; no session ABI is exported.

Implemented BoxShape: allocation configuration is captured once at file ingress
and passed as a private value to the borrowed core. This supersedes the two-bit-only
premise: `typed_object_exact_slot_helper_enabled` observes a different mode.
The capture owns unchanged runtime requirement flags (bits1/2) and a separate
`exact_slot_helper` boolean. Its sole issuer reads each of three settings once:

| Captured value | Existing exact predicate |
| --- | --- |
| Runtime bit1 | `HAKO_TYPED_OBJECT_STORE == "direct_slot_exact"` |
| Runtime bit2 | `HAKO_ARRAY_SLOT_STORE == "direct_array_i64_exact"` |
| Private helper boolean | `HAKO_TYPED_OBJECT_STORE == "single_thread_exact"` AND `HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER == "1"` |

Do not encode the helper as runtime bit4 or derive it from direct-slot mode.
No raw string lifetime, new global, semantic receipt or public ABI is needed.
Extend the existing common runtime-mode reader into this sole capture owner;
requirement declaration/check consume its flags without re-reading settings.

Boundary: selected pure compile file ingress -> Named allocation, related
field/method and runtime requirement output. Includes eight textual reads of
these three settings in `common.inc` and five nested predicate owners below;
excludes Rust runtime settings, other environment variables, source admission
and session ABI. The exclusive delete-set is the nested ambient reads in
`generic_direct_array_emit` (extracted from `generic_method_lowering`),
`same_module_generic_method_collection_emit`, `same_module_generic_method_string_emit`,
`same_module_typed_object_emit` and
`typed_object_plan`. The last reads both typed-store and helper settings.
The helper consumers include generic field get/set, same-module field get/set,
`same_module_typed_field_rmw_emit` and `same_module_body_emit`; they consume the
same private boolean. Named-only success cannot prove this obligation.

Acceptance covers typed-store {other,direct_slot_exact,single_thread_exact} ×
array-store {other,direct_array_i64_exact} × helper {other,"1"}: 12 input classes,
six capture outcomes. Also preserve unset/empty/unknown and helper "10"/"01"
non-matches. After capture, reverse ambient settings and check actual allocation,
field/method emission and runtime requirements against the captured value.
Mandatory counterexample: single_thread_exact + helper=1 emits the exact-slot
helper without a direct-slot runtime requirement. Keep Named60/role22/Dynamic,
document lifetime, rejection order and late plan reading evidence unchanged.
This fixes values after capture; it does not promise an atomic multi-variable
environment snapshot, all-config freezing, Map admission or parallel compile.

Capture evidence: C build, Named60/role22 parent comparison and document ASan8
pass. Dynamic LLVM/origins/machine code/relocations match. The mutation driver
passes16 cases; both walkers emit allocation, typed field get/set and Array get
from captured settings. Common is the only reader of these three settings.
The touched1104-line generic method file is split at existing function boundaries
into dispatch, direct Array and String slot-store includes, each below800.

Session decision: conditional physical representation and execution coverage
are separate obligations. This supersedes the earlier premise that all pattern/
skip outcomes must be known before Named query. Query states only: if this exact
site in the retained document is processed by the existing walker, its consumer
is X. It does not issue source admission, reachability, success or Bypassed.

The selector reads only target/dst/arg0, captured Array setting, walker and root
`typed_object_plans`. Existing `program.entry` document identity selects generic;
ordinary definitions exclude that same entry, and other Named-emitting bodies
(including Dynamic launch) use same-module. NewBox cannot pass numeric-leaf
validation. Definition-plan eligibility is therefore not a prerequisite for
conditional representation and stays at its original late read position.

Scan Named sites once with this existing selector and retain immutable outcomes,
including Invalid/Unsupported. Unrequested observations do not reject the whole
compile. Bind required observations to original MIR for Handle versus Alias(0)
projection; actual dispatch consumes the same retained outcome without selecting
again. Observation/peek never marks a frame obligation consumed. Only promoted
frame obligations require coverage before object publication; ordinary unused
observations do not gain an exact-once requirement.

Dynamic launch can also be emitted as an ordinary definition. Both use the same
conditional same-module consumer, but physical-instance coverage stays separate:
immutable observations may be read repeatedly; required rows retain their exact
consumption contract. Do not hide duplicate take with a symbol alias. Multiple
required physical instances need an explicit mapping by the actual emission owner.

Pattern emission can fail and continue generic, and reset-batch matching can
retain partial skips after rejection. These counterexamples remain real but do
not block conditional representation. Needed Map obligations replaced by an
optimizer require coverage from the actual successful replacement owner; do not
copy every matcher into query, pre-mark rows used, or permanently reject whole
pattern families. Replacement/instance coverage remains CutoverBlockerOpen.

Implemented BoxShape: the existing Named selector and root typed-plan
lookup are private explicit-root functions. Move the original lookup and numeric
reads, preserving first matching plan and builtin/File/alias-before-plan laziness.
Both actual emitters consume that same owner; all existing typed-plan callers
pass their current root. Delete lexical root/nested-helper dependencies, without
retaining a query-only selector or duplicate lookup. Keep symbol/slot semantics,
config capture and missing/invalid distinctions unchanged. Acceptance: selector
lazy negative cases, duplicate-plan first match, Named60 parent behavior, both
walkers' String difference, settings16 and C build. No public query/session ABI
or Map source switch in this extraction. C build, selector lazy negatives,
actual duplicate-plan/document-isolation test, Named60/role22 parent comparison,
settings16, published rows and corridor pass. All changed sources stay below800.
Implemented BoxShape: a private Named outcome owner borrows the same document
root and captured config. One scan records both finite walker conditions for
Named NewBox sites. Internal identity is function-pointer/instruction-pointer;
walker chooses its retained result. Use indexed lookup rather than repeatedly
scanning all rows. Names/coordinates are not private identity; eventual wire
uniqueness remains separately checked. No strings/operand graph are copied.

Named means `op=newbox` with no `target` member at all: null/malformed/unknown
intrinsic targets stay outside this owner. Existing intrinsic take remains before
lookup. InvalidPlan/Unsupported are retained, not compile-level early errors.
Array shape errors stay with existing validation. Checked allocation overflow/OOM
marks the table incomplete; actual Named lookup reports storage failure without
selector retry. Earlier schema failure/pattern success retains its old outcome.
A future query requiring completeness can report its own storage error.

The file wrapper initializes the owner and destroys storage before freeing the
document on every return. The direct borrowed-core config test follows the same
lifetime. No frees are distributed into emitters. Actual generic/same-module
NewBox consumers use lookup; delete both direct selector calls. Immutable reads
can repeat and do not set typed-row used bits or imply successful admission.

Acceptance: Named60/selector negatives/settings16/Dynamic comparison; repeated
lookup without reselection; duplicate names/block IDs remain separate by identity;
all target-member cases excluded; unreachable bad observations preserve failure
order; injected storage failure preserves schema/pattern outcomes but fails an
actual Named lookup; storage is destroyed before document on all paths. Preserve
existing duplicate-take/residual row checks. This is private production binding,
not public session/query ABI, Map frame/source switch or optimizer coverage.
Verification: C build, selector/outcome unit tests, Named60/role22 parent
comparison, settings16, published rows/corridor and ASan lifetime16 pass.
Dynamic LLVM/origins/machine code/relocations match. Fault injection caught the
new same-module lookup error returning-1 to a boolean caller; it now returns0
and both walkers reject without an object. Storage growth failure also releases
partial rows before the document. Both actual direct selector edges are deleted.
Next design: bind the existing program/definition owners and retained outcomes
into one private invocation before exposing session/query. Audit issue/lifetime,
conditional walker selection and actual host/frame consumer together; no
query-only sibling, optimizer coverage inference or source switch is authorized.


Implemented BoxShape: diagnostic side effects are removed from the existing
`PhysicalDefinitionPlan` reader/add operations. Retain read result and minimal
borrowed same-module overflow detail in that same plan. The sole production
consumer remains generic prescan at its original position, notifying overflow
first and generic plan failure second through the existing first-error owner.
Read position, partial membership, duplicate count and both capacities stay put.
Exclusive delete-set: reader/add -> compile lexical diagnostic dependency.
No new semantic receipt, query export, pattern/skip repair or source switch.
Acceptance: owner test without a diagnostic callback, missing/null/malformed and
partial input, leaf/same capacity difference, parent diagnostics/role22/Named60,
pattern success before malformed plan, document lifetime and Dynamic comparison.
Verification: owner unit test has no diagnostic callback; role22 (including
capacity diagnostics) and Named60 match parent behavior. C build, published
rows/corridor and document ASan8 pass, including the early pattern return with
a malformed later plan. Dynamic LLVM/origins/machine code/relocations match.
Next, exact-site role/skip/host binding remains design work; do not mark the
full session contract executable from this local change.

Implemented BoxShape (read-only C worker and Rust host audit): consolidate
existing document/config/Named outcomes/program/definition storage into one
private invocation, initialized in its final stable address. Outcomes borrows
its config; never copy or return the initialized owner by value. The wrapper
and direct-core config test transfer document ownership to it and destroy it
once (outcomes before document). Core borrows it; no independent program/plan
copies remain. Mutable function/cursor/selection/lowering state stays local.
Program and plan readers run once at their existing ordered positions, with
unchanged diagnostics and pattern precedence. No readiness flags or external
query exist yet; zeroed unread storage is never published as an empty success.
Delete split wrapper ownership/cleanup, separate core product storage and
separate document/outcome arguments. Acceptance: both callers, lifetime16,
settings16, Named60/role22, Dynamic comparison, C build, published rows/corridor;
no source/public ABI change. Rust audit confirms the sole static host caller,
plugins stub and existing frame-first capability order; those switch only with
the complete session/frame/C consumer. This slice consolidates the production
owner chain, not a query-only sibling.
Verification: shared plan unit, C build, Named60/role22 exact parent comparison,
settings16, ASan lifetime16 and published rows/corridor pass. Dynamic LLVM,
origins, machine code and relocations match. Production program/plan readers
still have one call each in their original ordered core sections. Reader and
symbol predicate are shared file-scope implementations, not nested copies.
### Query-to-frame binding decision

C worker audit at79c919352e and Rust consumer audit close the conditional role
question. For a Named site, `function == program.entry.fn` selects its retained
Generic condition; every other potentially Named-emitting function selects
SameModule. This is conditional representation only. Numeric leaf cannot consume
NewBox; definition eligibility, actual emission and optimizer coverage remain
with their current owners. Do not reopen definition readiness or whole-skip
census as prerequisites for this query.

The existing program reader is an effect-free observation. When an actual query
caller is connected, cache exactly `Unobserved / Available / Unavailable` in the
same invocation. Query may observe it once; compile consumes the same result at
its current program stage, retaining all earlier schema/pinned/exact-seed/Dynamic
gates and the old trace/break. Definition plan still reads at current prescan.
Do not add this state before its real query caller or interpret unread data as
an empty valid program.

Internal function/instruction pointer identity remains authoritative. Copied
wire coordinates require complete string-length/NUL/duplicate-function checks,
integer/u32 block IDs and function-local uniqueness, u32 instruction ordinals,
and no two internal identities projecting to one site. Unaddressable observations
must not alias another site or globally reorder an unrelated compile terminal.
Query binding outcomes are finite: `Bound(consumer, typed-plan projection)`,
`ProgramUnavailable`, `Unaddressable`, `StorageFailed`, `NotObserved`.
InvalidPlan/Unsupported remain Bound consumer outcomes, not overall discovery
failure. A required binding failure rejects without selector retry; unrequested
observations never imply admission, execution, bypass or row consumption.

Ordered implementation inside the existing V2 series:

1. Connect copied physical Named observations to the existing `MapBodyIndex`
   and planner, changing demand/domain/original-lane closure together. This is
   implemented planner dependency; source issuers, host and public ABI stay put.
2. Connect the same invocation's query, unique coordinates and program cache to
   the actual host/frame consumer. Retain bytes/Library/handle across planning,
   close before Library unload on every path, and compile those same observations.
   Do not export an isolated query or keep a query-only alternative authority.
3. Finish the V2 frame/C consumers and input-aware capability, then switch the
   static host/source once. Delete V1 host/stub/frame/file transport and the six
   literal edges at the series cutover; no V1 Map fallback.

Implemented Named-binding contract:
- Owner/issuer: existing C Named selector issues physical consumer observations;
  existing published-frame/MapBodyIndex owner binds copied outcomes to the exact
  original MIR site. No new semantic Verified/Prepared wrapper or name classifier.
- Bound host-backed Array/Map/File outcomes provide Handle; DirectArray and
  TypedObject outcomes reject demanded projection until their escape owner exists.
  AliasOperand0 follows that exact
  Named NewBox's original `args[0]`, never a copied operand graph. Missing/wrong
  site, duplicate binding, missing alias operand and required Invalid/Unsupported
  reject; observations for intrinsic allocation cannot grant Named admission.
- Alias dependencies join the existing finite fixed point, including aliases
  discovered through another alias, PHI/Select and canonical actual/formal edges.
  Alias domain comes from its operand; Handle domain requires the observed
  host-backed consumer. Preserve Unresolved for all other unproved producers.
- Add an explicit Named-alias physical action to the existing V2 vocabulary;
  ordinary Copy is not a NewBox opcode. Map-only Float retains exact bits without
  manufacturing an old lane. If an original consumer needs the alias result,
  propagate its old operand demand through the same original MIR relation.
- Exclusive delete-set: demanded Named allocation's unconditional demand-leaf,
  unresolved-domain and default all-args original-demand treatment. Non-Map
  consumers and non-admitted producer boundaries keep their existing contracts.
- Acceptance: renamed Named targets cannot change an observed disposition;
  Handle versus Alias(0), exact Float bits, nested alias/PHI/Select/formal closure,
  Map-only versus mixed original-use demand, malformed/missing/duplicate bindings,
  required Invalid/Unsupported and unrequested failures. Reuse existing body-index,
  domain, original-demand and V2 row tests; run the selected focused Rust gate.
- This step is frame-planner dependency implementation. Synthetic observations
  are not production query, source-to-OBJ/EXE, Map cutover or R7 evidence. Next
  checkpoint must identify actual host/query wiring still open, not invent a
  standalone backend capability or mark all Named sites admitted.

Verification: `MapBodyIndex::with_named_allocations` now validates exact Named
sites and duplicate/alias operands and consumes the index on binding failure.
Demanded missing/InvalidPlan/Unsupported fail; unrequested invalid observations
are retained without granting coverage. Demand/domain/original propagation all
use that bound outcome. `NamedAliasOperandZero` is V2 action8 in Rust/C vocabulary,
separate from Copy; original MIR still owns the operand. Focused `map_literal_`
passes28 tests, including four new binding witnesses with renamed targets,
exact NaN bits, nested aliases, PHI/Select/formal cycles and mixed old uses.
Initial new PHI fixture omitted mandatory `type_hint`; corrected before the
passing run. No baseline waiver or source acceptance change.
### Internal query integration — verified dependency

Read-only C/host audit atde2113241d confirms static V2 is currently schema-only:
no C validator/emitter consumes `hako_llvmc_published_static_frame_v2`; lifecycle
physical_v2 is unrelated. At that audit Rust had no FrameHeader builder or production
caller; owned candidate rows are now implemented below. Full frame/C consumption is therefore mandatory before public session
cutover, but not before private query-to-planner dependency evidence.

Freeze the remaining order: private query plus real MIR binding/same-invocation
compile evidence -> complete V2 frame/leaf/expanded-function/capability and both
C consumers -> atomic public session/host/source switch and V1 retirement.
Do not repeat readiness/definition/optimizer census or add a new helper-only
production milestone between these steps.

Next implementation owner is `HakoLlvmcInvocation`. Add private query using its
existing Named outcomes and the accepted one-time program observation state;
project unique wire coordinates and the finite query outcomes above. Core
consumes the cached program at its old stage; definition reading stays late.
The existing typed rows begin/finish/end lifecycle is shared with an internal
retained-invocation compile path. Keep V1 public gates/rows validation/file-read
order intact; do not copy gates, bypass them with a test-only direct-core compile,
reparse after query, or retry a failed retained compile through the file wrapper.

One test-only subprocess driver may include the actual private C implementation:
keep the invocation alive, send query observations to a Rust test, then await
compile/cancel after binding through the actual `MapBodyIndex` API. No new
shared-library query export is needed. Non-Map physical witnesses exercise the
shared rows lifecycle and real object emission; Map witnesses cancel after
planner validation and never enter V1 compile. Planner failure/cancel destroys
the same owner. Retire this temporary driver/protocol when public V2 host tests
cover the connection; it is not a second runtime or compiler authority.
For a Map cancellation witness, the production view remains unsupported until
V2. Use the existing test-only unpublished-candidate exporter, borrowing the
same module without clone/refresh; assert published body export still rejects.
Non-Map compile evidence uses the production published-body exporter. Neither
fixture is source admission, and no test may relax the production Map stop.

Exclusive delete-set: duplicate program observation across query/compile,
typed rows lifecycle's mandatory pathname compile coupling, and hand-supplied
observations as the only MIR binding evidence. V1 public export/file transport
remain until final cutover and are not counted as deleted in this dependency.
Acceptance: actual query -> Rust binding, parse/selector/program read once,
query leaves globals inactive, real compile activates/finishes/clears rows,
Generic versus SameModule/Dynamic identity, duplicate/unaddressable sites,
required/unrequested bad outcomes, query storage error, planner cancel and
compile failure/success cleanup, existing duplicate/residual/failure ordering.
Reuse Named60/role22/lifetime16/settings16 and focused MIR tests as affected;
no new mandatory guard or public ABI, no Map OBJ/EXE or source claim.

Verification: private query -> actual Rust binding -> same-invocation compile/
cancel passes the ignored integration test (three scenarios). The non-Map input
file is removed before compile; the linked object exits30. Map-only Float and
planner rejection both cancel with document freed once and no artifact. The
instrumented driver verifies parse/free once, program read at most once, unchanged
selector count, and inactive rows before query/after compile. Query17 includes
Dynamic helper/launch roles, duplicate/NUL/u32 coordinates, target exclusion,
unavailable program, storage failure and residual-row/consumer failure cleanup.
Named60 and definition22 match parent results; ASan lifetime16, settings16,
published rows and corridor/pointer guards pass. Dynamic LLVM/provenance and
machine/relocations match parent. Focused map tests pass28 plus the separately
run ignored integration witness. Initial fixture omissions (route/definition
plans and using production export for an intentionally unsupported Map) were
corrected; no production gate was relaxed or baseline waiver taken.

Next: complete the already accepted V2 frame, leaf/expanded-function projection,
capability and both C consumers, then atomic public session/host/source cutover.
This checkpoint removes duplicate program reads and pathname-only internal
compile coupling; it does not retire public V1 or open Map production.


Bind the same borrowed core, existing program view and
physical definition plan to the eventual opaque session. Open takes immutable
JSON bytes+length, leaves its output handle null on error and activates no
global rows. Query borrows session-owned observations. Compile consumes the
session once on success or failure; it activates rows/context only for the
execution and clears them on all exits. Close frees document/owned observations
before Rust unloads the retained library, including early planner error/drop.
Rust's MIR frame planner receives exact-site dispositions only; it never loads
C or infers a role from a function spelling. The current fixed
`hako_llvm_in.json` transport is not retained in the eventual selected session.

Host binding inventory: `published_mir_object.rs::compile_published_view_object`
is the sole static production caller of
`capi_transport::compile_published_static_method_v1`; OBJ and EXE already share
that private branch. The lifecycle V4 branch is outside this static cutover.
Replace the static frame-first/file-write/call/file-remove sequence together,
including its plugins-disabled stub, only when the complete frame/C consumer
is ready. Other compatibility file consumers keep their existing owner.

The host-private scoped invocation retains serialized bytes, loaded Library,
resolved function pointers and opaque handle through query, MIR planning and
compile. Close the handle on planner error and normal/error compile exits before
Library destruction; no query pointer escapes that scope. Keep C error-buffer
ownership explicit. The MIR planner sees copied exact-site physical observations
bound to the original borrowed view, never `libloading` or a C-to-Rust callback.
Byte serialization uses `emit_published_view_body` from that same view; it is not
a replacement MIR or a fresh source resolver.

The static capability path currently calls whole-module exact-numeric validation
before frame construction (`backend_capability.rs`). Its eventual Map-aware
acceptance must consume the actual completed static frame for covered values,
while retaining compatibility-ingress, ownership and all unrelated checks.
V4's input-aware numeric boundary is a pattern, not permission to skip static
validation or select a different backend spelling.

Original-lane binding in `map_original_demand` now handles admitted Named aliases
through the bound original MIR operand, consistently with demand and domain
propagation. A Map-only Float alias does not force an old lane through NewBox;
a real original consumer still propagates that operand demand. Complete frame
and host consumption of this binding remain CutoverBlockerOpen, not optional
post-cutover cleanup.

Settings that affect allocation/related method consumption must share a captured
value through query and emission; changing only the two Named reads does not
freeze all compile configuration. Existing global rows, other ambient settings
and temporary LLVM names remain explicit concurrency debt. This design does
not advertise parallel compile or hold active globals while Rust plans.

Query the retained program's Named sites once, returning exact-site consumer or
explicit unavailable/invalid outcome. Do not invent a role for an un-emitted or
bypassed body, or reject a whole program merely because discovery visits an
unrequested unsupported site. Rust binds relevant outcomes to the original
`MapBodyIndex` instructions before domain/leaf admission. Handle follows the
selected host-backed consumer; AliasOperand(0) follows the original MIR operand,
including exact Float bits. `map_body_index.rs` now follows bound alias operands
in demand, domain and original-lane propagation together, without manufacturing
an old Float lane or copying an operand graph. A new demanded alias input uses
the already retained Named outcomes rather than triggering another role query.

CutoverBlockerOpen until session/frame/C connection closes: generic StringBox
prescan versus alias/typed-plan fallback, same-module early typed-plan facts,
indexof/pinned-text/region/skip bypasses and every unconsumed selected site.
V2 must preserve selected exact-seed/replay rejection and account for dedicated
launch and expanded Map formals that are not ordinary numeric leaves. Global
layout validation and later consumer failure remain distinct from a selector's
successful choice. The shared selector and extracted plan alone cannot claim
allocation-wide admission, Map OBJ/EXE execution or legacy retirement.

Decision: retain kind/payload across **Map-demanded formal positions**, using
the existing published input/frame's physical projection and canonical call
edges. Keep one definition/body; do not specialize by observed constants or add
source annotations. Propagate demand through exact Copy/PHI/Select and caller
actual/formal correspondence to a finite fixed point. Preserve both lanes at
needed call boundaries, even when two current callers happen to use the same
kind; a first-call or currently monomorphic inference is not a permanent ABI.
Non-demanded formal positions retain their existing contract. A changed
physical signature must be projected consistently at definition and every
admitted caller, under an explicit compiler-input revision; the current v1 C
row layout cannot be reused with silently changed field meanings.

### Complete demanded projection mapping — physical escape boundary

The existing `map_projection` owner supplies one domain/action mapping from
original constants, canonical Integer-result calls and exact site plans. Bound
Named Array/Map/File outcomes use the registered host-handle representation;
DirectArray and TypedObject outcomes reject when demanded. VariantTag is I64;
Project of Integer/Bool/String retains its exact storage/type agreement. Unit or
payload VariantMake results and general Box/Array/Future/Weak payload escape have
no proved Map representation yet and reject. Local construction needed only to
produce a scalar/String projection is not itself a Map value demand.

This supersedes the earlier allocating-outcome/VariantMake => OriginalHandle
premise. Kernel execution at parent9b2931a42c found valid direct-array and typed
object tokens rejected by literal_store kind5 (status2, no insertion), while a
registered String handle succeeds. Legacy collection tests that round-trip raw
bits as IntegerBox do not prove object category or lifetime.

| Physical representation | Existing owner | Current Map disposition |
| --- | --- | --- |
| Registered host handle | host_handles / Arc-owned NyashBox | Handle5 |
| Typed negative index | reclaimable typed-object store | CutoverBlockerOpen |
| Direct-slot typed pointer | configured TLS typed-object store | CutoverBlockerOpen |
| Direct-array tagged pointer | TLS direct-array buffer | CutoverBlockerOpen |

Decision: keep Handle5's existing meaning. Reject incompatible demands before
artifact rather than changing allocation selection or guessing kind from bits.
C checks the actual captured consumer for Named and intrinsic Array. Rust has
exact Named outcomes, but intrinsic configuration/representation still needs
same-session binding before public activation. These are included blockers, not
parked families or a smaller replacement completion target.

The runtime owner must issue any escape keep/promotion with explicit identity,
shared mutation, reclamation, lifetime and thread-transfer contracts. NyashBox
requires Send+Sync; raw TLS indices/pointers cannot establish those guarantees.
The direct-slot materialized-view helper is test-only and is not a production
bridge. No new token wrapper or semantic receipt is authorized by equal i64 width.

Transfer/formal lowering may proceed independently over the admitted domain,
preserving every explicit unsupported result. Before full public cutover, close
these physical representations and the required source/Map readback acceptance.
Focused `map_literal_` Rust tests verify the corrected refusal and retained
scalar/String/alias/call mapping. Source cutover and general object escape remain
unproved; the full Map series retains both obligations.

### Runtime escape ownership — next design boundary

Decision (2026-09-09): source Map-slot destination/transfer comes before runtime
retention. The [owned-slot target](../../../../reference/language/ownership.md#intrinsic-map-slot-destination-target)
is accepted: transfer existing compatible Home/carrier obligations, never invent
share from insertion. Runtime-first SafeMutex retention remains withdrawn.
Source authority + canonical issuer: existing ownership/carrier laws; Map slot
destination/transfer issuer is missing in the raw/Core -> entry -> cleanup boundary.
Non-authority: kind/payload, ValueId/child ordinal, allocation outcome, transport,
token signs, host Arc/clone behavior and test-only snapshots.
Fail-fast boundary: keep current unsupported demands before artifact and runtime
InvalidContract/no insertion; no implicit share, guessed Home or carrier forwarding.
Smallest next slice: close intrinsic Map construction/end responsibility, then
co-seal exact obligation-preserving compatibility under the fixed one-entry law in
[OWN-FIELD-CONTAINER-DEST-D0](../investigations/hakorune-home-ownership-task-2026-08-04.md#map-slot-dependency-of-the-selected-compiler-cutover).
Existing resolver session owns source sites; semantic-package issuance/co-seal
must join candidate/destination and pre/post-commit cleanup states before Recipe.
Those are design assignments, not existing executable Map issuers.
Non-claims: new receipt, runtime implementation permission, whole Home activation,
scalar-only replacement completion, TLS closure or public/source switch.

Source audit covers raw/Core MapLiteral child issuance -> entry operands ->
retained root/cleanup. Child identity/evaluation order is retained, but neither
Home demand/transfer nor Dynamic carrier forwarding into a Map slot is issued.
Passive Home vocabulary, declaration I64UnitTrivial ABI and numeric Array cleanup
cannot substitute. Existing SelfContainedDynamicCarrier has its own exactly-once
lifecycle and must not be collapsed into source Home. The linked existing task
owns the Home/carrier/borrowed-handle destination matrix and its failure/read/
replacement/remove/parent-end commitments; runtime tags cannot select among them.

Independent runtime audit covers actual allocator -> storage -> Map intake/reclaim.
`crates/nyash_kernel/src/plugin/map_literal.rs` resolves host handles today.
Indexed `typed_object_store_backend.rs` owns one Vec<Option<TypedSlotObject>>;
reclaim detaches payload, so a host wrapper alone cannot keep it valid. Direct-slot
and direct Array storage are TLS; snapshot helpers are test-only and lose identity.
Boxed Make uses the same typed allocation. Numeric fields do not retain child Homes.
Map value decode/visible reads clone unknown boxes; replacement/remove/clear can
drop old values under the Map lock. Adding typed-wrapper retain or fini in Drop
would therefore invent share/cleanup timing rather than consume a source contract.

After destination meaning is sealed, reuse the indexed storage owner for the
authorized Move/Shared/carrier operations, preserving a single actual payload.
Do not choose Arc-for-all, a second storage table or a public Handle5 expansion.
Actual profile agreement, child lifetime, shared mutation where authorized,
finalizer thread/lock discipline and TLS/direct storage remain included cutover
obligations. No safe exclusive deletion set exists before the source decision.
Existing codec/kernel tests prove their bounded ABI behavior only, not this
destination/ownership contract or full runtime dependency completion.

### Map construction prerequisite for owned-slot transfer

Decision: intrinsic Map acquisition, key residence and construction/end order
are fixed by the [language lifecycle owner](../../../../reference/language/lifecycle.md#intrinsic-map-construction-and-end).
Allocation Normal acquires the empty Map responsibility; prepared keys are native
entry residence; completion forwards the same responsibility. Live value cleanup
uses reverse successful install order, including replacement and construction
Fault. Child cleanup runs outside storage locks, with first-Fault preservation.
This closes the source policy, not its compiler/runtime implementation.

The [Map destination task](../investigations/hakorune-home-ownership-task-2026-08-04.md#map-slot-dependency-of-the-selected-compiler-cutover)
accepts exact PlainI64NoHook construction/destruction evidence for unchanged
Home transfer; it next connects Normal/transfer flow to root cleanup. A prior
Home list or physical Pending/Emitted state is not that proof. Existing
callable-row/Script-product loans supply same-issuance core and body shape;
no generic root adapter or standalone core extraction is needed.

Runtime follow-up must replace clone-based insertion with the authorized
transfer/detach/end transaction in the existing storage owner, preserving the
native outside-lock teardown boundary. Current HashMap storage carries no successful-install order; keys/values
sort by public key text. Neither behavior proves the new end contract. Required
acceptance includes allocation/preinstall Fault, duplicate replacement, C-then-B
cleanup after a:=A/b:=B/a:=C, old cleanup Fault and ending-state re-entry refusal.
Keep runtime escape Stops until source, physical and actual runtime consumers
close; no second payload table, implicit share or numeric-Array substitution.

### Map-owned canonical object residence

Decision: source PlainI64NoHook compatibility does not make an indexed object a
host Handle5. Keep the actual payload in the existing typed-object indexed store;
the common Map storage owner receives its authorized end responsibility in a
distinct owned residence through the checked intrinsic facade described below. Preserve storage profile, stable storage identity,
exact type and admitted end evidence. No second payload table, field snapshot,
uniform Arc promotion or cloneable pseudo-Home wrapper.

Observed chain: selected `lifecycle_v4_emit.inc` calls
`nyash.object.checked_new_v1`; `exports/typed_object_store_backend.rs` allocates
`Vec<Option<TypedSlotObject>>` and returns `-(index+1)`. `static_v2_emit.inc`
currently calls `nyash.map.literal_store_v1`; its Handle5 path requires a live
positive host handle and clones through `map_value_from_live_object` into
`MapBox.data: HashMap<MapKeyDomain, Box<dyn NyashBox>>`. The actual projection
refusal is `map-frame/named-non-host-handle` for TypedObject/DirectArray, with a
separate boxed-object escape refusal. Neither Stop may be erased by retagging.

Missing physical owner: `MapOwnedCanonicalObjectResidenceMissing`. Required
path is same indexed payload -> Map entry responsibility -> explicit detach ->
existing indexed reclaim. `take_indexed` removes an Option once with exact type
checking; slots never shift or reuse indices. SafeMutex has a process-wide store;
SingleThreadExact uses thread-local storage, so profile/index/type alone is not
cross-thread identity. Preserve origin-thread affinity or reject the unsupported
transfer before mutation; SafeMutex evidence is not TLS evidence.

MapBox is in the root crate while indexed storage is in nyash_kernel, which
already depends on root. The residence interface must respect that dependency
direction. No root-to-kernel dependency or global callback registry is authorized.
Accepted interface: root defines a non-Clone `Send + Sync` canonical-object
residence, independent of NyashBox, with consuming `end(self: Box<Self>)`.
Kernel implements it with private SafeMutex indexed identity/type fields and
existing reclaim. Its result carries only finite physical storage failure facts;
kernel maps those to existing CheckedStorageError/FaultFrame handling. Root
imports no kernel FaultFrame and no global callback registry is introduced.
Precommit rejection returns the original candidate without end; successful
install returns detached-old/no-old after the Map owns the new responsibility.
Either outcome of explicit end consumes that attempt; never retry the old value.

TLS is unsupported before residence creation/mutation until confinement is
proved. SafeMutex permanent indices need no new ABA generation. The interface
alone does not authorize owned slots: existing get/get_opt/values/Clone use
clone/share and toJSON exposes values. JSON/GC now use scoped owner access;
get_data is removed. Native facade publication must exclude Owned structurally;
checked consumers need supported projection or explicit refusal. Owned values
cannot silently become missing keys, ordinary Boxes or cloned payloads. Remove,
clear and terminal end must detach responsibilities and end outside locks.
Current Send+Sync NyashBox and its mandatory clone/share methods are not a
substitute. Source/Fault/end consumer migration remains required before intake.
Read/observer migration is an intake prerequisite, not a reason to defer source
flow, Completion or physical lowering implementation. Keep that work in the same
construction/end series and retain the escape Stops until the full gate closes.
Even a source with no Map reads does not prove observer isolation: `birth.rs`
registers MapBox in host handles; `gc_controller.rs` snapshots those handles and
calls `gc_trace::trace_children` under `RcDiagnostic`. That mode observes
reachability only; it does not reclaim cycles and `Off` does not trace. Any
nonreachability alternative must account for this real native entry as well as
public read/clone surfaces. Send+Sync does not itself require cloning entries;
silently changing Map Clone into share would still change its existing contract.

Native mutation teardown now detaches before dropping outside the lock in
`insert_key_str`, `remove_key_str` and `clear_entries`. Clear drains while
preserving table capacity. Map focused tests8 cover reentry, committed visibility,
once-only native Drop and existing key/ordering behavior. This removes the three
shared native mutation lock-held teardown edges; owned residence, ordered source
finalization and terminal Map Drop are not established by that evidence.

Completion cleanup retains one source flow result with terminal order and exact
Map operation successors; its existing binding-list API is only a projection.
The [Map destination task](../investigations/hakorune-home-ownership-task-2026-08-04.md#map-slot-dependency-of-the-selected-compiler-cutover)
owns the finite operation mapping and candidate-descriptor implementation order.
Root cleanup reuses the suffix builder, selecting ordinary-object versus
intrinsic-Map end at the existing progress owner. Install Normal and detached-old cleanup are distinct operations;
source transfer is never reconstructed from physical progress. End-to-end
acceptance includes native end once, no old-local double end, duplicate-key
replacement, pre/postcommit Fault, Map/root cleanup, drift and profile/thread
mismatch. This accepted design is not runtime activation or source execution.

### Map publication premise reset: native facade and checked intrinsic facade

Decision (c4a1889a19 audit): replace the premise that the public NyashBox MapBox
can acquire non-Clone Owned entries after local read-signature migrations.
Preserve the native facade's copied-table Clone contract. The Owned-capable
intrinsic facade does not implement NyashBox or Clone and cannot be published
through native host handles. Both facades use one common storage implementation;
each Map owns one payload table. No shadow payload table, clone-to-share change,
omitted entry, panic wrapper or ordinary Box standing in for an Owned obligation.
The native facade is permanently Native-only: checking a flag once at publication
and allowing Owned promotion after a native alias escapes does not suffice.
This changes physical exposure, not source Map key/transfer/end semantics.

Counterexample: allocate a public MapBox, retain a native alias, then install an
Owned entry and invoke Clone/clone_box/clone_arc through that alias. Copied-table
cloning requires duplicating the non-Clone obligation. None of those methods can
return an error. Read Result patches cannot remove this contradiction. A broad
NyashBox clone-capability migration could address it but is not selected here.

Audit boundary: native Map read/clone entries -> root surface/native host/service/
kernel terminals. Includes public get/get_opt/scalar/values, mandatory clone
methods and the listed consumers below. Excludes JSON/GC internals, unrelated
source families, other backends and external Rust clients. This targeted audit
is not an Exhausted claim about the full Map cutover.

| Existing consumer | Observed terminal / required disposition |
| --- | --- |
| Map surface catalog / interpreter method handler | Result reaches VMError; current surface errors only cover arity |
| native host slot203 | independent i32 status; Some(error) terminates, None would enable fallback |
| MapService / ring1 provider / adapter | Option; current missing-key String is observable native behavior |
| kernel slot_load / scalar / compatibility / runtime-data aliases | bare i64, missing/nonscalar can already be0; no separate failure channel |
| MapBox Clone / NyashBox clone_box / clone_arc | infallible copied-table contract; Owned exposure is incompatible |

The selected resolution is enforced exclusion of the checked facade from those
native entries, not a claim that their error contracts have already migrated.
Existing native success/missing behavior remains its own contract. Checked
source borrow or Dynamic publication still needs its own authorized lifetime or
self-contained carrier; this facade split grants neither.

The identity/lifetime question is resolved by the following physical decision;
its consumers remain to be implemented. Current `exports/birth.rs` immediately
registers Arc<dyn NyashBox> via `nyash.map.birth_h`; the new facade cannot reuse
that publication merely behind a private wrapper. Audit the actual Map allocation
Normal result, retained physical representation, install target, root cleanup,
required runtime descriptor/session and C caller before choosing storage form.
Do not infer an indexed registry, host token, opaque stack address or semantic
receipt from the facade name. Root owns common storage; kernel supplies the
existing indexed child-residence end implementation, with no reverse dependency.

Ordered acceptance: checked allocation -> exact Owned install -> checked
observation/refusal -> detached-old and reverse-install terminal end. Also prove
native publication and legacy-reader ingress cannot acquire the checked facade,
including aliases. Retirement target is selected intrinsic allocation/publication
through birth_h, followed by the existing old set/install Stop retirement; native
compatibility remains Native-only. No source activation, general Dynamic escape,
ABI implementation or full Map cutover is claimed by this Decision.

### Checked intrinsic Map physical lifetime decision

Decision: the selected direct-local cohort uses caller-owned opaque Map storage,
with one stable allocation-site region per function invocation. The runtime
archive descriptor issues size/alignment/revision; the existing invocation owner
checks them and C allocates the region. No host handle, runtime identity registry,
extra heap identity wrapper or pointer-to-i64 transport is introduced. This is a
new physical placement decision, not an already implemented semantic issuer.

Source authority remains home_new_prefix/home_map_flow from the same lowering
input, retained by ordinary_new_coseal in Completion. Existing Map flow identifies
allocation Normal/Fault, destination, entry transfer, displaced old and terminal
live order. Existing common install Stop remains until the full consumer exists.
The allocation Invoke owns acquisition; its Normal-only InvokeNormalResult
projects a distinct non-host Map reference to the region. Preallocated bytes do
not imply acquisition. CanonicalObjectId, MirType::Box and old origin tags do not
issue the reference. Install/checked observation/end are its only physical uses.

| Opaque region state | Legal operation / outcome |
| --- | --- |
| Fresh bytes | initialize native bookkeeping -> Unissued |
| Unissued | checked allocation Fault stays Unissued; Normal -> Live |
| Live | validated install transfers only on Normal; old outcome is separately consumed |
| Live | terminal end first marks Ending, then detaches/ends live entries outside locks |
| Ending | reject admission/re-entry; attempt remaining cleanup while retaining first Fault |
| Ended | no live obligations; native storage may be disposed |
| Unissued | native storage may be disposed without source Map end |

Either terminal-end outcome consumes the live Map responsibility and reaches
Ended after best-effort suffix cleanup. Host panic/abort recovery remains outside
this contract. Fresh/aligned/unique initialized storage and no concurrent dispose
are unsafe caller obligations; a header cannot validate arbitrary pointers.
No reinit, byte-copy or abandonment of Live/Ending storage. Recursive invocations
have distinct regions; a repeated site requires a new ended lifetime, never a
second acquisition over a live one. The initial cohort issues no loop lifetime.
Map and detached-outcome layouts travel in one explicit descriptor revision with
required symbols, readers, session checks and C consumers; no V1 padding reuse.

Complete source flow does not mean alias-zero. home_prefix_local_flow observes
Map as Handle and an unused `local alias = map` can retain the original binding.
Such an exact alias introduces no new residence or cleanup; preserve its existing
binding projection without fabricating a clone. Actual read/escape needs its own
authority. Reject generic Copy/host carrier/Call/return/slot-store escapes of the
physical reference; do not reject an otherwise Complete unused source alias just
because an overbroad physical rule assumed there were no aliases.

Cleanup must cover every acquisition origin: root terminal, Map allocation Fault,
entry precommit Fault, postcommit old-end Fault, and a later ordinary New's
allocation/Birth Fault. That New's prior_homes already contains earlier Maps.
Use one ordinary-object/Map end origin at the existing suffix/progress owner;
selected.rs prior cleanup and root cleanup must not independently reinterpret
bindings. Extend root_cleanup_graph's HomeRelease-only/one-release assumptions
for exact mixed origins and validate both published and retained optimized graph.
A root-only Map end implementation does not cover this cohort.

Current C lifecycle admission assigns LV4_HANDLE to every Normal projection and
emission loads i64. Replace both with operation-derived Map pointer projection;
existing per-function frame/result storage is the placement owner. Do not route
the reference through ORG_MAP_BIRTH, generic kind/payload or ptrtoint. Static V2
and generic NewBox intrinsic allocation callers switch together when selected;
legacy native birth_h export remains available only to native compatibility.

Acceptance spans empty/multiple Maps, Map followed by ordinary New, and unused
local alias through source/Completion/MIR/retained optimized graph/OBJ/EXE.
Exercise allocation Fault, precommit failure, a:=A/b:=B/a:=C replacement, old-end
Fault with committed C then B cleanup, first-Fault retention and native end once.
Reject Normal-before-use inversion, foreign projection, missing/double end,
live reinit, native publication/escape and profile/thread mismatch. Native-only
Map cloning and key/iteration behavior remain unchanged. No general Map return,
parameter/capture forwarding, Dynamic escape or loop support is issued here.

Implementation starts with the common actual Map storage owner and checked
non-NyashBox lifecycle facade, including real SafeMutex indexed-residence end.
Keep the native facade statically Native-only. Verify precommit candidate return,
commit/detach, reverse successful-install end, refusal after Ending, and no locks
or live mutable ABI borrows across child end. Data-structure choices must support
these transitions without allocation/hook/fallible work between commit and output;
end must attempt every remaining obligation after a returned Fault. This is the
runtime dependency of the same series, not source cutover. Then wire opaque ABI/
descriptor, existing Invoke/projection/mixed cleanup consumers, and finally switch
source while deleting selected birth_h/set/MapLiteralEntryWrite/install-Stop edges.

Runtime dependency implementation: native MapBox and CheckedMap now store their
payloads in common MapTable. Type parameters permanently exclude canonical
residences from Native MapBox. Checked install reserves table and empty end-buffer
capacity before commit. End moves rather than copies entries into that buffer and
sorts descending unique install order without allocation. The kernel's private
SafeMutex residence validates exact identity/type; rejection leaves the payload
with its prior owner and consuming end uses existing indexed reclaim.

CheckedMap/DetachedMapEntry do not implement Clone or NyashBox. Their explicit
end protocol is a placement-caller obligation; Rust Drop does not run source
finalization. Map require_disposable checks Unissued/Ended. Opaque ABI disposal
still must enforce this and reject Ready detached outcomes; no raw ABI is added
by the Rust dependency. Physical end reports retain first/eight suppressed facts
plus total suppressed count, which may exceed stored capacity. Native projection
of present owned entries explicitly refuses while missing remains None. The
initial checked payload is the source-authorized indexed residence only; this
does not admit additional source/native candidate families.

### Prepared key lifetime at the opaque ABI boundary

The [accepted opaque ABI](../../../../reference/runtime/runtime-data-dispatch.md#checked-map-opaque-abi-contract)
includes temporary key storage in the same descriptor revision as Map/outcome.
This closes a physical mapping gap: MapKeyDomain owns String storage, while the
old C String-handle key path does not transfer that native key residence.
Preparing inside install would move possible allocation failure after child
evaluation, contrary to lifecycle.md's existing order. Source authority remains
the ordered key/entry relation; runtime issues native key storage, not a semantic
Prepared receipt. The function invocation owns its temporary region.

Prepare exact UTF-8/length before child; child Fault cancels Ready key natively.
Install validates region/profile/state contracts first, then consumes the key
before indexed preparation/attempt. Normal and returned Fault after consumption
both leave Consumed; preflight InvalidContract leaves Ready. Key dispose supports
Ready cancellation and Empty/Consumed release. Map has no hidden pending-key
field, String handle/cache repair, registry or synthetic String Home.

Finite consumer inventory: fault.rs target descriptor issuer; host
runtime_abi_descriptor.rs archive decoder/session/required symbols; capi_transport
C row/projection; hako_llvmc_ffi.h session layout; lifecycle_target_session validator;
lifecycle_v4_emit allocation placement; descriptor tests, published_lifecycle_v4
C driver and nyash_lifecycle_kernel launcher symbol assertion. Revise all opaque
layouts together. Runtime Rust Map green is not evidence for these wire consumers.

Key acceptance adds child-Fault cancellation, install-Fault native release,
length/NUL/UTF-8/numeric-domain preservation, Ready overwrite/double consume and
region nonoverlap. Compiler validation owns live-reinit rejection; fresh-storage
init must not inspect uninitialized bytes to guess whether misuse occurred.
Exact runtime status/Fault merge/disposal contracts are in the reference above.
No source fixture, ABI symbol or descriptor revision is implemented by this audit.

### Detached install outcome physical contract

Decision: use caller-owned opaque result storage for the committed install
outcome. It is neither a source Home nor a host handle, and introduces no token
registry or separately allocated outcome box. The runtime owns its layout;
versioned size/alignment/contract revision travel through the existing target
archive descriptor and invocation session. The current fixed 200-byte V1 cannot
hide these fields in padding: revise descriptor, decoder, required symbols and
C allocation together. C never guesses a Rust trait-object layout.

| Storage state | Allowed transition | Ownership |
| --- | --- | --- |
| Fresh storage | init -> Unissued | valid aligned unique caller storage; no payload |
| Unissued | install Fault -> Unissued; install Normal -> ReadyNoOld or ReadyOwned | Fault transfers nothing; Normal commits new slot and detaches old |
| ReadyNoOld | detached end -> Consumed | mandatory consumption, no child end |
| ReadyOwned | move payload and mark Consumed, then child end | either child outcome consumes old attempt; new slot remains committed |
| Unissued / Consumed | dispose | no outstanding obligation; reuse requires a new valid lifetime |

Ready states cannot be copied, reinitialized, overwritten, disposed or abandoned.
Storage init has the same fresh/aligned/unique unsafe caller obligation as
FaultFrame init; a header cannot validate arbitrary pointers. Reject end before
issuance and repeated end without altering storage. NoOld is not Unissued.

Precommit validates frame, Map state, candidate profile/type/identity, output
state and valid nonoverlapping output storage; key preparation and capacity
reservation precede mutation. Commit-to-output contains no allocation, hook,
formatting or fallible work. Map install never runs detached-child end itself.
Release Map/indexed-store locks and end any Rust mutable borrow of opaque storage
before invoking callback-capable child end; similarly do not keep a reentrant
FaultFrame borrow across that callback. Record Fault afterward without rollback.

MIR keeps one operation-derived InvokeNormalResult with a distinct detached
result class. For the selected direct-root shape, its exclusive Normal landing
contains that projection and immediately terminates in the matching detached-end
Invoke. The sole permitted use is that end: no Copy, Phi, Call, return or storage.
Both end successors have consumed the result. Admission rejects escape, missing
end, duplicate end and drift instead of adding a general affine-flow engine.
Apply the same rule to the optimized retained graph, not just initial MIR.
Physical slot preparation/disposal belongs to backend lifetime handling, not
another semantic receipt. Install Fault disposes Unissued; either end outcome
disposes Consumed. Recursive or repeated executions must not share a live slot.
The current C universal LV4_HANDLE classification/i64 load must be replaced by
operation-derived projection for this result, not reused as a token encoding.

Implementation order: checked Map identity/lifetime and permanent native facade
exclusion; common Map storage/end primitives and checked observer behavior;
target descriptor plus checked opaque ABI;
existing lifecycle emission/validation and root cleanup; then source activation
and selected old-set/MapLiteralEntryWrite/install-Stop retirement. These remain
one Map series. Native teardown tests prove only their current native boundary;
no schema-only change or test behind install Stop proves public cutover.

### Checked Map reads and native observers

Missing consumer: `MapOwnedReadProjectionConsumerMissing`. The checked facade
Decision above excludes the following native path until an explicit checked
publication contract exists; migrating its signatures alone is not the next
Owned-intake slice. Kernel
`map_slot_load_str_with` currently gets a visible clone, then
`encode_runtime_value_carrier` publishes scalar/host handle through
MixedI64OrHandle. `map_aliases` load_hi/hh, `map_compat` get_h/get_hh and
`map_runtime_data_get_any_key` expose bare i64; missing/invalid paths already
collapse to zero. Owned projection unavailability must not join that value.

The checked projection must preserve `Result<Option<...>, ...>` through the
actual selected status/out consumer, writing output only on success. Source
borrow uses a scope-bound consumer and keeps the slot obligation in place;
Dynamic publication requires an existing self-contained carrier authority.
A bare negative indexed handle with a forgotten borrow lifetime is neither.
No existing checked Map read ABI/FaultFrame consumer closes this chain yet;
a standalone try_get accessor does not authorize owned-slot intake.

GC now obtains native child projection from the Map owner and propagates
storage/module-root errors through the controller to kernel metrics. JSON
conversion borrows children through scoped Map-owner access and returns finite
errors through public set. The raw get_data accessor is removed. Native GC keeps
child clone semantics; neither that projection nor JSON borrowing establishes
owned-native coverage. Public read/clone, owned end and selected checked
publication remain included intake blockers.

### Native Map diagnostic observer transition

Decision: before owned entry intake, move GC's raw Map table access into the
Map owner's explicit native child projection and carry failure through the real
RcDiagnostic controller and kernel metrics output. Preserve existing native
`clone_box` behavior; this is not implicit share or owned projection support.
The owner returns a complete native child projection or a storage error. GC
never skips an unavailable Map as if it had no edges.

The module-root snapshot also returns failure instead of an empty root list on
lock error. The controller propagates both failures and stores one finite last-
completed observation: NotRun, Complete(nodes, edges), or Incomplete(reason).
Replace the two independent last-count atomics; callbacks and traversal never
run under the result mutex. Concurrent trials publish in completion order; this
is not an atomic snapshot of the mutating object graph. Failed attempts replace
old Complete results and still update attempt count/duration. Do not add a
sequence authority or change trigger policy.

Kernel JSON metrics report numeric trial counts only for Complete. NotRun,
Incomplete and absence of this controller report null counts with an explicit
status/reason. The existing text sink also identifies incomplete observation;
no old summary or numeric zero substitutes for failure. Existing env-controlled
logging remains optional. Native clone panic and host termination retain their
existing behavior; this transition does not turn arbitrary panic into Fault.

Boundary inventory: root snapshots -> builtin Array/Map native projection ->
controller observation -> kernel metrics. Includes module-root lock failure,
Map lock failure and all current trace/last-result callers. Excludes external
crate callers, owned values, JSON conversion, language reachability semantics,
cycle reclamation and graph-wide concurrent snapshot consistency.

| Owner | Actual consumer / terminal | Replacement |
| --- | --- | --- |
| host_handles snapshot | controller roots | existing native snapshot retained |
| modules_registry snapshot_boxes | controller roots | typed unavailable, never empty-on-error |
| MapBox native trace projection | gc_trace | owner access, explicit storage failure |
| gc_trace | controller queue | Result propagation, native Array behavior retained |
| controller last observation | kernel entry | one finite outcome, no stale complete counters |
| kernel metrics JSON/text | env-selected output | null/status/reason for nonComplete |

GC's get_data caller and silent skip plus module-root empty-on-error are the
exclusive deletion set. The subsequent native JSON Result transition removes
the other caller and get_data itself. Owned Native/Owned entry design,
public fallible read/clone, ordered end, checked ABI and source activation all
remain included later steps, not completed by this native observation repair.

### Native JSON observation Result terminal

Decision: change the existing public Rust JSONBox::set directly to
Result<Box<dyn NyashBox>, JsonSetError>. Preserve its successful `ok` Box;
replace non-object Error String and source/destination lock failure with finite
SourceMapUnavailable, DestinationUnavailable and DestinationNotObject errors.
This is an explicit Rust signature change, without a legacy panic wrapper or
second checked setter. It does not add a source-language dispatch.

The selected boundary is Map storage -> native borrowed conversion -> public
JSONBox::set. Existing root set callers are direct observation tests; builtin
factory, optional Json method table, native host and kernel/plugin inspection
found no root JSON set dispatch. Historical archived dispatch is not active.
External Rust clients and plugin JSON implementations are excluded; do not
claim compiler activation or external caller-zero compatibility.

MapBox lends native key/value iteration within its lock and reports storage
failure without exposing HashMap or RwLock. JSON's recursive conversion returns
Result and propagates nested Map errors; it still borrows rather than cloning
children. GC reuses owner access while retaining its distinct native clone
projection. The get_data accessor is removed; poisoning tests use a cfg(test)
owner module, not a production raw-storage escape.

Evaluate key and convert the input, then dispose the top-level native input
before taking the destination write lock. This preserves disposal-before-commit
and lets input observation/Drop re-enter the destination without that lock.
Validate destination storage and object kind, then commit the owned JSON value.
Err performs no direct destination write; arbitrary input callback side effects
are not rolled back. Inputs are consumed once on both outcomes. Keep existing
Array observation and fallback stringify behavior; this does not turn native
panic, host allocation termination or cyclic traversal into language Fault.

The one-row deletion set is raw Map table exposure plus its JSON caller,
infallible native conversion and non-object Error String. Acceptance exercises
public set success/failure, nested Map errors with destination retention,
non-object/destination refusal, no child cloning and input Drop outside the
write lock. Owned entry projection and fallible public Map read/clone/end remain
subsequent intake blockers; this native Result does not invent those authorities.

### Map source-shape preservation decision

Decision: retain ordered Map keys and exact EntryValue source relations in the
existing body-shape product; preserve executable call admission with an explicit
unsupported Map-entry continuation terminal. This replaces accidental MissingParent
diagnostics, not the missing ownership/commit contract.
Source authority + canonical issuer: ShadowResolverV0::record_expression_shape
and shadow/expr traversal, sealed by seal_shadow_body_shape into the existing
VerifiedResolvedBodyShapeInventoryV1. Keys retain duplicates and source order;
child ordinals/paths must be exact, complete and unique.
Non-authority: literal syntax facts are not capability/Home/carrier issuance;
no VerifiedMap receipt, runtime tag, Recipe key or alternate graph is introduced.
Fail-fast boundary: reject malformed key/child relations at the existing seal.
normal_script_source_continuation::find_terminal rejects EntryValue ancestry
before direct/NonDirect call classification; no newly executable Map child calls.
Implementation checkpoint: source-shape tests20, continuation rejection tests3,
existing Script direct/NonDirect acceptance tests2 and pointer/corridor guards pass.
Smallest next slice: single-entry candidate/destination ownership issuance and
commit co-seal at the matching source root; source-shape preservation is closed.
Non-claims: ownership commit, Map runtime escape, source/OBJ/EXE cutover.

Worker audit closes one concrete coupling: adding EntryValue relations alone
can move bound-receiver NonDirect calls past MissingParent; their issuer does not
use the direct-static terminal validator. The shared continuation owner must
therefore retain refusal explicitly for both classes. Do not report a structure
parent missing once it exists. This is an information-preserving BoxShape with
unchanged execution admission and an intentional unsupported diagnostic refinement.

Function products consume the existing callable batch. Script products instead
flow through resolve_script_forest_with_declaration_views, then
normal_script_semantic_source::seal_ast_with_forest and
VerifiedScriptSourceContinuationV1::issue after package construction. Do not
put Script Map continuation into OrdinaryNewClaimLedger. Later ownership commit
requires genuine candidate/destination co-seal at the matching root.

Acceptance: empty/duplicate-key/nested/mixed-child Maps retain exact source order
and cardinality; missing/duplicate/foreign child relations reject. Both direct
and NonDirect nested calls reject before effects, while existing accepted
continuations retain their behavior. Existing Array shape tests stay green.
The removed edge is MapLiteral -> Other at the shared resolver; legacy source
Map allocation/write edges remain until the full cutover.

### Demanded-formal ingress and use closure

Decision: definitions gaining Map-demanded lanes use one **internal physical
symbol**, selected by the existing canonical key/definition relation. Keep
semantic MIR names and logical arity unchanged. The same existing physical
projection owns the internal target, expanded formal positions and each typed
call site's lanes. Public root entry remains zero-argument with its current
ABI. This is a planned new Map consumer boundary, not permission to internalize
unrelated static/free functions or remove an independently specified export ABI.

The ingress audit covers demanded selected definitions -> typed/compatibility
calls -> generated entry/OBJ linkage. It excludes source-family admission and
runtime decoding. Current code does not prove a closed caller set:
`same_module_function_emit.inc` and `module_leaf_function_emit.inc` emit external
`define i64`; `alwaysinline` is not internal linkage. The shared Method emitter
can call a retained user-box plan's `target_symbol`. Its usual producer is
LegacyCallV0-only, but rejecting legacy instructions alone does not reject all
stale compatibility plan targets. A bare symbol rename is insufficient.

| Ingress | Required disposition in the new projection |
| --- | --- |
| Typed Static/Free, recursive or nested | Exact key/site plus matching logical ordinal -> internal target and expanded lanes together. |
| Rowless Global, LegacyCallV0, Call(Extern), Call(Value) | Preserve their existing published rejection/unsupported boundaries; do not synthesize tags. |
| Shared Method/user-box/global/direct-symbol plan | Reject a reference to the changed definition before LLVM; never pass old argument formatting to the internal target. |
| Ordinary generated entry | Preserve zero-argument entry; a demanded formal cannot use this ingress. |
| External OBJ call | The new expanded target is internal, with no old-name guessed-tag wrapper. Any separately admitted external export requires its own explicit ABI and blocks this switch until accounted for. |
| Old leaf emission | Every expanded definition, including a recursive component, must use the expanded emitter; it cannot fall through the old external leaf definition. |

Acceptance must inspect LLVM linkage and OBJ symbols as well as executing
recursive/nested exact calls. Missing/wrong lane maps, retained compatibility
plans, duplicate physical targets and old exported aliases reject before object
publication. The audit proves the current exposure, not absence of every
possible promised export contract; the new consumer must not claim a general
external language ABI from incidental old linkage.

### Expanded ingress — physical consumer binding

Decision: the existing V2 frame admission builds one expanded-function index from
the fourth table and exact document definitions/formals. Typed call formatting,
definition signatures, ingress rejection and old-leaf exclusion share this index.
Rust frame construction alone does not close ingress or prove internal linkage.

C consumes `metadata.lowering_plan` through first matching block/instruction site;
`route_json::build_lowering_plan_json` projects global/builtin-global/user-box
route metadata into it. Checking only Rust mirror arrays is insufficient. A
legacy global/user-box entry targeting an expanded logical name is shadowed as
call ingress only when the typed Static/Free row binds its selected target to the
exact published definition/expanded index and the matched body Global shape,
args/arity/result coordinates agree. JSON callee-name equality is not required
and must not reissue target identity. Typed dispatch terminates without retry. Row-coordinate existence, Print/Array rows, or Method
with a Global row never establishes this. Shadowing does not skip prescan or
other metadata validation. Nonshadowed old-ABI ingress rejects before artifact.

Compatibility/direct-symbol references to an expanded internal target reject;
existing runtime registry checks stay physical and do not resolve source names.
Program entry/selected launch identity comes from the existing C program owner,
not a Rust name convention. A required Formal row cannot use the ordinary root
entry. Expanded members bypass old numeric-leaf eligibility and old same-module
signatures; declaration/call/definition all use the same index and internal target.
The emitted registry remains execution progress, never the planned layout owner.

Next implementation: connect this index to admission and both C walkers after
owned rows. Acceptance includes actual typed shadow versus forged shadow,
nonshadowed global/user-box plans, direct internal-symbol ingress, missing/wrong
Formal rows, root formals, old-leaf re-entry, recursive/nested signatures and
LLVM internal/OBJ export evidence. The delete-set is expanded members' old
argument formatting/signature/leaf/export-alias edges; planner tests cannot claim
these deletions. Read-only worker audit at9063d4008a found no missing semantic
issuer; this is the remaining physical consumer implementation.

### Selected call activity and V2 publication boundary

The existing published-call owner now stores explicit Inactive/V1/V2 activity
alongside borrowed rows and the consumption ledger. V1 still rejects an empty
table; private V2 accepts exactly NULL/0 or a valid nonempty table. Failed nested
activation preserves the existing binding/ledger. Selected Global/Extern absence,
exact-seed rejection, object selection and no-compatibility-retry use activity;
row lookup and take still use the actual table. No dummy row supplies activity.
This remains process-global storage, not a concurrent compile guarantee.

Verified dependency at parent dd8e54cfd2: C build, row-owner tests, ASan document
lifetime22 (including six empty-V2 cases), Named60 and definition22 identical to
parent observations, query17 and actual Rust/C integration1. The empty-V2 driver
exercises non-Map bodies through the real core; it does not validate a full frame
or prove Map execution. Public V1 admission and public Map stop remain intact.

The private retained compiler now binds the invocation-owned index and call
activity, writes to same-directory staging, finishes both ledgers and renames only
on success. Generic lowering checks residual rows before object emission; the
outer owner also checks after early optimizer returns. Failure removes staging
and preserves a previous final object. Both walkers consume one frame-wide ledger
using original function/instruction identity; function-context snapshots never
copy that ledger. Query/prescan only observe. The expanded-function table remains
explicitly unsupported until its shared index/signature/ingress consumer lands.

Both actual walkers emit intrinsic Map allocation and writes to
`nyash.map.literal_store_v1`. ExactBits retain Integer/Bool/F64/Void. The shared
consumer handles all seven selected Operation discriminators, direct String/Map,
host-backed Named/Intrinsic Array allocations, canonical Static/Free Call results,
and boxed Tag/Integer/Bool/String projections. Operations use retained body operands;
reachability and actual-emission ledgers remain separate. Missing, extra or
wrong-kind inputs reject. Original rows are consumed only after successful emission.
Typed-store and direct-array tokens are not host handles: demanded allocations,
boxed Make escape and unproved general handle projections remain explicit Stops
under the physical escape boundary above. Transfer/formal actions still reject as
`value-consumer-unsupported`; all these gaps remain compiler cutover blockers.

Integer comparisons bypass dynamic kind dispatch. Bool results preserve i1 for
old consumers and issue an i64 side payload at the producer. Local boxed aliases
and constants use the existing value owner; Bool payloads normalize i1 at Make
and Project consumption. String concat consumes already-proved String payloads
directly, then uses existing result publication; accepted operation rows also
supply declaration requirements in the existing declaration owner. It never reclassifies a
projected String from old origin metadata. Constant folding remains in the existing owner.

Map success is i32 status0; all nonzero statuses trap without continuation or
source Fault. String constants retain byte length in the existing StringConst
record, including owned concat and Copy-alias lookup. V2 uses the length-aware
runtime entry and checks nonzero handles at actual instruction emission; generic
hoisting stays on the V1 path. The existing globals/owned storage remain sole
owners. V1 retains its historical length/runtime projection. Map writes now count
as uses in the existing literal observer, so a live key is not eliminated.

String/Map/boxed continuations use the existing exact-status labels and PHI
predecessor owner. Every successful V2 boxed dispatch, including nondemanded sites,
has a common continuation. Preplanning scans the same sites with checked field stores,
including not-yet-emitted backedges. No second tail table, literal pool, runtime
wrapper or operand graph is added. Arbitrary String specialization/optimizer/C1
combinations are not proved by this bounded consumer; input-aware capability
must close changed-consumer coverage before public activation.

Verification on the Copy/NamedAlias consumer change (parent05e6169bda): normal
and ASan C drivers each pass239 private compile cases, including125 linked-kernel
readbacks, eight injected traps and one runtime domain probe. Copy chains preserve
Integer/Bool/F64/Void/Handle lanes, NaN payload bits and original-required values.
Integer/String operations consume copied operands; i1 Bool results retain their
producer normalization. Generic NamedAlias follows its retained outcome, while
same-module and changed Named outcomes reject. Missing/cyclic sources, illegal
fields, original-demand gaps and CopyOwned reject before replacing an artifact.
Earlier Original/Operation/status/boxed-tail tests remain green. C library and
corridor/pointer guards pass; Named60/definition22 observations match the parent.
No Rust source changed in this consumer slice. The fixture reader remains test-only;
source/host cutover stays open, including escape and expanded signature obligations.

Next: close original PHI predecessor references/width normalization after the
private Phi/Select/Formal and expanded-index consumers. Keep physical escape
ownership and intrinsic allocator session observation
as CutoverBlockerOpen; admitting the already-proved host domains does not close
all Original producers. The selected boxed Unit comparison owner currently Stops
before emission (`static_v2_boxed_compare_consumer_pending`); its internal splits
need the existing PHI tail contract before activation. Generic legacy comparison
can use an earlier dynamic dispatcher, so this Stop is not a universal comparison
guarantee. Input-aware capability must account for actual selected consumers.
Then complete capability and atomic public session/host/source switch with V1
and six literal-edge retirement. No public V2/source cutover claim.

### Transfer consumer implementation order

Decision: Copy/NamedAlias join a shared kind/payload formatter first; Phi/Select
then join the existing PHI/condition owners, followed by Formal/expanded ingress.
Source authority + canonical issuer: existing V2 projection and retained body SSA;
NamedAlias additionally requires the exact captured walker outcome AliasOperandZero.
Non-authority: StringBox spelling, mutable alias discovery and old i64 width do
not issue representation. Transfer destinations use fixed physical side names.
Fail-fast boundary: missing source row, invalid fields, original-demand mismatch
or wrong Named outcome rejects before artifact; CopyOwned keeps its unsupported
physical owner rather than discarding an ownership operation.
Implemented: Copy/NamedAlias forward both lanes; flags0 bypasses the old producer,
flags1 keeps it and consumes the row only after actual success.
Smallest next slice: original PHI reference/width normalization in its existing owner.
Non-claims: original PHI reference/width coverage, object escape and
public/source cutover remain open; implemented transfer lanes do not close them.

Read-only worker audit at05e6169bda found two Phi integration constraints: the
same-module flags0 prepass currently skips Phi registration, and existing PhiRec
silently truncates inputs at16. The next Phi consumer must register map-only PHIs
and reject overflow or remove truncation in that owner. Emit both lanes within
the existing PHI group, with existing predecessor labels; never insert zext there.
Original-required Phi validates actual incoming widths through the existing type
owner before emission; incompatible i1/i64 inputs reject until predecessor
normalization exists. Bool kind alone cannot establish width: a boxed local alias
can retain i1 while the Project records I64. Select may retain its existing
normalization and must share one normalized condition for both Map lanes.

Admission uses finite transient physical domains over the same row/body graph.
Keep String distinct from other Handle; Copy/NamedAlias forward domains, Phi
unions every incoming edge, and Select unions both arms without constant pruning.
Seed from validated producers only; Operation outputs require supplied inputs.
After convergence mark still-empty domains Unresolved and propagate again, so
one seeded merge cannot hide a seedless arm. Each selected Operation requires
its inputs' whole domain to match. Reachability, physical domain validation and
actual-emission ledgers stay separate; no new semantic issuer or operand graph.
Private implementation now emits both Phi/Select lanes in those owners. Fixed
Original payload side names support future aliases on Map-only backedges. A
future Operation's validated encoding supplies its actual width before emission;
Select resolves actual alias types before normalizing its condition/arms.
Original-required PHIs stop on incompatible widths, not-yet-emitted boxed aliases
or Select width dependencies. These are CutoverBlockerOpen: the existing original
value/predecessor owner must provide stable references and normalization before
those cases can cut over. Do not turn these Stops into a completed Phi census.
Verification: normal/ASan315 full cases each, including145 runtime readbacks,
eight traps and one non-host domain probe. The final pending-Select Stop adds two
negative cases; control78 then passes in both builds. C build and guards pass;
Named60/definition22 match parent. The two reproduced width failures (future
Operation and boxed Bool Select alias) are current-change failures fixed here.

Expanded signatures/calls reuse one index; expanded leaf-only members must join
the same-module emission traversal without changing original definition intent.
Forward LLVM definitions need no separate declarations in the existing backend.

Preparation at parent998103b469: the 758-line generic prepass's PHI block is
lexically extracted to `hako_llvmc_ffi_pure_compile_generic_phi_prescan.inc`;
expansion matches the parent byte-for-byte (prepass676, extracted block84).
C build, ASan Original63 including both-walker backedges, definition22 parent
comparison and pointer/corridor guards pass. This is editing headroom only,
not PHI/Select consumer completion or a new physical owner.

### Original PHI stable-reference decision

Decision: preserve existing original values/types/aliases; publish a stable
normalized i64 reference from the actual flags1 producer for shared i64 PHI inputs.
Source authority + canonical issuer: unchanged published V2/body relations;
existing original producer/type/alias owner issues this physical reference.
Non-authority: Map payload, future mutable alias/type caches and wire Bool kind
alone cannot issue the original value or its width.
Fail-fast boundary: unavailable original width/reference rejects before artifact;
pending i1 input projections remain CutoverBlockerOpen, not guessed-bit fallbacks.
Smallest next slice: producer normalization plus shared mapped/nonmapped i64 PHI
input consumption; preserve existing original rDst/type and actual PHI group.
Non-claims: i1 normalization, non-host escape, boxed comparison and public cutover.

Accepted after read-only worker audit at270643e311. This scope covers
`flags1 mapped original producer -> shared i64 PHI input -> old arithmetic/return`;
includes downstream nonmapped i64 PHIs and backedges. It excludes new source
admission and unresolved i1 input projections, which remain named cutover blockers.
The existing selected-PHI-only checks missed the downstream nonmapped observer;
do not claim closure by promoting only the three existing selected Stop fixtures.

| Existing producer | Stable original i64 reference |
| --- | --- |
| Exact Const | Read the existing original constant owner, never Map payload. |
| Original String/allocation/Call/boxed Tag or Project | After actual success and alias resolution, normalize the original with add/zext. |
| Operation | Normalize the actual emitted result with add/zext. |
| Copy/NamedAlias | After original producer success, normalize its original reference. |
| Select | Keep actual rDst and its chosen i1/i64 width; normalize after emission. |
| Formal | flags1 already owns an i64 rParam; refer to it directly. |
| Selected i64 PHI | Refer directly to its actual rDst PHI; no extra PHI or in-group add. |

Use the shared `emit_phi` input formatter for mapped and nonmapped i64 PHIs;
retain the existing predecessor labels and group ordering. Do not globally
change an old PHI or Select destination to i64. Keep the existing compatible i1
path and explicit Stops for pending aliases/widths. A selected i1 PHI used as an
i64 input requires normalization after the entire PHI group, as implemented by
the after-group decision below; never insert zext between PHIs.

Delete the future boxed/Select pending checks and speculative width/alias chase
only for i64 edges closed by the new physical reference. Do not delete the i1
failure boundary. No new semantic receipt, operand graph, public ABI or guard.

Acceptance reuses control cases in both walkers: future boxed I64 aliases,
future Select, Bool/I64 i64 joins, Copy chains, backedges and downstream nonmapped
i64 PHIs. Also use original PHI results in old arithmetic/return to make exit30
depend on the original value; Map readback plus an unrelated constant return is
insufficient. Preserve compatible i1 positives and pending/incompatible i1 Stops.
The existing runtime probe expects one kind per execution: mixed-kind loop
witnesses can observe only the final write while still executing the original
merge. This is test arrangement, not permission to prune domain alternatives.

### Original i1 PHI after-group decision

Decision: normalize an emitted selected flags1 i1 PHI only after its entire
existing PHI group; shared i64 inputs use that fixed original reference.
Source authority + canonical issuer: unchanged V2/body relation; existing PHI
prepass/refinement and actual emitter own width and SSA. Group-end owns placement.
Non-authority: Map kind/payload, alias guesses and a second CFG cannot issue width.
Fail-fast boundary: missing emission or unsupported actual width rejects; future
boxed/Select inputs into i1 PHIs retain their existing pending/width Stops.
Smallest next slice: one shared group-end normalizer and the two existing walker
calls, then PHI-only i1 reference selection in the shared i64 formatter.
Non-claims: public cutover, typed/direct escape, intrinsic binding, boxed comparison.

Read-only worker audit identifies the generic active walk's complete PHI loop
and the same-module body emitter's complete PHI loop as the two actual callers.
Emit before entry runtime/layout work or skip-plan/body work respectively.
Iterate the existing block PHI inventory; after successful flags1 Map PHI emission,
T_I1 emits `%map_original_dst = zext i1 %rDst to i64`. Keep original SSA/type,
Map lanes and predecessor labels unchanged. Add no branch, graph or receipt.
Shared reference formatting uses this fixed name for PHI/T_I1 and existing rDst
for PHI/T_I64. Formal retains its separate i64 contract. Unknown width rejects.
Delete only the selected-i1-PHI-to-i64 pending Stop after both consumers land;
do not remove the i1 input pending checks or move normalization inside a PHI group.

Acceptance in both walkers: selected i1 -> selected/nonmapped i64 PHI, Copy
chains, forward/backedges, multiple PHIs in a block, and original arithmetic
feeding exit30. Retain original i1 consumers and incompatible/pending rejection.
Both walker group-end calls and the shared formatter now implement this
projection. Pending future boxed/Select i1 inputs remain explicit Stops;
public/source cutover is not claimed.

Implementation checkpoint: normal/ASan434 each, including control106, selected
C build, pointer/corridor and parent-equal Named60/definition22 pass. Both walkers
execute selected/nonmapped i64 consumers, Copy chains, future backedges, multiple
PHIs and original arithmetic yielding exit30 while retaining original i1 consumers
and incompatible/pending input rejection. Source max712. The former selected
i1-to-i64 Stop is deleted; next runtime escape design is separate and remains open.

### Versioned compiler frame decision

Decision: replace the selected static ingress with one v2 synchronous frame;
retain body JSON as the sole operand/CFG graph. The existing published frame
owner reads finalized MIR before lossy JSON and owns all buffers until C returns.
No second MIR, projection-register namespace, source receipt or independently
issued call graph is added.

`PublishedStaticMethodCFrameV2` now owns the four wire arrays and all strings,
reusing the existing selected call-frame backing. It builds complete actions and
original-use closure from the same view and copied Named observations. Formal
rows select expanded definitions; body params remain the only formal ordering.
Internal target generation skips collisions with all original function names;
typed call rows keep their canonical logical target for the C expanded index.
Empty arrays use null pointers and zero counts. Header pointers remain valid
across owner moves and after the original view/module is dropped.

Verification: Map36 plus live query integration1 pass. Owned-frame witnesses
cover recursive Integer/Bool/F64 actuals, Map-only versus original-needed formal
flags, unchanged logical call arity/target, collision avoidance, empty tables,
foreign observations and original-required Float rejection. The actual C query
result also constructs this frame before compile/cancel dependency checks.
Corridor/pointer guards pass; sources stay below800. This is wire construction,
not C admission, expanded LLVM emission/internal linkage or source cutover.
Next is the shared C frame/expanded index and both consumers, including exact
site/row closure and ingress rules above; then input-aware capability and public
session/host/source switch. No further schema-only public entry is authorized.
 The planned compile export is
`hako_llvmc_compile_published_static_method_v2(session, frame, obj_out, err_out)`.
This consumes the same retained invocation used by query, not a pathname that
would reparse the body. Public open/query/compile/close and the host switch land
together only after the full V2 consumer; the former json_path proposal is
superseded. The frame carries revision/byte size and pointer/count pairs for four tables:

| Table | Payload and identity |
| --- | --- |
| Existing selected calls | Keep current call/Array row layout and meaning; exact selected site and canonical target projection. |
| Map operations | Function + block/instruction site and allocation/write discriminator. Receiver/key/value/dst come from the matched body instruction, not another operand array. |
| Value projection | Function + SSA ValueId, finite action and original-required flag. Admission binds a unique body definition or formal. |
| Expanded functions | Canonical definition's logical physical name and one internal LLVM target. Formal order comes from body params and Formal value rows; no duplicate formal list. |

Value action is a tagged finite payload: ExactBits(tag,u64),
OriginalValue(tag,encoding), Copy, NamedAliasOperandZero, Phi, Select,
Formal(source ordinal), or Operation(selected physical consumer). Operation has one finite wire field;
non-Operation rows keep it zero. Its result kind/encoding are fixed by the
selection, not independently chosen by the caller.
Encoding is existing i64 bits or Bool i1 zero-extension only. ExactBits tags use
runtime v1 I64/Bool/F64/Void; Handle uses a proved original allocation/result.
Copy/Phi/Select carry no operand/edge arrays: C reads them from the exact body
instruction. Formal rows bind body params by ordinal, never by spelling. Unused
variant fields, invalid tag/encoding combinations and duplicate definitions
reject. Counts, null pointers, revision/size, definition/opcode identity and
reference closure are checked before artifact output. This is a fixed-width
C representation with Rust-owned backing, not an extensible JSON blob.

The planner computes two physical demands over the same SSA graph: Map
representation and original-consumer use. Map writes seed the former for value
and the latter for receiver/key. Existing non-Map consumers keep original uses;
Copy/Phi/Select propagate demand on their supplied edges and Select keeps its
original condition. Ownership-bearing CopyOwned retains its existing original
operation. Exact call key/ordinal propagates both demands to a finite fixed point
before any signature is emitted. No first-call specialization or runtime kind
inference is involved.

A demanded formal always receives Map kind/payload. It additionally retains an
original lane only when an original consumer needs it. This avoids fabricating
an unused old-i64 argument for a Map-only Float formal. Both caller and definition
read the same planned layout. OriginalValue requires an original producer;
Map-only ExactBits/Copy/Phi/Select need not emit an unused old value. Do not erase
an effectful producer or re-evaluate an actual. If an original lane is required
but its existing backend contract is unavailable, preserve that explicit
unsupported boundary; do not fill zero/poison or overwrite it with Map bits.
No optimization based merely on equal transport widths is admitted.

Both C walkers use one frame admission/index, Map operand formatter and shared
operation emitter. Register all expanded function layouts first, including
recursive components. The same-module save/activate/restore changes lookup
context, not the frame-wide consumption ledger. Expanded definitions bypass
old leaf emission. A region shortcut cannot skip a demanded producer and leave
an undefined side value; preflight and residual checks must catch it.

Map status branches pre-plan their LLVM tail labels through the existing
`phi_predecessor_label` owner, including backedges. Emit tag/payload PHIs in the
existing PHI group. Bool zero-extension belongs at the producer/predecessor,
never between successor PHIs. Both Select lanes reuse the original condition;
boxing occurs only at the runtime Map write. Exact Float constants use their
retained bits without modifying a lossy original register.

Acceptance spans both walkers: exact constants, Copy, mixed PHI/Select, Bool
normalization, F64, Map-status-split predecessors/backedges, recursive/forwarded
formals, original-needed versus Map-only formals, leaf bypass, private linkage,
and malformed/missing/duplicate/residual rows before object. Source acceptance
and six-edge retirement remain the series finish line below. The v1 export is
retired with all selected host callers at cutover; do not leave a Map v2 route
beside an independently selected static v1 route or add a fallback adapter.
Lifecycle V4 and explicit generic compatibility remain separate existing owners.

### Formal / expanded consumer implementation boundary

Decision: one invocation index binds expanded definitions and exact incoming calls;
its ordered body params plus Formal rows select both signature and actual lanes.
Source authority + canonical issuer: existing published call/definition relations
and Rust MapBodyIndex demand closure; no new source or semantic issuer.
Non-authority: C names identify physical targets only; prepass, JSON callee names,
first caller and transport observations cannot select a formal's value kind.
Fail-fast boundary: malformed/incomplete correspondence, old-ABI ingress, internal
symbol intrusion or missing original representation rejects before publication.
Smallest next slice: consolidate exact Static/Free output in both walkers, then
connect the shared expanded index to admission, closure, signature and call output.
Non-claims: public V2/source cutover, object escape and original PHI normalization
remain separate open acceptance requirements.

Read-only audit at f242a2320a fixed the physical insertion points and task order:

1. `mir_call_dispatch.inc::emit_published_i64_call` becomes the single selected
   Static/Free argument/output/consumption owner for both walkers. Preserve the
   existing original-i64 behavior during this consolidation.
2. Extend `HakoStaticV2Index`, not a sibling signature/operand table. Bind Formal
   by exact body param ordinal; bind each incoming actual through its selected
   call site. Closure follows these cross-function edges for reachability,
   original demand and whole domains, including recursive components. Formal
   has no literal domain seed. Body coverage exempts only those actual lanes
   whose exact demanded Formal has original-required clear.
3. The existing same-module signature/prepass emits internal targets, two Map
   lanes and only demanded original lanes. Record Formal consumption on actual
   entry emission. Exclude expanded identities before numeric-leaf eligibility;
   enumerate expanded leaf-only members from the existing plan's leaf/same-module
   union without rewriting plan intent. Keep emitted progress at logical identity.
4. Both typed callers use the same index and original Call.args; results keep the
   existing i64 contract. Reject expansion of the entry selected by program owner.
   Check actual first-match lowering_plan physical targets (global/user-box
   target_symbol, extern symbol and generic-method runtime symbol), not arbitrary
   JSON strings/source_symbol. Only a valid exact typed row at that same site may
   shadow old ingress; it must actually select the new call. Preserve metadata
   validation/prescan and the existing no-op forward declaration policy.

Acceptance includes both walkers, ordered/aliased actuals, leaf-only definitions,
recursive forwarding, multiple callers with different kinds, flags0 Float,
flags1 original use, valid/forged typed shadow, nonshadow old ingress, internal
symbol collisions/intrusion and root Formal refusal with previous artifact intact.
Do not close this row with signature output alone: old signature, numeric-leaf
re-entry, independent old argument loops and unconditional param-i64 handling
must disappear for expanded definitions together.

Consolidation verification: Original67 passes, including four Static/Free,
generic/same-module ordered-argument and Copy-alias kernel readbacks. C build,
corridor/pointer guards pass; Named60/definition22 remain parent-identical.
Two independent selected argument/output loops are replaced by one helper;
Formal/expanded admission and execution now join the same invocation index.
The private consumer binds body params/exact incoming calls, converges cross-call
reachability/domains/original demand, emits internal signatures and normalized
actual lanes, and excludes old numeric-leaf emission. Entry/alias collisions are
checked against the existing program selection, not document names alone.
Worker implementation audits found that missing check; it is fixed and covered
by default/custom entry collision witnesses. Normal/ASan each pass396 full cases
with175 kernel readbacks, eight status traps and the non-host domain probe.
Final coverage adds duplicate/formal-wire/unplanned-definition and StaticMethod
witnesses; final focused85 cases pass in each build (28 kernel readbacks each). Existing
Named60/definition22 match parent; C build and corridor/pointer guards pass.
The former unimplemented-Formal diagnostic assertion is replaced by rejection of
Formal attached to an instruction: it is now a supported action with a required
parameter identity, not an unsupported action. No public/source cutover or
original PHI normalization claim.

### Projection preservation and runtime-first task order

Premise correction: requiring every non-Map consumer to gain tagged semantics
before implementing Map is too broad. Preserve its existing operands and
physical dispatch. Only a consumer whose input is changed by this projection,
or whose output supplies the Map demand, is a cutover dependency. Existing
non-Map behavior is not thereby proved correct. Keep separately named static
findings distinct from test-reproduced baseline debt.

The unannotated `stash(x) { local m = %{"v": x}; print(x); return 7 }`
counterexample is not direct-static ExactI64: Print has no result-proof statement
arm and produces UnsupportedStatementKind/ResultUnavailable. An explicit i64
return annotation bypasses that body proof, so that different case cannot be
excluded by this argument. Neither case has been executed in this audit.

Map projection must not overwrite the existing register with a differently
interpreted payload. I64/Bool/Handle/Void may share a physical lane only when
identity of the payload is established. Float's exact Map bits require a
separate physical projection from the currently lossy generic register; merely
sharing64-bit width is invalid. Copy/PHI/Select and demanded formal forwarding
must preserve these projections on the same exact edges, with no reevaluation.
This is physical representation of one SSA value, not another semantic issuer.
Print/origin, generic integer arithmetic and Any Array decoding remain named
static findings outside this change only while their original input and dispatch
are unchanged. A changed input is CutoverBlockerOpen, not a baseline waiver.

Decision: implement the already closed runtime v1 contract first as a bounded
contract dependency of the same Map cutover series. It has an exact input,
existing storage/clone/cache owners and an observable runtime terminal; it does
not depend on solving the compiler frame. This deliberately replaces the prior
blanket prohibition on all code until the compiler mapping closes. Source/MIR,
compiler production route and fixtures for source capability stayed closed in
that runtime dependency. The next compiler series follows the frame decision
above; design evidence is not execution evidence.
Runtime tests prove that ABI only, never Map source/OBJ/EXE or retirement.

Ordered work:
1. `MAP-LITERAL-RUNTIME-CONTRACT-I0` is implemented: explicit-kind store, shared
   live Map decode branch and length-aware String entry. Eight Map ABI tests
   (including export-name composition and StringView lifetime), two String tests,
   38 codec and11 legacy Map tests pass under the jobs4 locked quick kernel target.
   Scalar paths allocate boxes directly without registering handles; this is
   code-inspection evidence, not a global allocation counter or performance claim.
2. `MAP-LITERAL-COMPILER-CUTOVER-I0`: use three ordered implementation steps.
   First add the MIR/Core operation contract, owner-local projection/frame and
   focused verifier/transport tests, leaving literal issuers unchanged. Next
   implement v2 admission and both C walkers with malformed-input and physical
   LLVM coverage. Finally switch source issuers and all selected static host
   callers together, retire v1 ingress and the six literal edges. Intermediate
   contract evidence cannot claim production Map execution.
3. Natural populated/nested/mixed-formal Map-only OBJ/EXE and the six old-edge
   deletion close the series. Runtime completion alone cannot advance this line.

Runtime acceptance includes scalar classes, Integer/live-handle collision,
malformed Bool/Void, exact F64 bits, invalid handles/no mutation, retained String
lifetime, nested clone behavior, duplicate keys, canonical numeric key versus
noncanonical text, and empty/UTF-8/NUL keys and String values. Compiler acceptance
must prove evaluation order and no temporary scalar handle registration.

Fail-fast boundary: missing intrinsic allocation/write/representation products
reject before artifact, without Named recovery or legacy retry. Map-only real
OBJ/EXE acceptance is required; empty Map cannot stand in for populated values.
Non-claims: this design review changes no source/code/runtime behavior, adds no
Map lifecycle receipt and proves neither full Map execution nor R7 completion.

Finite design inventory: source raw/normal Script/Core issuers; existing target
product and Core verifier/lowerer; clone/remap/Core13; JSON emit/import and printer;
published view/frame/exact-site C allocation/write consumers; target-sensitive
method/map/typed-object origin observers; explicit named-new/Main callers; and
nonselected backend terminals. Classify wildcard observers explicitly: compiler
exhaustiveness alone cannot prove no intrinsic-to-named reclassification.
Step1 must account for these mechanical readers/writers and their explicit
intrinsic or unsupported treatment. Compiler exhaustiveness does not cover
wildcard observers. This is a series acceptance inventory, not an Exhausted
claim from source/C design reviews.

Bounded reader checkpoint (static inspection at the Step1 substrate):

| Boundary | Observed treatment / remaining obligation |
| --- | --- |
| Core verifier -> lowerer | `effect_validators.rs` checks three write IDs and empty IntrinsicMap args; `effect_emission.rs` copies target/operands unchanged. |
| MIR uses/effects -> remap | `instruction/methods.rs`, `query.rs`, `value_consumer.rs`, JoinIR remappers and simplify-CFG flow preserve three uses/no result and mutation/IO; freshen collector/remapper/verifier retain the Core effect. |
| MIR -> generic JSON / printer | Explicit intrinsic target and result-free write in `mir_json_emit/emitters/mod.rs` and `printer_helpers.rs`; no Named reconstruction. |
| JSON -> nonselected import | `mir_json_v0/module.rs`, `json_v1_bridge/parse/instruction.rs` and vm_hako subset reject explicit target before Named type handling; unsupported opcode remains unsupported. |
| Published static view / V4 | Static view currently stops the two new forms before artifact; V4 `physical_program_json.rs` returns instruction-unsupported. v2 C admission/emission remains open. |
| Interpreter / WASM | Named-only NewBox arm excludes IntrinsicMap; unsupported-instruction terminal handles it and the new write. No backend expansion. |
| Map content observation | `value_representation_fact.rs` invalidates receiver-origin collection facts at a write, including aliases; focused mutation evidence belongs to the substrate commit. |
| Named origin observers | `map_missing_empty_route_plan.rs` requires Named MapBox; ordered-map and generic flow origins retain Named/metadata logic. These are not intrinsic admission authorities; stale plans and metadata-fed consumers still require cutover validation. |

This checkpoint covers the listed product/reader edges only. It does not close
every wildcard observer, stale compatibility plan, C region shortcut or exported
ingress; those remain Step1/Step2 obligations in the finite inventory above.

Retirement set for the eventual series: both Map literal Named allocations and
both literal birth emissions and both named literal set emissions when the
explicit write consumer is proven. Keep shared birth helper (`decls.rs` still calls it
for named argv Array), named New/provider behavior and live generic C routines.
Acceptance must cover natural empty/populated/nested Map-only source, named
shadow noninterference, key/duplicate/evaluation order, undefined-child boundary,
explicit named New, missing/malformed/duplicate/residual rows and actual OBJ/EXE.
Earlier MIR parity tests remain dependency evidence only. A supported source
must not be replaced by an easier fixture to hide an earlier terminal.

Map AST-free lifecycle remains a separate open obligation: body shape now
retains ordered keys and exact EntryValue relations, but Script final handoff
still seals Array lifecycle only. Passive Map structure is not slot transfer.
Do not borrow Array claim/Home/Fault proof. Absence of this lifecycle issuer alone
also does not prove generic Map artifact execution impossible: the static path
can select mixed modules. The planned dedicated write consumer replaces that gap; Map execution remains
unproved until source/caller switching and artifact acceptance complete.

### Static V2 host checkpoint and remaining selection (2026-09-09)

The selected static host retains Library/document/config across actual Named
query, existing frame planning and compile, then closes before unload on every
path. The C invocation is initialized at its final address. Public V1 file
entry and Rust feature branches are deleted; V2 has the feature-disabled stub.
Generic scalar/nested Map retains the runtime copy/boxed contract, distinct from
checked V4 Home transfer. Raw/Core literal Named allocation/birth/set are replaced
by IntrinsicMap/MapLiteralEntryWrite in their original child evaluation order.
No query-only host route or V1 retry exists.

Finite retirement boundary: selected Rust static object/EXE host -> C published
V1 ingress -> compile core, and the two raw/Core literal emission arms. Includes
host/stub/export and dedicated proof callers; excludes explicit generic
compatibility, shared call-row schema/core, checked V4 and callable checked Map
selection. The old V1 symbol/caller search is zero in src and lang/c-abi source;
the shared frame backing is retained. Six literal emission edges are deleted,
not six whole semantic families. Whole Map/R7 completion is not claimed.

Evidence: natural materialized Script6 -> direct/linked EXE12 exits30 (empty,
all scalar classes/UTF-8, shadow MapBox, duplicates, nested, prior Local); existing
Array5 -> EXE10 passes unchanged after rebuilding the required release runtime.
Initial Array shadow linking lacked `nyash.box.from_i8_string_const_len_v1` in
the stale archive; nm and reproduced linker error proved the missing dependency.
No String downgrade or fixture rewrite. Map planning/opcode36 and literal4 pass;
Named60, ASan document22, query16 and published row/preartifact proofs pass.
Rust actual-C query/host test and both exact Script source positive/negative
tests also pass; the earlier zero-test module filter is not counted as evidence.
Logs use `/tmp/hakorune-static-v2-*.log`. Initial normalizer/Local test expectations
counted legacy MethodCall; they now check ordered intrinsic/write output.
The query-only invalid document retains its source and now rejects at the V2
function index before schema/residual; the full residual proof remains in the
published-row test. The stage order is fixed in
[the ABI reference](../../../../reference/abi/nyrt_c_abi_v0.md#selected-static-compiler-v2-retained-invocation).


Static V2 closeout: Map source6/direct+linked EXE12, unchanged Array5/EXE10, Map36/literal4, exact Script2 and
actual-C query/host1 pass. C Named60/document22/query16 and published-row proofs pass. `CARGO_BUILD_JOBS=4
cargo check --locked --profile quick --lib --no-default-features` fails identically on parent41912dd3bc and
current at 11 unchanged runtime plugin-stub sites (plugin_loader_unified.rs and semantics.rs;
E0308/E0609/E0599). Classified known baseline debt, not non-plugins green; logs
`/tmp/hakorune-static-v2-no-plugins{,-parent}.log`. Parent used tracked source archive plus the unchanged
local Cargo.lock. No dependency resolution or stub repair included.

The temporary query bridge retains its original non-Map compile / Map-only
cancel boundary. The latter fixture has no executable physical root result;
trying to compile it is not additional Map acceptance. Do not replace its input
with an easier result or count it as a source proof. Runtime ABI/body contracts
and full ambient/global-row concurrency remain separate from source meaning.

### Declared-root Map install selection

Decision: stop unissued selected callable Map consumption at existing package
install; generic clone compatibility is not owning-slot admission. This replaces
our earlier generic/checked routing premise, under ownership.md's slot-destination
and ordinary noescape parameter contracts. Runtime scalar representation proves
neither an owning acquisition nor Map residence/cleanup permission.

Change: existing `preflight_map_install` enumerates `batch.declarations()` ->
`body_shape().expressions()` -> exact `MapLiteral` site and matches the existing
ledger flow. Delete the owner-none unconditional install-success edge for these
sites; keep existing annotation/destination/terminal validation.
Contract: body-shape issues membership only; existing ordinary_new_coseal and
Completion/Home flow issue ownership. Missing flow uses existing
`MapLifecycleConsumerMissing` before catalog mutation or child lowering. No new
receipt, AST walk, inferred capabilities or retry through generic emission.
Done: admitted AppMain and no-Map declarations remain accepted; non-AppMain
formal, nested/return/argument/control Map without flow rejects with unchanged
catalog. Existing Script V2 cohort remains separate. Run focused package tests
and existing pointer/corridor guards; no backend expansion.
Stop: missing shape ownership or a consumer outside this finite inventory
requires separate mapping, never a skipped Dynamic slot or default permission.

This census covers: declared-root callable batch body-shape -> package install.
Includes same-owner nested/branch/block/argument Maps and Dynamic-selected
**declaration roots**. Excludes Script root (batch issuer requires DeclaredFunction),
separately transferred lambda bodies and explicit compatibility entry. Script
programs may still contain declared functions in this batch. No all-executable-Map
or Exhausted claim. Worker premise and API audit confirmed this boundary.

Validation: Map home-flow11 pass, including five unissued declaration-root
source cases and retained admitted AppMain/no-Map declarations; unchanged
materialized Script6/direct+linked EXE12 exits30. The initial argument fixture
used an undeclared call and stopped at UnissuedDirectCallObservation; declaring
Helpers.consume preserves the argument Map and reaches the intended install
Stop. Logs `/tmp/hakorune-map-install-{inventory-fixed,script-regression}.log`.
No new backend or speed claim.

Next ordered cutover blocker: source capability plus Map residence/Fault/cleanup
co-seal for mixed-formal and
other unissued candidates. Existing parameter OpaqueHandle/ExactTrivial/ExactText
and physical tagged formals do not complete those obligations. Keep the same Map
series open; this Stop is retirement of invalid admission, not execution coverage.

### Transferred lambda Map: selected C terminal inventory

ParkedSealed at 6494f39ff8, read-only worker audit. Boundary: callable/Script
lambda source owner -> selected static V2 JSON export or lifecycle V4 physical
input. Includes inline/body_id-backed closures containing Map and nested lambda
bodies; excludes nonselected backends and a future canonical lambda-body consumer.
This is a pre-artifact/body-execution Stop, not a pre-MIR-mutation guarantee.

| Owner / path | Existing terminal or handoff |
| --- | --- |
| Source owner construction/resolver | Callable child uses SelectedCallable; Script child uses ScriptLambdaLeafV1, which rejects nested Lambda. Child body-shapes are issued separately. |
| Callable batch issuer | Keeps declared-root shape; transferred child shape is not part of its Map preflight inventory. |
| raw_lambda_capture_lifecycle / raw_lambda_closure_emission | Consume source capture relation; retain body AST in NewClosure/module closure_bodies without lowering body Map to generic MIR. |
| mir_json_emit/emitters/calls.rs::emit_new_closure | Body ID or nonempty body rejects with closure-body-wire-unavailable. Static host exports before output-parent preparation and C invocation. |
| published backend physical_program | NewClosure is absent from both ordinary/Script allowlists; instruction-unsupported. Callee::Value may stop even earlier at UnsupportedBeforeObject. |

Non-authority: capture receipt, retained AST body and absent child shape cannot
issue owning-slot permission. An empty body descriptor cannot contain Map and
is not a bypass. Existing body_backed_new_closure_rejects_lossy_wire_projection
is a local refusal test, not a natural-source lambda Map execution proof; no
build or test was run for this census.
Reopen on selected acceptance of body-backed NewClosure or connection of a
lambda body -> canonical MIR consumer. Before that consumer is enabled, retain
source owner, exact child Map membership, capability and cleanup together; no
AST rescan or parent-owner repair. General lambda lineage/capture remains with
its existing later migration owner, not a new Map implementation lane.

### Owning entry and ordinary callee: accepted dependency design

Decision: connect entry capability and per-callable cleanup at the existing
package source issuance; preserve the owning-slot target. Generic clone-backed
static execution remains compatibility evidence, not an alternate owning family.
The required final result includes natural caller -> ordinary callee -> Map
install/replacement/end -> OBJ/EXE, with source-authorized transfer and Fault order.

| Existing authority / input | Entry meaning and retained obligation |
| --- | --- |
| ExactTrivial parameter contract | Currently exact I64 only. Store the proven value without a child Home; Map/key residence still needs its own cleanup. |
| Available selected Home local | Transfer the same acquisition once at install commit; remove only that local's responsibility. |
| Literal/Trivial local source relation | Use its actual issued capability, never a fabricated Home. Current Map walk does not yet admit it. |
| ExactText / OpaqueHandle formal | Handle contract alone gives no owning acquisition. Keep refusal; caller literal bits cannot rewrite the callee contract. |
| Alias/consumed/uninitialized/unknown or unsupported transfer formal | No owner recovery, implicit share, or fallback. Preserve distinct source refusal. |

For every admitted entry, precommit failure leaves the candidate with its prior
owner. Install commits once, then detached old-end runs; an old-end Fault cannot
undo commit. Trivial values have no child-finalization action but occupy the
same ordered slot/replacement structure. Map/key native cleanup and first-Fault
preservation remain mandatory. These are the existing language laws, not new
capability inference from physical tags.

The package issuer passes its retained parameter contracts to
`issue_ordinary_source_cohort_v1` in the same declaration loan as New candidate
and Completion issuance (60b0abe3d6). The Home scanner now consumes those exact
contracts without erasing I64 to a coarse HomeDemand. That existing issuance
remains the connection point. `MapHomeEntry` currently means
Home acquisition/transfer; its internal representation must distinguish a
proven Trivial value from that obligation without sentinel acquisitions or a
parallel receipt. A per-callable Completion must preserve the callee's normal
result and cleanup/Fault obligation; the present single root_completion cannot
be relabelled as arbitrary-callee proof.

Ordered tasks, all inside the existing Map series:
1. Fix the source-to-Completion mapping in the existing package/Home walk:
   exact EntryValue + declaration-issued capability + availability + normal/
   Fault obligations. Enumerate all current entry arms and opaque inputs;
   specify ordinary-callee completion alongside root completion. Delete the
   implicit all-entries-are-Home assumption only with its actual replacement.
2. Connect the required value/residence cases in existing checked Map storage
   and install ABI. Today InstallIndexed/IndexedResidence require object ID and
   indexed handle; I64 bits cannot use them. Retire the selected indexed-only
   assumption while retaining its real Home consumer, key failure, detached-end
   failure and reverse-order cleanup. No generic MapBox clone adapter.
3. Extend existing physical program/compiled-entry invocation for ordinary
   callable completion. Today roles/functions are Root+BirthUnit and Invoke Call
   accepts only BirthConstructor. Specify arguments, normal-only result and
   Fault/cleanup transfer with the existing Fault ABI before enabling the role;
   never borrow the infallible static calling convention as a silent substitute.
4. Connect the natural full source chain, validate normal/Fault/duplicate-key/
   alias refusal and both direct/linked artifacts, then remove the corresponding
   package/physical Stops and obsolete exclusive paths in that cutover series.

Acceptance cannot be met by scalar-only source, physical-tag tests, parser green
or test-only issuance. Keep current unissued install rejection until the selected
source/consumer chain is complete. No extra semantic program, MapManager or
physical-kind authority. No new Home-transfer syntax, Bool/Float formal contract
or general lambda capability is implicitly authorized.

Concrete ownership decision: non-AppMain Completion stays in the existing
completion_seed -> result_contract -> installed callable loan path. AppMain is
not in that selected seed cohort and retains its distinct root owner. Do not
move or duplicate it mechanically. Entry representation separates proven value
storage from acquisition/binding transfer; no sentinel Home for Trivial.
Ordinary callee uses the existing borrowed Fault frame, exact physical arguments
and normal-only out-result (none for proven Unit). Cleanup precedes publishing
the result. Fault returns the same frame; root alone reports/disposes/checks
process result. Extend the existing V4 role/program/contract; never encode an
ordinary callee as Birth or infer Unit from absent result annotation.

Decision: resolve Dynamic ownership before ordinary Completion/S6C issuance.
Source authority + canonical issuer: existing Dynamic source/Recipe/exit co-seal;
only its successful selected slot controls subsequent ordinary Home exclusion.
Non-authority: candidate presence and a failed Dynamic attempt never choose an
ordinary fallback. Parameter and constructor authorities remain unchanged.
Fail-fast boundary: Dynamic errors precede Completion/S6C errors on jointly
invalid input; package issuance returns without install. This is an explicit
diagnostic-order change, not a behavior-preserving BoxShape claim.
Ordering prerequisite implemented: complete Dynamic selection now precedes
seed / S6C / result cohort, with products and downstream inputs preserved.
Package139 tests pass, including individual refusals and combined first error,
Dynamic selection/exclusion, S6C exclusive take and Map preflight. Log:
`/tmp/hakorune-map-issuance-order.log`. Issuer remains742 lines; no runtime change.
Non-claims: this prerequisite does not make seed Home-aware or delete every
Completion verification. Dynamic admission has its own Completion; its later
ordinary seed/physical-header consumers require separate disposition before any
single-verification claim. No new source family, runtime or Map activation.

After that ordering prerequisite, connect the existing candidate/Home-aware
verifier to the sole ordinary seed issuance with declaration capability input.
Dynamic exclusion and S6C exclusive take must be preserved, while AppMain keeps
its separate root owner. Co-seal and physical mapping remain prerequisites for
production promotion; no deferred-error storage or second Dynamic classifier.

### Successful Dynamic Completion ownership cutover

Decision: replace successful Dynamic's ordinary seed/result ownership with a
borrow of its retained canonical authority Completion. Ordinary rows remain owned.
Source authority + canonical issuer: existing Dynamic source/exit co-seal; existing
seed validation still checks parameter coverage, identity and declared result.
Non-authority: skipping a row, absent header or MIR signature cannot replace the
APrimeI64 physical demand's Completion checks.
Fail-fast boundary: preserve unsupported result/owner/coverage diagnostics and
installed loan mismatch checks before physical lowering.
Implemented: borrowed result/header view and Dynamic-authority-scoped callback;
successful slot second verifier and owned seed/result row are deleted. Package140
passes, including identical Completion pointer, three ordinary sibling rows,
physical header/APrime demand and S6C tests. Log:
`/tmp/hakorune-dynamic-completion-borrow-fixed.log`. Initial missing module
qualification was a current-change compile failure and is fixed.
Non-claims: Dynamic candidates that Decline and later verify as ordinary remain
outside this deletion; no all-verifier-once or Map execution claim.

Actual consumer includes APrimeI64PhysicalDemand's owner/target/return-sites/
cleanup checks. Keep the separate catalog Dynamic physical header. Declared
instance target lookup remains ordinary-only (InstanceBoxMethod); do not widen
that target boundary. Record the validated optional result representation in the
existing selected Dynamic product, not a new Completion owner. S6C never consumes
this Dynamic slot. Acceptance: package/APrime tests, same Completion pointer at
installed borrow, no Dynamic owned result row, retained ordinary sibling, existing
unsupported annotations and S6C take. No runtime/schema change.

### Ordinary source loan fusion

Decision: one existing ordinary source issuer returns the ledger and existing
Completion seed cohort; remove the independent seed batch/source traversal.
Source authority + canonical issuer: exact batch declaration and existing New
candidate/source input. Construct the seed inside the same loan as its candidates.
Non-authority: owner-key pairing after the loan, empty formal contracts and a
second Completion map cannot substitute this connection.
Fail-fast boundary: after Dynamic selection, process batch declaration order;
complete each owner's preflight before the next and take S6C only after all
succeed. Joint-invalid first-error order changes explicitly; no deferred errors.
Smallest next slice: fuse current work without changing verifier selection:
ordinary plain Completion, AppMain Home-aware/root ledger, TopLevel no seed,
Dynamic retained-authority validation. Return a tuple, not a new manager/receipt.
Non-claims: no Map/capability widening or V4 activation. Next slice replaces the
eligible ordinary verifier with Home-aware issuance using the same loan.

Delete-set: independent completion_seed loop and its second source loan. Keep
parameter cardinality/identity, declared result and Completion validation;
ordinary Completion moves directly to seed, then S6C or result cohort. Existing
Map install Stops, AppMain root owner, Dynamic header borrow and TopLevel scope
remain. Acceptance: package tests, explicit batch-order diagnostic case, existing
S6C/Dynamic/AppMain Map/constructor evidence and old issuer caller-zero.

Completed prerequisite: candidates and ordinary Completion share the existing
declaration loan; the separate seed traversal/source loan is deleted. Package141
passes, including the compound candidate/result diagnostic case, Dynamic
same-pointer borrow and existing S6C/AppMain tests. Command: jobs4 locked quick
lib `mir::normal_callable_semantic_package:: -- --test-threads=1`; log
`/tmp/hakorune-ordinary-source-loan-fixed.log`. Existing corridor guard follows
the renamed issuer and passes; changed source maximum733. No Home-aware ordinary
verifier switch yet. Next audit fixes parameter capability input and the owning
Map entry relation inside this loan; checked storage and V4 remain downstream.

### Ordinary Map capability and Completion: bounded implementation

Source slice completed: the Map-bearing ordinary verifier replacement, exact
formal projection, Value/TransferHome sum and Home-only physical refusal are
implemented. Existing Completion cleanup is preserved when attaching root flow.
Package146 and resolved_control_flow33 pass with jobs4 locked quick lib, serial
test threads; logs `/tmp/hakorune-map-capability-final.log` and
`/tmp/hakorune-map-capability-control.log`. The added Text test originally used
unsupported `String`; correcting it to the existing `StringBox` contract resolved
that test-only failure without changing the issuer. Source max699, existing
pointer/corridor guards and diff checks pass. No C/runtime or EXE evidence added.
Next: design the value-capable checked storage/install ABI in ordered task2,
then ordinary Completion borrowing/finishing and callable Fault ABI; the current
install refusal remains until those consumers exist. Do not repeat source-loan
or entry-capability census to select another preparatory row.

Decision: replace plain Completion for Map-bearing Cataloged ordinary declarations
with existing Home-aware issuance in the same source loan. Extend the existing
Map entry sum with Value versus TransferHome; do not introduce a new receipt.
Source authority + canonical issuer: declaration parameter contracts and exact
EntryValue/initializer relations, consumed by the existing ordinary source issuer
and Home scanner. Completion moves into the existing seed/result owner.
Non-authority: Trivial capability is not an i64 wire kind; root ledger presence,
MIR tags, caller literal values and absent observations cannot grant admission.
Fail-fast boundary: preserve unavailable source observations; unsupported physical
Value entry rejects at existing map_install_owner before catalog commit, and at
private emission entry before progress/MIR changes. No generic fallback.
Smallest next slice: one source capability extension plus its required consumer
refusal. No-Map ordinary/S6C keep plain verification, AppMain keeps its root
Completion, successful Dynamic keeps borrowed Completion, TopLevel keeps no seed.
Non-claims: source retention does not implement value storage, ordinary callable
Fault ABI, mixed-formal execution, or Map OBJ/EXE completion.

Boundary: declaration contracts + exact source loan -> Home flow -> sole ordinary
Completion seed/result -> existing install refusal. Includes parameter coverage,
all current observed Map entry kinds and AppMain's shared scanner. Excludes
receiver/capture/transfer-formal admission, new expression capability issuers,
runtime/ABI promotion and no-Map source families.

Implementation order inside this slice:

1. Pass a borrowed ordinal/BindingRef/HomeDemand projection of existing formal
   rows through function_control_new_homes into the Home scanner. Verify exact
   source Parameter owner, binding, ordinal and complete unique coverage. Replace
   blanket parameter EntryDemandMissing; receiver/capture remain unavailable.
   ExactTrivial (currently I64) initializes Trivial availability; ExactText and
   OpaqueHandle initialize borrowed Handle, never an acquired Home.
2. In existing MapHomeEntry retain shared exact site/key/replacement deltas and
   an internal Value or TransferHome payload. Value retains a source binding when
   applicable; TransferHome retains acquisition plus binding. Integer/Bool literal,
   observed Trivial local and exact-I64 formal are Value evidence. Handle, consumed,
   uninitialized, unknown, Float/Void/String/call/nested collection stay unissued
   unless their own capability relation is implemented. No default scalar tag.
   Only TransferHome consumes a Home or updates its outer transfer position.
3. For seed-eligible ordinary declarations with Map membership, replace the plain
   verifier with the Home-aware verifier using the same candidates and formal
   projection; move Completion into the seed inside the loan. Attach root flow to
   existing Completion cleanup rather than replacing crossed-scope obligations
   with explicit_empty. No second Completion in the root ledger.
4. Keep map_flow as a source borrow. At map_install_owner require TransferHome-only
   entries for the present Indexed consumer. begin_map_emission and
   map_candidate_object explicitly reject Value before progress/MIR mutation;
   never fabricate acquisition, binding or object ID. Retain current ordinary
   MapLifecycleConsumerMissing until result-cohort borrowing and callable ABI land.

Delete-set: Map-bearing ordinary plain-only Completion emission; blanket formal
entry refusal; unconditional all-entry Home consumption. Existing source, SSA,
ownership and FFI checks remain. Source files start at Home scanner651,
Map flow232, local flow127, wrapper116, co-seal683; keep each below800 and design
responsibility splits at760 without a new semantic owner.

Acceptance: source package retains Value and Map cleanup for an exact-I64 formal;
repeated Trivial use stays legal without Home transfer; borrowed formal unused in
Map does not invalidate the walk, but storing it is unavailable. Preserve Home
transfer, duplicate-key replacement order and reject repeated Home/consumed alias.
Reject missing/duplicate/foreign formal coverage. AppMain Value remains source
Complete but install rejects with vacant catalog; Home-only AppMain still passes.
Private Value emission refuses before progress/MIR change. Package/S6C/Dynamic
borrow regressions and existing pointer/corridor guards must pass. No new guard
family or runtime benchmark is required for this source-only boundary.

Next consumer transition after this slice: borrow ordinary Completion from the
existing result cohort through the installed callable loan. Replace root-only
ledger lookup for that selected callable without cloning Completion. Coordinate
Map local progress/finishing and value-capable runtime/ordinary Fault ABI before
removing install refusal. Source Complete alone is not physical admission.

Mandatory ordinary connection blocker: the old crossed-scopes-only summary is
removed in the Completion-borrow slice below. Consumers borrow the original
Completion, preserving terminal Map obligations and unavailable analysis.
Ordinary cleanup/frame/result and caller Invoke issuance remain open; no execution
bypass was demonstrated by the earlier static check. Missing/unknown analysis
must never become a generic no-cleanup proof. APrime keeps its own explicit
source-family check and separate canonical invocation cleanup authority.

### Ordinary Completion borrowing and callable ABI frontier

Decision: remove misleading Completion summaries and lend the existing product;
keep ordinary Map execution closed until its caller normal/Fault issuer exists.
Source authority + canonical issuer: existing result-contract cohort owns ordinary
Completion; selected Dynamic lends its canonical authority Completion in install.
Non-authority: crossed scopes, terminal emptiness, function ordinal, symbol,
physical signature and Trivial capability cannot issue cleanup or call meaning.
Fail-fast boundary: APrime retains its exact source-family Completion shape;
ordinary missing call/result/cleanup correspondence rejects before physical input.
Smallest next slice: BoxShape replacing summary/forwarders with actual Completion
borrows in result contract/header and their existing consumers; no new receipt.
Non-claims: ordinary Invoke issuance, generalized cleanup-free predicate,
ordinary ABI activation, changed invocation cleanup or nonselected backend parity.

The bounded source audit covers selected Dynamic source admission -> APrime
package-header validation, including source shape, Completion issuance/borrow,
APrime source relation and canonical invocation cleanup; it excludes whole
Dynamic and ordinary physical ABI. dynamic_admission uses the plain Completion
verifier. dynamic_full_body_source fixes root/loop/inner-return statements and
all expression sites; a_prime_source co-seals that exact source/Recipe coverage.
install lends that same canonical authority, never a second owner-matched row.
APrime therefore requires crossed scopes empty AND root_flow absent as its
family-specific shape check. Some(empty terminal Homes) is not admitted.
This is not cleanup-free: invocation_cleanup separately retains substring V10
End at indexOf Fault, inner return and backedge. Preserve that existing owner.

Implementation/deletion: replace result-contract/header summary wrappers with
actual Completion access; update APrime and declared-instance owner checks.
Delete completion_cleanup_is_empty and its sole production use. Tests check
ordinary Map obligations/unavailable through the same borrow, actual APrime
positive/source rejection, and ordinary NotSelectedDynamic. Existing guards and
exact old-name caller search suffice; no new guard or cached cleanup boolean.

Completion-borrow checkpoint: result contract/header now lend the original
Completion; misleading summary and scalar Completion forwarders are removed.
APrime's actual-family positive and ordinary refusal remain green. Tests verify
header/result pointer identity with Map obligations and unavailable analysis,
and additional Map source syntax is refused by APrime's exact source issuer.
Package148, source6, control33 pass with jobs4 locked quick lib, serial tests;
old completion_cleanup_is_empty definitions/callers are zero in src. Existing
pointer/corridor guards and diff check pass; changed source maximum755 (tests).
No C/runtime ABI change or ordinary activation occurred. Next is source issuer
closure in step1 below; no new retained correspondence can be issued before it.

Source issuer decision (audit at9eeed39595): the first ordinary execution
series uses AppMain terminal `return helper(30)` and a Cataloged static helper
with explicit I64 formals, supported Map prefix, and literal I64 return. Exact
Integer literal arguments use the existing resolver argument sites and formal
ordinal/binding contracts; no fallible argument-frame policy is inferred.
The boundary covers AppMain direct-call source -> package -> Builder Call ->
finalized retention; excludes instance/Dynamic/Script and arbitrary callgraphs.

Existing issuer/consumer inventory: direct_call observations retain target/site/
arguments; issue_app_main_direct_call_loan_v1 co-seals header/published identity;
direct_call_loan owns the affine disposition. build's
lower_prepared_app_main_direct_call_v1 currently descends raw arguments and
canonical_direct_call emits scalar Call. Neither issues normal/Fault cleanup.
The Home return classifier has Integer/OtherTrivial/IntegerField/I64Add and
uncovered expressions; Call remains uncovered. NewFaultContinuation requires
a direct-local New and cannot stand in for Call. Ordinary Completion stays in
result_contract, while ordinary_new_coseal now retains its terminal relation through seed/result.
Extend these existing owners and finalized_root_handoff; no sibling graph.

| Source disposition | Selected behavior |
| --- | --- |
| Exact terminal Cataloged call, literal I64 args, exact I64 formals/result and complete Map obligations | Co-seal caller/callee relations; named pre-effect Stop until physical consumer exists |
| Selected lifecycle target with missing/foreign argument, result or cleanup | Reject before effects; no scalar retry |
| Existing non-lifecycle scalar call | Preserve existing scalar owner |
| TopLevel, instance, Dynamic, nested/local-initializer call, nonliteral argument, recursion | No implicit promotion by this series; preserve existing refusal/owner |

The bounded BoxShape preserves parameter contracts through the existing
Home walk instead of projecting to HomeDemand and losing exact I64. Its sole
production caller is ordinary_new_coseal -> function_control_new_homes ->
scan_new_home_flow -> PrefixLocalFlow.install_parameters. Delete that lossy
projection; retain binding/ordinal/cardinality checks and OpaqueHandle/ExactText
handle semantics. No new type, cache or physical authority is required.
Acceptance: repeated and aliased I64 formal values retain kind and no transfer;
missing/duplicate/foreign parameters and borrowed-formal entry refusal remain;
ordinary package install still stops. Existing package/control tests and guards
suffice. This does not implement Call issuance or ordinary execution.
Implementation checkpoint: the sole production projection now passes the exact
contract kind; PrefixLocalFlow retains I64 through aliases and rejects foreign,
duplicate, missing or wrong-ordinal bindings. Package149/control33 pass; ordinary
Map install still stops. Changed source maximum700. The selected Call co-seal
is next; do not repeat the closed source/physical census or activate scalar retry.

Ordinary terminal retention is the first required co-seal edit. The existing
Completion seed/result row moves the source walk's optional TerminalRelation;
its existing Ref borrows it. AppMain still moves its relation only to its root
ledger; plain ordinary and Dynamic retain no inferred terminal. S6C consumes
all seed parts and refuses an unexpected terminal instead of silently dropping
it through take_completion. Delete that lossy accessor. Source classification,
Completion ownership, annotation contracts and runtime admission do not change.
Acceptance uses ordinary literal/site/owner, unavailable terminal absence,
Dynamic original-pointer/no-owned-row and existing S6C exclusive-consumption
checks. This is BoxShape, not a new Call or cleanup authority.

AppMain target validation must remain at its current early point before
parameter/Dynamic/Completion failures. Bind lifecycle correspondence after the
existing callee products are available; do not move early validation wholesale.
The existing AppMain Home walk also needs selection for a terminal Call without
local New/Map; ordinary callee presence alone cannot manufacture caller cleanup.
Retention checkpoint: ordinary TerminalRelation now moves through seed/result,
including the natural AppMain -> helper(30) Map callee; unavailable flow retains
None. Dynamic borrows its original Completion with no ordinary terminal, and
S6C consumes complete seed parts with explicit unexpected-terminal refusal.
The lossy take_completion and its unused owner getter are removed. Package150
passes with existing S6C/Dynamic coverage; changed source maximum700. No ordinary
Call/ABI/EXE activation. Next is source Call co-seal, not another retention layer.

Then co-seal the exact terminal Call normal/result/Fault relation in the existing
AppMain disposition and source Completion walk; retain ordinary terminal relation
in its existing result owner. Recipe alone issues logical continuation keys;
physical lowering alone allocates block/value identities. Replace the selected
raw-argument/scalar-materialize edge with its lifecycle consumer (or pre-effect
Stop during the same series). Connect finalized caller/callee and retire the
ordinary Map install Stop only with direct EXE/linked OBJ exit30 and injected
callee-Fault -> callee cleanup -> caller cleanup -> one root report evidence.
Source success or Stop arrival cannot close that execution acceptance.

Terminal Call source connection: the existing AppMain Home walk accepts an
exact Map-targeted terminal Call using the affine row's verified header and
parameter contracts. TerminalRelation::Call owns return/call sites and literal
values only; target and argument-site vectors are not copied into it. Caller
Completion owns the live Home list and outward function, including earlier New
Homes. After callee results exist, co_seal_lifecycle validates the same target,
I64 formal bindings, callee literal terminal and Complete Map flow, then marks
the affine row Lifecycle. Every Map-owned target is checked before disposition;
missing correspondence is a typed source mismatch, never Scalar fallback.

The existing build.rs consumer now extracts Scalar through a fallible terminal
before raw argument descent; Lifecycle stops with lifecycle-consumer-missing.
Finalized Call source remains artifact-call-consumer-missing and supplies no
invented legacy result-ABI variant. Ordinary Map package installation remains
stopped. Non-Map scalar calls and early target/arity diagnostic order remain.
No C schema/LLVM/OBJ/EXE activation is claimed by this source connection.

Source connection checkpoint: package153/control33 and the canonical corridor
guard pass; changed source maximum739. The initial negative integer fixture
used unary `-5`, outside the accepted literal relation; positive uses literal5
and unary remains an explicit rejection. This was a current-change fixture
mismatch, corrected without widening admission. No runtime evidence added.

Ordered ordinary execution frontier (physical inventory at e77b7ab57b retained):
1. Source co-seal is implemented: existing AppMain affine row and
   Completion/Home walk retain exact Call normal/result/Fault correspondence.
   Early target validation remains first; lifecycle binds after callee contracts
   exist and the terminal-Call root is scanned even without local New/Map. Builder
   Invoke Call currently emits Birth only; do not relabel ordinary scalar MIR
   or fabricate a placeholder physical/semantic product. Unconnected execution
   remains stopped.
2. Retain exact ordinary membership, formal/result and cleanup correspondence
   through existing finalized/compiled-entry owners once step1 is closed.
   Existing catalog keys and result/header contracts are available; the header
   explicitly is not a runtime ABI. Do not recover meaning from emitted MIR.
3. Implement caller/callee together: proposed internal status + out-i64 result,
   borrowing caller Fault frame. Callee writes result only on Normal after its
   cleanup; caller normal landing alone consumes it. Root alone reports,
   disposes the shared frame and applies process result policy.
4. Add explicit ordinary role to existing physical program/schema/index/emitter;
   remove only affected nonroot-means-Birth assumptions. Preserve actual Birth
   receiver/layout checks. Connect ordinary Completion and retire its Stop.

Physical inventory: physical_program and compiled_entry_contract retain root/
Birth only; physical_program_json routes both Call forms through encode_birth_call;
C physical_v2 requires nonroot birth_unit/receiver; indexed_flow and emit use
nonzero function index as Birth. Removing only fi==0 Map checks is insufficient.
Acceptance for the eventual series: actual root -> ordinary Map -> i64 result ->
EXE/linked OBJ; callee Fault cleanup before caller cleanup and one final report;
foreign target/formal/result, Fault-result use, missing cleanup, borrowed-frame
disposal and role-mislabel rejection. Keep AppMain source/Fault acceptance.

### Ordinary Call finalization and continuation frontier

Decision: accepted bounded retention, after source checkpoint 11aff90193 and
read-only owner review at cdb45a8ec8. Boundary: installed package -> scoped
lowering -> completed lifecycle -> finalized handoff -> compiled-entry. Source
reclassification and nonselected backends are excluded.

Source authority + canonical issuer: `scan_new_home_flow` issues direct terminal
Call/site/literal values and reverse live Homes; the existing Completion verifier
retains target_function and cleanup. `co_seal_lifecycle` binds that same relation
to the affine target/formal/result row. No Call continuation issuer is missing
for this bounded literal-only terminal shape.
Non-authority: scalar MIR, physical header, Borrowed defaults and a retention
test do not authorize ordinary execution. Loop/If JoinSig contracts are specific
to those constructs; requiring Loop JoinSig here was an incorrect premise.
Fail-fast boundary: preserve ordinary install/artifact and non-Birth Invoke
rejections during retention; execution changes only with their consumer slice.
Smallest next slice: consuming scope finish coupled to successful package
completion, moving existing rows into completed/finalized owners.
Non-claims: no ordinary ABI/LLVM activation, semantic wrapper or EXE proof.

Change: `normal_default_root_catalog_lifecycle.rs` currently drops its installed
package after lowering returns module/root validation/construction. Move the
required existing `result_contracts`, `selected`, `parameter_contracts` through
`BuilderPrivateCallableLoweringScopeV1` into
`CompletedNormalDefaultRootCatalogLifecycleV1`, then `FinalizedRootHandoffV1`.
This removes their package-end drop edge; it does not remove execution Stops.

Contract: consume only after all loans end and existing
`NormalCallableSemanticPackagePortV1::complete` succeeds. Scope
`lowering_started` proves opening, not successful completion. Couple finish to
that existing all-selected/locator/object-definition consumption boundary and
successful outer lowering; failed or dropped ports cannot finalize. Use scope
physical progress, not a new semantic completion receipt. Move original
non-Clone Completion and its same-package membership/formals; no whole package
retention (AST/capabilities), source re-resolution, cloned Completion or sibling
graph. View/compiled-entry borrow from the existing final owner. Keep root,
ordinary, Dynamic and S6C ownership distinct; do not manufacture absent rows.

Done: focused real scope/finished-owner tests show same original Completion
and exact key/formal/result correspondence survive; missing/foreign rows,
unfinished/failed port and repeated finish cannot publish a completed product.
Retain diagnostic and artifact consumer distinctions, with ordinary execution
still stopped. Use existing package/bridge/finalization tests and corridor guard;
no C build is required for retention alone. Sources remain below800.
Stop: if moving the existing products requires reconstructing meaning, retaining
AST or bypassing port completion, return to this owner decision before coding.

Retention implementation checkpoint: existing result cohort now owns the moved
selected/formal context after successful package completion; scope finish is
called only after successful outer lowering. Existing completed lifecycle and
final handoff retain the cohort. Diagnostic validation keeps it for its own
lifetime and does not imply artifact admission. Completion's address is preserved
across the real selected loan and across natural-source artifact finalization.
No package/AST retention, second Completion or post-finalization bind was added.
Changed source maximum712; package155 and bridge4 pass; pointer/corridor guards
pass. A first artifact fixture used a scalar-only root without issued root
Completion and correctly hit artifact-root-completion-unavailable; the retention
test now uses the existing Home-root artifact path plus an ordinary definition.
That fixture correction changes no acceptance or Stop.

Root suite: current24 pass/4 fail; parent5ac03c0e47 gives23 pass/the same4 fail.
Classified known baseline debt, not waived current-change failures. Both used
`CARGO_BUILD_JOBS=4 cargo test --locked --profile quick --lib
mir::builder::normal_default_root_catalog_lifecycle:: -- --test-threads=1`.
Parent ran in an isolated worktree using the same ignored Cargo.lock and explicit
shared `--target-dir /home/tomoaki/git/hakorune-selfhost/target`; Cargo remained
serial. Parent initially lacked the ignored lock; no dependency update or flag
relaxation was used. Logs: `/tmp/hakorune-call-retention-root-final.log` and
`/tmp/hakorune-call-retention-parent.log`.
All four tests are under
`mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::`:

| Test | Matching parent/current cause |
| --- | --- |
| actual_string_helpers_general_result_row_reaches_its_first_loop_carrier | raw-compat/runtime-box-fate-retired/static |
| source_bound_static_result_owner_reaches_the_raw_terminal | raw-compat/runtime-box-fate-retired/static |
| parser_scan_package_passes_callable_source_handoff_without_fallback | static-result-ingress/no-exact-static-target |
| source_backed_package_failure_is_terminal_before_builder_effects | observed RootExpansion, expected CallableSemanticSeal |

The next implementation is the accepted Invoke dependency below; do not reopen
the completed ownership census. Ordinary install/artifact and ABI execution
remain open until their actual consumer and EXE/Fault evidence close the series.

Accepted next dependency: existing selected emitter and ledger realize the
source-connected Call using the same root ledger's Completion/terminal borrow
and the moved affine row. Lifecycle remains a tag; do not accept an arbitrary
Completion parameter or copy target/scope/cleanup into another receipt.

```text
Invoke with shared Fault frame
  Normal -> I64 result projection -> clean cleanup -> Return(value)
  Fault  -> full pending cleanup -> ReturnFault(frame)
```

Literal-only arguments acquire no temporary Home; both exits consume the same
source-issued Home order, with different pending result/Fault. This does not
extend to nonliteral/nested Call, early return or catch/defer. Existing
`emit_root_home_exit_payload` builds a remaining suffix for Fault during cleanup;
Call Fault needs a pending entry BEFORE the first Home. Extend that existing
cleanup builder, never reuse its partial suffix as the full pending entry.

Physical Call result dependency: `InvokeOperation::Call { call, result }` now
uses finite Unit/I64 representation. Unit Birth retains no Normal projection;
I64 requires one. Effects, operands and rewrite share the same Call arm.
Function verification checks Birth/Unit or same-module StaticBoxMethod/I64 and
structural arity, embedded destination absence, exclusive Normal projection and
Fault dominance. This is physical validity only, not source admission.
Existing Birth producers/observers explicitly use Unit. Physical program and
JSON export reject ordinary I64, and the Birth actual collector only consumes
Unit Birth. No new source producer or ordinary C role is activated by this edit.

Physical result checkpoint: Invoke16/package155/physical program+JSON10 pass,
including I64 projection, missing/duplicate projection, embedded destination,
wrong role/arity and Fault-side use rejection. Unit Birth rewrite and existing
source emission checks remain green. The physical input gate rejects ordinary
I64 and mislabeled I64 Birth before JSON. Changed source maximum744 and existing
pointer/corridor guards pass. This closes representation only: source Call
emission/progress and actual binding drift checks remain the next dependency;
no ordinary execution or runtime test was claimed.

Next source-consumer binding must consume the SAME affine Lifecycle row plus
original root ledger Completion/terminal relation, and record target/arguments/
result/frame/cleanup with existing ledger progress. NormalResult physical
support alone must not remove the ordinary package install Stop. Actual target,
argument, projection and cleanup drift checks belong to that consumer, followed
by retained callee role/formal/result/cleanup validation and selected ordinary ABI.

Cleanup physical boundary detail: `root_cleanup_graph::validate_original`
currently requires one entry Jump and 2N-1 releases (N clean, N-1 pending suffix).
Call Fault needs the full pending entry, including the first Home: N pending
releases if materialized in the current graph shape. Extend existing recorded
boundary validation for the two named ingress paths and preserve original edge
identity through finishing. Do not treat an arbitrary incoming edge as permitted,
or skip the first Home by borrowing the partial suffix. Zero-Home Call still
needs Normal-only result and Fault propagation. No generic continuation receipt,
second cleanup graph owner or blind release-count relaxation is authorized.

Use existing RootOwned/Borrowed frame state and ordinary callable validation;
NewFaultContinuation remains New-specific. Normal alone projects result; Fault
runs callee cleanup before caller cleanup, borrowed frames cannot be disposed,
and root alone reports/disposes. Direct EXE and linked OBJ exit30 plus injected
Fault evidence retire ordinary install/artifact Stops in the same series.

### Checked Map Value storage/install ABI: accepted next slice

Runtime slice completed: inline I64/Bool/Residence payload, exact rejected
candidate/detached ownership and shared Indexed/value install helper are in place.
The new value export and C header use named I64/Bool kinds; no scalar residence
wrapper or native projection was added. Storage7 and kernel checked_map9 pass;
the updated quick runtime archive passes the existing host descriptor read test
(explicit --ignored, 1 test) and contains exactly one value-install T symbol.
C header syntax and existing corridor/diff checks pass; changed source max376.
Logs: `/tmp/hakorune-map-inline-{storage,kernel,archive,descriptor}.log`.
Commands use jobs4 locked quick; kernel test/build selects -p nyash_kernel --lib.
Next selected boundary is MIR/physical/C value-install consumption with explicit
representation proof. Ordinary callable cleanup/frame/result connection and the
cleanup-summary blocker above remain open. Runtime green is not Map EXE evidence.

Decision: extend the sole CheckedMap payload owner with I64, Bool and Residence
variants. Keep one MapTable and the existing install/end algorithm. Reject the
boxed scalar residence proposal: a Trivial value needs neither a per-value Box
nor an invented child-Home obligation or virtual no-op end.
Source authority + canonical issuer: source Value/TransferHome stays with the
existing source Completion; runtime only consumes explicit physical ABI kinds
and validates Indexed identity. Runtime payload construction issues no capability.
Non-authority: raw payload bits, object type IDs, native MapBox/clone and symbol
presence cannot grant source ownership or ordinary callable execution.
Fail-fast boundary: reject unknown kind and Bool payload outside0/1 before Key
consumption. Preserve frame/profile/storage/nonoverlap checks, InvalidContract
versus returned Fault, and unchanged install/outcome/disposal transitions.
Smallest next slice: existing CheckedMap payload sum plus one checked value
install export using the same private install state machine as Indexed install.
Non-claims: no compiler admission, public read/clone/host publication, Float/Text/
Handle value kind, C emission or ordinary callable Fault ABI activation.

Boundary: repository CheckedMap payload constructors -> install -> rejected
candidate/detached/terminal end; includes src/boxes and kernel checked Map exports
and tests. Excludes external crate consumers and the later MIR/C/source switch.
Inventory: map_box_checked.rs owns Entry/MapInstallFailure/DetachedMapEntry/end;
map_box_checked_tests.rs owns Probe/pointer-identity checks; kernel
checked_map_residence.rs and its tests own Indexed preparation/reclaim;
fault_checked_map.rs owns checked install; fault_checked_map_storage.rs owns
opaque OutcomeState placement. No second payload table or residence owner.

Implementation:

1. Use a non-Clone/non-Copy CheckedMapPayload enum with I64(i64), Bool(bool),
   Residence(Box<dyn CanonicalMapResidence>). Entry, rejected candidate and
   DetachedMapEntry retain this same payload. One private end match discards
   Trivial values and invokes actual Residence end once. Indexed prepare returns
   Residence without changing identity validation or reclaim semantics.
2. Add nyash.map.checked_install_value_v1(frame, profile, site, map, key,
   kind:u32, payload:i64, outcome)->Status, with named ABI constants I64=1/Bool=2
   in the runtime ABI owner. Do not infer a Handle from i64. Indexed export stays
   the real Home path. Factor its state machine into one private helper with
   candidate preparation after Key Ready->Consumed; do not duplicate validation,
   capacity reservation, commit or outcome publication in the new export.
3. Value kind/bits validation precedes Key consumption. Existing Indexed prepare
   remains after Key consumption. Returned install Fault leaves Outcome Unissued
   and old entry intact, with the exact candidate returned by storage. Dropping
   a rejected Residence wrapper never reclaims its caller-owned object. Commit
   installs new payload before detached old end; old-end Fault never rolls back.
4. Keep native observation of present entries ProjectionUnavailable, even Value.
   Preserve Ending/reentry refusal, reverse order, first/suppressed errors and
   explicit outcome consumption. Host allocator abort remains outside returned
   Fault evidence; inline scalar payload construction itself needs no allocation.

Delete-set: the universal boxed-residence assumption in storage, failure return
and end; indexed install's embedded state machine moves into the shared helper.
Do not delete the actual Indexed Home consumer or add scalar residence wrappers.

Acceptance: existing checked Map/Indexed/Fault tests plus I64 extrema, Bool0/1,
unknown kind/Bool2 preserving Key Ready and Outcome Unissued; Value->Residence,
Residence->Value and Value->Value replacement; failed old Residence end with new
Value retained and remaining real Homes ended. Preserve failed-candidate pointer
identity, Value contents, reverse end, reentry/dispose rejection and native-read
refusal. Verify actual opaque target sizes/alignments (especially OutcomeState),
descriptor decode and new exported ABI. No layout constant/padding assumptions.
Current source owners are245/333/48/80 lines; keep below800.

The later C consumer has explicit indexed-only sites: MapInvokeOperation,
physical_program_json/physical_abi, physical_v2 admission, lv4 index/kind and
indexed_flow, map_emit and runtime_map_symbols. Value installation must consume
an already-proven physical I64/Bool representation, produce an Outcome, consume
Key on the same returned statuses and never transfer a scalar as an indexed
origin. Those edits and the real symbol requirement belong to its selected
consumer slice. Current fi==0 restriction, ordinary callable Fault/frame/result
and result-cohort borrowing are separate explicit execution blockers; a new
runtime symbol or source Complete cannot remove them.

## Remaining source obligations

The original wider source cutover still requires materialized Script Loop,
implicit completion and unclassified nested/opaque/result/alias families to
reach their own selected source-to-Recipe-to-terminal mappings. Existing Core
identity support and physical witnesses do not close them. Map/Main/provider
and named-new semantics remain separate. Do not reopen the closed admitted
Script series merely to repeat its census, or label these uncompleted families
as completed/invalid because another boundary succeeds.

### Map scalar representation retention and selected consumer order

Decision: retain existing exact Integer/Bool evidence through the source local
flow and Map entries before adding the physical Value consumer. Worker physical
inventory and independent source audit at c350d1e354 found no local-kind issuer
in Birth: scalar_actual_kind explicitly rejects Local. Trivial is capability,
not a representation proof. No new receipt, resolver or source family is needed.
Source authority + canonical issuer: expression_source literal relation and
existing PrefixLocalFlow exact BindingRef observations; MapHomeEntry retains
that evidence in the existing Completion flow.
Non-authority: HomeDemand::Trivial, MIR types, raw payload bits and Birth's local
argument row cannot select Integer or Bool. Formal annotation consumption remains
open; unknown scalar representation is retained as unknown, never defaulted.
Fail-fast boundary: assignment/uncovered prefix, uninitialized/consumed binding,
Handle alias and unsupported expression preserve existing unavailable/Stop paths.
Smallest next slice: source retention below is implemented; connect its explicit
representation evidence to the existing selected MIR/physical/C consumer next.
Non-claims: no source execution, formal kind issuance, widened Birth arguments,
Float/Text/Handle scalar, ordinary Fault ABI or Map cutover completion.

Ordered implementation within the existing Map series:

1. Source retention (BoxShape, implemented): replace kind-erasing local storage/observations;
   Map Value retains exact literal or binding plus optional issued scalar kind.
   No second binding index. Acceptance: exact Integer payloads, Bool values, aliases,
   repeated local use, mixed Home/Value replacement, unknown formal kind and
   unchanged unavailable cases. Existing source install Stop stays authoritative.
2. MIR/physical/C: add finite InstallValue alongside InstallIndexed, with explicit
   I64/Bool kind and no object_id/layout/origin transfer. Update invoke_map roles,
   immediate Key consumer, live Map checks, physical exact keys/site/SSA checks,
   index outcome kind, indexed flow, map emitter and required runtime symbol.
   Bool uses the existing normalized i64 payload lane. Preserve fi==0, SafeMutex
   and non-faulted admission;
   native and ordinary callee remain refused until their own selected contracts.
   Both status paths consume Key; only Normal issues Outcome; scalar remains
   available and never acquires/transfers an Indexed Home origin. Share existing
   install status/disposal code. Acceptance includes kinds/SSA/opaque misuse,
   Value/Indexed replacement, prepare/install/old-end Fault, OBJ and linked EXE.
   Physical fixtures alone are dependency evidence, never source cutover evidence.
3. Source connection: per entry, after prior EndOutcome, materialize only pure
   literal Const or retrieve the already bound scalar before PrepareKey. Preserve
   the immediate Key-to-install verifier. Do not hoist all entries or move calls,
   field reads, conversions, allocation or nested collections across PrepareKey.
   No runtime lookup/clone is hidden in binding access. Connect retained evidence
   directly; do not reclassify source in the ABI or infer stale kinds past writes.
4. Ordinary callee frame/result and actual Completion cleanup borrowing remain
   mandatory before removing its Stop; finish the existing natural caller/callee
   source-to-OBJ/EXE series and retire the exclusive old edge.

AppMain Value source implementation: both preflight predicates now admit only
retained known scalar evidence alongside prior Home transfers. The emitter
records pure literal Consts in the existing bindings before each PrepareKey,
checks exact local site/binding correspondence, and keeps map_candidate_object
Home-only. Finishing checks both install families, scalar kind, literal binding
and recorded instructions before/after optimization. Unknown representation and
ordinary callee permission remain rejected. The extended semantic package passes
148/148, including kind/value/family/literal drift mutations after finishing.
Actual source acceptance: issued_map_source_direct_exe_and_linked_object_exit_30
passes with the rebuilt dedicated lifecycle release archive: 16 source cases,
each direct EXE and independent linked OBJ exit30. The mixed Value/Home probe
covers value-install Fault and post-Value-outcome Fault, checking remaining outer
Home ends, Key/Outcome/Map disposal and report-before-frame-dispose. Existing
Home-only probes remain; the shared C physical proof and its 26 negative inputs
also pass. Changed source maximum700; pointer/corridor guards and diff check pass.
The source/property suite validates generated and optimized bodies. This retires
representable AppMain Value's pre-install and begin-emission Stop edges, without
promoting ordinary callee, unknown scalar, Script or transferred-lambda contracts.
Next is step4's ordinary Completion/frame/result audit; no implementation mapping
for that separate boundary is inferred from AppMain or synthetic C execution.

AppMain source connection audit at c40220153a: the next bounded cutover replaces
both transfer-only admission checks (ordinary_new_coseal::map_install_owner and
ordinary_new_local_commit/map::begin_map_emission) for Integer/Bool and exact
Local(Some kind). Local(None) still stops before install/progress; ordinary
callee, Script and transferred-lambda ABI are not promoted by this row.
The selected Map emitter uses the retained MapValueSource. A local read consumes
its exact source site and is checked against value_for_exact_binding; a literal
Const is recorded in existing bindings before its own PrepareKey, after prior
EndOutcome. map_candidate_object stays Home-only. No scalar object/acquisition
or second progress ledger is permitted.
Finishing must enumerate both install variants and match source entry, map,
scalar kind and exact literal Const/value association through FinishedBindings.
Preserve precommit index and committed index+1 cleanup; Value changes no outer
Home ownership. Acceptance extends the existing ignored source direct-EXE and
independent linked-OBJ test with Integer/Bool, aliases/reuse and both mixed
replacement orders, generated/optimized verification and exit30. Include Value
Fault probes/mixed cleanup and finishing mutations for kind/family, swapped
entry/value and literal drift. Replace known-Value Stop assertions while retaining
unknown-kind and Home-only candidate rejection. Retire only representable
AppMain Value -> map-value-consumer-missing in this slice, not the ordinary Stop.

Physical consumer implementation checkpoint: explicit InstallValue is connected
through MIR opaque-role checks, wire projection, physical parser, indexed type/
lifetime admission and shared C install emission. I64/Bool values keep availability;
Home origin transfer remains exclusive to InstallIndexed. Required runtime symbol
inventory includes value install. Source install Stop and ordinary callee ABI
remain open; this checkpoint does not implement step3/4 or claim source cutover.

C verification: selected C build and physical parser preartifact test pass.
The existing Map execution proof links target/quick/libnyash_kernel.a and passes
I64 extrema, Bool Copy chains/reuse, alternating Value/Home replacement, six
Value status paths with actual Key/Outcome disposal and exact observed payloads,
and 26 malformed inputs preserving the previous object. The attempt-Fault probe
uses the shared real indexed rejection transition, not simulated allocator OOM.
Existing V4 Pair/Bool/range and 14 malformed/tag/schema tests also pass. No
new source acceptance or measured compile-time speedup follows from these tests.

Final restored-source Rust checks: mir::verification::invoke:: 13/13,
map_value_wire_kind_is_explicit 1/1 and requires_each_opaque_entry_exactly_once
1/1 pass (jobs4, locked quick lib, serial). The earlier runtime_map_symbols path
filter also selected zero tests and was replaced by the observed real test name.
Changed source maximum744; pointer/corridor guards and diff check pass. No source
Value execution, ordinary callable admission, global baseline reset or removal
of all C searches is claimed. Next is the existing step3 source connection.

Red classification: the initial wrong Rust module filter executed zero tests
and is not evidence. The broader map_ run reports 235 passed/5 failed/3 ignored;
parent6effe70603 sources with the identical jobs4 locked quick command report
233/5/3. All five names and assertion values match: the four historical route/
Map-get failures recorded in the ownership task, plus
mir::function::published_backend_view::tests::intrinsic_map_substrate_stops_before_v1_object_without_compatibility
(CanonicalTyped versus UnsupportedBeforeObject). These are known baseline debt,
not whole-library green. Logs: /tmp/hakorune-map-value-parent.log and
/tmp/hakorune-map-value-rust.log. Parent comparison restored all working sources.

Physical Bool correction: the initial i1-to-i64 proposal was disproved by the
new Bool execution proof (llc type mismatch; current-change failure). The bounded
worker re-audit confirmed const_bool and Bool Copy chains already produce i64
0/1 in Indexed V4. Birth formals stay Tagged, not Bool; comparisons/PHI/Select
are not admitted Bool producers here. Pass that i64 lane directly, preserving
explicit Bool kind, exact index-kind validation and runtime bit validation.
No native/static-V2 representation change or general conversion is authorized.

Source-retention verification: map_value_completion_tests 7/7, semantic package
148/148 and resolved_control_flow 33/33 pass with jobs4 locked quick lib tests,
serial test execution. Source maximum658; pointer/corridor guards and diff check
pass. Existing Map Value install refusal is asserted before catalog publication.
No C build or source-to-artifact execution was part of this BoxShape. Next is
step2, then step3; ordinary cleanup/frame/result obligations remain step4.

When editing the selected C emitter, task13 may remove eager value-budget scans
from indexed lookups; native recursion protection and preceding physical checks
remain. No broad performance claim or new general cache is authorized.

### CONSTRUCTOR-ARRAY-CURRENT-DOCS-R0

Documentation-only reconciliation is complete after Array execution/retirement
and terminal relation cleanup. Its three owners are this SSOT,
`constructor-lifecycle-llvm-lowering-ssot.md` and `src/mir/builder/README.md`.
Keep one current capsule, live contracts and ordered open work; Git owns deleted
migration chronology. Preserve source/physical/Fault/ABI/retirement rules,
unresolved obligations and classified baseline evidence. Compare real callers,
retained products and CURRENT_STATE; verify incoming anchors, outgoing local
links, pointer/corridor guards and diff check. No source/test deletion or build.

The terminal enum row is closed at `a1d86db262`; semantic-package README cleanup
was already closed and is excluded. Physical storage wire-tag naming is also
closed in the [constructor follow-up queue](constructor-lifecycle-llvm-lowering-ssot.md#feedback-reconciliation-follow-ups-2026-09-08).
Next is the existing Map construction obligation above, then the rolling
backend/runtime order; no second implementation task is added.
