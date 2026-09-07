---
Status: Accepted design; staged implementation
Scope: Array literal construction-target preservation; selected LLVM C consumer
---

# Collection literal construction

## Current capsule

- Decision: preserve named versus intrinsic construction in the existing allocation products.
- Implementation: string-only NewBox remains; no intrinsic source cutover yet.
- Next: behavior-preserving Named target conversion, then selected consumer preparation.
- Production stop: source intrinsic emission waits for consumer and preservation acceptance.
- Retirement: Array literal birth callers/effects disappear in the cutover series; Map/Main remain.

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

The published view must select intrinsic allocation itself. Today selection is
based on Call/ArrayElementWrite rows and the C entry requires a nonempty session;
an empty literal cannot rely on an unrelated Print or mutation to become selected.

Extend the existing physical frame with kind 8, IntrinsicArrayNew, using its exact
function/block/instruction coordinate and required destination. Target symbol,
arity and unrelated array-write payload are zero/absent; flags contain only the
existing destination-present bit. This is a physical
projection of the issued target, not a second semantic registry. Keep the existing
row struct layout; Rust/C kind admission changes together and old libraries must
reject an unknown kind rather than interpret it.

The body writer at runner/mir_json_emit/emitters/calls.rs carries the explicit
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

1. Named conversion (BoxShape): replace string target fields and adapt producers,
   structural readers/reissuers and legacy wire handling. Existing source producers
   emit Named only. Establish explicit intrinsic preservation/rejection rules;
   no source acceptance change or birth deletion. Focused remap, old-wire and
   named-construction tests plus the existing canonical corridor guard.
2. Consumer preparation: intrinsic body/frame kind8, view selection, the four C
   consumers and preartifact validation. Synthetic physical tests are boundary
   evidence only. No intrinsic source producer until this step is verified.
3. Array source cutover: raw/typed-local/Core Array arms switch together, remove
   Array literal helper birth caller and Core Array birth effect. Retain Map and
   Main helper callers. Existing Core push lowering remains; no new Method
   interpretation. Verify natural-source empty/populated/nested/typed/Loop arrays,
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

Read-only root/worker audits support this design. Core push uses the existing
receiver-is-array-like/known-write branch, while actual Loop, Core13 and empty
source acceptance remain required implementation evidence. No build or runtime
test was run for design acceptance.
