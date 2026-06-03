---
Status: Active
Date: 2026-05-27
Scope: boundary between hako_check source perf-surface and MIR-level method shape observation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH.md
  - docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
  - tools/hako_check/README.md
---

# hako_check / MIR Observation Boundary

## Decision

`hako_check perf-surface` remains a source-level observation tool. MIR-level
shape observation is normally a separate adapter/app surface.

Exception: `hako_check fastpath-explain` may consume an already-emitted MIR JSON
artifact to print direct-memory diagnostic coverage. This exception is read-only
and exists so future `direct {}` / FastPath obligation failures have one
developer-facing explanation surface. It must not emit MIR, rewrite source,
select keepers, or own lowering policy.

The same read-only exception also covers compiler-emitted optimization outcome
metadata for direct-exact hot-core calls. The compiler/MIR metadata remains the
truth; `hako_check fastpath-explain` only renders that truth for users. It must
not infer that a call is direct-exact, decide that a method is a hot core, or
reimplement the optimizer's eligibility rules.

Exception: `hako_check state-explain` may consume an already-emitted MIR JSON
artifact to print state bucket and residence metadata coverage. This exception
is read-only and exists so future `RecordStateResidencePlanV0` work has one
developer-facing explanation surface. It must not emit MIR, rewrite source,
select keepers, migrate `PageState`, infer public semantics, or own record-state
lowering policy.

Exception: `hako_check fastpath-explain` may also render compiler-emitted
`RouteDecisionV0` rows. This exception is read-only. `hako_check` must not
choose the preferred route, compute proof facts, infer miss reasons, or decide
fallback policy. The compiler/MIR metadata remains the only truth.

Developer convenience exception: `tools/hako_check/fastpath_explain.sh --app`
may emit a temporary MIR JSON artifact before invoking the stable
`fastpath_explain.py` adapter. This wrapper is allowed only as a tool entrypoint:
it does not build the compiler, persist MIR by default, run benchmarks, select
keepers, or change the read-only Python contract.

The same wrapper exception applies to `tools/hako_check/state_explain.sh --app`.
It may emit temporary MIR JSON before invoking `state_explain.py`, but it does
not become a MIR producer or state-residence planner.

```text
hako_check perf-surface:
  source-level risk and keeper suggestion

MIR method shape adapter:
  actual lowered MIR shape for selected methods

hako_check fastpath-explain:
  MIR JSON diagnostic adapter for DirectArray / Span / RequiredFastPath metadata
  direct-exact HotCore call-plan metadata, and RouteDecisionV0 outcomes

hako_check state-explain:
  MIR JSON diagnostic adapter for user-box field buckets, DirectState metadata,
  record layout facts, and future RecordStateResidencePlanV0 metadata

keeper diff adapter:
  before/after source report + MIR report + measurement evidence
```

## Rationale

`hako_check` should not become an optimizer or a MIR analyzer. Its job in the
mimalloc parity lane is to identify suspicious `.hako` source surfaces and
suggest one next keeper candidate.

MIR observation answers a different question: whether a selected source risk
actually lowers into costly calls, field operations, copies, PHIs, branches, or
runtime checks.

Keeping these contracts separate prevents hako_check from accumulating backend
responsibility and keeps each row narrow.

The fastpath-explain exception is intentionally narrower than the general MIR
method-shape adapter: it reads named metadata fields that are already part of
the direct-memory diagnostic contract and reports missing/passing obligations.
It does not infer new method shape, route ownership, or performance keepers.

For HotCore/direct-exact call planning, the same rule applies. The adapter may
display `HotCoreMethodSummaryV0`, `DirectExactHotCoreCallPlanV0`, and lowering
result fields when the compiler emits them, but it cannot synthesize those
plans from method names, source text, or MIR instruction patterns.

For state/record residence planning, `state-explain` may display field buckets,
`DirectStatePlan`, record layout rows, and `RecordStateResidencePlanV0` rows
when the compiler emits them. Bucket labels are explanatory only and must not
be used as proof that a source migration or backend lowering is legal.

