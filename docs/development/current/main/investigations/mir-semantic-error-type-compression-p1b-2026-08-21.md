---
Status: parked design stop — not the current execution row
Date: 2026-08-21
Decision: MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B
Parent: docs/development/current/main/investigations/mir-semantic-error-type-compression-p1a-2026-08-21.md
ProductionCaller: existing Brand projection, claim-ledger, and constructor manifest/loan paths only
ReplacementCell: retain typed issue variants until one existing outer diagnostic boundary
Classification: BoxShape; no accepted shape, authority, route, or physical owner change
Execution row: parked; select only after the live identity/source lane is closed
---

# MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B

## Six-line brief

Decision: Restore typed error transport at the remaining semantic state
boundaries without widening the common recursive-port API. Claim ledger,
Brand projection, constructor manifest, and constructor loan issues remain
typed until one already-owned outer diagnostic boundary.

Source authority + canonical issuer: each existing issue enum remains owned by
the operation that detects it: the claim ledger, resolver-issued Brand
projection, physical-demand manifest, or installed constructor semantic loan.
P1B only transports those errors; it does not issue a new semantic receipt.

Non-authority: diagnostic strings, `format!`, AST names/spans, source paths,
`ValueId`/`MirType`, compatibility success, `Unavailable`, `Absent`, and raw
fallback cannot classify or repair an issue.

Fail-fast boundary: preserve the typed issue at the first owner boundary and
map it to a string only once at the existing route/session outer diagnostic
edge. A typed issue cannot return to an ordinary route, retry, or become a
neutral state.

Smallest next slice: `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B` design-first
census, then one bounded owner family at a time: claim/Brand projection
before constructor manifest/loan. Keep the common recursive port and the
760/800-line owners unchanged unless a responsibility split is prepared.

Non-claims: no source admission, Brand consumer cutover, constructor physical
consumer, raw retirement, `MirInstruction::Call` rewrite, ABI/backend,
performance, production switch, or canonical Script activation.

## Classification-completeness table

| state | authority / issuer | pre-effect behavior / transition | allowed terminal / continuation | fallback policy |
|---|---|---|---|---|
| `TypedError` | the detecting ledger/projection/manifest/loan enum | stop at the owner boundary; retain the exact variant | one outer diagnostic mapping or candidate discard | never branch on its string or re-enter ordinary lowering |
| `Unavailable` | an ingress with no applicable semantic owner | do not enter the typed owner | explicit compatibility/test terminal | not a conversion from a typed issue |
| `Absent` | an exact source site with a verified no-row contract | leave the owner unchanged | only the route's documented no-row continuation | no issue-to-`Absent` conversion |
| `Rejected` | the same typed owner when the issue is a contract violation | freeze before child/MIR effects when pre-effect; discard candidate otherwise | stable rejection terminal | no retry, reparse, or raw fallback |
| `Completed` | existing owner after all typed state is exhausted | return the existing product/value to its current consumer | existing continuation/diagnostic owner | no alternate owner |
| `NoSafeSlice` | design boundary when a common-port or cross-family refactor is required | stop before edits | parked design task | never invent a catch-all string/default |

`Unavailable` and `Absent` are neutral ingress states, not error sinks.
`Rejected` is a source/contract disposition, while `NoSafeSlice` is a
development state. No wildcard, `Option::None`, `unwrap_or(default)`, or
`format!("{error:?}")` may merge these rows.

## Scope and ownership order

P1B begins with a read-only census of the current stringification sites:

1. `normal_script_semantic_lowering_state.rs` — preserve all eleven
   `ScriptDirectStaticClaimLedgerIssueV1` variants;
2. `function_call_brand_source_demand.rs` and Brand lowering projections —
   preserve missing-site, foreign-owner, relation-outside-inventory, and
   duplicate-site issues;
3. `normal_instance_constructor_demand_manifest.rs` and
   `normal_instance_constructor_demand_loan.rs` — keep `NoManifest` distinct
   from `ManifestMissing`, and retain duplicate/foreign/reuse/incomplete
   states;
4. `normal_callable_semantic_package/instance_constructor_loan.rs` — map its
   issue enum only at its existing package/session boundary.

The first implementation sub-slice must touch one owner family and its
outermost existing diagnostic boundary only. It must not change
`RecursiveChildLoweringPortV1`, `raw_expression_dispatch/mod.rs`, or the
source transport owner. A new child module is required before any owner would
reach 760 lines.

## Acceptance

- every listed issue remains distinguishable by enum variant through the
  owner boundary;
- the outer diagnostic boundary is the only new `String` conversion site;
- no route, fallback, retry, AST re-resolution, or `ValueId`/`MirType`
  inference depends on the diagnostic text;
- `NoManifest` and `ManifestMissing` remain distinct;
- claim/Brand issues still reject before child effects where their current
  owners do so, while post-claim failures discard the candidate;
- focused tests assert variants, not only `is_err()` or substring matches;
- a reusable guard pins the finite state vocabulary, one outer mapping, no
  common-port signature rewrite, and all touched owners below the 760/800
  limits.

## NoSafeSlice conditions

Remain parked if typed transport requires a whole-port `Result<_, String>`
signature rewrite, a second semantic issuer, AST/name re-pairing,
`Unavailable`/`Absent` collapsing, a fallback after a typed issue, or a
cross-family change to constructor/Brand physical consumption. P1B may be
reopened only as a behavior-neutral BoxShape after the current pointer selects
it explicitly.

