---
Status: Design consultation stop / no implementation authorization
Date: 2026-07-25
Decision: LANGUAGE-DOCS-TRY-CATCH-D1 (questions fixed; semantics unresolved)
Scope: reconcile language-level exception, scope-cleanup, and compatibility status
Related:
  - docs/reference/language/status-index.md
  - docs/reference/language/grammar-contract.md
  - docs/reference/language/semantic-kernel.md
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/lifecycle.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/stage-profiles.md
  - grammar/language-v1-registry.toml
---

# LANGUAGE-DOCS-TRY-CATCH-D1 consultation

This card fixes the questions that must be answered before any grammar,
parser, runtime, backend, or registry edit. It intentionally does not select
a permissive interpretation from conflicting documents.

## Evidence that requires one decision

```text
statement try:
  grammar-contract = reserved/rejected
  EBNF             = default accepted behind compatibility controls
  historical ref   = legacy compatibility

postfix catch:
  grammar contract/registry = canonical
  EBNF                     = Stage-3 and four-gate guarded
  semantic kernel           = canonical Fault is terminal/non-catchable
  scope-exit SSOT           = catch/cleanup routes exist

cleanup/fini:
  scope-exit SSOT = cleanup is a scope-exit channel and fini is a legacy name
  grammar/registry = fini-shaped rows are canonical
  EBNF            = no complete standalone/local cleanup row

EBNF exception lowering:
  describes exception/rethrow/finally and backend no-op degradation,
  which conflicts with fail-fast and explicit compatibility-profile law.
```

## Questions to answer

### Q1 — What does postfix `catch` handle?

Choose exactly one semantic owner:

```text
A. canonical Fault handling
B. a separate scope-exit/handler signal that is not canonical Fault
C. compatibility-only syntax with no canonical semantic owner
```

The answer must define whether `Fault` remains terminal, whether a handler can
resume or transform it, and what happens when a backend cannot materialize the
chosen handler route. “No-op” degradation is not an accepted answer; unsupported
behavior must reject before user-visible effects.

### Q2 — What is the status of statement `try`?

Fix one named profile for each status:

```text
Canonical language profile: accepted | reserved/rejected
Compat2025 profile:        accepted | reserved/rejected
Default profile:            explicit named choice, never ambient env behavior
```

If compatibility acceptance remains, define its owner, stable gate/reject tag,
failure/result model, and sunset condition. `NYASH_FEATURES` or parser toggles
must not silently change the canonical profile.

### Q3 — Are `cleanup` and `fini` one syntax or two named profiles?

Define the source-level authority for:

```text
cleanup { ... }
local x cleanup { ... }
fini { ... }
local x fini { ... }
box.fini()
```

The answer must distinguish scope cleanup from object finalization, state which
spellings are canonical/compatibility-only/reserved, and list the exact EBNF
and registry rows required. No parser support may be inferred from the topic
SSOT alone.

### Q4 — What is the gate model?

Replace the current mixture of “canonical”, Stage-3, four environment gates,
and backend caveats with one explicit vocabulary:

```text
grammar status  = canonical | compatibility_only | reserved | rejected
availability    = live | guarded | transport | pending | deferred |
                   prohibited | historical
```

For every selected row, specify the profile owner, gate/reject tag, allowed
backend set, and whether the feature is permitted in source, metadata, or
transport only. A feature cannot be both unconditional canonical and guarded
without a named compatibility/profile boundary.

### Q5 — What is the failure and ownership law?

Define the typed boundary for:

```text
parse rejection
profile rejection
handler preparation failure
cleanup/finally lowering failure
runtime Fault
process-result projection
```

Each must have one owner, no retry/fallback, and a clear point before or after
user-visible effects. `throw` remains prohibited unless a new language decision
explicitly reopens it; it must not be used as an implicit implementation of
`try` or `catch`.

### Q6 — What is the migration and sunset contract?

For every compatibility-only or guarded feature, define:

```text
profile name
stable gate/reject tag
current consumers
canonical consumers (must be zero if compatibility-only)
promotion or retirement condition
owner of the next decision
```

Historical `LANGUAGE_REFERENCE_2025.md` remains evidence only and cannot
override the selected current profile.

## Required decision output

The response must include a single matrix for `try`, `catch`, `cleanup`,
`fini`, `box.fini()`, and `throw` with:

```text
grammar status
availability/profile
semantic owner
parser/registry action
backend behavior
stable gate/reject tag
sunset/promotion condition
```

Until that matrix is accepted, the status index must keep affected rows as
`status_conflict` and no implementation row may start.

## Conservative working recommendation (not accepted)

The audit's least-surprising candidate is recorded only to make the trade-off
explicit, not to preselect the answer:

```text
canonical v1:
  cleanup is the scope-exit spelling
  box.fini() remains the separate object-lifecycle API
  throw remains rejected

Compat2025 only:
  fini { ... } / local ... fini aliases normalize to cleanup
  statement try and postfix catch handle only an explicitly named
  compatibility failure channel, never canonical Fault

until a separate outcome decision:
  canonical catch is not claimed
  no-op backend degradation is forbidden
```

An alternative that introduces a new canonical recoverable outcome must be
treated as a separate semantic decision, not smuggled into this docs cleanup.

## First executable row after acceptance

```text
LANGUAGE-DOCS-TRY-CATCH-D1-CLOSEOUT
```

That closeout is still docs-only: update the authority pages and status index
from the accepted decision. Parser/runtime/backend/registry implementation is
a later, separately authorized row.

## Non-claims

```text
try/catch semantic resolution
throw activation
postfix catch parser activation
cleanup/fini grammar activation
concurrency feature activation
Stage0/Stage1 behavior change
runtime/backend/LLVM/VM change
normal-entry cutover
JSON/REPL/executor/selfhost/CUT0
```
