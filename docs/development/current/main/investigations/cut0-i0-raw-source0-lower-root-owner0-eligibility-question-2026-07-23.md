# RAW-SOURCE0 LOWER ROOT0 OWNER0 — ELIGIBILITY0 question

Status: **Closed — Decision ELIGIBILITY-prime-r1; S0 execution task is next**
Date: 2026-07-23

## Context

`PACKAGE0` is closed. `SourceBoundRawRootPackageV1` now co-seals the token,
owned source, typed continuation, Builder configuration, module name, and
tokenless root plan. No Raw physical owner is open yet.

The next boundary must decide which source shapes are eligible to open a
physical owner. The current code still has five authority gaps:

```text
runtime args / entry safepoint are read from ambient Builder state
unknown AST shapes fall through to RuntimeStatement
callable/declaration catalog coverage is partial
closure and static-data plans are not exact source authority
instance/constructor lowering mutates process-global method-slot state
```

Please choose one coherent policy for Q1–Q5. Do not answer with “support it
later” without naming the typed rejection and the future widening row.

## Recommended candidate: ELIGIBILITY-prime-r1

```text
Q1  Capture explicit runtime inputs once at Raw ingress
Q2  Use a narrow exhaustive source classifier; no wildcard RuntimeStatement
Q3  Admit only a provably complete narrow catalog; reject partial coverage
Q4  Reject closures and static data until exact source plans exist
Q5  Reject every process-global slot shape until SLOT0
```

The smaller alternative is `ELIGIBILITY-minimal-r1`: reject all ambient
runtime-dependent shapes until `RUNTIME0`, seal only locator coverage with an
explicit deferred-body disposition, and keep Q3–Q5 as typed rejections.

## Questions

### Q1 — runtime-input authority

Legacy lowering reads `NYASH_SCRIPT_ARGS_JSON`/`HAKO_SCRIPT_ARGS_JSON` from
`src/mir/builder/decls.rs` and the entry safepoint flag from
`src/mir/builder/module_lifecycle.rs`. These values are not in the Raw
package.

Choose:

```text
A. CaptureOnce
   Parse and validate both values at compiler Raw ingress. Retain the
   immutable snapshot by value in the continuation/package. Lowerers perform
   no environment reads. Malformed input is a typed ingress error; absent
   input is an explicit None/false disposition.

B. RejectAmbient
   Admit only sources that require neither value. Return
   UnsupportedRuntimeInputs for the rest and defer capture to RUNTIME0.
```

Acceptance: one snapshot or one typed rejection disposition; lowerer ambient
reads = 0; physical effects = 0.

### Q2 — source-work authority

`raw_root_plan0.rs` currently maps a wildcard AST branch to `RuntimeStatement`.
This can silently claim support for future or unowned shapes.

Choose:

```text
A. NarrowExhaustive
   Classify every top-level shape exactly once as accepted runtime/root,
   declaration, callable, access, or typed unsupported. Known Loop/If/
   Print/Assignment forms may remain eligible; Using/Import/Lambda/unknown
   forms never enter RuntimeStatement.

B. CoverageDeferred
   Seal locator cardinality and order, and make RuntimeStatement an explicit
   deferred-body capability. Physical eligibility admits only the
   no-deferred subset; no wildcard may silently claim support.
```

Acceptance: deterministic order, exact coverage, unknown rejection or explicit
deferred disposition, and no AST re-resolution after package sealing.

### Q3 — callable/declaration coverage

The existing catalog/projection does not cover all constructors, top-level
functions, instance-box methods, sync/record boxes, or every static form.

Choose:

```text
A. CompletePlanNow
   Add one owner-keyed RawCompleteCallablePlan with stable source locator,
   semantic identity, physical symbol, arity, and source role for every row.

B. NarrowReject
   Admit only Script and a plain static-Main App shape whose rows are already
   provable. Reject top-level functions, non-Main boxes, instance boxes,
   constructors, sync/record boxes, and incomplete catalog shapes.
```

Partial catalog cardinality or symbol matching is not a completeness proof.

### Q4 — closure and static-data authority

Closure lowering currently computes captures during body traversal. The current
static-data projection loses values, qualified identity, and duplicate proofs.

Choose one policy for each:

```text
closure:
  reject with UnsupportedClosureAccess until a source-site plan exists, or
  define a complete site/consume plan now; ad-hoc Lambda scanning is forbidden.

static data:
  reject with UnsupportedStaticDataAuthority until STATICDATA0, or promote
  the existing pure spec/plan conversion into an exact pre-physical authority
  with values, identity, ranges, and duplicate checks.
```

Shell metadata publication remains a later `DECLACCESS0` concern either way.

### Q5 — process-global slots

`get_or_assign_type_id`, `reserve_method_slot`, and
`resolve_slot_by_type_name` mutate process-global state without rollback.

Recommended decision:

```text
instance methods, constructors, and allocating birth-slot shapes
  -> UnsupportedProcessGlobalSlot before physical-owner open
  -> SLOT0 is a future capability-widening row
```

Silent use, rollback-by-convention, and fallback are not accepted policies.

## Required answer format

```text
Decision: ELIGIBILITY-<candidate>-r1
Q1: A or B; malformed and absent input law
Q2: A or B; exact accepted grammar
Q3: A or B; first eligible catalog subset
Q4: closure policy + static-data policy
Q5: typed rejection or SLOT0; global-state law
Next row: RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0
Non-claims: child/root lowering, declaration install, slot publication,
  closure/static publication, Main/condition batch, drain, finalization,
  postprocess, external commit, public ingress, JSON, and CUT0 activation
```

## Hard stop

Until this answer is locked, the following remain zero:

```text
Builder/session/shell/collector/ledger/tracker construction
process-global slot mutation
child traversal and root lowering
production consumers and physical effects
```

The rejection product must retain the whole `SourceBoundRawRootPackageV1` and
expose inspection plus discard only. No retry, re-pairing, fallback, or
source re-resolution is permitted.

Cleanup census (`CLEAN0-*`) is intentionally a separate lane and is not part
of this design decision.

## Decision closeout

`ELIGIBILITY-prime-r1` is selected.

```text
Q1 = CaptureOnce
  Raw script args and entry safepoint are parsed/validated once before token
  issuance, retained in RawSourceContinuationV1, and never re-read by a
  lowerer. Malformed input is a typed ingress rejection; absent input is
  Absent/Disabled.

Q2 = NarrowExhaustive
  ScalarControl0 is the first eligible grammar. The classifier is recursive
  and wildcard-free; RuntimeStatement is never a silent catch-all.

Q3 = NarrowReject
  Only Empty Script and one plain static-Main App with an exact Main-local
  catalog are eligible. Partial or foreign declaration coverage rejects.

Q4 = reject
  Closure and static-data shapes reject until CLOSURE0/STATICDATA0 provide
  exact source authority.

Q5 = reject
  Process-global slot shapes reject with UnsupportedProcessGlobalSlot until
  SLOT0 supplies invocation-local authority.
```

The next executable row is
`RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0`. It may produce only a
non-Clone eligible package or a discard-only rejection. Physical owner open,
child/root lowering, declaration install, publication, and production
consumers remain zero.
