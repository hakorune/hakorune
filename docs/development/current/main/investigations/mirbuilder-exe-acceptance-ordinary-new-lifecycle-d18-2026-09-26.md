# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-LIFECYCLE-D18
## Census: Artifact-Unowned Lifecycle Site — EXE Lane, JSON Aggregator Boundary (D18)

Date: 2026-09-26
Status: decided — bounded next slice accepted
Family: callable / gate1 / Gate 1 unified lane / ordinary-new lifecycle / EXE acceptance
Row reference: workstream row H (Gate 1 unified selfhost lane)
Blocking observation: after S5 routed source-backed `--emit-mir-json`
through `into_artifact_parts`, the production source
`apps/json-stream-aggregator/main.hako` terminates at
`[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`
(`ordinary_new_local_commit/root_validation.rs:250`).
`JsonStreamAggregator.ingest/1` may later still be declined by the
unchanged ledger-less VM spine (Recorded Non-Claim — Gates 2–4)

## Exact residual site (measured)

Instrumented freeze evidence (temporary diagnostic, reverted):

```text
owner=FunctionOwnerIdV1 { compilation: 1, slot: 25 }   // statsFor
block=BasicBlockId(31)
insn=Call(MirCall { dst: None,
  callee: BirthConstructor { key: UserStats.birth/1, receiver: ValueId(18) },
  args: [ValueId(1)], ... })
```

The instruction is a **bare `Call{BirthConstructor}`** — no `Invoke`
operation, no fault frame, no `InvokeNormalResult` projection.

## Issuer / owner / consumer map

- Emission site: `lower_ordinary_raw_new_with_port_v1`
  (`ordinary_new_admission.rs:62-74`) — the **raw ordinary-`new` lane**:
  `MirInstruction::NewBox` + `MirInstruction::call(None,
  Callee::BirthConstructor { key: recipe.target(), receiver: dst })`
  when a `Birth` recipe exists.
- The canonical claim-lane emitter `ordinary_new_admission::selected::emit`
  produces a different physical shape: `Invoke{NewBox}` +
  `InvokeNormalResult` + `Invoke{Call{BirthConstructor}, fault_frame}` +
  `ReclaimUnpublished` cleanup — all recorded via
  `ledger.record_new_emission` bindings.
- Route choice: `new_expression.rs:192-213` — `Ordinary` route +
  `selected_ordinary_claim` → claim emission; otherwise raw lane.
- Selection: `prepare_ordinary_claim_v1` → `try_take_ordinary_new_claim`
  → `prepare_ordinary_new_emission` →
  `OrdinaryNewClaimLedgerV1::prepare_new_emission`
  (`ordinary_new_local_commit.rs:384-459`). `available =
  claim.construction().is_ok() && home_prefix.is_ok() && all
  prior.end_available()`; `!available` → row becomes
  `NewEmissionProgress::RetainedUnavailable` and `prepare` returns
  `false` → raw lane with the claim's `Birth` recipe.

## Measured classification (production app, temporary diagnostic)

```text
raw-new class=MapBox               claim=false birth_recipe=false
raw-new class=ArrayBox             claim=false birth_recipe=false
raw-new class=UserStats            claim=true  birth_recipe=false
                                   construction=Err home_prefix=Err
raw-new class=ArrayBox             claim=false birth_recipe=false
raw-new class=JsonStreamAggregator claim=true  birth_recipe=false
                                   construction=Err home_prefix=Ok
```

Both `new UserStats(user)` (`statsFor` body initializer) and
`new JsonStreamAggregator()` (`main` root initializer) are **claim-issued
but construction-ineligible** → `RetainedUnavailable` → raw-lane
emission. `Builtin MapBox`/`ArrayBox` `new`s are claim-free
compatibility sites (by design — `ordinary_new_candidate.rs:38-44`).

## Why `construction()` is `Err` — eligibility scope

`issue_construction_plan` (`instance_construction.rs`) deliberately
admits only:

- all stored fields `i64`-declared, no weak/delegate/extends/invariant/
  transition/type-parameter/sync/static/attr/initializer features, and
- a `birth` body of plain `me.field = <i64 literal | parameter>` stores
  covering every field, plus optional trailing unit `return`.

`UserStats.birth(name)` stores `me.name = name` — a non-`i64` field
contract — and `JsonStreamAggregator.birth()` contains `me.stats = new
MapBox()` — a nested `new` RHS — so both are
`FieldContractUnsupported`/`BodyCoverageUnsupported`. Widening that
eligibility is its own family (box-typed fields, nested construction
reclamation), not this slice.

## Is `RetainedUnavailable` a designed state?

Yes. `prepare_new_emission` marks the row
`RetainedUnavailable{PendingExpression}` and `prepare` returns `false`
**by design**; `prepare_ordinary_claim_v1` then keeps `ordinary_claim`
(so the raw lane reuses the claim's verified `Birth` recipe — the only
remaining birth carrier after `birth-global-legacy-stopped`); the raw
executor lowers physically while the row advances
`PendingExpression → ExpressionCompleted → Installed` through
`complete_new_expression`. `RetainedUnavailable{Installed}` counts as
`is_complete()` and the seal explicitly documents it: *"RetainedUnavailable
remains unavailable: never invent an empty actual"*
(`finalized_root_handoff.rs:165`).

What is missing is only that the raw executor's emitted instructions are
not attributed to the owning row — `lifecycle_bindings` skips
`RetainedUnavailable` rows entirely (`root_validation.rs:313`).

## What `into_artifact_parts` actually certifies

