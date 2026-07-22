# HDR0-P0 Execution Task

Status: **Active — decisions accepted; authority-erasure slice next**
Date: 2026-07-22
Scope: complete HeaderPort reader replacement and prepare one atomic all-route CUT0

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-hdr0-p0-open-questions-2026-07-22.md`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md`
- `tools/checks/lib/headerport_header_reader_census.py`
- `tools/checks/lib/headerport_authority_erasure_guard.py`

## Accepted decisions

| Question | Decision | Durable law |
|---|---|---|
| Q1 method-index freshness | Pure projection | An explicit header loan projects sorted candidates directly and never reads or updates the ambient legacy cache. |
| Q2 static tail resolver | Thread one short HeaderPort loan | Lower arguments first, then retain one explicit authority through resolve, emit, and annotation. |
| Q3 materializer presence | Explicit split | Invocation header presence and legacy compatibility presence are mutually exclusive modes. |
| Q4 lifecycle activation | Atomic all-route CUT0 | A sealed adapter may be built disconnected, but production activation occurs once at the outer module entry for every route. |

These decisions add no production capture, drain, finalizer, or external-commit
consumer by themselves.

## Central invariant

Once a call-lowering route selects an explicit HeaderPort authority, that
authority must remain present through selection, emission, and annotation.
The route must not erase it by entering a lookup-less legacy facade.

```text
lower arguments completely
-> collect nested declarations
-> borrow one short HeaderPort
-> resolve
-> emit with the same lookup authority
-> annotate with the same lookup authority
-> end loan
```

The explicit route must not reach:

```text
emit_legacy_call
lookup-less emit_unified_call
annotate_call_result_from_func_name
legacy method_candidates
current_module function-map readers
```

## Implementation queue

### HDR0-P0-AUTHORITY-ERASURE0 — next

Close exactly the three known authority-loss sites:

1. explicit tail resolution selects and emits through the same lookup-aware
   route;
2. unique static recovery emits through the same lookup-aware route;
3. canonical materializer recovery annotates through the selected presence
   authority.

Use an exclusive presence type or two named public facades with one private
exclusive core. The current `Option<&lookup> + legacy_presence: bool` policy
surface must not remain able to express contradictory modes.

Acceptance:

- unique, ambiguous, and missing tail fixtures under the existing environment
  gate;
- explicit misses do not read the ambient module and do not change
  instruction or ValueId state;
- unique static recovery and materializer annotation preserve lookup
  authority to completion;
- a static guard rejects any explicit-authority path that reaches the five
  forbidden legacy reader/facade families above;
- `python3 tools/checks/lib/headerport_authority_erasure_guard.py` is green;
- production invocation capture/commit and CUT0 consumers remain zero.

This is a BoxShape slice. It must not add a new accepted call shape, resolver,
cache, environment variable, or production route.

### HDR0-P0-METHODTAIL-COMPAT0

Make the selected Q1 boundary executable:

- `Some(headers)` always uses deterministic pure projection and bypasses the
  legacy cache;
- `rebuild_method_tail_index_with_headers` remains disconnected or is removed;
- `prepare_module` clears the legacy cache and resets its source-length
  witness;
- legacy rebuild sorts source symbols and every candidate list.

Acceptance fixtures:

- same-size collector A to collector B exposes only B candidates;
- different insertion order produces the same projection order;
- fresh legacy snapshot and explicit headers agree for unique, ambiguous, and
  missing results.

### HDR0-P0-CALLER-CENSUS0

Re-run the full reader/caller census after the two code slices. Every
materializer mode and method-tail caller must have one named owner. Located
legacy observation remains diagnostic/disconnected unless new evidence
selects a separate task.

### HDR0-G0

Close only when:

```text
unclassified production function-map readers = 0
explicit HeaderPort -> legacy authority erasure = 0
second invocation header cache = 0
production capture/commit consumers = 0
```

