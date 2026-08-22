Status: Implementation in progress; one bounded Ready-edge cutover
Task: MIR-CALLABLE-LOOP-ROOT-UNPUBLISHED-SCOPE-I0
Date: 2026-08-23
Priority: bind the existing root candidate and collector to the Ready physical adapter
Design: docs/development/current/main/investigations/mirbuilder-callable-loop-root-unpublished-scope-d0-2026-08-23.md
Current execution row: MIR-CALLABLE-LOOP-ROOT-UNPUBLISHED-SCOPE-I0
CurrentCard: docs/development/current/main/investigations/mirbuilder-callable-loop-root-unpublished-scope-i0-2026-08-23.md
NextCard: none until I0 evidence is complete
---

# Callable Loop root unpublished scope I0

## Six-line brief

Decision: Implement only the accepted root-scope handoff. The existing `ModuleBuilderInvocationSessionV1` owns the unpublished root candidate; the same-root `ModuleDraftCollectorV1` owns draft admission; a private borrow-scoped facade co-binds them under the invocation brand.
Source authority + canonical issuer: Existing source Facts issuer and semantic Recipe issuer remain sole semantic issuers. The named physical adapter is the one consumer and receives the private root scope plus Recipe.
Non-authority: bare adapter `&mut MirBuilder`, child `CanonicalFunctionLoweringSessionV1`, disconnected `ModuleLoweringInvocationV1`, AST re-scan, second Facts/Recipe issuer, fallback, and external publication.
Fail-fast boundary: all Ready physical work occurs inside the existing unpublished root candidate; every error returns before collector drain/commit and the outer root lifecycle rejects/discards the candidate. This I0 does not make PlanVerifier effect-free.
Smallest next slice: add the private scope, thread it through the one Ready edge, remove the adapter's production bare-Builder signature, and add focused success/reject/zero-publication evidence.
Non-claims: no symbolic CorePlan, no general capability redesign, no Outside consumer, no I9 transaction work, no legacy retirement, no publication protocol change, no performance work.

## Authorized implementation cells

```text
T1  private root scope
    - co-bind existing ModuleBuilderInvocationSessionV1 candidate
    - co-bind the same-root ModuleDraftCollectorV1
    - retain invocation-brand pairing
    - no independent state, no second session, no generic Builder getter

T2  Ready edge
    - thread the scope only through RawInvocationChildPort -> Ready adapter
    - keep Facts issuance and Recipe issuance at exactly one each
    - keep Outside terminal-only and Builder-effect zero
    - no lower_loop_or_freeze_v1 fallback for Ready

T3  failure boundary
    - preserve the existing rejected root session owner
    - collector drain/commit only after the whole root path succeeds
    - no retry, compatibility fallback, or second source observer

T4  evidence
    - positive Ready path
    - rejected physical path has no collector drain/commit/publication
    - success has one drain and one commit
    - adapter caller/constructor guards
    - focused tests, pointer, diff, and source-size checks
```

## Required structural guards

```text
root Ready adapter caller count = 1
adapter production bare `&mut MirBuilder` entry = 0
root-scope constructor outside ModuleBuilderInvocationSessionV1 = 0
CanonicalFunctionLoweringSessionV1 opened by root Ready path = 0
ModuleLoweringInvocationV1 introduced into live root path = 0
source Facts issuer = 1
semantic Recipe issuer = 1
collector drain = 1 on success
collector commit = 1 on success
collector drain/commit = 0 on rejected Ready path
Ready -> lower_loop_or_freeze_v1 = 0
Ready -> fallback/retry = 0
Outside Builder effect = 0
```

## Stop conditions

Return to the D0 design stop if the scope would expose arbitrary mutable
Builder access, require a second module/function session, let collector drain
before Ready success, or require `PlanVerifier` to become effect-free. Do not
expand I0 into `SymbolicCorePlan`, ordinary Outside consumption, or publication.

## Closeout evidence

The I0 closeout must name changed files, focused test commands and results,
guard commands and results, source-size results, commit SHA, and pushed remote
state. If any required evidence is missing, leave the card partially open and
record the next bounded action instead of claiming completion.
