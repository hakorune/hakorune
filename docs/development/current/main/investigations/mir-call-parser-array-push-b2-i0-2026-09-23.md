---
Status: closeout__source_binding_carrier_projection_core
Task: MIR-CALL-PARSER-ARRAY-PUSH-B2-I0
Parent: mir-call-parser-array-push-route-overlap-d0-2026-09-22
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B2-NEGATIVE-I0
Implementation permission: finish one source-backed GenericLoop physical projection for the existing ArrayPush owner; preserve raw V0/V1 neutrality and the existing runtime/C frame owner
---

# ArrayPush BindingRef carrier projection B2-I0

## Six-line brief

```text
Decision: use the already admitted source relation to project loop-carried
BindingRef values into the existing physical carrier map and retain one exact
source ArrayPush handoff; do not create a second loop or array authority.
Source authority + canonical issuer: CallableGenericLoopSourceRelationViewV1
from the GenericLoop source admission, consumed by the existing source
expression port and CallableGenericLoopV1PhysicalAdapterV1.
Non-authority: variable names, selector spelling, MIR instruction scans, raw
V0/V1 precedence, optimization mode, or a fresh BindingRef->ValueId registry.
Fail-fast boundary: session/site/owner drift, missing or foreign BindingRef,
missing physical carrier, duplicate source item, value-demanded/non-statement
push, or unmapped ArrayPush site must stop before Builder mutation.
Smallest next slice: prove the retained typed carrier reaches one two-push
source fixture, its post-loop read, and the existing C frame handoff, then add
focused negatives for each boundary above.
Non-claims: no production caller switch, no new semantic Verified*/Prepared*
receipt, no runtime Text ownership proof, no VM/AOT parity, no old-edge deletion.
```

## Authority chain and current WIP

```text
source resolver BindingRef + exact source item
  -> CallableGenericLoopSourceRelationViewV1
  -> CallableLoopCarrierRelationV1 (existing source evidence)
  -> CallableLoopSourceExpressionPortWithRelationV1
  -> existing GenericLoop Recipe/physical adapter
  -> retained NamedArray / ArrayElementWrite source handoff
  -> existing C frame owner
```

The relation view and source admission are the only source authorities. The
physical port may ask the relation for the carrier belonging to the resolver
BindingRef, then use the already-issued physical carrier map for that Recipe
step. It must not reconstruct a BindingRef from a label or method name. The
adapter may publish the final induction value through the existing callable
ledger owner after the plan has verified and lowered; it may not publish a new
semantic result.

The source-projection checkpoint lands the following implementation files;
the negative matrix remains a separate bounded card and is not silently
counted as complete:

```text
src/mir/builder/normal_callable_loop_physical_adapter.rs
src/mir/builder/normal_callable_loop_source_port.rs
src/mir/builder/normal_callable_semantic_lowering_state.rs
src/mir/compiler/normal_default_pipeline/published_backend_view.rs
src/mir/normal_callable_semantic_package/issuer.rs
src/mir/builder/control_flow/plan/expression_port.rs
src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs
```

The named `named_array_source_tests` file is part of the checkpoint because
the source fixture and its value-demanded negative are now observed. The
remaining duplicate/missing-row/shape/finish negatives are tracked by
`MIR-CALL-PARSER-ARRAY-PUSH-B2-NEGATIVE-I0`.

## B2 bounded task queue

1. **Inventory and binding map** — enumerate the existing relation carriers,
   source read sites, physical labels, and the single normalizer map that is
   allowed to supply physical `ValueId`s. Record owner, site, and coverage;
   reject a missing map entry before lowering.
2. **Source-port projection** — finish the existing
   `CallableLoopSourceExpressionPortWithRelationV1` path so a source read first
   consumes the resolver BindingRef, then projects through the relation. Keep
   ordinary non-carrier reads on the callable ledger and keep all source sites
   exact.
3. **ArrayPush co-seal** — connect the existing `take_source_array_push` row
   to the exact source statement call in the same port. Require statement
   position, receiver relation, arity, source-item disposition, and one-shot
   consumption before emitting the retained typed write. Do not scan MIR or
   infer an array from a name.
4. **Final induction publication** — retain the existing adapter-side final
   BindingRef publication after `PlanVerifier` and physical lowering. The
   callable ledger must finish with no residual source read/rebind/push rows;
   failure is terminal and leaves Builder state unchanged where the current
   transaction boundary allows it.
5. **Positive evidence** — run the finite source rows: one literal push,
   substring push, two literal pushes, optimize off/on, a loop-carried read,
   and a post-loop read. Confirm retained typed write and existing C frame
   shape. This is source-to-physical evidence, not serializer completion.
6. **Negative evidence** — add named tests for foreign BindingRef/session,
   missing physical carrier label, duplicate source item, value-demanded push,
   missing ArrayPush row, receiver/arity drift, and residual ledger rows.
7. **Reusable guard and closeout** — add one structural guard for the sole
   relation-aware port/adapter edge, no name-based lookup, no second physical
   carrier map, and the 760/800-line boundary. Update the module README,
   this card, and `CURRENT_STATE.toml` only after focused tests and the guard
   pass.

## Explicit exclusions and stop rules

This slice excludes nested loops, If/try/fastmem/task-scope/catch, VM and
compatibility lanes, runtime Text allocation/lifetime, CAPI cleanup, Windows or
macOS capability work, publication caller switching, and old-edge deletion.
A failure outside this finite boundary is not converted into a fallback or a
new receipt; record the exact owner and return to design stop.

Do not accept a route because `BindingRef` labels or method names happen to
match. Do not add a fresh `BindingRef -> ValueId` table, a MIR scan, an
unconditional `Ok(None)`, or a source-to-runtime serializer claim. The finish
line is one source owner, one physical port, one existing ArrayPush handoff,
all named positive/negative evidence, and a reusable guard.

## Source-projection checkpoint

The checkpoint evidence is:

* `named_array_source_reaches_retained_typed_write_and_c_frame` passes literal,
  substring, and two-push source rows with optimization both off and on;
* `named_array_value_demand_rejects_before_published_consumer` passes before
  the published consumer is reached;
* `source_carrier_projection_rejects_missing_physical_label` rejects a missing
  physical carrier; and
* `physical_adapter_rejects_relation_owner_mismatch_before_builder_effect`
  remains green for the foreign-owner boundary.

The new reusable gate
`rust_mirbuilder_generic_loop_source_carrier_projection_b2_guard.sh`, the
existing source-route guard, the pointer guard, and `git diff --check` pass.
The remaining negative inventory is intentionally open in the next card; this
checkpoint makes no production caller-switch or old-edge-retirement claim.
