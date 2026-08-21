---
Status: design accepted; next bounded implementation cell is `SCRIPT-A-SOURCE-WINDOW-COSEAL-I0`
Date: 2026-08-21
Decision: SCRIPT-A-SOURCE-WINDOW-COSEAL-I0-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-source-capability-d0-2026-08-21.md
ProductionCaller: `ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle_with_target`
ReplacementCell: existing neutral window -> one pre-effect source-window handoff -> later A capability
Classification: T2 BoxShape/ownership prerequisite; no A/C meaning or production switch
NextCard: implement `SCRIPT-A-SOURCE-WINDOW-COSEAL-I0`
---

# SCRIPT-DIRECT-STATIC-A-SOURCE-WINDOW-COSEAL-I0

## Six-line brief

Decision: adopt the existing `PreparedScriptRootAdmissionV1` as the sole
canonical Script source-window authority. Split it from the neutral window by
move before target/Builder effects, carry it inside the existing pre-effect
source observation, and return it linearly to the later work plan. Do not
invent a second window or reconstruct terminal coverage.

Source authority + canonical issuer: `PreparedCanonicalScriptNeutralProgramWindowV1`
is the source-window issuer. `NormalScriptPreEffectSourceObservationIssuerV1`
co-seals its moved admission with the same parser witness, resolver Complete,
and owned lookup. A later private A issuer may consume only this complete
observation; no Builder work plan may issue or repair the window.

Non-authority: `PreparedProgramRootWorkPlanPartsV1`'s current
`script_root_admission` slot, Builder semantic/work-plan products, AST scans,
`VerifiedScriptSemanticSource<'source>`, lookup rows alone, source ordinal,
name, digest, pointer/address, empty/default admission, Recipe/Join, and
physical IDs cannot issue or pair a canonical window.

Fail-fast boundary: after the neutral window and lookup are available, but
before `install_pinned_text_target_capability`,
`prepare_normal_default_module`, package install, Bundle/Recipe/Join, or any
Builder child effect. A missing, foreign, duplicate, or incompatible window
must reject there with zero downstream effects.

Smallest next slice: `SCRIPT-A-SOURCE-WINDOW-COSEAL-I0` changes only ownership
and transport. It splits the existing neutral product into a source-window
part and a post-install physical-source remainder, moves the source-window
part into `PreEffectCompleteSourceObservationV1`, and returns it to the
existing work-plan consumer by move. It adds focused positive/negative tests
and one reusable structural guard.

Non-claims: no A candidate/noncandidate, C disposition, Recipe/Join redesign,
physical writer change, fallback, compatibility retry, old Recipe retirement,
production switch, parser cohort expansion, ABI, backend, or performance.

## Why this blocker is real

The current code already has the right source-window issuer, but its lifetime
is split at the wrong boundary:

```text
PreparedCanonicalScriptNeutralProgramWindowV1
  owns PreparedScriptRootAdmissionV1 + instance/constructor cohorts
  -> current lifecycle calls into_parts() after target capability install

NormalScriptPreEffectSourceObservationIssuerV1
  owns resolver forest/projection/boundary/demand/lowering/continuation
  + VerifiedScriptDirectStaticCallLookupV1
  -> currently receives only &neutral_window

PreparedProgramRootWorkPlanPartsV1
  -> currently receives script_root_admission after Builder preparation
```

The admission contains the sealed
`VerifiedScriptRootDemandWindowV1` and its deferred-residual registry. It is
the only existing product that can prove statement coverage, final Return
admission, retained terminal runtime, and a real zero-statement window. The
pre-effect observation currently drops it, so an A issuer placed before
Builder effects would have to borrow it, reissue it, or infer it from another
product. All three are wrong.

This is why the A capability remains a design stop even though the resolver,
typed Deferred, source coverage, and owned lookup are already pre-effect.

## Ownership diagram

