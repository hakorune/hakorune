Status: design_stop; selected-Dynamic live publication is blocked before the publication stages
Task: SCRIPT-STATIC-IMPORT-TARGET-AUTHORITY-D0
Date: 2026-08-22
Priority: name one source-owned imported declaration/target/result authority before allowing the existing static publication ingress to proceed
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-CENSUS-D0
PreviousCard: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
NextCard: none until this D0 is accepted
---

# Imported static target authority D0

## Six-line brief

```text
Decision: keep `TargetUnavailable` as a typed design-stop blocker. The selected-Dynamic public fixture reaches an imported static call whose declaration/result authority is not present in the current same-module catalog; it must not be downgraded to `Absent` or retried through a legacy route.
Source authority + canonical issuer: the parser/source-plan import bundle must own the imported declaration, canonical target, and result contract under one parser/source invocation. The existing `ScriptDirectStaticCallLookupIssuerV1` is an audited candidate boundary, not yet an accepted consumer for the generic physical ingress.
Non-authority: same-module declarations alone, alias strings, `using_import_boxes` alone, AST/name/arity lookup, `resolve_static_receiver_box_name`, raw Builder state, empty/default catalogs, `Absent`, and fallback.
Fail-fast boundary: imported declaration/target/result co-seal must complete before `StaticResultPublicationIngress` consumes a handoff or any physical call effect. Missing or foreign dependency evidence discards the unpublished session.
Smallest next slice: `SCRIPT-STATIC-IMPORT-TARGET-AUTHORITY-D0-A/B`, census the actual import bundle and name the sole issuer plus one source-owned handoff into existing ingress; no code, fixture, fallback, or production switch.
Non-claims: no generic import loader, no Dynamic body change, no Loop Compare publication, no A/C/Recipe redesign, no result inference, no backend, and no old-route retirement.
```

## Observed blocker

The unchanged fixture has an explicit dependency:

```text
parser_scan_loop_box.hako
  -> using ... parser_common_utils_box as ParserCommonUtilsBox
  -> ParserCommonUtilsBox.i2s(...)
```

`static_result_publication_ingress.rs` currently resolves the target through
`VerifiedSameModuleCallableDeclarationCatalogV1::declaration_for(...)`. The
catalog is sealed from the current root AST, while the imported dependency's
declaration/result contract is not proven there; the ingress therefore returns
`[freeze:contract][static-result-ingress/target-unavailable]`. The existing
public-root test passes by asserting this typed `RootLower` stop.

The import-aware source lookup already accepts `import_rows`, but its alias
view is branded to the same declaration catalog and requires the canonical
import owner to exist in that catalog. An alias map is consequently not an
imported target authority. The exact missing dependency bundle and its result
contract must be identified before any consumer is connected.

`resolve_static_receiver_box_name` is a separate Builder heuristic that uses
`variable_map`; this D0 must not silently treat a change there as proof of an
imported target. The current evidence is sufficient to name the imported
authority gap, but not to claim a receiver-classification fix.

The read-only worker confirms the same boundary: `ParserCommonUtilsBox.i2s`
is correctly classified as a static receiver; the primary failure is the
imported target authority gap between import-aware lookup and the same-module
ingress. A raw-name path without the import relation is a separate transport
defect, not permission to downgrade the current error.

## Authority and state boundary

```text
parser/source import bundle
  -> imported declaration + canonical target + result contract co-seal
  -> source-owned handoff (one take)
  -> existing StaticResultPublicationIngress
  -> physical Call/result publication
```

Required states are distinct:

| State | Meaning | Allowed next step |
| --- | --- | --- |
| `ImportAuthorityUnavailable` | dependency declaration/result source is absent | typed reject; unpublished discard |
| `ImportTargetComplete` | one invocation-branded imported target/result row is complete | source handoff preparation |
| `TargetAbsent` | a complete authority proves no matching target | explicit ordinary noncandidate route only |
| `IntegrityInvalid` | alias, invocation, declaration, target, or result relation conflicts | typed reject; no repair |
| `Ready` | one co-sealed handoff is available | existing ingress consumes once |

`TargetAbsent` must never be produced merely because the imported catalog was
not loaded. The source owner must distinguish unavailable authority from a
complete negative observation.

## Bounded D0-A / D0-B

1. **D0-A — import authority census:** identify the production import loader,
   the exact `ParserCommonUtilsBox` declaration source, its parser invocation
   identity, and the result-contract owner. Count whether the current public
   compile request transports that bundle or only an alias/config row.
2. **D0-B — handoff contract:** decide whether the existing source lookup can
   lend one imported target/result row to `StaticResultPublicationIngress`
   without a second catalog, AST rescan, name join, or Builder adapter. Name
   the sole issuer, one-shot consumer, failure terminal, and negative state.

Acceptance requires an exact source/declaration/target/result relation, one
issuer and one consumer, a pre-effect rejection path, and a reproducible
explanation of why `ParserCommonUtilsBox.i2s` is `Ready`, `TargetAbsent`, or
`ImportAuthorityUnavailable`. If the dependency bundle is unavailable, remain
`NoSafeSlice`; do not manufacture an empty catalog or alter the ingress error.

## NoSafeSlice conditions

Keep this design stop if any of the following remains true:

```text
only alias/name/arity or Builder import state identifies the imported target
same-module catalog is asked to prove a foreign dependency
target and result facts come from separate issuers without one co-seal
the public request has no owned dependency source/bundle
TargetUnavailable would be converted to Absent to reach publication
the proposed fix requires a second physical call or generic fallback
receiver classification and imported target resolution cannot be separated
```

No Loop publication implementation is authorized while this card is open.

## Downstream prerequisite recorded

The selected Dynamic Compare transaction-hardening review is recorded in
docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md.
It is intentionally parked behind this import-authority D0. Its first bounded
cell is the typed EOF reject in the Dynamic operation cursor; later cells cover
pre-effect claims, writer preparation before V13 reservation, private
definition/ledger co-sealing, and the OuterReturn/Header-current relation.
This link does not change the current execution row or authorize production
publication.
