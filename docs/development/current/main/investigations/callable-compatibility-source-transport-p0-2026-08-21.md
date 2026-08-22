---
Status: bounded P0 implementation complete — transport only
Date: 2026-08-21
Decision: CALLABLE-COMPATIBILITY-SOURCE-TRANSPORT-P0
Parent: docs/development/current/main/investigations/callable-compatibility-source-admission-d0-2026-08-21.md
ProductionCaller: existing compatibility materialization/request/root path only; no new caller
ReplacementCell: carry existing parser/macro origin facts without changing the compatibility route
Classification: BoxShape candidate; no new accepted source shape or semantic authority
Execution row: CALLABLE-COMPATIBILITY-SOURCE-TRANSPORT-P0
---

# CALLABLE-COMPATIBILITY-SOURCE-TRANSPORT-P0

## Six-line brief

Decision: Preserve one typed parser/macro compatibility origin through the
existing materialization → request → prepared-root boundary so the old route
can explain its source without pretending to be SourceBacked.

Source authority + canonical issuer: parser
`NormalParserSourceLineageV1` and macro
`NormalCallableTransformCompatibilityV1` remain the sole issuers of their
facts. A private aggregate only co-seals and moves those already-issued facts;
it cannot issue semantic membership, a resolver ledger, Recipe, Join, or target.

Non-authority: AST/name/ordinal reconstruction, filename/display path,
`NormalCompileAdmissionV1`, `NormalCallableSemanticPackageMode::Compatibility`,
raw success, warning text, `UnlocatedCompatibility`, and empty/default fields
cannot create or repair the origin.

Fail-fast boundary: after materialization has both typed facts and before
`NormalCompileRequestV1` is issued. Missing, foreign, duplicate, or drifted
origin stops before request/root/lifecycle/Builder effects. Source-free public
AST/JSON/VM/REPL entrances carry no origin and remain `Unavailable`.

Smallest next slice: introduce one private non-Clone
`NormalCallableCompatibilityOriginV1` transport aggregate containing the
transformed compatibility AST together with its reason and lineage, move that
single carrier through the compatibility request and prepared root, and expose
it only to the existing compatibility owner. Add focused positive/negative
tests and a structural guard; do not admit a cohort, issue a semantic receipt,
or switch a production route.

Non-claims: no parser grammar change, no SourceBacked promotion, no new
callable shape, no resolver/Brand/FunctionCall classification, no Recipe/Join/
physical Call, no raw retirement, no ABI/backend change, and no performance or
promotion claim.

## Classification-completeness receipt

The transport must classify the full finite domain before any Builder effect.
The aggregate is an evidence carrier, not a new semantic state machine.

| state | issuer / condition | before effects | allowed terminal | fallback |
|---|---|---|---|---|
| `SourceBacked` | parser final source product is present | keep existing callable-source handoff; no compatibility aggregate | installed semantic package path | no downgrade to Compatibility |
| `TypedCompatibility` | parser/macro issued a compatibility reason plus parser lineage and the transformed AST | move one aggregate containing all three, without later pairing | existing compatibility-only root/lifecycle | no SourceBacked inference or raw retry |
| `Unavailable` | AST/JSON/VM/REPL request has no parser/macro origin | issue no aggregate and make no source-backed claim | explicit compatibility-only owner | never synthesize a reason/lineage |
| `Neither` (`NoCandidate`) | caller proves the request is outside the callable lane | no request-side compatibility origin or semantic package | caller-owned no-candidate terminal | never use empty aggregate or `Complete(empty)` |
| `Rejected` | missing/foreign/duplicate/contradictory origin or source drift | typed freeze before request/root/Builder effects | stable rejection terminal | no AST re-scan, name pairing, retry, or fallback |

`Discarded` remains an outer isolated-candidate terminal and must discard the
whole candidate. It is not represented by an optional origin and must never be
converted to `Unavailable` or `TypedCompatibility`.

## Proposed carrier contract

The candidate aggregate has private fields only:

```text
NormalCallableCompatibilityOriginV1 {
  ast: ASTNode,
  reason: NormalCallableTransformCompatibilityV1,
  lineage: NormalParserSourceLineageV1,
  private seal
}
```

Required properties:

- `Clone` is not implemented; the materialization result moves exactly once;
- no constructor accepts an AST, name, ordinal, or display path as authority;
- the aggregate is created only once, in
  `materialize_normal_callable_program_with_identity_v1`, after parser lineage,
  macro reason, and transformed AST are all available;