## Atomic lifecycle queue

The production policy is fixed now, but CUT0 is not yet executable. After
HDR0-G0 and the existing bounded compatibility-policy consultation, use the
following queue.

### CUT0-S0 — disconnected linear owner

Build one sealed, production-disconnected ownership chain:

```text
ModuleLoweringInvocationCandidateV1
-> active lowering
-> MainPendingInvocationV1
-> MainCapturedInvocationV1
-> CompleteInvocationV1
-> PreparedInvocationDrainV1
-> DrainedModuleCandidateV1
-> DrainedModuleFinalizationInputV1
-> FinalizedModuleCandidateV1
-> external commit capability
```

Required structural closures:

- add a successful candidate handoff;
- make root completion a real typed transition;
- drain the exact same invocation state without reconstructing
  `shell + collector`;
- implement `finalize_drained_module_once` as Builder-free, collector-free,
  HeaderPort-free, and retry-free;
- do not pass a bare `MirModule` across the post-drain boundary.

All production invocation capture, drain, finalizer, and external-commit
consumers remain zero through CUT0-S0.

### CUT0-P0 — disconnected all-route proof

All nine route rows must execute the same sealed outer adapter. Prove success,
primary error, cleanup error, admission error, root error, drain error,
finalizer error, and panic. A passive row declaration alone is insufficient.

### CUT0-I0 — one production switch

Replace the outermost module entry once for raw, A+/trivial,
acyclic/recursive, Main/condition_fn, drain, finalization, and external commit.
Route-specific activation flags, partial production wiring, fallback, and
retry are forbidden.

Acceptance:

```text
outer invocation owner = 1
collector              = 1
drain consumer         = 1
finalizer consumer     = 1
external commit        = 1

direct pre-drain insertion       = 0
restore-then-publish             = 0
explicit header -> legacy retry  = 0
post-drain current_module read   = 0
failure retry                    = 0
```

### CUT0-G0 — retire old owners

Remove old closure terminals, restore-then-publish, direct module insertion,
direct callable publication, and current-module header fallback after their
consumer counts are zero.

## Existing consultation retained

`CUT0-COMPAT-POLICY-CONSULT0` remains after HDR0-G0 and before CUT0-S0. It
decides duplicate Main source behavior and optional callable-Main failure
propagation. Q4 decides activation topology, not those remaining semantic
policies.

## Non-claims

This card does not claim:

- that CUT0 gates are currently met;
- production HeaderPort capture/commit or lifecycle activation;
- completed root-state transitions, success handoff, or post-drain finalizer;
- retirement of all `current_module` readers;
- FACTSESSION0, finalization-repair removal, FastMem, LLVM, JoinIR retirement,
  or selfhost migration progress.

After CUT0-G0 the fixed queue resumes at FACTSESSION0. FastMem remains parked
until `MODULE-FINALIZE-VERIFY-CUT0` as recorded by the current state.

## HDR0-P0-AUTHORITY-ERASURE0 closeout

The first code-facing slice is closed on 2026-07-22. The explicit call route
now borrows headers only after argument descent, passes the same lookup through
unique static and deterministic tail recovery, keeps materializer presence in
an exclusive invocation/legacy enum, and carries the lookup into the
post-success annotation receipt. The legacy port still returns `None` and
retains its compatibility annotation path.

Evidence:

```text
python3 tools/checks/lib/headerport_authority_erasure_guard.py = green
python3 tools/checks/lib/headerport_candidate0_guard.py . = green
cargo check -q = green
cargo test -q explicit_header_authority_survives_unified_call_post_success --lib = green
cargo test -q explicit_projection_is_sorted_and_ignores_non_methods --lib = green
cargo test -q raw_invocation --lib = green (16 tests)
```

Production invocation capture/commit, lifecycle drain, external commit, and
CUT0 remain zero. The next code-facing row is
`HDR0-P0-METHODTAIL-COMPAT0`.
