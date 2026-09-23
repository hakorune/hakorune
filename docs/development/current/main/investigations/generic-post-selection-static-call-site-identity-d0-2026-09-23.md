---
Status: design_stop__exact_terminal_source_identity_unresolved
Task: GENERIC-POST-SELECTION-STATIC-CALL-SITE-IDENTITY-D0
Date: 2026-09-23
Parent: generic-legacy-observation-front-g0-premise-recheck-2026-09-23.md
PreviousCard: generic-loop-source-facts-accessor-compile-repair-i0-2026-09-23.md
NextCard: none__design_decision_required
Implementation permission: false; one exact terminal/source-identity audit only. No source edit, diagnostic behavior change, fixture, receipt, route, caller switch, or fallback.
---

# Generic post-selection static-call source identity D0

## Six-line brief

```text
Decision: the pinned G0 front's current named static-call stop is not correlated to the earlier Loop tags; identify its existing source context before selecting any semantic repair.
Source authority + canonical issuer: existing RawInvocationSourceContextV1 / SourceExprSiteV1 transport and StaticResultPublicationIngressPortV1; exact result issuance remains VerifiedStaticCallResultPublicationOwnerV1.
Non-authority: target name/arity, planner tags, the unrelated int_to_str canary, AST or method-name reconstruction, RawLegacy success, or VM behavior.
Fail-fast boundary: UnissuedStaticCallRetirementV1::GenericCompatibility occurs after inline-helper decline and before argument lowering; keep it there unless an exact existing Selected handoff is proven.
Smallest next slice: for the one pinned run's first legacy-fallback-retired occurrence, determine (caller, SourceExprSiteV1, target, ingress state) or name the exact existing Unavailable reason.
Non-claims: no LoopRecipe/carrier defect, no publication/result expansion, no production cutover, VM restoration, fallback, backend parity, or legacy retirement.
```

## Fixed observation boundary

```text
run SHA:        fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f
run time:       2026-09-23T14:13:39+09:00
case:           generic_loop_continue_strict_shadow_vm
fixture:        apps/tests/phase29ca_generic_loop_continue_min.hako
command:        NYASH_BIN=target/quick/hakorune HAKO_BIN=target/quick/hakorune bash tools/smokes/v2/profiles/integration/joinir/generic_loop_continue_strict_shadow_vm.sh
first terminal: [freeze:contract][static-call/legacy-fallback-retired] owner=StringHelpers method=to_i64 arity=1
result:         exit 1; wrapper expected exit 4
```

`LoopCondContinueOnly` and shadow-adoption tags appeared earlier, but the
terminal message has no caller or source site. The fixture itself contains an
integer loop; output order is not a same-callable relation. Do not label this
`InLoopTerminal` or join it to
`StringHelpers.int_to_str/1 Body(0).Initializer(0)` without exact identity
evidence.

The terminal owner is
`src/mir/builder/method_call_handlers.rs::UnissuedStaticCallRetirementV1::GenericCompatibility`
at `handle_static_method_call_with_descent`. It is reached only after inline
record-helper lowering declines and before `completion.lower_all(self)`, so
arguments and physical Call effects have not run. Both current-owner policy
paths can reach it only after publication ingress reports `Unavailable`.
`RawLegacyChildLoweringPortV1` returns `Unavailable` unconditionally; the
located ingress can also report it for absent context, unlocated compatibility,
or non-static cataloged lineage. Exact cataloged static rows instead pass
through the existing declaration/target/result catalogs and one-shot
publication handoff.

## Design task

Audit only the fixed terminal occurrence and the existing source-transport
seams. Produce one of these finite results:

| Result | Required evidence | Consequence |
| --- | --- | --- |
| `CatalogedStatic(caller, site, target)` | Exact `CanonicalSameModuleCallableKeyV1`, `SourceExprSiteV1`, target, and the ingress/owner product for this same occurrence. | Reuse the existing source-bound result/publication owner and physical handoff; a later implementation row must name that exact tuple and its physical success receipt. |
| `CompatibilityUnavailable(reason)` | Exact input port and one existing reason: raw-legacy port, absent context, unlocated compatibility, or non-static cataloged lineage. | Keep the named pre-effect retirement terminal. Decide whether this occurrence belongs to the selected G0 front; do not create semantic authority to force admission. |
| `SourceContractError(error)` | Existing source-backed missing/foreign/location/catalog error at the same occurrence. | Keep the typed fail-fast boundary and route only to that existing owner. |
| `NoSafeSlice(reason)` | Existing transport cannot expose a same-occurrence caller/site or a finite unavailable reason without inventing a matcher/authority. | Seal this observation; choose another already-inventoried family rather than broadening this lane. |

The audit must compare the result against the known admitted row
`StringHelpers.int_to_str/1 Body(0).Initializer(0) ->`
`CurrentOwnerStatic(StringHelpers.to_i64/1)`, but equality must be established
from identity, not inferred from the common target. If the fixed wrapper only
reports the target, state exactly which existing context is unavailable and
whether an observational-only follow-up is possible from already-carried
context. Do not add that diagnostic in this D0.

## Canonical owner and rejection contract

If the exact occurrence is a cataloged static row, reuse the existing chain:

```text
VerifiedSameModuleCallableDeclarationCatalogV1
+ VerifiedSourceStaticCallTargetCatalogV1
+ VerifiedSameModuleCallableResultCatalogV1
  -> VerifiedStaticCallResultPublicationOwnerV1 one-shot take
  -> VerifiedStaticCallResultPublicationHandoffV1
  -> lower_selected_static_result_publication_v1
  -> CompletedUnifiedValueCallEmissionV1
  -> PreparedStaticCallResultPublicationV1
```

No new issuer, `Verified*`/`Prepared*` product, result family, ValueId
source, or duplicate publication owner is allowed. The handoff may be consumed
only for its exact caller/site/target and only after the physical call receipt.
Missing, foreign, unlocated, wrong-namespace, target-only, or unavailable
context cannot become `Selected`; it stops before argument/Builder effects.
The terminal never retries another route. VM remains a retired route.

## D0 acceptance and handoff

- Record the exact caller/site/target and ingress state for this one terminal,
  or close as `NoSafeSlice` with the precise missing existing product.
- Reconcile explicitly whether the known `int_to_str/1` row is this occurrence.
- Keep the existing `GenericCompatibility` rejection before argument descent;
  do not count the wrapper as passing (it must still exit `4`).
- Select exactly one bounded successor: a behavior-preserving diagnostic
  projection of already-carried caller/site/ingress context into this named
  terminal. If this exact path carries no such context, close as
  `NoSafeSlice` and return to the family scheduler; do not synthesize it or
  jump directly to result publication.
- Update the G0 front card and current pointer with the classification.

The G0 wrapper remains unaccepted until it exits `4` with its required Loop
tag. This D0 does not authorize serial P1 route observations, S6E production,
source-to-Recipe expansion, production selection, or old-edge deletion.

## Worker audit

Two read-only workers independently confirmed that the terminal diagnostic
does not contain caller/site identity and that its occurrence cannot be joined
to the printed Loop function or known `int_to_str/1` row from current evidence.
The second audit traced the existing ingress: `RawLegacyChildLoweringPortV1`
returns `Unavailable`; cataloged static callers use the existing
`StaticResultPublicationIngressPortV1` and publication handoff. Neither
worker edited files or ran Cargo.