`into_artifact_parts`
(`normal_default_root_final_validation.rs:92-216`) is **object-
compilation admission**, not document validation:

- `root_validation.validate(module, artifact=true)` — adds
  `artifact-source-unavailable` (root observation must be
  `SourceCompleteAtFinalization`/`NoSelectedLocalNew`) and
  `validate_artifact_lifecycle_coverage` (every
  `requires_lifecycle_validation` instruction must appear in the
  boundary-projected cleanup bindings),
- module-wide `has_lifecycle`/`has_exact_field_read` coverage scans
  (`uncovered-lifecycle-function`, `unowned-exact-field-read`,
  `uncovered-birth-definition`).

These certify the module is compilable through the canonical lifecycle
artifact path. `RetainedUnavailable` claim sites **can never pass it**
until construction eligibility covers them — the freeze is the gate
working correctly, applied by the wrong consumer.

## Contract boundary (census conclusion)

`--emit-mir-json` is a **document publication**: it serializes the
built module plus its retained source handoffs. It does not compile an
object, so it must not demand object-cohort lifecycle ownership — the
same reason D17 kept it out of `admit_lifecycle`/backend admission.
The sole finalized-handoff authority
(`seal_finalized_root_birth_handoff`) requires only
`RootNewValidation::FinishingChecked`, which the **non-artifact**
finishing validation (`validate(module, false)` →
`validate_after_compiler_finishing` /
`validate_finalized_child_functions(artifact=false)`) already reaches:
all root-body, cleanup-boundary, field-read, terminal, emission-
projection and observation-drift checks still run; only the
object-admission extras are skipped.

## Decision

Decision: emit-mir-json document publication consumes a **document
completion** contract — full non-artifact finishing validation plus the
sole finalized-root-handoff seal plus named-array discharge — not the
artifact-admission cohort. `into_artifact_parts` stays the exclusive
gate of `compile_normal_with_published` (object/backend admission).

- Source authority + canonical issuer:
  `CompletedNormalDefaultRootCatalogLifecycleV1` document completion —
  `RootValidation::validate(module, false)` (children
  `validate_finalized_child_functions(artifact=false)` + root
  `validate_finished_root(false)` → `FinishingChecked`) →
  `seal_finalized_root_birth_handoff` → `with_named_arrays` +
  `validate_named_arrays` → `bind_finalized_root_handoff` →
  `build_published_body_root`.
- Non-authority: `RetainedUnavailable` rows are not promoted,
  reclassified, or given synthetic lifecycle bindings;
  `validate_artifact_lifecycle_coverage`,
  `artifact-source-unavailable`, `uncovered-lifecycle-function`,
  `unowned-exact-field-read` remain object-admission checks only;
  `reject_unretained_module` stays the raw/diagnostic guard;
  `emit_mir_json_for_harness` stays the compatibility writer;
  no `.hako` workaround, no silent fallback for retained modules.
- Fail-fast boundary: non-artifact finishing validation (root body,
  cleanup boundary, terminals, field reads, emission projections,
  observation drift), seal (`artifact-root-not-finished`,
  `artifact-root-completion-unavailable`, `artifact-local-commit-
  incomplete`, `artifact-birth-*` integrity checks), named-array
  coverage — any residual freezes before emit.
- Smallest next slice (S6): a document-completion finalization —
  `into_document_parts`-style variant of
  `CompletedNormalDefaultRootCatalogLifecycleV1` that runs the
  non-artifact `validate(module, false)` + construction
  `validate_after_compiler_finishing`, then takes named-array
  emissions, seals the finalized root handoff, attaches and validates
  named arrays — wired into `compile_normal_for_mir_json` in place of
  `into_artifact_parts`. Focused pins: document completion admits a
  `RetainedUnavailable` claim module while artifact admission still
  refuses it; handoff seal still discharges named-array requirements;
  raw/harness path untouched.
- Non-claims: does not extend construction eligibility (non-i64 fields,
  nested `new` birth bodies stay unavailable and claim-owned); does not
  bind raw-lane instructions into canonical lifecycle bindings (that
  belongs to object admission, a later slice); does not claim the
  external ny-llvmc consumer accepts `Call{BirthConstructor}`/
  `SameModuleInstance` JSON — the bare birth call will surface as the
  next backend-side residual if emit succeeds; does not touch VM
  `ingest/1` or Gates 2–4.

## Expected next residual (if emit advances)

1. `artifact-root-completion-unavailable` — if `main`'s retained
   `root_completion` is `Err` (issued under the app-main fallback at
   `ordinary_new_coseal_issue.rs:290-304`), the seal refuses; that is
   its own census.
2. A produced `mir.json` → the runtime/EXE boundary consumer's own
   admission of bare `Call{BirthConstructor}` + `NewBox` + bare
   `Callee::SameModuleInstance` instructions.

## Production evidence

`NYASH_BIN=target/debug/hakorune --backend mir --emit-mir-json
/tmp/agg_mir.json apps/json-stream-aggregator/main.hako`:

```text
❌ MIR compilation error:
[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site
owner=FunctionOwnerIdV1 { compilation: 1, slot: 25 }
block=BasicBlockId(31)
insn=Call(... callee: BirthConstructor { key: UserStats.birth/1,
receiver: ValueId(18) }, args: [ValueId(1)] ...)]
```

Temporary diagnostics (reverted after measurement): freeze-site
owner/block/instruction dump in `root_validation.rs`; raw-new lane
claim/construction/home_prefix flags in `new_expression.rs`.
