---
Status: active__design_stop__2026-09-22__StringPhysicalContract
Task: MIR-CALL-PARSER-RECURSIVE-STRING-RESULT-AUTHORITY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
NextCard: same-card__string-contract-decision
Implementation permission: false; task planning complete, physical contract decision next
---

# Parser String result authority: corrected decision and task queue

## Current Capsule

- **Current decision:** extend the existing result/publication chain; normal-return representation does not require termination proof.
- **Current implementation status:** source receiver proof exists; callable result dispositions and publisher currently support I64/nominal Box only. String physical contract is open.
- **Next ordered task:** settle the String carrier, ownership, Fault and publication contract in the existing owners.
- **Production stop line:** preserve TargetOnly until source result and physical consumption are both justified; no partial package coverage.
- **Retirement finish line:** canonical merged parser acceptance, selected caller cutover, then verified exclusive old-edge removal.

## Six-line brief

```text
Decision: reopen the internal String result design gap; remove termination as a prerequisite.
Source authority + canonical issuer: VerifiedSameModuleCallableResultCatalogV1 and existing VerifiedStaticCallResultPublicationOwnerV1; reuse source-bound ExactStringOnSuccess evidence.
Non-authority: helper names, MIR type inference, VM/Compatibility probes, a second solver, or a default String disposition.
Fail-fast boundary: exact source membership and complete call coverage before selection; only a successful physical Call receipt permits publication.
Smallest next slice: decide the String representation/carrier/ownership/Fault contract through PreparedStaticCallResultPublicationV1.
Non-claims: no String implementation, canonical acceptance, caller switch or legacy deletion in this task-planning change.
```

## Premise correction and evidence boundary

User supplied a Pro audit based on `3038bfb1`, reporting unpushed commit
`1a67ee4532b33d05b6a7eef5bf4ec825c9851163`. That object/patch is not available
in this checkout. Its reported guard/mutation results are external evidence,
not locally executed receipts or an imported commit.

Local readback confirms `src/mir/source_core_receiver/README.md` explicitly
excludes success, termination and purity from `ExactStringOnSuccess`.
`callable_result_representation/README.md` likewise defines a representation-only
proof. The former requirement for a recursive measure/termination proof was
incorrect and is superseded. Absence of a physical String contract remains a
real internal design task, not an external wait.

| Entry | Current boundary | Consequence |
| --- | --- | --- |
| StringBox direct probe / VM import | Compatibility, no source publication owner; `legacy-fallback-retired` | Dependency evidence only; changing parser selection does not establish source-backed admission. |
| Canonical merged parser | `TargetOnly/RecursiveDependency` for earlier dependencies | Loop/source handoff exists; selected `parse/2 -> starts_with/3` is `ExactI64`, required ordinals `[]`. |

Prior local tests recorded in I3 prove the selected tuple and focused consumer,
not completion of the full merged parser. No compiler probe was rerun here.

## Finite scope and existing owners

This census covers: source declarations of `ParserStringUtilsBox.i2s/1` and
`StringHelpers.int_to_str/1` -> existing result catalog -> publication demand ->
physical Call/result publication. Includes their source call sites, String
literal/left-String Add, substring, local and loop-merge result propagation as
required by the unchanged bodies. Excludes general recursive type inference,
mutual-recursion promotion, VM repair, all-String/backend support, and Map
OwnedText T3 promotion. Inventory rows identify source witnesses, not name-based
acceptance policy; exact target/site/catalog brands remain authoritative.

| Responsibility | Existing owner / decision needed |
| --- | --- |
| Normal-return String fact | `src/mir/source_core_receiver/`; reuse exact source proof, preserve right operand evaluation. |
| Result propagation and call coverage | `src/mir/callable_result_representation/` (`expression_proof`, `call_proof`, `function_proof`, `solver`, `disposition`); separate result dependency from nested-call traversal. |
| Exact one-shot handoff | existing `VerifiedStaticCallResultPublicationOwnerV1`; retain source identity, duplicate/residual rejection. |
| Physical result commit | `src/mir/builder/calls/static_result_publication.rs` and `static_result_publication_physical_bridge.rs`; choose actual carrier/return convention and lifetime, not just a MirType annotation. |

## Ordered tasks and completion criteria