```text
source package HRTB
        |
        +--> PreparedCanonicalScriptNeutralProgramWindowV1
        |       |
        |       +-- source-window part: PreparedScriptRootAdmissionV1
        |       |       (sealed window + deferred residuals)
        |       |
        |       +-- post-install remainder:
        |               instance transfer cohort + constructor source cohort
        |
        +--> ScriptDirectStaticCallLookupIssuerV1
        |
        +--> NormalScriptPreEffectSourceObservationIssuerV1
                |
                +--> PreEffectCompleteSourceObservationV1
                        source window + resolver/source facts + lookup
                        (AST-free, non-Clone, same invocation)
                                |
                                +--> future private A issuer
                                +--> future named consumer returns window
                                      to PreparedProgramRootWorkPlanPartsV1
```

The source-window part is not a new semantic decision. It is the existing
admission moved to the point where its source authority is still visible.
The post-install remainder may continue to carry the instance/constructor
cohorts; it must not carry a second copy or a second window authority.

## Required API shape

The exact names may be finalized in the implementation card, but the type
relations are fixed:

```rust
PreparedCanonicalScriptNeutralProgramWindowV1
  --split_for_pre_effect(self)-->
(
    PreparedScriptRootAdmissionV1,
    PreparedCanonicalScriptPostInstallRemainderV1,
)

NormalScriptPreEffectSourceObservationIssuerV1::issue(
    package,
    source_window: PreparedScriptRootAdmissionV1,
    lookup,
    declaration_facts,
    resolver,
) -> Result<PreEffectCompleteSourceObservationV1, Issue>

PreEffectCompleteSourceObservationV1 {
    source_window: PreparedScriptRootAdmissionV1,
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    lookup: VerifiedScriptDirectStaticCallLookupV1,
    private seal,
}
```

`ScriptDirectStaticCallLookupIssuerV1` may borrow the source window before the
split completes, or accept `&PreparedScriptRootAdmissionV1`; it must not issue
another window. The later bind callback may expose the moved admission to the
named downstream consumer, but no public getter may let dispatch store or
recombine it with a different source.

The implementation must use a linear move state, not a parallel optional
field:

```text
NeutralWindowReady
  -> SourceWindowMovedToPreEffectObservation
  -> A/C consumer returns the same admission by move
  -> WorkPlanAdmissionConsumed
```

`Option<PreparedScriptRootAdmissionV1>` is allowed only where the existing
Main/compatibility outer transport already distinguishes source-backed Script
from other modes; it must not be used as an internal “maybe we forgot the
window” repair state. `unwrap_or_default`, an empty window, or a second
`VerifiedScriptRootDemandWindowV1::seal` is forbidden.

## Finite state and error boundary

| State | Owner | Meaning | Next edge |
| --- | --- | --- | --- |
| `NeutralWindowReady` | neutral window issuer | one invocation-bound sealed admission exists | split by move |
| `SourceWindowMoved` | pre-effect issuer | admission is inside the source observation attempt | co-seal resolver/lookup |
| `Complete` | pre-effect issuer | window, resolver, terminal/continuation, and lookup share the witness | future A issuer |
| `SourceAuthorityUnavailable` | source package/window owner | no source authority or parser loan | typed reject before effects |
| `ObservationDeferred(cause, site)` | resolver owner | resolver explicitly deferred | typed reject before effects |
| `Incomplete(reason, site)` | coverage/window validator | required row/window is absent | typed reject before effects |
| `IntegrityInvalid(reason, site)` | co-seal validator | foreign, duplicate, stale, or contradictory relation | typed reject before effects |
| `AdmissionReturned` | named downstream consumer | the same admission is handed to work planning | no second source issuer |

The existing `NormalScriptPreEffectSourceObservationIssueV1` vocabulary is
retained. This cell does not convert `Deferred` to `Incomplete`, and does not
turn an empty or missing window into `CompleteEmpty`. A real empty Script is
the already-sealed admission with zero entries; missing admission is a reject.

## Co-seal checks required before A

The source-window cell must prove, without a second AST observer:

- the moved admission, parser package loan, lookup, and resolver facts carry
  the same `ParserInvocationWitnessV1`;
