---
Status: Selected design; normal typed corridor revalidation pending
Date: 2026-08-20
Decision: MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: existing selected Dynamic LLVM boundary only
ReplacementCell: observation/structural guard; no code replacement
---

# MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0

## Six-line brief

Decision: Revalidate one normal typed native corridor after the R7 census. The
existing Dynamic-only guard is retained as a dependency; this row must prove
the Static/Free/Print published path without changing Call representation or
callers.

Source authority + canonical issuer: the existing late callsite canonicalizer
and selected Dynamic `reject_selected_dynamic_legacy_callsites` boundary are
the authority; the new guard only observes their source wiring and delegates
reject semantics to `legacy_callsite_reject_code`.

Non-authority: JSON-v0/VM compatibility rows, test fixtures, comments,
`ValueId::INVALID`, `func` text, backend output, and source hit counts cannot
prove selected-corridor canonicality.

Fail-fast boundary: after final post-RC canonicalization and strict
verification, before the published callback/C static V2 artifact boundary, the
guard must reject selected `LegacyCallV0`, targetless calls, and an unconsumed
typed row. A missing boundary is a guard failure, never a compatibility
default.

Smallest next slice: use the R7 manifest as observation input and design one
narrow structural check for the normal Static/Free/Print callback through C
static V2. Do not modify MIR, loader, optimizer, backend, printer, JSON-v0,
or compatibility emitters.

Non-claims: no `Option<Callee>` deletion, no `LegacyCall`, no native producer
rewrite, no JSON-v0 retirement, no Script transport or production cutover,
no optimizer/backend semantic change, and no performance claim.

## Guard contract

The guard must verify all of the following for the selected normal corridor:

```text
final post-RC callsite canonicalization is present
selected normal callback verifies the module before backend execution
selected normal path consumes only typed Callee rows
LegacyCallV0 / targetless call is rejected before artifact creation
JSON-v0 canonicalization remains a separate compatibility schedule
Dynamic V4 and compatibility projections are outside this corridor
changed source/check files remain below the 760/800 line limits
```

The guard must not claim that all `callee: None` source mentions disappeared:
the D0 census records 3 compatibility producers and 16 test fixtures. It only
closes the selected-corridor boundary. A later retirement row still needs a
runtime/module census and a caller-zero proof for every other family.

## 2026-09-11 normal-corridor revalidation design

The selected corridor is the ordinary typed Static/Free/Print path:

```text
NormalDefaultPublishedPipelineV1
  -> finish_built_module -> optimizer / verifier / RC / metadata refresh
  -> callsite canonicalization -> published callback
  -> PublishedMirBackendView -> C static V2 consumer -> object / EXE link
```

`MirInstruction::Call(MirCall)` and `MirCall::new` remain the canonical
issuers. `LegacyCallV0.func`, names, VM tail lookup, JSON-v0, llvmlite, and C
whitelists are non-authorities for this corridor. The R7 manifest is
observation input only and is not re-scanned by this row.

The current Dynamic-only `reject_selected_dynamic_legacy_callsites` guard does
not prove normal Static/Free/Print typed-row consumption. In particular,
`PublishedMirBackendView::try_new` can still observe a `LegacyCallV0` with a
typed global callee and `func == ValueId::INVALID`; the normal path needs a
named pre-artifact reject or an equivalent typed-row consumption assertion.
This is a guard design finding, not evidence of an observed unsafe execution.

### Revalidation acceptance and stop conditions

Acceptance must cover final post-RC canonicalization, selected normal callback
verification, typed Callee-only consumption, pre-artifact rejection of
`LegacyCallV0`/targetless calls, and separation from Dynamic V4 and
compatibility projections. No MIR/loader/optimizer/backend/printer behavior
change is allowed. Return to `NoSafeSlice` if the normal callback cannot be
assigned a single owner, `func` and typed Callee remain co-authorities, C
re-resolves a target by name, or the guard needs production behavior changes.

The smallest implementation is one narrow observation/structural extension
to the existing guard, reusing the R7 manifest and existing owner anchors. It
must not add a second guard family or a new semantic receipt.

## Historical Dynamic guard evidence

Reuse the existing selected Dynamic tests in `src/runner/product/llvm/mod.rs`:

- a missing-callee call is rejected with `call-missing-callee`;
- an empty canonical module is accepted by the scanner;
- compatibility and JSON-v0 rows are not routed through this guard.

If the guard cannot distinguish the selected production branch from the
compatibility `ny_llvmc_emit_*` projections, stop and return `NoSafeSlice`
instead of broadening the match or inferring a route from names.

## Historical closeout evidence

```text
guard: bash tools/checks/mir_call_canonical_corridor_guard.sh (pass)
focused tests: CARGO_BUILD_JOBS=4 cargo test --profile quick --lib selected_legacy_callsite_scan (2 passed)
source changes: none; Call/JSON-v0/compatibility/backend routes unchanged
line limits: guard and touched documentation remain below 760/800
```

The selected corridor is now mechanically pinned, but this does not retire
the 25 legacy-target mentions from the D0 census. JSON-v0/VM compatibility,
raw/compatibility lanes, canonical Script transport, and the `Option<Callee>`
retirement remain separate prerequisites. No next execution row is opened by
this guard-only closeout.
