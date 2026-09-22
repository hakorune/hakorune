---
Status: accepted__design_closeout__2026-09-22__SourceToMirString
Task: MIR-CALL-PARSER-RECURSIVE-STRING-RESULT-AUTHORITY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
NextCard: MIR-CALL-PARSER-STRING-RESULT-S1 (same card)
Implementation permission: S1 mapping accepted below; this user-requested turn is design/docs only
---

# Parser String result authority: corrected decision and task queue

## Current Capsule

- **Current decision:** extend the existing normal-result catalog and sole MIR publisher with `ExactString`; the bounded physical carrier is a Call destination `ValueId` typed `MirType::String`.
- **Current implementation status:** design accepted, not implemented. Runtime String return ownership is not implied by a MIR emission receipt.
- **Next ordered task:** restore the stale ingress guard, then implement S1 source-to-MIR String publication in the existing owners.
- **Production stop line:** S1 does not activate a String runtime ABI or relax I64-only Loop consumers. Executable acceptance and runtime admission remain required before cutover claims.
- **Retirement finish line:** selected parser acceptance/cutover and exclusive old-edge removal; catalog propagation alone is not legacy retirement.

## Six-line brief

```text
Decision: accept S1 String normal-result propagation through the existing source-to-MIR publication chain.
Source authority + canonical issuer: VerifiedSameModuleCallableResultCatalogV1 -> VerifiedStaticCallResultPublicationOwnerV1 -> PreparedStaticCallResultPublicationV1.
Non-authority: method names, MIR inference, Compatibility probes, new recursion solver, or physical i64 interpreted as semantic String.
Fail-fast boundary: exact source coverage before handoff; MIR emission failure produces no publication; unsupported runtime ABI remains a backend rejection.
Smallest next slice: ExactString in existing facts/dispositions/call rows and sole destination publisher, preserving RHS evaluation and stable call coverage.
Non-claims: no runtime success/termination/purity, String ABI activation, merged-parser completion, production cutover or physical legacy deletion from S1 alone.
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
incorrect and is superseded. The physical contract is now split at its actual boundary: S1 publishes MIR
representation; the later executable row must co-seal runtime ABI and ownership.

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
| Physical result commit | `src/mir/builder/calls/static_result_publication.rs` and `static_result_publication_physical_bridge.rs`; S1 selects the MIR destination/type mapping below; executable return/lifetime remains D2/I2. |

## Accepted mapping: source -> MIR, not runtime completion

Decision (2026-09-22): reuse the existing monotone solver and general-result
publication. Add one exact String normal-result alternative to the existing
fact/outcome/disposition/representation family; no parallel solver, registry,
semantic receipt family, or helper-name special case is introduced.

| Layer | Accepted mapping / owner |
| --- | --- |
| Source membership | Same branded declarations, target catalog and canonical SourceExprSiteV1. Helpers are witnesses, never name-based selection rules. |
| Result proof | Existing expression/function proof and worklist issue `ExactString` when every normal value return agrees. No termination/purity claim. |
| Portable handoff | Existing general call row, demand and one-shot publication owner carry String without AST, ValueId or a physical ABI guess. |
| Physical Call | `static_result_publication_physical_bridge` preserves ordered argument descent, arity checks and the existing generic unified Call terminal. |
| Result publication | `PreparedStaticCallResultPublicationV1` maps exact String to `MirType::String` at the emitted destination; ordinary and external destinations use the same mapping. |
| Runtime | Separate admission/exit/ownership owner. No new retain/release, pointer, tag, OwnedText carrier or C ABI is minted by S1. |

`CompletedUnifiedValueCallEmissionV1` is constructed after
`builder.emit_instruction` in `calls/unified_emitter/physical_terminal.rs`.
It proves compiler emission, not execution of the callee. A lowering error
propagates without result publication and disposes the failed compilation
through the existing caller. At runtime, ordinary evaluation/Fault behavior
must remain intact; S1 cannot convert a faulting/diverging RHS into a value.

The existing general publisher is extended in place. The I64-only activation
and Loop source requirement owners keep their present admission boundaries;
do not broaden `has_exact_i64_result` just because the catalog can describe
String. A newly encountered unsupported consumer is reported at its exact
boundary, not silently projected to I64 or counted as parser completion.

## Finite proof transfer decisions

| Source/result shape | S1 decision |
| --- | --- |
| String literal | ExactString. |
| Add with proved String left operand | Visit both children first; result is String on normal return, independently of RHS result dependency. Keep RHS call sites and all errors. |
| Subtract/multiply/divide/modulo/unary minus | Numeric-only result proof; never pass a String fact through unary minus. |
| Exact generated String Core row | Known String receiver plus generated result-kind/arity row supplies String; Bool/NoValue/Dynamic stay distinct. |
| Local/assignment | Existing environment transports String; receiver evidence is a derived projection of this same result, not a second independent classification. |
| Branch/loop merge and returns | String with String merges; mixed String/I64/Box or unknown incoming path does not become exact. Keep loop entry, backedge, break and continue coverage. |
| Same-module String call | Same branded target/call row carries ExactString. Result has no I64-result ordinal requirement; all physical arguments still undergo ordinary validation/evaluation. |
| Unproved direct/mutual cycle | Existing Pending -> RecursiveDependency remains; no SCC guesses or synthetic base result. |
| Missing return, unsupported syntax/annotation | Existing named unavailable/reject rules remain. |

`call_row::result_representation` currently assumes every CoreStringMethod row
is I64. S1 must derive its projection from the already-validated generated
result kind when String rows are added; it must not relabel substring as I64.
The final stable solver pass must retain the recursive call row after the
normal-result proof closes. A known enclosing String never licenses dropping
unknown/unsupported nested targets from package coverage.

## Runtime contract boundary and followup

The selected C non-expanded call branch
`lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc::emit_published_i64_call`
currently emits `call i64` and `set_type(T_I64)`; that is not a String ownership
contract. `FinalizedRootResultAbiV1` in
`normal_callable_semantic_package/ordinary_new_local_commit.rs` has no general
String return alternative. These are concrete followup boundaries, not proof
that all runtime String support is absent.

Existing kernel `handle_abi_borrowed_owned_conformance` tests the reusable rule:
borrowed arguments remain borrowed; escaping a borrowed handle as an owned
return requires an independently releasable handle, and the caller releases
that owned return. Those primitives do not prove either helper's source exit
co-seal. The runtime followup must choose the existing selected Text/handle
carrier from its semantic exit owner, bind caller/callee return agreement,
and reject unsupported representations before artifact execution. Do not
route a handle through integer arithmetic or infer ownership from MirType.

Before executable activation, the same runtime row must name creation/escape,
callee cleanup, caller release, and Fault cleanup for every selected return.
It must verify zero/positive/negative returned content, survival across callee
cleanup, release exactly once, and no published value on Fault. If a selected
backend gap is exposed, taskify that exact owner; neither VM fallback nor
whole OwnedText/serializer work is a prerequisite for S1.

## Ordered executable tasks

| Order | Task / owner | Completion and retirement boundary |
| --- | --- | --- |
| G0 — closed | Existing ingress guard recovery | Four ingress states, both rejecting consumers and split transport paths checked; local PASS plus 13/13 rejected mutations, bash syntax and diff checks green. No compiler semantics change. |
| S1 — next | MIR-CALL-PARSER-STRING-RESULT-S1; existing catalog + publisher | Implement the mapping above as one coherent result-family change; natural helper source yields String, final call rows are present, generic Call destination publishes String once. Remove superseded String-as-KnownNonI64 classification only for proved shapes. This is no shared legacy deletion credit. |
| A1 | Canonical source-to-MIR recheck | Pin current binary, use real merged parser, observe whether RecursiveDependency is removed, take real parse/2 -> starts_with/3 once and finish empty. Classify any new terminal; no fixture shrinking. |
| D2/I2 | Selected runtime String call/return contract and implementation | Co-seal semantic exit, carrier, caller/callee ABI, ownership and Fault in existing package/C owners; add executable content/lifetime/Fault evidence. Never use S1 as ABI authority. |
| T5 | Existing I3 selected caller cutover | After required acceptance, switch a remaining actual caller, or prove it already uses the selected owner; distinguish existing handoff from completed execution. |
| R0 | Existing selected legacy retirement | Enumerate exact remaining caller and retained shared users, prove selected edge zero, delete exclusive code/tests/guards and add re-entry protection. |

S1's source witnesses are the two unchanged helper declarations and their
recursive Return/Add call sites. The downstream caller is the already-recorded
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` source tuple.
R0 must resolve the exact old caller function/branch from that production
switch; its physical delete set is not yet proven here. It is an explicit
required output of T5, not a fabricated deletion promise or prerequisite to
implement result proof. Shared GenericLoop/LegacyCallV0 and retained I64 owners
cannot be deleted by this row.

