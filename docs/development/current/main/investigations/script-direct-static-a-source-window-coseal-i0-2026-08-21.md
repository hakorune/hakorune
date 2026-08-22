---
Status: source-window I0 is pushed as `f0d80d6943`; A/C linear-consumer design is accepted and selected for fast execution
Date: 2026-08-22
Decision: SCRIPT-A-C-CONSUMER-SERIES-I0-R0
Parent: docs/development/current/main/investigations/script-direct-static-a-source-capability-d0-2026-08-21.md
ProductionCaller: `ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle_with_target`
ReplacementCell: complete pre-effect observation -> private A -> immediate C -> one required post-install consumer
Classification: T2 BoxShape/in-place authority replacement; no new accepted language shape
NextCard: fast `SCRIPT-C-NAMED-CONSUMER-SEAM-R0`
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

### Commit 2 — `SCRIPT-C-NAMED-CONSUMER-SEAM-R0`

Prepare the required downstream seam before issuing A. The Script lowering
input and claim state must use one closed, non-optional disposition:

```text
CompleteNoDirectStaticClaims(source-backed C witness)
DirectStaticClaims(nonempty downstream input)
```

The first state is not an empty Bundle and does not authorize a missing-row
claim result. Once selected Script owns either C
state, a static-claim ingress with no exact row is an integrity failure; it may
not retry the generic static lookup. This commit is BoxShape-only and must not
create a detached production route.

### Commit 3 — `SCRIPT-A-CAPABILITY-C-DISPOSITION-I0`

Consume `PreEffectCompleteSourceObservationV1` in a module-private, non-Clone
capability. Its private issuer performs one total correspondence pass and
immediately moves Ready through the named A issuer and C issuer. Dispatch never
receives or stores a capability or A observation. The resulting required C
transport owns the same admission, pre-effect parts, lookup authority, and one
closed disposition.

The direct arm carries nonempty candidate-site rows plus key-free required
argument source facts. The non-direct arm carries the exact A zero reason and
all explicit non-direct rows. Neither arm owns Recipe/Join keys or physical
IDs.

### Commit 4 — `SCRIPT-A-C-CUTOVER-R0`

At the sole pre-effect callpoint, replace the direct
`PreEffectCompleteSourceObservationV1` storage with the A/C facade result.
After package install, exactly one named consumer moves the same admission to
the work plan and:

- projects the direct arm into the existing downstream Facts/Recipe/Join
  boundary; or
- installs `CompleteNoDirectStaticClaims` and skips empty Bundle/Recipe/Join
  construction.

Delete the selected-Script empty-Bundle/claim-`Absent` proof edge in the same
series. Deferred, Incomplete, or Invalid must stop before target installation,
Builder effects, Recipe/Join, Call, publication, or fallback.

Commits 2-4 are one acceptance unit. They may be local review commits, but A
or C is not independently complete and the series is not pushed/closed until
the production caller, both named consumers, old-edge retirement, focused
tests, and structural guard are all green.

## Guard and evidence plan

The reusable guard for Commit 1 asserts:

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

For the selected A/C series, the guard must additionally assert:

```text
private capability production constructor/caller = 1/1
capability or A observation returned/stored        = 0
C transport production consumer                   = 1
selected-Script claim Absent fallback              = 0
failure -> target/Builder/Recipe/Call/publication  = 0
AST rescan / resolver rerun / catalog rebuild      = 0
duplicate window/coverage/lookup owner             = 0
Builder/Recipe key/MIR ID inside A or C             = 0
wildcard/default/unwrap_or_default state merge      = 0
touched production files                            < 760 lines
```

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

## Implementation closeout

`f0d80d6943` closes this bounded ownership cell:

```text
PreparedCanonicalScriptNeutralProgramWindowV1
  -> split_for_pre_effect
  -> PreparedScriptRootAdmissionV1 moves into
     PreEffectCompleteSourceObservationV1
  -> resolver/lookup co-seal under the same parser invocation
  -> split_for_work_plan
  -> the same admission moves to PreparedProgramRootWorkPlanV1
```

