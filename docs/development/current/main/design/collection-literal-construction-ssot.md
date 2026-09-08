---
Status: Active contract; retained Script Array execution and bounded retirement verified
Scope: intrinsic literal identity, retained Script numeric Array lifecycle and selected native LLVM C execution.
---

# Collection literal construction

## Current capsule

- **Current decision:** preserve Named versus IntrinsicArray identity, one source lifecycle/Recipe and one runtime storage/contract owner.
- **Current implementation status:** raw/typed-local/Core literals preserve intrinsic identity; selected retained Script Arrays execute through the checked native ABI and bound V4 OBJ/EXE path. Literal birth and selected duplicate/preparation/projection/Stop edges are retired.
- **Next ordered task:** implement the accepted three-step Map compiler cutover below, after the completed runtime ABI dependency; remaining canonical/compatibility and backend/runtime work follows.
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
MIR effects. A direct Map value MethodCall has no MapEntryValue parent body-shape
relation and therefore reaches continuation `MissingParent`, wrapped as source
`IntegrityInvalid`. The natural counterexample is
`local text = "abc"; local map = %{"n": text.length()}`. This is a static code
conclusion, not an executed fixture. Core value-if's separate pure-branch checks
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
Named allocation, boxed-sum site/ABI binding and ownership capability retain
their separate obligations. In particular, a Rust TypedObjectPlan match does
not prove which C NewBox consumer wins: generic builtin arms precede typed
plans, StringBox may alias arg0, and the same-module consumer has a different
accepted subset. `NamedAllocationPhysicalAdmissionMissing` remains an open
mapping; copying a Named-spelling classifier into Rust or rejecting all Named
inputs to claim cutover is not an accepted resolution.

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
Next design binds retained conditional outcomes to actual dispatch and required
frame consumption; no new whole-skip census is a prerequisite.

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

Original-lane binding must also change `map_original_demand`'s default
`instruction.used_values()` handling for admitted Named aliases: otherwise a
Map-only Float alias forces a legacy lane through the NewBox argument. Bind the
selected operand from original MIR simultaneously in demand, domain and original
use propagation. These are CutoverBlockerOpen, not optional post-cutover cleanup.

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
selected allocating consumer; AliasOperand(0) follows the original MIR operand,
including exact Float bits. Allocation is currently a demand leaf in
`map_body_index.rs`; alias binding must extend demand, domain and original-lane
propagation together, without manufacturing an old Float lane or copying an
operand graph. A new demanded alias input must not trigger a second role query.

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

### Versioned compiler frame decision

Decision: replace the selected static ingress with one v2 synchronous frame;
retain body JSON as the sole operand/CFG graph. The existing published frame
owner reads finalized MIR before lossy JSON and owns all buffers until C returns.
No second MIR, projection-register namespace, source receipt or independently
issued call graph is added. The planned compile export is
`hako_llvmc_compile_published_static_method_v2(json_path, frame, obj_out, err_out)`.
The frame carries revision/byte size and pointer/count pairs for four tables:

| Table | Payload and identity |
| --- | --- |
| Existing selected calls | Keep current call/Array row layout and meaning; exact selected site and canonical target projection. |
| Map operations | Function + block/instruction site and allocation/write discriminator. Receiver/key/value/dst come from the matched body instruction, not another operand array. |
| Value projection | Function + SSA ValueId, finite action and original-required flag. Admission binds a unique body definition or formal. |
| Expanded functions | Canonical definition's logical physical name and one internal LLVM target. Formal order comes from body params and Formal value rows; no duplicate formal list. |

Value action is a tagged finite payload: ExactBits(tag,u64),
OriginalValue(tag,encoding), Copy, Phi, Select, Formal(source ordinal), or
Operation(selected physical consumer). Operation has one finite wire field;
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

Map AST-free lifecycle is a separate open obligation: body shape currently
retains Map as Other; shadow traversal records value paths but not Array-style
cardinality/key/entry relations, and Script final handoff seals Array only.
Do not borrow Array claim/Home/Fault proof. Absence of this lifecycle issuer alone
also does not prove generic Map artifact execution impossible: the static path
can select mixed modules. The planned dedicated write consumer replaces that gap; Map execution remains
unproved until source/caller switching and artifact acceptance complete.

## Remaining source obligations

The original wider source cutover still requires materialized Script Loop,
implicit completion and unclassified nested/opaque/result/alias families to
reach their own selected source-to-Recipe-to-terminal mappings. Existing Core
identity support and physical witnesses do not close them. Map/Main/provider
and named-new semantics remain separate. Do not reopen the closed admitted
Script series merely to repeat its census, or label these uncompleted families
as completed/invalid because another boundary succeeds.

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
