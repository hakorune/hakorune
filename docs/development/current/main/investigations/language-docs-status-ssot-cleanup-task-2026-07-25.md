---
Status: Accepted docs-only task order / semantic design stop
Date: 2026-07-25
Decision: DOCS-POINTER-ALIGNMENT0
Scope: make language feature status and authority discoverable without changing parser or runtime behavior
Related:
  - docs/reference/language/README.md
  - docs/reference/language/quick-reference.md
  - docs/reference/language/grammar-contract.md
  - docs/reference/language/stage-profiles.md
  - docs/reference/language/semantic-kernel.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Language docs status / SSOT cleanup task

## Problem

The question “is feature X live, guarded, pending, deferred, rejected, or
legacy?” cannot currently be answered from `docs/reference/language/` alone.
The README is a topic list, `quick-reference.md` omits exception and
concurrency status, and status vocabulary is split across grammar, topic,
profile, and historical documents.

This is a documentation authority defect. The first row is docs-only and must
not change grammar, parser gates, runtime behavior, or backend behavior.

## Confirmed drift

```text
statement try:
  grammar-contract.md = reserved and rejected
  EBNF.md             = default accepted; opt-out no-try-compat
  LANGUAGE_REFERENCE_2025.md = legacy compatibility

postfix catch/cleanup:
  grammar-contract.md = canonical without gate qualification
  EBNF.md             = four Stage-3 gates and experimental/backend caveat
  semantic-kernel.md  = canonical Fault has no catch operation

fini:
  grammar-contract/registry = canonical
  scope-exit-semantics     = legacy alias for canonical cleanup

cleanup:
  scope-exit-semantics = canonical standalone/local cleanup
  EBNF                  = fini/local-fini productions, no matching canonical rows

nowait/await/co, Channel<T>, sync box:
  concurrency docs own guarded/reference behavior
  EBNF/grammar registry have no corresponding Language v1 rows

throw:
  current docs consistently reject it, but quick-reference/status index omit it
```

The ownership note in `quick-reference.md` is the repair template: it names
accepted-but-inactive status and warns readers not to infer parser support.

## Authority and status vocabulary

Keep two axes separate:

```text
grammar status:
  canonical | compatibility_only | reserved | rejected

availability/profile status:
  live | guarded | transport | metadata-only | pending | deferred |
  prohibited | historical
```

Authority order:

```text
grammar-contract.md / registry -> spelling and normalization
topic SSOT                    -> semantic meaning and boundary law
stage-profiles.md             -> Stage0/Stage1 support profile
concurrency manuals           -> concurrency profile/runtime details
historical documents          -> evidence only; never override current SSOT
```

Parser acceptance and examples are evidence, not authority. A status index row
must state both axes when a canonical language shape is guarded or pending in a
profile. `prohibited` is an availability view; it must not erase the registry's
reserved-versus-rejected distinction.

## Phase 0 — `DOCS-POINTER-ALIGNMENT0`

Align the thin current pointers first:

```text
CURRENT_STATE.current_execution_row/current_blocker_token = authority
CURRENT_TASK / 05-Restart / 10-Now = thin mirrors only
workstream old “current executable row” text = historical/closed pointer
  historical normal-file D0 A/B/C choice = superseded by the accepted
  NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR decision; no legacy caller is
  selected and the forge row remains caller=0 until D2
```

Use the existing pointer guard. Do not create a per-row shell guard, move
historical cards, or reopen Script/normal-entry implementation.

## Phase 1 — `LANGUAGE-DOCS-STATUS-SSOT-D0`

Create one docs-local status index for:

```text
throw, try, postfix catch, cleanup, fini / box.fini()
co / nowait / await
Channel<T> / sync box / context / task_scope
worker_scope / parallel / raw thread / lock<T> / worker_local
```

Each row records `feature_id`, both status axes, grammar/semantic/profile
owners, implementation evidence, stable reject/gate tag, historical pointers,
and retirement/promotion condition. Contradictions are recorded as
`status_conflict`; this row does not silently resolve them.

## Phase 2 — `LANGUAGE-DOCS-TRY-CATCH-D1`

Stop before editing contradictory statuses. Produce a design decision for:

```text
statement try in Canonical and Compat2025
postfix catch/cleanup language status versus Stage-3 gates
whether postfix catch handles canonical Fault or another handler route
canonical cleanup versus legacy fini naming
standalone/local cleanup grammar rows
one stable reject/gate vocabulary and migration sunset
```

Until accepted, affected rows remain `status_conflict` and link to this design
stop. No parser/runtime implementation is authorized.

## Phase 3 — entry/navigation repair

Make the first two language entry points answer “where is the status?”:

```text
README -> status index -> grammar contract -> topic semantics -> profile manual
quick-reference -> compact status/gates block -> exact links
```

Convert the README's bare paths to real Markdown links. Normalize
quick-reference topic pointers. The status block must expose throw prohibition,
try conflict, postfix handler owner, concurrency profile ownership, and
scaffold/deferred boundaries. Do not duplicate grammar or semantic policy.

## Phase 4 — concurrency and lifecycle alignment

After the status decision, align quick-reference, stage-profiles, concurrency
manuals, scope-exit semantics, and lifecycle for:

```text
co / nowait / await / Channel / sync box / context / task_scope
cleanup / fini { ... } / local ... fini / postfix cleanup / box.fini()
throw inside cleanup/fini
```

Keep scope cleanup, object finalization, canonical Fault, and process-result
projection as separate owners. No feature activation is implied.

## Phase 5 — historical quarantine

Keep `LANGUAGE_REFERENCE_2025.md` as a historical snapshot. Add a visible
pointer near its exception/concurrency tables to the current status index and
mark noncanonical examples as historical/compatibility evidence. Do not delete
or move it in this task.

## Reusable guard and acceptance

Prefer one reusable docs/status guard. It must verify:

```text
one status-index row per affected feature
both status axes present on every row
current entries link to grammar/topic/profile authority
historical documents are labelled historical
throw prohibition and stable reject tag are discoverable
try/catch/fini conflicts link to the design stop until resolved
README exposes the index; quick-reference contains the status block
no src/, lang/, Cargo, parser, runtime, backend, or grammar behavior changes
```

Run:

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
git diff --name-only -- '*.rs' '*.hako' '*.toml' '*.json'
```

The last command must show no implementation changes for this docs-only row.

## Workstream alignment

The workstream contains older function-exit queue text that can look like a
current executable row. Thin mirrors must point to `CURRENT_STATE`; closed
queue entries remain historical/conditional. Do not reopen Script or
normal-entry implementation from embedded history.

## Non-claims

```text
try/catch semantic resolution
throw activation
postfix catch parser activation
concurrency feature activation
Stage0/Stage1 behavior change
runtime/backend/LLVM/VM change
grammar registry mutation
historical document deletion
normal-entry cutover
```

## First executable row

```text
DOCS-POINTER-ALIGNMENT0
```

The next row after pointer alignment is the status inventory and then the
explicit try/catch design stop, not a code patch.
