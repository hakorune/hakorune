# RAW-SOURCE0 LOWER ROOT0 — DECLACCESS0 design consultation

Status: **Design stop; no implementation authorized**  
Date: 2026-07-24  
Candidate: **DECLACCESS-prime-r1 (for consultation)**

## Why this is the next boundary

`ROOT0-PLAN0`, `OWNER0`, `ELIGIBILITY0`, `CHILDREN0`, and `CALLMAIN0` are
closed disconnected proofs. The retained `RawRootEnvironmentPlanV1` is still
only source authority: its declaration/index facts, callable catalog,
static-data facts, closure sites, access requirements, and runtime inputs have
not been installed into the candidate Builder or physical shell.

CALLMAIN0 therefore hands off to a ready product with an uninstalled
environment. BODY0 must not read `current_module`, rescan the AST, or infer
literal/source-site provenance from emitted MIR. DECLACCESS0 must close that
handoff first.

## Questions for decision

### Q1 — single installation owner

Which product should consume the ready Raw owner and install the exact
source-derived environment?

1. **Recommended:** a Raw-root-specific consuming
   `prepare_environment(self)` / `commit(self)` pair. Preparation validates
   destination emptiness and full coverage without mutation; the private
   commit moves one co-sealed Builder-index projection and shell-metadata
   projection into the same candidate physical owner.
2. Reuse the old declaration indexer and `current_module` metadata writes.
3. Install Builder facts and shell facts through separate terminals.

The recommended choice keeps one owner for declaration truth and prevents
partial Builder/shell installation.

### Q2 — source manifest and literal provenance

Should the environment plan be extended into one owned, source-derived
installation manifest containing:

```text
declaration/index facts
callable catalog
static-table source facts (or an explicit unsupported seal)
closure source sites (or an explicit unsupported seal)
access requirements
runtime-input snapshot
statement/method source locators needed by BODY0
literal and source-site payloads needed by ScalarControl0 lowering
```

The manifest must be produced once before Builder effects. A classifier that
only records shape (`Literal` without payload/site) is not sufficient to drive
BODY0. Decide whether the plan owns exact literal/site payloads or exposes a
bounded source lookup port; both must forbid whole-AST rescans and
post-binding ambient reads.

### Q3 — installation order and failure owner

Should the order be fixed as:

```text
consume ready invocation
-> validate token/family/session/physical brands
-> validate complete source coverage and empty destinations
-> prepare co-sealed Builder-index + shell-metadata product
-> infallible install into the candidate owner
-> issue DeclaredRawRootInvocationV1
```

Every preflight failure should return a discard-only rejected owner retaining
the complete unpublished chain, exact source manifest, and typed cause. No
retry, replacement manifest, partial installation, or BODY0 entry is allowed.

### Q4 — deferred capabilities

Which source facts are admitted in the first slice, and which reject before
physical mutation?

```text
ScalarControl0 declaration/index facts       = ?
callable catalog                         = ?
static data                              = unsupported until STATICDATA0, or ?
closures                                 = unsupported until CLOSURE0, or ?
process-global type/method slots          = unsupported until SLOT0, or ?
imports/plugin signatures                 = snapshot-only, or ?
```

No silent empty projection is acceptable. Unsupported capability must remain
a typed source-bound rejection.

### Q5 — handoff to BODY0 and ROOTBATCH0

Should DECLACCESS0 success issue a non-Clone
`DeclaredRawRootInvocationV1` that retains the installed environment and
exact body payload, while preserving these laws?

```text
BODY0 is the only next body owner
root tracker remains untouched until BODY0 starts
physical main/0 is not published yet
Main/condition ledger reservations remain zero
ROOTBATCH0 begins only after body completion
```

### Q6 — structural guards

Which guard should be the reusable Raw-root lane guard for this and later
rows? At minimum it must prove:

```text
environment installation producer = 1
Builder-index/shell-metadata projections share one source manifest
current_module reads during prepare/body handoff = 0
AST rescan after binding = 0
partial install / retry / fallback terminals = 0
production consumer = 0
all modified source/check files < 800 lines
```

## Explicit non-claims

This consultation does not authorize declaration installation, body lowering,
callable-Main re-selection, Main/condition root batching, physical drain,
finalization, postprocess, external commit, public ingress, JSON changes,
legacy retirement, or CUT0 activation.

The old `MainPending`/`MainCaptured` protocol, `build_static_main_box_typed`,
`lower_root`, and `finalize_module` remain disconnected legacy and are not
acceptable adapters for DECLACCESS0.

## Recommended next slice after decision

Lock one source-derived environment manifest and one mutation-free preflight
with a named rejected owner. Only after that green proof should a tiny
DECLACCESS0 implementation row be opened. BODY0 then consumes only the
declared/installed product and never reacquires source authority.