The admission now owns its `ParserInvocationWitnessV1`. Instance-Box transfer
and constructor-source cohorts move in a separate
`PreparedCanonicalScriptPostInstallRemainderV1`; they cannot issue, copy, or
repair the canonical source window. No A/C meaning, Recipe/Join edge, physical
effect, fallback, or production-switch claim was added.

Evidence:

- `cargo check --profile quick --lib`: green;
- `cargo test --profile quick --lib pre_effect_source_observation`: 4 passed,
  including real empty Script, final root Return, ordinary zero-call, and
  foreign parser invocation rejection;
- `cargo test --profile quick --lib normal_script_neutral_window`: 2 passed;
- `script_direct_static_source_reown_window_r0_guard.sh`: green;
- `current_state_pointer_guard.sh` and `git diff --check`: green;
- touched production owners are 262, 429, 213, and 600 lines, all below the
  760 split trigger and 800 hard stop.

## A/C design closure — 2026-08-22

Decision:
reuse `PreEffectCompleteSourceObservationV1` as the sole moved payload of a
private capability. Do not duplicate its window, resolver facts, continuation,
coverage, or lookup. Ready is consumed immediately into A, then C, then one
required post-install consumer.

Source authority + canonical issuer:
the complete pre-effect observation owns the same-invocation window, resolver
Complete product, terminal/continuation, total call coverage, and owned
target/result lookup. `CanonicalScriptASourceCapabilityIssuerV1` is the sole
private co-seal issuer; `CanonicalScriptAObservationIssuerV1` and
`CanonicalScriptCDispositionIssuerV1` are its immediate linear consumers.

Non-authority:
the pointer-branded old ResultBundle, empty Recipe/Join, `ClaimLedger::Absent`,
AST/name/ordinal/digest, Builder work plan, generic static retry, and physical
IDs cannot issue A zero, A rows, or C disposition.

Fail-fast boundary:
after `NormalScriptPreEffectSourceObservationIssuerV1::issue` succeeds and
before `install_pinned_text_target_capability`. All A/C issue failures have
zero target install, Builder, Bundle, Recipe/Join, Call, publication, and
fallback effects.

Smallest next slice:
execute the three-commit `SCRIPT-A-C-CONSUMER-SERIES-I0-R0`, beginning with
`SCRIPT-C-NAMED-CONSUMER-SEAM-R0`. The complete series is one BoxShape
acceptance unit and one push boundary.

Non-claims:
no source cohort expansion, target-absence language change, general nominal
result support, Recipe/Join redesign, backend/ABI change, broad Builder
cleanup, compatibility/raw retirement, or performance-optimization claim.

### Finite states

| State | Exact meaning | Allowed terminal |
| --- | --- | --- |
| `A.Zero.EmptyScript` | sealed source window has zero statements and zero calls | `C.NonDirect` |
| `A.Zero.NoMethodCalls` | nonempty complete window has zero MethodCall rows | `C.NonDirect` |
| `A.Zero.ObservedNonDirect` | every call has an explicit source-backed non-direct reason | `C.NonDirect` |
| `A.Rows` | one or more lookup-backed ExactI64 direct-static rows; remaining calls have explicit non-direct rows | `C.DirectStatic` |
| `ObservationDeferred` | resolver supplied typed cause/site | pre-effect reject |
| `Incomplete` | expected window/coverage/lookup/continuation/result/required-argument fact is missing or outside bounded I0 | pre-effect reject |
| `IntegrityInvalid` | foreign witness, duplicate/extra row, or site/order/owner/terminal contradiction | pre-effect reject |
| private `Ready` | all required relations are complete and consistent | immediate A consume only |
| `C.NonDirect` | source-backed complete-zero disposition | named no-claim consumer |
| `C.DirectStatic` | nonempty candidate disposition | named direct consumer |

