# RAW-SOURCE0 LOWER ROOT0 — BODY0 design question

Status: **Design stop — BODY0 owner and payload boundary are not selected**  
Date: 2026-07-24  
Scope: the inline Raw Script/App root-body transition after DECLACCESS0.

`DECLACCESS0-S0` is closed in `e2fb0839f5`. Its compiler-side
`declare_environment(self)` terminal now returns a route-specific
`DeclaredRawRootInvocationV1`; Builder/shell installation, source manifest
retention, and discard-only rejection are proven. No BODY0 consumer exists.

The worker audit found that the next boundary is not a small implementation
detail. The existing root lowerers are AST-based and carry legacy Main-only
or `current_module` authority. BODY0 therefore needs one new typed owner
before any code is wired.

## Current evidence

```text
DeclaredRawRootInvocationV1
  owns installed Builder/session + physical shell/collector/ledger/tracker
  retains RawRootPostInstallManifestV1
  retains exact located ScalarControl0 body facts
  has no begin_body consumer yet

RawRootPhysicalStateV1
  has static-child and callable-Main terminals only
  has no root-body terminal

RootBodyCompletionTrackerV1
  is fresh and untouched after DECLACCESS0
  existing borrow schedule already names RootBodyDrive -> RootBodySeal
  -> CompletedRootBody

legacy lower_root / build_static_main_box_typed / MainPending / MainCaptured
  are disconnected and must not become BODY0 adapters
```

The exact source body authority is the located `RawScalarControl0` payload
already produced by `RawRootSourceFactsV1`. BODY0 must not rescan the AST,
rerun the classifier, read `current_module`, or reconstruct literals from MIR.

## Questions to decision-lock

### Q1 — BODY0 owner and entry

Which single owner may start inline root lowering?

```text
1. Recommended: one consuming `DeclaredRawRootInvocationV1::begin_body(self)`
   terminal. It returns route-specific active Script/App body owners and is
   the only BODY0 entry.

2. Reuse `PendingMainDraftV1` / `MainPending` as a compatibility adapter.

3. Open BODY0 from the raw plan, physical state, or source package directly.
```

The first choice keeps DECLACCESS0's installed owner intact and prevents a
second session, shell, collector, or source authority from being created.

### Q2 — exact body payload boundary

Where should the located ScalarControl0 payload live at the Builder boundary?

```text
1. Recommended: move one exact, non-Clone neutral body recipe into a shared
   `src/mir` contract. It owns literal/name/operator/path/span and Script/App
   body shape; Builder consumes it directly.

2. Let Builder import compiler-private `RawLocatedScalar*` facts.

3. Convert the located payload back to AST and reuse the legacy lowerer.

4. Expose a bounded source lookup port and let BODY0 query the AST lazily.
```

Options 2–4 either invert the dependency, violate the no-rewrite/no-rescan
law, or create a second source authority. The selected recipe must be fully
lowerable; a `Some` fact with no matching lowerer is not an eligible proof.

### Q3 — physical terminal and root draft

How does BODY0 create the unpublished physical `main/0` draft?

```text
1. Recommended: add one Builder-sibling consuming terminal on
   `RawRootPhysicalStateV1`. It takes the exact body recipe, owns the short
   tracker/collector loans internally, and returns a named route-specific
   unpublished root-body product or rejected owner.

2. Export `(session, shell, collector, ledger, tracker)` to compiler code.

3. Call `lower_root`, `build_static_main_box_typed`, or `finalize_module`.
```

The terminal must leave the physical `main/0` and `condition_fn/1` root batch
unreserved. Root-body completion produces a draft/witness only; ROOTBATCH0 is
the later owner of Main/condition reservation and publication.

### Q4 — first body grammar and provenance

Should BODY0 lower the whole currently admitted ScalarControl0 grammar, or
should eligibility narrow first?

The current classifier admits literals, variables, unary/binary operators,
print, assignment, compound assignment, local, if, loop, loop-range, return,
break, continue, and scope-box. The lowerer must cover every admitted form.

Before exact path/span provenance is claimed, fix or explicitly exclude the
known `Return` and `ScopeBox` path-collision cases in the source facts
classifier. App Main payload also currently drops return type, uses, attrs,
and parameter metadata; decide whether to extend the manifest or reject those
shapes before BODY0.

```text
1. Full ScalarControl0 with a neutral typed lowerer and provenance fixture.
2. Recommended conservative alternative: narrow eligibility to the first
   body subset whose recipe, metadata, and lowerer are all complete; open the
   remaining grammar in a separate BoxCount row.
```

No `Some` recipe may reach BODY0 without a total lowerer.

### Q5 — tracker, sequencing, and success product

Fix the order as:

```text
consume DeclaredRawRootInvocationV1
-> begin BODY0 tracker activity
-> lower exact Script/App body recipe
-> close all child/header/pending loans
-> issue CompletedRootBodyV1
-> issue route-specific unpublished root-main product
```

`RootBodyCompletionTrackerV1` remains untouched before `begin_body`, and its
completion witness is the only input allowed to the later ROOTBATCH0 row.
Callable-Main disposition/receipt remains evidence only; BODY0 must not
reselect or recount it.

### Q6 — failure and atomicity

Every source, lower, cleanup, tracker, or admission failure must return a
discard-only rejected BODY0 owner retaining the declared owner, successful
prefix evidence, exact typed nested cause, and tracker/physical state. No
retry, fallback, body re-entry, root-batch continuation, `catch_unwind`, or
typed panic claim is added by this row.

At minimum, the rejection matrix must prove:

```text
source recipe mismatch before physical mutation
natural primary / cleanup / during-cleanup lower failure
tracker loan mismatch
collector admission failure before root publication
foreign brand/family
```

### Q7 — ROOTBATCH0 handoff

Should the success product be a new `RawRootBodyCompleteInvocationV1` whose
only continuation is the later ROOTBATCH0 terminal, with these invariants?

```text
main/0 draft remains unpublished until ROOTBATCH0
condition_fn/1 remains unpublished
root ledger reservations = 0
physical root shell has no published root function
production consumers/public ingress = 0
```

## Forbidden in this consultation

```text
BODY0 implementation or production wiring
AST rescan or AST reconstruction
current_module inventory lookup
MainPending/MainCaptured adapters
legacy lower_root/build_static_main_box_typed/finalize_module
Main/condition reservation or root batch commit
drain/finalization/postprocess/external commit
retry/fallback/catch_unwind
JSON/public-ingress changes or CUT0 activation
```

## Required decision output

```text
Decision: BODY-prime-r1 (or another candidate)
Q1 = ...
Q2 = ...
Q3 = ...
Q4 = ...
Q5 = ...
Q6 = ...
Q7 = ...
first executable row = RAW-SOURCE0-LOWER0-ROOT0-BODY0-S0
non-claims = root batch, drain, finalization, postprocess, commit, ingress
```

No BODY0 code is authorized until this question is answered and the current
pointer is moved to a dedicated execution task.
