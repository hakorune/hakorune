# CUT0-I0 RAW-SOURCE0 LOWER ROOT0 OWNER0-ELIGIBILITY0 consultation

Status: **Design stop — worker audit complete; implementation paused**
Date: 2026-07-23
Scope: decide which source/runtime capabilities are eligible to open a Raw
physical owner after PACKAGE0. No session, shell, collector, ledger, tracker,
slot registry, publication, or production consumer is authorized here.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-owner0-package0-execution-task-2026-07-23.md`
- `src/mir/compiler/raw_root_plan0.rs`
- `src/mir/compiler/raw_root_package.rs`
- `src/mir/builder/raw_source_projection.rs`
- `src/mir/builder/decls.rs`
- `src/mir/slot_registry.rs`

## Evidence from the worker audit

PACKAGE0 is now a real owner handoff, but the current plan still contains
facts that are not safe physical-owner inputs:

```text
runtime inputs       = source_file only; script args/safepoint are not captured
work schedule        = wildcard maps unknown AST nodes to RuntimeStatement
callable catalog     = partial Main/static-child/top-level-function rows
closure access       = unconditional UnsupportedUntilAccess0 marker
static data          = name/type/count only, without authoritative table plan
method slots         = legacy declaration path can mutate process-global registry
```

Opening a physical owner while any of these facts remain implicit would make
the owner reconstruct source authority from ambient state or `current_module`.
That is outside PACKAGE0 and must be decided before PHYSICAL0.

## Decision request for the next design consultation

Please lock Q1--Q5 as one decision, not as five independent implementation
permissions. The question is whether the first physical-eligible Raw slice
will have a complete, deterministic authority boundary. The recommended
candidate is **ELIGIBILITY-prime-r1**:

```text
Q1 = capture explicit runtime inputs once at Raw ingress
Q2 = exhaustive narrow source classifier; no wildcard RuntimeStatement
Q3 = accept only a provably complete narrow catalog; reject the rest
Q4 = reject closures and static data until exact source plans exist
Q5 = reject every process-global slot shape until SLOT0
```

The smaller alternative is **ELIGIBILITY-minimal-r1**:

```text
Q1 = reject ambient-dependent sources until RUNTIME0
Q2 = seal locator coverage and keep RuntimeStatement as an explicit deferred lane
Q3--Q5 = the same typed rejections
```

The consultant must choose one candidate, or state a precise hybrid. A hybrid
must still name the single accepted source grammar and the exact rejection
authority; “support it later” is not a physical-owner policy.

### Q1 -- runtime-input authority

Current legacy lowering reads `NYASH_SCRIPT_ARGS_JSON`/
`HAKO_SCRIPT_ARGS_JSON` in `src/mir/builder/decls.rs` and reads the entry
safepoint flag in `src/mir/builder/module_lifecycle.rs`. Neither value is in
the Raw package. Choose one:

```text
A. CaptureOnce:
   parse and validate script args and entry-safepoint at compiler Raw ingress,
   retain the immutable value in the continuation/package, and forbid all
   lowerer/Builder environment reads. Malformed input is a typed ingress
   rejection; absent input is an explicit None/false value.

B. RejectAmbient:
   seal only sources that do not require either input and return
   UnsupportedRuntimeInputs for the rest. RUNTIME0 later adds capture.
```

Evidence required either way: ambient lowerer reads = 0, one snapshot or one
typed rejection disposition, and physical effects = 0 in this row.

### Q2 -- source-work authority

`raw_root_plan0.rs` currently maps wildcard AST shapes to `RuntimeStatement`.
Choose one:

```text
A. NarrowExhaustive:
   explicitly classify every top-level shape into an accepted runtime/root,
   declaration, callable, access, or typed-unsupported disposition. Loop/If/
   Print/Assignment are not blanket-rejected when their source work is known;
   Using/Import/Lambda/unknown shapes never enter RuntimeStatement.

B. CoverageDeferred:
   seal locator cardinality/order and make RuntimeStatement an explicit
   deferred-body capability. Physical eligibility admits only the no-deferred
   subset; no wildcard may silently claim support.
```

Either choice must remove silent wildcard authority and forbid AST re-resolution
after the eligibility package is sealed.

### Q3 -- callable/declaration coverage

Existing catalog/projection does not cover constructors, top-level functions,
instance-box methods, sync/record boxes, or all static forms. Choose one:

```text
A. CompletePlanNow:
   add one owner-keyed RawCompleteCallablePlan with stable locator, semantic
   identity, physical symbol, arity, and source role for every declaration.

B. NarrowReject:
   admit only Script and a plain static Main App shape whose complete rows are
   already provable; reject every partial-catalog shape with a typed error.
```

Partial catalog must never be treated as complete by cardinality or symbol
matching alone.

### Q4 -- closure and static-data authority

Closure lowering currently computes captures from live function state and
interns bodies during traversal. Static-data planning currently drops values,
qualified identity, and duplicate proofs. Choose one policy for each:

```text
closure:
  reject until a source-site plan exists, or define a complete site/consume
  plan now; ad-hoc Lambda scanning is not acceptable.

static data:
  reject until STATICDATA0, or promote the existing pure spec/plan conversion
  into an exact pre-physical authority including values, identity, ranges, and
  duplicate checks. Shell publication remains later either way.