For bounded I0, a target absent from the complete catalog, unavailable result
authority, non-ExactI64 result, or unsupported required-argument source is
typed `Incomplete`; it is never forged into A zero or retried after A starts.
This preserves the current hard-stop semantics. A later source-backed
noncandidate expansion requires its own BoxCount Decision.

### One-pass and cost contract

Safety validation is one pre-effect correspondence pass over already-owned
products. It may use existing `BTreeMap` lookups, so the conservative bound is
`O(n log n)` for `n` observed call rows; C classification and move transport
are constant-time apart from moving owned containers. The implementation must
not:

- scan the AST again;
- rerun the resolver;
- rebuild target/result catalogs or perform a second lookup;
- clone complete coverage, lookup, source-window, or resolver containers;
- repeat the total correspondence check after C is issued;
- add broad benchmark or repository-wide test gates to this semantic slice.

Focused positive/negative tests and the structural guard are sufficient for
the slice. A benchmark is required only if implementation introduces an extra
observer or allocation not present in this contract. The intended final path
is lighter than the current route because empty Bundle/Recipe/Join creation,
post-effect source revalidation, and generic fallback disappear.

### File and size plan

Keep the 827-line `builder.rs` untouched. Add private children below the
213-line `normal_script_pre_effect_source_observation` owner:

```text
normal_script_a/model.rs                    target <= 300 lines
normal_script_a/issuer.rs                   target <= 500 lines
normal_script_a/required_argument_source.rs target <= 350 lines
normal_script_a/consumer.rs                 target <= 350 lines
normal_script_a/tests.rs                    focused matrix only
```

The 695-line `source_call_target/script_direct_static.rs` receives no semantic
growth. The 600-line lifecycle receives only the sole thin facade call. Split
at 760 and hard-stop at 800; do not compress or append tests to production
owners to evade the limit.

The independent Result-discard queue remains ordered as
`MIR-RESULT-DISCARD-CENSUS-D0` -> `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` ->
`MIR-RESULT-DISCARD-GUARD-I0`; `EmitReceipt` remains parked.

## A/C implementation checkpoint — 2026-08-22

The working tree now contains the selected A/C seam and the production
callpoint replacement, but this checkpoint is not a pushed closeout yet.
The linear path is:

```text
PreEffectCompleteSourceObservationV1
  -> private non-Clone A capability
  -> CanonicalScriptAObservationIssuerV1
  -> CanonicalScriptCDispositionIssuerV1
  -> one post-install C consumer
  -> required CompleteNoDirectStaticClaims or DirectStaticClaims input
```

The direct/no-direct lowering products are retained by one
`ScriptDirectStaticLoweringProductsV1` enum. They are not parallel optional
fields, and the C consumer remains the only place that projects canonical A
rows into the existing Bundle/Recipe/Join/required-argument boundary. The
resolver MethodCall admission is a bounded source-profile repair: ordinary
Script root MethodCall traversal is admitted while the narrower lambda-leaf
profile remains unchanged. No second resolver or AST observer was added.

Evidence recorded for this working tree:

- `CARGO_BUILD_JOBS=4 cargo check --profile quick`: pass; the repository
  baseline still emits 2,286 warnings and no new compile error;
- direct focused test binary: A/C 2 passed, pre-effect source observation 6
  passed, claim ledger 7 passed, and retained FunctionCall 1 passed;
- `script_direct_static_a_c_consumer_i0_guard.sh`: pass;
- updated `script_direct_static_target_guard.sh`: pass;
- source-reown, composite-admission, current-state-pointer, and diff guards:
  pass;
- the repository-wide `cargo fmt --all -- --check` remains red on existing
  formatting drift; no repository-wide formatter rewrite was applied;
- the broad `normal_script_semantic_source` filter remains a known parent
  baseline red on work-plan/contract fixtures and is not counted as focused
  A/C acceptance evidence.

The target guard is intentionally retained as a legacy broad audit and is
heavier than the new A/C guard because it invokes several cargo test filters
sequentially. The A/C series does not add those repeated cargo invocations to
its normal fast gate; its reusable guard is structural and its focused tests
are run once per evidence collection.
