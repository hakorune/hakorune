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