- the AST is bundled with the reason and lineage so later code cannot pair an
  independently transformed AST with a guessed source fact;
- the request/root transport preserves pointer/digest identity and does not
  reparse or normalize the source a second time;
- compatibility lowering may inspect the reason for diagnostics, but no
  semantic package, Brand map, FunctionCall target, Recipe, Join, or physical
  owner may be issued from it;
- source-backed requests do not receive a compatibility aggregate.

The likely implementation seams are the existing child/module owners:

```text
src/runner/modes/common_util/normal_callable_compatibility_origin.rs  carrier
src/runner/modes/common_util/normal_callable.rs  sole co-seal issuer
src/mir/compiler/normal_default_pipeline.rs request carrier
src/mir/builder/normal_default_root_catalog_lifecycle.rs prepared-root carrier
```

The macro outcome may remain a short-lived reason+AST boundary, but the
materializer is the sole place allowed to co-seal the final move-only carrier.
No public constructor may accept an arbitrary `reason + lineage + AST` triple.

If any seam would exceed the 760-line split trigger, extract a child module;
do not compress code or grow `raw_invocation_source_transport.rs` or the raw
expression dispatcher.

## Acceptance and stop line

Before switching to `fast`, the implementation plan must prove:

- a parser/macro positive reaches the existing compatibility owner with the
  same reason and lineage identity exactly once;
- SourceBacked positive remains on the installed package path and never gains
  the compatibility aggregate;
- source-free AST/JSON/VM/REPL positive is `Unavailable`, not guessed
  `TypedCompatibility` or SourceBacked;
- `Neither`, missing reason, missing lineage, foreign lineage, duplicate move,
  and digest/source drift each map to one named negative state before effects;
- no Builder/child argument/Resolver/Recipe/Join/physical Call effect occurs
  on a rejected transport;
- compatibility behavior and accepted syntax are unchanged;
- focused tests, a reusable guard, module README/contract receipt, and all
  touched source/check files below 760/800 are prepared.

Remain `NoSafeSlice` if the two facts cannot be co-sealed without making one
issuer a second authority, if the aggregate must be reconstructed from AST,
if any compatibility entrance requires a guessed default, or if transport
would implicitly admit a new cohort. A later cohort/package activation is a
separate task and must not be folded into this P0.

## Non-blocking follow-up

`ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P1` is a docs/tooling follow-up,
not a prerequisite for this transport P0. It may later check only the active
card for a finite outcome table with `state`, `authority/issuer`,
`pre-effect`, `terminal/continuation`, and `fallback` columns, plus an explicit
neutral outcome (`Unavailable`, `Absent`, `Unresolved`, or the row's equivalent)
and a `NoSafeSlice` stop line. It must accept row-owned vocabulary rather than
guessing `Candidate`/`Rejected`, and must not inspect or rewrite historical
closed cards. Until that optional guard is opened, the tracked
`agent-current-entry-contract-ssot.md` classification-completeness rule and
the table in this card are the review authority.

## P0 implementation receipt

The compatibility materializer now co-seals one non-`Clone`
`NormalCallableCompatibilityOriginV1` containing the transformed `ASTNode`,
`NormalCallableTransformCompatibilityV1`, and
`NormalParserSourceLineageV1`. The request and prepared root move that carrier
as an explicit `TypedCompatibility` state; the existing compatibility lifecycle
still owns it, while SourceBacked, source-free AST/JSON/VM/REPL, and no-candidate
paths remain separate. No resolver, Brand/FunctionCall target, Recipe, Join,
physical Call, fallback, or retry was added.

Evidence:

- `cargo check --profile quick -p nyash-rust` — passed (existing warning baseline only)
- carrier positive/negative tests — 2 passed
- compatibility materializer positive/parse-reject tests — passed
- request/prepared-root transport test — passed
- `bash tools/checks/callable_compatibility_source_transport_guard.sh` — passed
- `git diff --check` — passed

The broad `cargo test ... compatibility` census remains red only at the
pre-existing legacy comparison
`mir::compiler::legacy_candidate_session_tests::normal_pipeline_matches_legacy_compatibility_for_general_module`
(`[freeze:contract][mir/instance-constructor-source/cohort-missing]`); 38 compatibility tests pass
and this transport row does not change that legacy `compile_with_source` path.
That failure is recorded as baseline debt, not as a transport acceptance
failure. Cohort admission, semantic package issuance, compatibility fallback
retirement, and physical cutover remain separate rows.