- the admission window remains the one used for the resolver and lookup;
- each retained MethodCall continuation row has matching complete source
  coverage, receiver site, ordered argument sites, result site, and terminal
  relation;
- a true zero-call Script remains distinguishable from a missing window;
- the source admission returned to work planning is exactly the moved value,
  not a reconstructed or copied sibling;
- `PreparedScriptRootWorkPlanPartsV1` consumes that value once after the
  pre-effect decision, while the post-install remainder supplies only its
  existing physical-source cohorts.

These checks are source/Facts transport checks. They do not choose a target,
issue a candidate, prove ExactI64, create a Recipe key, or allocate a MIR ID.

## Implementation task sequence

### Commit 1 — `SCRIPT-A-SOURCE-WINDOW-COSEAL-I0`

Change only the linear ownership boundary:

1. add the neutral-window split/remainder transition;
2. pass `PreparedScriptRootAdmissionV1` into the pre-effect source issuer;
3. retain it in `PreEffectCompleteSourceObservationV1`;
4. return it through the existing bind/consumer path to the work plan;
5. keep instance/constructor cohorts in their current post-install role;
6. add tests and a structural guard.

Acceptance:

- positive real empty Script and final root Return retain one sealed window;
- positive composite Script retains provider transfer, call coverage, and
  terminal admission under the same witness;
- foreign window, missing window, and invocation mismatch reject before target
  capability and Builder effects;
- no second window seal, AST scan, pointer pairing, or default/empty repair;
- one source-window owner, one pre-effect caller, one work-plan consumer;
- touched Rust production files remain below 760 lines and hard-stop below
  800.

Non-claims: A/C is still closed after this commit.

### Commit 2 — `SCRIPT-A-CAPABILITY-I0`

Only after Commit 1 is green, consume the complete pre-effect observation in a
private, non-Clone A capability. The A issuer validates lookup coverage against
the continuation and emits the source-level direct rows or an explicit
complete-zero witness; it moves the source-window admission onward and does
not return a capability to dispatch.

### Commit 3 — `SCRIPT-C-DISPOSITION-CONSUMER-I0`

Add the named C disposition and two consumers. The direct consumer may adapt
the existing Facts/Recipe/Join input boundary; the non-direct consumer must
retain the Script continuation without using an empty Bundle or claim `Absent`
as proof.

### Commit 4 — `SCRIPT-A-CUTOVER-I0`

Move the complete A/C facade before target/Builder effects, prove zero old
Recipe/fallback edges for the migrated surface, and only then consider a
separate production caller switch/retirement row.

## Guard and evidence plan

The reusable guard for Commit 1 must assert:

```text
PreparedScriptRootAdmissionV1 source owner count = 1
pre-effect source-window handoff callers       = 1
post-install second window issuer              = 0
AST/name/ordinal/pointer pairing               = 0
empty/default admission repair                 = 0
pre-effect failure -> target/Builder effects   = 0
pre-effect failure -> Recipe/Join/fallback     = 0
source-window production files                 < 760 lines
```

The focused test names and classified baseline reds belong in the active card
at closeout. Local green is evidence for this cell only; it does not authorize
A/C, physical, or production claims.

## Independent queue remains separate

The confirmed ignored `ReleaseStrong` result in
`assignment_lowering.rs` remains the High correctness task in
[`mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md`](./mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md).
The `emit_instruction` strictness audit, `builder.rs` barrel cleanup,
`builder_init.rs` responsibility split, and branch integration remain queued
in [`mirbuilder-post-audit-follow-up-queue-2026-08-21.md`](./mirbuilder-post-audit-follow-up-queue-2026-08-21.md).
None of them may be used to bypass this source-window authority boundary.

## Design-stop exit

This design stop is closed for the bounded source-window cell because the
existing owner and failure boundary are now named:

```text
neutral window issuer
  -> move existing PreparedScriptRootAdmissionV1
  -> pre-effect source observation
  -> future private A consumer
  -> same admission by move to work plan
```

If implementation discovers that the admission cannot be split without a
second window issuer or an internal optional/default repair, stop immediately
and record the missing owner instead of adapting Builder products.