## Compiler Responsibility Boundary

The current language surface is intentionally small. The corresponding compiler
boundary must keep complexity out of the parser and MIRBuilder.

```text
MIRBuilder:
  preserve source shape, receiver origin, field origin, callsite identity,
  declared types, storage hints, and source spans
  do not own proofs, optimization decisions, or backend routes

Analyzer:
  produce origin/proof facts such as RangeIndexFact, ExtentFact,
  RegionStabilityFact, SpanBorrowFact, and EffectSummary

Planner:
  try fast routes first and produce plans / outcome rows such as
  DirectArrayAccessPlan, RecordStateResidencePlan, DirectExactHotCoreCallPlan,
  RequiredFastPathRegion, FastPathPlan, and RouteDecisionV0

Verifier:
  decide whether a plan is legal and fail fast when required facts are missing

Lowering:
  consume accepted plans / selected RouteDecision rows only; do not re-infer
  source policy, helper-name policy, or method-name special cases

hako_check:
  render emitted metadata and diagnostics; never synthesize optimization truth
```

One-line rule:

```text
MIRBuilder must not decide meaning. It must preserve enough information for
later facts/plans/verifiers to decide meaning.
```

## Seven-Box Classification

Before adding a feature, diagnostic, or optimization path, classify it into one
of these boxes.

| Box | Purpose | Examples |
| --- | --- | --- |
| Source Surface | User-visible syntax and types | `box`, `record`, `DirectArrayI64`, `gate`, `@rune` |
| Declaration Metadata | Source-attached facts, not behavior | `Inline(required)`, `Contract(...)` |
| Origin Facts | Where values/fields/calls came from | receiver origin, field origin, callsite span |
| Proof Facts | Why an access/effect is safe | RangeIndexFact, ExtentFact, StabilityFact, EffectSummary |
| Plans | What optimized route is requested | DirectArrayAccessPlan, RecordStateResidencePlan, DirectExactHotCoreCallPlan |
| Route Outcomes | Which route was selected and why | RouteDecisionV0 selected_route, fallback_policy, miss_reason |
| Diagnostics / Explain | User-facing visibility | `hako_check fastpath-explain`, `state-explain`, report fields |
| Lowering Consumers | Backend implementation of accepted plans | static exact call, direct array load/store, checked/proved-unchecked route |

If a proposal cannot name its box, stop and split it. In particular:

```text
Inline(required):
  small leaf call elimination only

multi-block hot core:
  DirectExactHotCoreCallPlan, not Inline(required)

record PageState:
  Source Surface uses existing record, but runtime residence requires
  RecordStateResidencePlanV0 before source migration

copy cleanup:
  RouteAwareMaterializationPlan with route preservation proof, not generic
  Copy deletion

fastpath-first:
  Planner emits RouteDecisionV0; MIRBuilder does not choose fast vs slow
```

## Planned Surfaces

### Source Perf-Surface v1

Owner: `tools/hako_check`.

```text
output_contract=hako-check-perf-surface-v1
loop_field_get_count
loop_field_set_count
loop_array_get_count
loop_array_length_count
allocation_like_in_loop_count
suggested_next_kind=box_count|box_shape|mir_diagnostic|none
confidence=low|medium|high
summary=ok
```

### MIR Method Shape v0

Owner: `tools/mir_check` initially.

```text
output_contract=hako-mir-method-shape-v0
input_kind=mir_json
selected_method
mir_instruction_count
call_count
field_get_count
field_set_count
array_get_call_count
array_length_call_count
phi_count
copy_count
branch_count
return_count
summary=ok
```

The first implementation should be Python. `.hako` migration comes only after
the contract and fixture expectations stabilize.

### Keeper Diff v0

Owner: adapter surface, not hako_check core.

