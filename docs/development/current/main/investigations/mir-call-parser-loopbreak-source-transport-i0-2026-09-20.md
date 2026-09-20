---
Status: closed__2026-09-21__ParserLoopBreakSourceTransport
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-TRANSPORT-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-TRANSPORT-I0
Date: 2026-09-20
Parent: mir-call-parser-loopbreak-source-consumer-d0-2026-09-20.md
Implementation permission: true for one-shot transport of the existing LoopBreak source candidate into the selected callable lowering scope; no physical route lowering
NextCard: MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0
---

# Parser LoopBreak source transport I0

## Six-line brief

```text
Decision: retain the already-issued package LoopBreak candidate and move it
  exactly once into the selected callable lowering scope before Builder work.
Source authority + canonical issuer: the existing
  VerifiedLoopBreakSourcePackageV1, issued by the existing LoopBreak Facts/
  source projector; the package remains the complete-row observer.
Non-authority: names, selected-key membership as route meaning, AST/MIR
  rescans, LoopRouteContext, legacy composer, synthetic Recipe, fallback, or
  a second LoopBreak receipt.
Fail-fast boundary: reject foreign owner, duplicate take, missing candidate
  row, or owner/site mismatch before the raw Loop entry can lower effects.
Smallest next slice: preserve the package product through install, lend one
  owner-scoped product through the package port, and store it in the existing
  CallableSemanticLoweringState for the later physical consumer.
Non-claims: no LoopBreak route selection, source-to-MIR success, production
  caller switch, publication, old-edge deletion, backend parity, or VM work.
```

## Authority and ownership

The package issuer already walks every resolver batch row and emits a typed
candidate or typed absence. The current install destructures that product as
`loop_break_source: _`, which makes the later physical owner unable to consume
the evidence and encourages a second source scan. This slice removes that
loss of ownership without changing the product's meaning.

The installed package owns the product. `NormalCallableSemanticPackagePortV1`
is the only lender and takes at most one product for a given
`FunctionOwnerIdV1`. The selected-source adapter transfers that owned product
to `CallableSemanticLoweringState`; the state is already the owner of
per-callable one-shot source projections. No package row or candidate may
escape the lowering callback.

## Focused implementation tasks

| order | task | completion condition |
| --- | --- | --- |
| 1 | Retain the existing package product at install | installed package owns the LoopBreak package; no `_` discard remains |
| 2 | Add a one-shot owner-scoped package-port take | candidate facts move once; foreign/missing/duplicate owner cases have named errors |
| 3 | Thread the moved facts into the selected source scope/state | state creation receives the product before raw lowering, with no clone or independent source scan |
| 4 | Add focused transport guards | direct candidate fixture proves ownership/take; typed absence and duplicate/foreign attempts reject |
| 5 | Closeout | record line counts and reds; select the physical adapter card; do not claim route execution |

## Explicit non-work

Do not extend `CallableGenericLoopSourceFactsIssuerV1` with a LoopBreak route
in this slice. Do not call `loop_break_composer`, `route_loop_break_recipe`,
`LoopRouteContext`, `lower_loop_v0_core`, or `reseal_branch_bindings`. Do not
change the raw `RouteNotFrontSelected` terminal, selected mapping, package
brand, VM/AOT lane, or any legacy delete set.

## Acceptance boundary

This row is complete when the existing candidate is retained through install,
one selected owner can take it into the lowering state, and duplicate/foreign
or missing products fail before Builder effects. The next physical card must
consume the exact source loop site and reuse the existing source-parts/loop-v0
owner. Until then the parser package remains stopped at its named dependency
terminal.

## Non-claims

The transport does not prove that a parser LoopBreak can lower, that the
selected `starts_with/3` caller reaches MIR, that the package is Cataloged or
Selected, or that any legacy caller is removable. Those require the physical
adapter, source-to-MIR acceptance, and an exclusive delete-set in later rows.

## Closeout receipt (2026-09-21)

The installed package now retains `VerifiedLoopBreakSourcePackageV1` through
the install boundary. The selected source-backed package port lends a
one-shot `LoopBreakSourcePackageTakeHandle`; the owner-scoped take moves either
the existing candidate or typed supported absence into the existing
`CallableSemanticLoweringState` without cloning or rescanning source. A
duplicate take fails with
`[freeze:contract][callable-loop-break/source-package/owner-row-duplicate-take]`.

Focused package evidence is 5/5 green:

* direct candidate package: owner take returns the candidate and a second take
  rejects it;
* unsupported and specialized shapes retain typed absence;
* parser scan package keeps its existing dynamic candidate and parameter
  coverage;
* package-row relation guards remain active for foreign, duplicate, missing,
  and unexpected rows.

`CARGO_BUILD_JOBS=4 cargo check --profile quick` passes in 25.24s with 1845
warnings. The remaining warning at `LoopBreakSourcePackageIssueV1::BatchLoan`
is pre-existing baseline enum-field dead code; this slice adds no transport
private-interface or state-field warning. `git diff --check` is clean.

The transport additions would have taken existing owners past the 800-line
hard stop. The source-scope helper is now in
`normal_callable_semantic_loan_port/source_scope.rs` (parent 715 lines,
helper 101), and the package install transport types/handle are in the
`install/` submodules (`install.rs` is 791 lines).

This closes transport only. The next card is
`MIR-CALL-PARSER-LOOPBREAK-SOURCE-PHYSICAL-I0`; route selection, physical
lowering, source-to-MIR acceptance, publication, old-edge deletion, and
backend work remain unclaimed.