```

### Q5 -- process-global slots

`get_or_assign_type_id`, `reserve_method_slot`, and
`resolve_slot_by_type_name` mutate process-global state with no rollback. The
recommended decision is typed rejection (`UnsupportedProcessGlobalSlot`) for
instance methods, constructors, and allocating birth-slot shapes until SLOT0;
silent use or rollback-by-convention is forbidden.

## Required answer format

```text
Decision: ELIGIBILITY-<candidate>-r1
Q1: A or B, with malformed/absent input law
Q2: A or B, with the exact accepted grammar
Q3: A or B, with the first eligible catalog subset
Q4: closure policy + static-data policy
Q5: typed rejection or SLOT0, with global-state law
Next row: one OWNER0-ELIGIBILITY0-S0 (or a justified split)
Non-claims: physical open, child/root lowering, declaration install, slots,
  closure/static publication, batch/drain/finalize/postprocess/commit/ingress/JSON/CUT0
```

The consultation is successful only when the accepted grammar, rejection
stages, and one next implementation owner are explicit. Until then, package,
session, shell, collector, ledger, tracker, and production consumer counts
remain zero.

## Questions to lock

### Q1 — runtime inputs

Should the first eligible Raw shape seal an immutable runtime-input snapshot,
or should every source requiring script args/entry safepoint be rejected with
`UnsupportedRuntimeInputs` until a dedicated RUNTIME0 producer exists?

### Q2 — source work schedule

Should every top-level AST shape be classified exactly once, with an explicit
unsupported disposition for Lambda/closure/unknown or otherwise unowned
shapes, instead of the current wildcard `RuntimeStatement`? Script runtime
forms such as Loop/If/Print/Assignment remain eligible only when their source
work meaning is explicitly classified; Using/Import must be preprocessed or
rejected, never silently treated as runtime work.

### Q3 — declaration/callable coverage

Should eligibility require a complete owner-keyed declaration/callable catalog
including non-Main boxes, instance methods, constructors, static init, fields,
delegates, and top-level functions, or reject incomplete source shapes before
physical effects?

### Q4 — closure/static-data access

Should closure-bearing sources and static tables use dedicated source plans,
or receive typed unsupported errors until ACCESS0/static-table authority is
sealed? A boolean `static_data` or unconditional closure marker is not enough.

### Q5 — process-global slots

Should instance-method/constructor shapes requiring `get_or_assign_type_id`,
`reserve_method_slot`, or allocating `resolve_slot_by_type_name` be rejected
with a typed capability error until SLOT0 supplies invocation-local authority?

## Candidate ELIGIBILITY-prime

The recommended boundary is a narrow fail-fast eligibility seal:

```text
SourceBoundRawRootPackageV1
-> borrow/validate complete source facts
-> RawRootEligibilityV1
   success: explicit supported dispositions only
   failure: RejectedRawRootEligibilityV1 retains the whole package
-> later OWNER0-PHYSICAL0
```

The first eligible source family should be intentionally small:

```text
no ambient runtime-input requirement
no closure or unsupported top-level shape
complete declaration/work classification
no instance/constructor global-slot requirement
static data only when an exact authoritative table plan is present
```

Every other shape is a typed rejection before physical effects. This is not a
fallback to `RuntimeStatement`, a silent omission, or a retry route.

```text
physical effects = 0
production consumers = 0
No child traversal = 0
```

## Proposed products

```rust
struct RawRootEligibilityV1 {
    runtime_inputs: RawRuntimeInputEligibilityV1,
    work_schedule: RawRootWorkEligibilityV1,
    declarations: RawDeclarationEligibilityV1,
    callable_catalog: RawCallableEligibilityV1,
    access: RawRootAccessEligibilityV1,
    slot_policy: RawSlotEligibilityV1,
}

struct RejectedRawRootEligibilityV1 {
    owner: SourceBoundRawRootPackageV1,
    stage: RawRootEligibilityStageV1,
    error: RawRootEligibilityErrorV1,
}
```

The exact names are provisional until Q1–Q5 are locked. The product must be
non-Clone, and failure exposes inspection plus discard only. It must not open
or mutate Builder/session/shell/collector/ledger/tracker state.

## Required fixture matrix after decision lock

```text
narrow Script/App with no runtime requirement -> eligible
unknown top-level AST shape                  -> typed rejection
Main requiring ambient args                  -> typed rejection
Lambda/NewClosure                             -> typed rejection
instance method/constructor                  -> typed slot rejection
incomplete/invalid static table               -> typed access rejection
duplicate/incomplete callable declaration     -> typed catalog rejection
every rejection retains source package        -> owner snapshot unchanged
```

## Explicit non-claims

This consultation does not authorize child traversal, callable-Main lowering,
root-body lowering, declaration/access installation, method-slot publication,
closure interning, static-table publication, Main/condition batch, drain,
finalization, postprocess, external commit, public ingress, JSON behavior, or
CUT0 activation.

Existing declaration indexer/callable catalog/static-table helpers are evidence
sources only. Wiring them into the Raw physical owner is a later implementation
row after this policy decision.

## Next decision

Lock Q1–Q5 as `ELIGIBILITY-prime-r1`, then create one bounded implementation
row. Keep `OWNER0-PACKAGE0` closed and all physical/production consumers at
zero until the eligibility product and its guard are specified.