```text
output_contract=hako-mimalloc-keeper-before-after-diff-v0
keeper_id
before_source_surface
after_source_surface
before_mir_shape
after_mir_shape
measurement_before
measurement_after
keeper_effect=accepted|no_effect|regressed|inconclusive
summary=ok
```

### FastPath Explain v0

Owner: `tools/hako_check`, read-only MIR JSON adapter.

```text
output_contract=hako-check-fastpath-explain-v0
input_kind=mir_json
tool_surface=hako_check_fastpath_explain
observation_only=1
rewrite_executed=0
direct_array_access_plan_count
direct_array_checked_plan_count
direct_array_proved_unchecked_plan_count
span_access_plan_count
required_fastpath_region_count
fastpath_obligation_count
fastpath_obligation_failed_count
missing_fastpath_plan_count
clean=0|1
summary=ok|failed
```

HotCore/direct-exact extension fields may be added to the same report once the
compiler emits the corresponding metadata:

```text
hotcore_method_summary_count
direct_exact_hotcore_call_plan_count
direct_exact_static_call_lowered_count
direct_exact_plan_lowered_to_fallback_count
generic_method_dispatch_count
dynamic_route_count
boxed_fallback_count
```

Intended user-facing output:

```text
Function: Main.runOne/2

FastPath summary:
  direct_exact_hotcore_call_plan: 3
  static_exact_call_lowered: 3
  generic_method_dispatch: 0
  dynamic_route: 0

Call edges:
  Main.runOne/2
    -> HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1
       route: static_exact_call
       summary: HotCoreMethodSummaryV0 ok
```

Failure output must explain the compiler-owned reason instead of hiding the
route:

```text
Missing direct-exact call plan:
  site: Main.runOne/2 -> objectLifecycleSmallAlloc/1
  expected: DirectExactHotCoreCallPlanV0
  actual: generic_method_dispatch
  reason: callee has public observer update
```

### State Explain v0

Owner: `tools/hako_check`, read-only MIR JSON adapter.

```text
output_contract=hako-check-state-explain-v0
input_kind=mir_json
tool_surface=hako_check_state_explain
observation_only=1
rewrite_executed=0
keeper_selection=0
target_box
user_box_decl_count
selected_field_count
record_decl_count
record_layout_plan_count
direct_state_plan_count
direct_state_positive_candidate_count
direct_state_mixed_candidate_count
selected_direct_state_plan_count
selected_direct_state_positive_candidate_count
selected_direct_state_mixed_candidate_count
record_state_residence_plan_count
record_state_residence_candidate_field_count
record_state_handle_reject_field_count
bucket_primitive_hot_state_field_count
bucket_public_semantics_field_count
bucket_public_semantics_proof_evidence_field_count
bucket_proof_evidence_field_count
bucket_diagnostic_only_field_count
bucket_observer_boundary_field_count
bucket_handle_cache_field_count
bucket_result_capsule_field_count
bucket_direct_array_owner_field_count
bucket_escape_unknown_field_count
record_state_source_migration_selected=0
whole_record_abi_enabled=0
public_materialization_enabled=0
ordinary_box_auto_recordification=0
record_to_box_conversion=0
clean=0|1
summary=ok
```

## Stop Line

- `hako_check` does not rewrite source.
- `hako_check fastpath-explain` does not emit MIR; it consumes a caller-provided
  MIR JSON file only.
- `tools/hako_check/fastpath_explain.sh --app` is a developer wrapper that emits
  temporary MIR JSON before calling the read-only adapter; it is not a MIR
  analysis owner.
- `hako_check state-explain` follows the same boundary: it renders metadata and
  explanatory buckets only, and does not select or implement
  `RecordStateResidencePlanV0`.
- MIR method shape does not select keepers by itself.
- `hako_check fastpath-explain` does not infer HotCore summaries or direct-exact
  call plans; it renders compiler-emitted metadata only.
- Diff adapter does not implement keepers.
- Provider activation, process allocator replacement, hooks, globals, and
  winner claims remain closed unless a separate decision row opens them.