## S1 execution brief

**Change:** add one exact normal String result across the existing solver,
call-row transport and sole publisher. Replace the bounded old String
classification atomically; preserve unproved-cycle rejection.

**Contract:** same canonical source/site/target identity, complete nested-call
coverage, full argument evaluation, unchanged effects, one-shot handoff and
residual checks. Generic Call emission and destination representation are the
physical scope; runtime ABI/ownership activation is separate.

**Done:** both natural helper bodies prove String; literal/Add/substring/local/
loop positive rows and mixed/unary-minus/unknown receiver/direct-cycle negatives;
recursive RHS row survives final seal; duplicate/foreign/residual and both
destination publisher tests; lowering failure leaves no published type. Retain
existing I64/nominal Box guards and update the owner README/reference. Run
focused quick-profile tests from one Cargo process (up to 4 jobs), reusing the
same binary; no platform CI needed for this compiler boundary.

**Stop:** a missing source-to-result mapping, dropped nested row, defaulted
Core kind, new source authority, or competing publisher returns to this card's
specific decision. A downstream runtime capability gap does not invalidate
proved MIR representation, and it does not authorize runtime activation.

## Evidence and checkpoint

Read-only worker and local source audit agree on this mapping. No Cargo/runtime
probe/CI was run for the design; S1 is accepted but unimplemented. The user
requested design and taskification, so this turn closes at the design checkpoint.

At `3038bfb1e6`, the ingress guard returned exit 1 because of deleted `Absent`
and pre-split paths; G0 retains that known baseline debt. Pro's unpushed patch
and reported mutation checks remain external evidence, not imported local work.
Windows lifecycle stays user-deferred and warning cleanup stays paused at I147.

G0 local receipt (2026-09-22): stale guard recovered from baseline `2f066b149b`;
normal copied-tree check passes and 13 mutations reject. S1 is the next row.
