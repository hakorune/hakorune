---
Status: Accepted decision
Date: 2026-07-21
Scope: `FINALIZE0-FACTSESSION0-I0` production ownership selection.
Related:
  - docs/development/current/main/investigations/mirbuilder-finalize0-p0-production-census-task-2026-07-20.md
  - src/mir/builder/fact_session.rs
  - src/mir/builder/function_lowering_state.rs
  - src/mir/builder/function_state_transaction.rs
  - src/mir/builder/calls/function_session.rs
  - src/mir/builder/module_lifecycle.rs
Decision: Candidate A-prime accepted; `FINALIZE0-MODULEDRAFT0-S0` is next.
---

# FACTSESSION0-I0: physical owner and draft-collection boundary

## Accepted decision — Candidate A-prime

`MODULEDRAFT0` is the mandatory prerequisite to FACTSESSION0. It creates one
unpublished-draft collector before any production fact-session connection;
`FACTSESSION0-ACTIVEBIND0` then binds generation and an open receipt ledger to
the existing `FunctionLoweringStateV1` fact lanes; only `FACTSESSION0-I0`
seals those lanes inseparably with the already-unpublished draft.

```text
MODULEDRAFT0
  -> FACTSESSION0-ACTIVEBIND0
  -> FACTSESSION0-I0
```

The first code-facing row is `FINALIZE0-MODULEDRAFT0-S0`. It introduces only
private disconnected vocabulary: `UnpublishedFunctionDraftV1`,
`PreparedFunctionDraftAdmissionV1`, `ModuleDraftCollectorV1`, and
`CompletedDraftSignatureViewV1`. It has zero production consumers, no draft
clone, no fact-session connection, no PHI repair, and no finalization change.

## Question

Which production boundary may first make Candidate F-prime live without
breaking its paired-draft law?

```text
successful physical function
  -> one function-generation-branded active fact session
  -> CompletedFunctionDraftWithFactsV1
  -> one module collector
  -> later module-completion candidate
```

The answer must select an ordering that preserves this pairing. A facts-only
success collection or a single-root-only cutover is not admissible.

## Source evidence

The disconnected vocabulary is correct but has no production attachment:

```text
OpenFunctionFactSessionV1
  owns six TypeContext lanes + two diagnostic-origin lanes + open receipt ledger

CompletedFunctionDraftWithFactsV1
  owns draft + sealed facts inseparably

ModuleFactSessionV1
  is the only completed-draft collector
```

The real builder currently keeps the eight lanes in `FunctionLoweringStateV1`:

```text
type_ctx
value_origins
```

There are 198 direct `function_state.type_ctx` uses. Replacing those maps with
a second session-owned map would create duplicate truth. The legacy nested
transaction moves TypeContext only for `LegacyRestore`; its
`BoxCompilationPartialClear` path clears only three lanes and does not capture
diagnostic origins. It therefore cannot be renamed into the F-prime owner.

The current physical completion order creates a harder contradiction:

```text
child session success
  -> publishes bare MirFunction into current_module

root finalization
  -> takes current main MirFunction
  -> returns MirModule
```

F-prime instead requires a successful function to yield
`CompletedFunctionDraftWithFactsV1`, which only `ModuleFactSessionV1` may
consume. No current production module-draft collector owns that product.

## Why the current order cannot directly implement I0

The documented order is:

```text
FACTSESSION0-I0
  -> REMATFACT0-P0 / producer receipt closures
  -> MODULETX0-P0
```

But `FACTSESSION0-I0` cannot seal and collect a successful function without
withholding its draft from the current module. That withholding is already a
module-draft transaction responsibility. Conversely, a facts-only collection
would let draft and facts move independently, contradicting Candidate F-prime.

The following shortcuts are rejected:

```text
collect facts and discard them after ordinary module insertion
  // breaks draft/facts inseparability

attach a second TypeContext to OpenFunctionFactSessionV1
  // duplicates the 198-use live builder truth

connect only build_module() or one canonical ingress
  // leaves the five root and four child physical entry families divergent

store ModuleFactSessionV1 in MirBuilder or CompilationContext
  // violates explicit lowering-session input ownership

let an old child transaction retain its BoxCompilation partial-clear behavior
  // does not transport all eight lanes
```

## Candidate A-prime — accepted draft-collection prerequisite

Introduce a narrow prerequisite boundary before FACTSESSION0-I0:

```text
FINALIZE0-MODULEDRAFT0-SELECT
  select one complete unpublished-draft collection boundary

-> FINALIZE0-MODULEDRAFT0-S0/P0/I0/G0
   every root and child completion returns an unpublished draft product;
   module aggregation is the sole consumer

-> FINALIZE0-FACTSESSION0-I0
   bind each active FunctionLoweringStateV1 fact lane set to a fresh
   generation and seal it atomically with the already-unpublished draft
```

`MODULEDRAFT0` is not PHI repair, metadata publication, derived refresh, or
the later M-prime module-completion transaction. Its only purpose is to make
the existing physical function-completion boundary able to own a complete
draft until one module collector accepts it.

The physical active owner then has this form:

```text
FunctionLoweringStateV1
  type_ctx + value_origins       // existing one live map set; no copy
  + ActiveFunctionFactBindingV1  // generation + open receipt ledger only

module-session argument
  opens/binds active function
  takes the same lanes at close
  seals them with the unpublished draft
  collects before parent restore
```

The module session remains an explicit argument at all five root and four
child entry families. Its issuer may be compiler-lifetime so compiler reuse
receives a distinct module brand, but the opened `ModuleFactSessionV1` is never
a `MirBuilder`, `CompilationContext`, metadata, or `TypeContext` field.

## Fixed acceptance law

The following conditions are mandatory for the series:

1. Each successful production function keeps its draft and sealed eight fact
   lanes inseparable until exactly one module collector consumes them.
2. All five root and four child production entries use the same module-session
   port; `condition_fn` receives a real empty disposition or rejects before
   effects.
3. The active facts remain the existing `FunctionLoweringStateV1` lanes while
   lowering; no parallel TypeContext is introduced.
4. Child success collects before parent restore; primary error, cleanup error,
   and unwind abort before restore; root failure discards the whole module
   session.
5. No receipt issuer, PHI repair, unused-Phi deletion, type pipeline,
   finalization repair, JoinIR conversion, or CUT0 is included.

`MODULEDRAFT0-M0` must census every `current_module.functions` lowering-time
reader before I0. A signature/header-only reader may use a read-only view of
the same collector-owned draft; any body/metadata reader stops the series.
Legacy replacement and canonical duplicate rejection are distinct prepared
admission policies. `FunctionFactGenerationV1` is never a draft identity.

Child success validates cleanup and prepares every collector failure point
before collection, then performs infallible collect-before-restore. Primary,
cleanup, publication, and unwind paths abort before restore. Main and the
synthetic `condition_fn` use the same collector port; the latter receives an
empty fact disposition until `FINALIZE0-CONDITIONFN-RET0`.

The fixed task order is:

```text
MODULEDRAFT0-S0 -> MODULEDRAFT0-M0 -> MODULEDRAFT0-P0
  -> MODULEDRAFT0-I0 -> MODULEDRAFT0-G0
  -> FACTSESSION0-ACTIVEBIND0-S0/P0
  -> FACTSESSION0-I0 -> FACTSESSION0-G0
  -> REMATFACT0-P0 / producer receipt closures -> MODULETX0-P0
```