| Order | Task | Required completion evidence |
| --- | --- | --- |
| 0 | Restore stale publication ingress guard | Reconcile current four states and split consumer paths; both consumers reject TargetOnly; original red and focused negative mutations retained. No compiler semantics change. |
| 1 — selected | String physical contract decision | Name normal-result representation, carrier/return convention, ownership transfer/cleanup, Fault behavior and sole publisher for both helpers. Reuse existing owners; record decision in owning reference and README before implementation. Identify exact caller/old-edge delete tuple. |
| 2 | Extend existing result proof and publisher together | Propagate only justified String facts through the required source forms; preserve child traversal and final stable call-row sealing. Connect the same demand to physical publication; do not expose a selected representation without a consumer. |
| 3 | Focused source-to-physical validation | Both unchanged helpers, literal/local/substring/loop merge, mixed result rejection, wrong catalog/site/target, duplicate take and residual checks. Prove recursive RHS evaluation and Fault propagation; no early return that drops nested calls. Verify returned String content and lifetime at the selected physical boundary. |
| 4 | Canonical merged parser acceptance (I3 T4c) | Use pinned current binary and source-backed MIR entry; reach real `parse/2 -> starts_with/3`, consume once and finish residual-free. Classify any next terminal without claiming full acceptance. |
| 5 | Selected production caller cutover (I3 T5) | Confirm which caller already uses the source owner; switch only a remaining old edge and prove selected production behavior. Existing Selected handoff alone is not cutover evidence. |
| 6 | Exclusive old-edge retirement (R0) | Enumerate remaining callers, prove zero for the selected edge, physically delete its exclusive code/tests/guards, keep re-entry protection. Shared LegacyCallV0 deletion is not implied. |

Task 0 is known guard debt and may be closed independently; task 1 owns the
semantic decision. Tasks 2–6 are queued, not implementation permission from
this planning card. The accepted decision must fix the mapping before changing
`work_mode` to fast. No new task file is needed for each step.

The read-only worker confirmed that `call_proof::prove_core_string_method`
currently classifies generated `StringValue` results as `KnownNonI64`; this is
the existing substring propagation boundary. Source receiver proof covers only
literal/left-Add syntax; locals and loop results must use the existing environment
and merge owners. Check normal and external destinations at publication.

The focused negative inventory includes mixed String/I64 returns, non-String
loop merges, unknown substring receiver, right-String-only Add, unproved
`return self_call(...)`, foreign catalog/target, and unconsumed call rows.
Audit every affected operator arm: unary Minus currently forwards its operand
fact, so adding String must not make `-String` acquire a String result proof.
For runtime positives, verify both helpers at zero, positive and negative inputs
and compare returned content, not merely successful compilation.

## Execution boundary for the first semantic slice

**Change:** issue justified String normal-result evidence through the existing
catalog and publisher once task 1 closes; replace only the selected unsupported
String disposition, preserving rejection for unproved cycles.

**Contract:** representation proof does not prove totality, purity or absence of
Fault. `"-" + self_call(...)` may have known String normal-result representation
while the RHS must still be evaluated. Required String merges must agree across
all normal returns. String semantic representation is not automatically an
OwnedText ABI. Missing/foreign evidence remains a named rejection.

**Done:** focused positive/negative source-to-physical evidence, complete source
call coverage, one-shot/residual invariants, reusable ingress guard, and owner
README/reference in the same implementation slice. Canonical parser acceptance
and retirement remain separately observable tasks 4–6.

**Stop:** unknown carrier/ownership/cleanup, new competing authority, lost nested
call rows, altered evaluation order, unsupported merge, or a second physical
publisher returns the slice to this design decision. An internal missing owner
must be designed here rather than relabeled as external waiting.

## Local planning checks — 2026-09-22

At clean `3038bfb1e6`, `script_static_result_publication_ingress_guard.sh`
returns exit 1: it still requires deleted `Absent` and pre-split consumer paths.
This is known baseline guard debt, not a failure introduced by these docs.
Task 0 owns restoration; the Pro report is not treated as a local fix.
No Cargo, runtime probe or CI dispatch is required by this planning-only change.
Windows lifecycle remains user-deferred; warning cleanup remains paused at I147.
