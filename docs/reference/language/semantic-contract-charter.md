# Hakorune Semantic Contract Charter

Status: SSOT
Decision: accepted
Date: 2026-07-10
Scope: Cross-cutting language laws, normative precedence, and the change
protocol for Hakorune language v1.

Related:

- `docs/development/current/main/workstreams/language-v1-convergence-current.md`
- `docs/reference/language/semantic-kernel.md`
- `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/types.md`
- `docs/reference/language/lifecycle.md`
- `docs/reference/language/scope-exit-semantics.md`

## Purpose

Selfhost migration proves the language contract; it does not define that
contract. This charter makes language coherence the prerequisite for migration
and supplies laws that every syntax, semantic, parser, verifier, runtime, and
backend row must obey.

This charter does not itself change accepted syntax, runtime behavior, type
checking, ownership, failure handling, parser implementation, or backend
lowering. Those decisions remain in their topic owners until their Language v1
macro row lands.

## The Seven Laws

1. Same syntax, same guarantee.
   A spelling must not mean "metadata" at one boundary and a runtime contract
   at another. If a temporary distinction is unavoidable, the spellings must
   differ and the migration owner must be explicit.
2. Meaning is separate from representation.
   Source semantics must not be inferred from storage layout, MIR hints, route
   names, planner metadata, or backend choices. Representation-only knowledge
   belongs in facts, Plans, or Runes.
3. Absence, recoverable failure, and Fault are distinct.
   Ordinary absence, caller-handled failure, and violated language/runtime
   contract must not silently convert into each other.
4. Identity is separate from lifetime.
   Equality/identity, strong/weak reachability, ownership authority, and
   finalization are separate relations. A storage reference count is not the
   source-language ownership model.
5. Sugar preserves evaluation.
   Each source sub-expression is evaluated exactly once, in source order.
   Sugar and its semantic core form are observationally equivalent.
6. Compatibility is explicit.
   Canonical source is the default. Legacy spellings require a named profile,
   normalize immediately to a canonical shape, and cannot become implicit
   success paths.
7. Unsupported behavior fails before effects.
   An unsupported parser, verifier, runtime, or backend route must reject
   before user-visible mutation, publication, I/O, or fallback execution.

## Normative Precedence

The following order resolves conflicts.

1. This charter owns cross-cutting laws and the change protocol only.
2. `semantic-kernel.md` owns evaluation, Outcome, Place, cleanup, and sugar
   equivalence for Canonical v1.
3. `EBNF.md` owns canonical grammar. The future grammar registry may generate
   EBNF/support views but must not generate parser implementations.
4. Topic SSOTs own their declared semantics: `types.md`, `lifecycle.md`,
   `scope-exit-semantics.md`, `option.md`, and other named topic owners.
5. Stage profiles and generated support views report acceptance/support; they
   never create semantics or override grammar/topic owners.
6. Parser, verifier, runtime, and backend code are evidence of implementation.
   A conflict with higher-level normative text is a bug or an explicit
   migration gap, not an alternate language rule.
7. Historical, archive, investigation, and compatibility notes have no
   canonical authority.

Topic SSOTs retain ownership outside the semantic-kernel scope. A conflict with
the kernel is resolved by this precedence order, not by current implementation
behavior.

## Compatibility Contract

```text
default profile = Canonical
legacy profile = named and explicit
implicit compatibility = forbidden
compatibility output = canonical normalized shape or fail-fast
```

The grammar row owns the concrete profile name, registry rows, parser flags,
and migration schedule. This charter only forbids compatibility from becoming
an unstated default or a fallback after canonical rejection.

## Language Change Protocol

Every language-affecting change follows this order:

1. Name the semantic owner and state `Decision: proposed`, `accepted`, or
   `rejected` in the relevant SSOT.
2. Define canonical syntax or explicitly state that the change is semantic-only.
3. Define source authority, non-authority, fail-fast boundary, compatibility
   behavior, and unsupported-backend behavior.
4. Add positive and negative fixtures. Semantic changes also add evaluation or
   state-transition witnesses; counts and source paths are not proof.
5. Implement one durable semantic slice at a time. Parser implementations stay
   independent even when they share fixtures or witnesses.
6. Verify every supported runtime/backend route and fail fast before effects on
   unsupported routes.
7. Refresh generated support/document views, run conformance, and record the
   closeout or next explicit blocker.

No AST text substitution may stand in for a semantic preservation proof. A
sugar row must use the semantic-kernel contract once it is available.

## Current Boundary

```text
charter = active and normative
semantic kernel = accepted; evaluated-Place implementation active
grammar registry = queued
type contract activation = queued
failure/null migration = queued
ownership/lifecycle behavior change = queued
capability verifier activation = queued
selfhost migration = parked
```

## Acceptance Record

```text
language_constitution_owner_count = 1
constitutional_law_count = 7
normative_precedence_defined = 1
language_change_protocol_defined = 1
compatibility_requires_explicit_profile = 1
unsupported_fails_before_effect = 1
parser_behavior_changed = 0
runtime_behavior_changed = 0
backend_behavior_changed = 0
selfhost_claim = 0
```
