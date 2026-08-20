---
Status: Active fast row
Date: 2026-08-20
Decision: MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: existing selected Dynamic LLVM boundary only
ReplacementCell: observation/structural guard; no code replacement
---

# MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0

## Six-line brief

Decision: Add one structural guard for the already-selected native/canonical
MIR corridor. Prove the existing final canonicalization/rejection boundary
without changing the Call representation or any caller.

Source authority + canonical issuer: the existing late callsite canonicalizer
and selected Dynamic `reject_selected_dynamic_legacy_callsites` boundary are
the authority; the new guard only observes their source wiring and delegates
reject semantics to `legacy_callsite_reject_code`.

Non-authority: JSON-v0/VM compatibility rows, test fixtures, comments,
`ValueId::INVALID`, `func` text, backend output, and source hit counts cannot
prove selected-corridor canonicality.

Fail-fast boundary: before selected Dynamic backend execution, the guard must
find the late canonicalization schedule, selected-module verification, and
stable `call-missing-callee` rejection. Missing boundary, selected fallback,
or a `callee: None` constructor in the selected production branch is a guard
failure, never a compatibility default.

Smallest next slice: add the reusable guard and focused source/test evidence;
leave JSON-v0, compatibility emitters, `project_module_to_legacy_calls`,
`MirInstruction::Call`, and backend code unchanged.

Non-claims: no `Option<Callee>` deletion, no `LegacyCall`, no native producer
rewrite, no JSON-v0 retirement, no Script transport or production cutover,
no optimizer/backend semantic change, and no performance claim.

## Guard contract

The guard must verify all of the following:

```text
late callsite schedule = MirOptimizerLateCallAndInline
selected Dynamic path verifies the module before backend execution
selected Dynamic calls legacy_callsite_reject_code before execution
call-missing-callee is the stable reject code
JSON-v0 canonicalization remains an explicit separate schedule
compatibility projection is not counted as selected Dynamic authority
changed source/check files remain below the 760/800 line limits
```

The guard must not claim that all `callee: None` source mentions disappeared:
the D0 census records 3 compatibility producers and 16 test fixtures. It only
closes the selected-corridor boundary. A later retirement row still needs a
runtime/module census and a caller-zero proof for every other family.

## Focused evidence

Reuse the existing selected Dynamic tests in `src/runner/product/llvm/mod.rs`:

- a missing-callee call is rejected with `call-missing-callee`;
- an empty canonical module is accepted by the scanner;
- compatibility and JSON-v0 rows are not routed through this guard.

If the guard cannot distinguish the selected production branch from the
compatibility `ny_llvmc_emit_*` projections, stop and return `NoSafeSlice`
instead of broadening the match or inferring a route from names.
