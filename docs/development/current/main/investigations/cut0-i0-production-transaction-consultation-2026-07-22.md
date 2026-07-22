# CUT0-I0 Production Transaction Consultation

Status: **Closed — Candidate T-prime-r1 selected**
Date: 2026-07-22
Scope: define one route-neutral owner/commit contract before production CUT0

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md`
- `src/mir/builder/module_invocation_route_matrix.rs`
- `src/mir/builder/route_owned_invocation_inventory.rs`
- `src/mir/compiler/module_session.rs`
- `docs/development/current/main/investigations/cut0-i0-t-prime-r1-execution-task-2026-07-22.md`

## Decision status

`Decision: accepted` — Candidate T-prime-r1 is selected. The original
T-prime text below remains the consultation input; the r1 corrections and
executable row order are authoritative in the linked execution task.

The production census proves that the current raw and canonical routes have
different Builder owners and publication seams. A wrapper around
`MirBuilder::build_module` cannot supply the required all-route transaction.

## Evidence boundary

```text
raw:
  MirBuilder::build_module -> live current_module -> finalize_module

A+/trivial:
  CanonicalModuleLoweringSessionV1 -> candidate Builder -> commit

acyclic/recursive:
  unpublished callable drafts -> candidate atomic batch -> session commit

post-build:
  finish_built_module -> refresh/optimize/verify/RC/canonicalize

disconnected proof:
  ModuleLoweringInvocationCandidateV1 -> shell + collector -> drain
```

The following are not interchangeable proof products:

- `RawExpansionReceiptLedger` is an authority/event ledger; it does not own
  the physical `MirFunction` drafts.
- `ModuleDraftCollectorV1` is the sole physical draft/header owner.
- `CanonicalModuleLoweringSessionV1` is an isolated Builder candidate, not a
  proof that the raw capture/drain lifecycle is wired.
- Returning `MirCompileResult` is an API result boundary, not yet a typed
  external-commit proof.

## Candidate T-prime (consultation input; corrected by r1)

### 1. One outer ingress token

Mint one private `ModuleInvocationIdV1` and one sealed family token only after
source/plan preflight and before Builder effects. The token has five families:

```text
Raw
CanonicalSingle { schedule: APlus | BindingSsaTrivial }
BindingSsaAcyclic
BindingSsaRecursive
```

The existing nine route-matrix rows remain phase-entry witnesses inside the
family token. They are not nine caller-selectable flags or independent
production owners. The invocation ID is embedded in the collector and the
route source proof so a receipt from another invocation fails as
`ForeignInvocation` before either owner is consumed.

### 2. One collector conversion boundary

The collector remains the only physical draft owner. Each route first admits
its actual drafts through the existing typed admission API, then co-seals the
source proof and collector into one opaque product:

```text
Raw ledger events
Canonical single receipt
Callable batch receipt
        |
        v
ModuleDraftCollectionSealV1
        |
        v
CollectedInvocationDraftSetV1
```

The seal validates invocation ID, key, symbol, arity, cardinality, and
publication policy. It never accepts a caller-supplied symbol inventory and
never creates a second catalog. Raw receipts validate ledger history only;
callable batches are collected from their existing unpublished draft owner,
not through `publish_into(&mut MirModule)`.

### 3. Explicit Builder/session boundary

Generalize the existing canonical candidate session into a route-neutral
`ModuleBuilderInvocationSessionV1`:

```text
BuilderInvocationConfigV1 snapshot
-> fresh candidate MirBuilder
-> short phase-scoped &mut candidate borrow
-> invocation shell/collector completion
-> postprocess
-> one commit(&mut live_builder)
```

The config snapshot must explicitly carry the persistent settings currently
missing from `CanonicalModuleLoweringSessionV1`: `repl_mode`, quiet logging,
import aliases, plugin signatures, and source-file hint. The invocation is
never stored in `MirBuilder` or `CompilationContext`; live Builder
snapshot/restore and current-module mirrors remain forbidden.

### 4. Route-owned root policy

Keep `RouteOwnedInvocationInventoryV2` as the sole root-policy authority:

```text
Raw             -> main required, condition_fn required
A+/trivial      -> exact canonical owner, condition_fn forbidden
acyclic/recursive -> exact callable catalog, condition_fn forbidden
```

Production does not use `ConditionFnPolicyV1::Optional`. Carry a typed
`InvocationRootPolicyV2` through drain and finalization. Remove the current
unconditional `main`/`condition_fn` assumptions from canonical verification;
an unexpected canonical `condition_fn` must fail fast rather than be silently
removed or synthesized.

### 5. Typed postprocess and external commit

The one-shot post-drain chain is:

```text
FinalizedModuleCandidateV1
-> PostprocessedModuleCandidateV1
-> ModuleExternalCommitPortV1::commit(...)
```

Refresh, optimizer, verifier, legacy-RC policy, semantic refresh, and
canonicalization run before the commit capability is consumed. Fatal
postprocess errors drop the unpublished candidate and do not retry. The
external commit count is exactly one and must name a concrete owner; a bare
`MirCompileResult` return is not sufficient evidence.

### 6. Failure and poisoning projection

The route matrix remains the single failure-law SSOT:

```text
child failure:
  typed child error, parent restore once, collector prefix unchanged,
  same failed child never retried; outer invocation may continue with a
  fresh sibling child

root/batch/drain/postprocess failure:
  invocation candidate consumed/dropped, publication = 0, retry = 0

identity/history corruption:
  source ledger poison is permanent; later reserve/complete/seal fails
```

Do not collapse child restoration and outer invocation abortion into one global
`poisoned` bit. Preserve primary-plus-cleanup errors and keep the existing
`LedgerPoisoned` invariant boundary.

## Required decisions before implementation (resolved by r1)

The following must be accepted in one SSOT decision record:

1. family-token shape and private invocation-ID ownership;
2. `ModuleDraftCollectionSealV1` source variants and validation rules;
3. explicit Builder config snapshot fields and state-transfer owner;
4. canonical root policy (`condition_fn` forbidden) and `main` verification;
5. postprocess ordering and the exact external-commit owner;
6. child-vs-invocation failure scope, poison, and no-retry behavior.

## Smallest slice after decision lock (superseded by the r1 task order)

Only after the six decisions are accepted:

1. add the route-neutral types and disconnected unit fixtures;
2. add one static guard proving zero production consumers;
3. run the existing S0/P0 and pointer gates;
4. review the API-only slice before any production ingress change.

Production CUT0 activation remains a separate later slice and must still be
atomic across all route families.

## Non-claims

This consultation does not claim production capture, collector wiring,
condition policy activation, postprocess ownership, external commit, or
selfhost compiler progress. It records the minimum design boundary needed to
continue without inventing a second authority or a route-specific fallback.

## T-prime-r1 closeout

The accepted revision adds four corrections to the consultation input:

1. one invocation ID brands token, session, shell, collector, every receipt,
   every candidate, postprocess, and the prepared external commit;
2. callable batches complete one whole-batch preflight before an infallible
   collect-all terminal;
3. raw Main completion and canonical owner/catalog completion use distinct
   typed states and converge only at `CompleteInvocationV1`;
4. any production child failure restores the parent locally and then aborts
   the outer invocation; later sibling descent, fallback, and retry are zero.

T-prime-r1 also preserves existing route semantics for Builder CoreContext
seeding, postprocess order, legacy non-fatal verifier results, canonical final
verification as a commit barrier, and recursive module capability transport.

The design stop is closed. Production activation remains forbidden until the
disconnected identity, collection, session, root, postprocess, and real-route
proof rows in the execution task are all closed.
