---
Status: Active contract; retained Script Array execution and bounded retirement verified
Scope: intrinsic literal identity, retained Script numeric Array lifecycle and selected native LLVM C execution.
---

# Collection literal construction

## Current capsule

- **Current decision:** preserve Named versus IntrinsicArray identity, one source lifecycle/Recipe and one runtime storage/contract owner.
- **Current implementation status:** raw/typed-local/Core literals preserve intrinsic identity; selected retained Script Arrays execute through the checked native ABI and bound V4 OBJ/EXE path. Literal birth and selected duplicate/preparation/projection/Stop edges are retired.
- **Next ordered task:** the existing storage wire-tag BoxShape and backend/runtime queue; wider source families remain explicitly open.
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
was already closed and is excluded. Next is existing physical storage wire-tag
BoxShape in the [constructor follow-up queue](constructor-lifecycle-llvm-lowering-ssot.md#feedback-reconciliation-follow-ups-2026-09-08),
then the rolling backend/runtime order. No second implementation task is added.
