# CUT0-I0 Production Activation Consultation

Status: **Closed — Candidate ACT-prime-r1 selected; P0-R1 is active**
Date: 2026-07-23
Scope: connect the closed canonical physical drain product to finalization,
postprocess, external commit, and the eventual atomic CUT0 ingress.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-root0-drain0-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-production-transaction-consultation-2026-07-22.md`
- `src/mir/compiler/canonical_physical_completion.rs`
- `src/mir/compiler/mod.rs`
- `src/mir/compiler/module_session.rs`
- `src/mir/builder/module_lifecycle.rs`

## Why the design stop is required

`ROOT0-DRAIN0-P0/G0` is now closed as a disconnected proof. The canonical
physical products are real and route-specific:

```text
CanonicalDrainedInvocationV1::Single
CanonicalDrainedInvocationV1::Callable
```

They retain the original invocation token, source continuation, candidate
Builder session, drained module, receipt evidence, and callable capability
witness. No production consumer exists yet.

The current public ingress still uses a different owner chain:

```text
compile_resolved
  -> CanonicalModuleLoweringSessionV1
  -> build_resolved_*_function_module
  -> finish_built_canonical_module
  -> session.commit

compile_resolved_callable_module / recursive...
  -> CanonicalModuleLoweringSessionV1
  -> build_*_callable_module_candidate
  -> finish_built_canonical_module
  -> session.commit

legacy
  -> live MirBuilder::build_module
  -> finish_built_module
```

`finish_built_module` currently owns rune refresh, optimizer, contract
refresh, verification, RC insertion, metadata refresh, and callsite
canonicalization. `finalize_drained_module_once` is still only a wrapper, and
the old `DrainedModuleCandidateV1` requires Raw-only `main`/`condition_fn`
inventory. `PreparedBuilderExternalCommitV1` commits a Builder session but is
not paired with a postprocessed module and the same invocation identity.

These are different authorities. A compatibility adapter that merely converts
the new drained product into the old Main-only candidate would hide the
missing route policy and would not be a valid CUT0 proof.

## Questions for decision

### Q1 — drained-to-finalizer boundary

Should finalization consume route-specific drained products directly?

```text
CanonicalDrainedInvocationV1::Single
  -> CanonicalSingleFinalizationInputV1

CanonicalDrainedInvocationV1::Callable
  -> CanonicalCallableFinalizationInputV1
```

The old `DrainedModuleCandidateV1` must not be widened or used as an adapter
unless the decision explicitly proves that its Raw `main`/`condition_fn`
authority is not leaking into canonical routes. The preferred candidate is a
one-shot route-specific finalization product that consumes the drained module,
receipt evidence, source continuation, and capability witness together.

### Q2 — postprocess owner and verifier semantics

Which owner should receive the existing postprocess order?

```text
rune refresh
-> optimizer
-> contract refresh/validation
-> pre-transform verifier
-> route-selected RC policy
-> semantic metadata refresh
-> callsite canonicalization
-> changed semantic refresh
-> canonical final verifier
```

The decision must preserve the current semantic split:

```text
legacy verifier Err
  -> reportable result, publication may remain successful

canonical final verifier Err
  -> unpublished candidate is dropped, commit = 0
```

No postprocess stage may re-read `current_module` or re-resolve source
inventory after the drained product is created.

### Q3 — external commit product

What is the single non-Clone commit product?

```rust
PreparedModuleExternalCommitV1 {
    invocation_token,
    builder_session,
    postprocessed_module,
    verification_evidence,
}
```

It must prove, before mutation of the live Builder:

```text
token/session/module brand equality
candidate Builder commit-readiness
postprocess completion
route-specific inventory preservation
```

The commit terminal must be MirCompiler-owned, consuming, and infallible.
There must be no bare `MirCompileResult` path that bypasses the paired
Builder/module owner.

### Q4 — atomic ingress cutover

How do the three public canonical ingresses and the legacy ingress enter one
atomic CUT0?

The production policy must choose one outer executor for:

```text
raw
A+ / trivial
acyclic
recursive
```

Disconnected all-route harnesses remain allowed for preparation, but partial
production wiring, route-specific fallback, retry, or old-session fallback is
forbidden. The answer must name the exact outer entry and prove that all old
publication consumers become zero in the same cutover.

### Q5 — retirement census

Which consumers are removed in the activation patch, and which remain only as
test/disconnected evidence?

At minimum the census must distinguish:

```text
CanonicalModuleLoweringSessionV1 production callers
MirBuilder::build_module production caller
build_resolved_*_function_module callers
callable publish_into(&mut MirModule) callers
finalize_module direct insertion callers
old DrainedModuleCandidateV1 callers
new finalizer consumer count
new external commit consumer count
```

The activation patch may not claim success from passive type presence. Every
row needs an actual caller count and a focused all-route fixture.

## Non-claims until this consultation closes

```text
canonical production ingress = 0
production capture/drain consumer = 0
production finalizer = 0
production external commit = 0
Raw/canonical convergence = 0
atomic CUT0 activation = 0
old session/direct live-builder retirement = 0
```

## Required response shape

Return one candidate decision that fixes Q1–Q5, names the sole owners, keeps
route-specific evidence intact, preserves legacy/canonical verifier semantics,
and lists the smallest executable rows before the atomic cutover. Do not add
new production code while this card is a design stop.

## ACT-prime-r1 decision closeout — 2026-07-23

Candidate ACT-prime-r1 is selected for Q1–Q5.

```text
Q1  CanonicalDrainedInvocationV1::{Single, Callable}
      -> direct route-specific finalization inputs
Q2  compiler-private ModulePostprocessOwnerV1 is the sole postprocess owner
Q3  PreparedModuleExternalCommitV1 pairs Builder readiness with the
      postprocessed module, verification evidence, and the same token
Q4  MirCompiler::execute_preflighted_module_invocation is the sole production
      outer executor for Raw and all four canonical families
Q5  the activation patch zeroes every old production caller atomically and
      fixes the result with P0-R1 plus static caller census
```

The old Main-only `DrainedModuleCandidateV1`, bare `MirModule`, bare
`MirCompileResult`, route-local fallback, retry through the old session, and
partial production wiring are rejected. Raw and canonical routes use
route-specific finalization first and converge only at the paired
postprocess/commit boundary.

The executable order is fixed in the linked task card:

```text
FINAL0 -> POST0 -> COMMIT0 -> P0-R1 -> atomic CUT0/G0
```

Production consumers remain zero until the atomic cutover row.
