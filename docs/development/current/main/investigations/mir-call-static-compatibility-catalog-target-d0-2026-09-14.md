---
Status: accepted__design_stop__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-D0
Date: 2026-09-14
Priority: recover the qualified static helper boundary exposed after raw-loop compatibility recovery
Parent: mir-call-r7-stringbox-lower-structural-membership-i0-2026-09-14.md
NextCard: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-I0
Implementation permission: false until the catalog issuer and compatibility result terminal are accepted
---

# Static compatibility catalog target D0

## Six-line brief

Decision: decide whether a qualified static receiver in the compatibility
Program(JSON v0) root may consume an exact same-module declaration-catalog
target; never restore the retired generic fallback or add a retry.

Source authority + canonical issuer: the compatibility root's sealed
`VerifiedSameModuleCallableDeclarationCatalogV1` and an existing static target
publication owner, if that owner can issue a complete target/result relation
without a source site.

Non-authority: owner/method/arity text scans, `current_static_box`, generated
headers, JSON/MIR indices, registry order, environment flags, fallback, retry,
and a fabricated source location or callable ledger.

Fail-fast boundary: before argument descent and MIR mutation, an unissued or
ambiguous static target keeps
`[freeze:contract][static-call/legacy-fallback-retired]`; a target-only
compatibility path must also name its result/publication terminal and cannot
silently enter the source-site handoff.

Smallest next slice: census the live `ParserStringUtilsBox.starts_with/3`
compatibility call and one exact catalog row, then choose Retain, Promote, or
Stop with a finite caller/terminal/delete tuple. No code or fallback change is
authorized by this D0.

Non-claims: no StringBox owner acceptance, Generic G0 migration, source
artifact transport, static schema retirement, backend parity, or whole-R7
caller-zero.

## Boundary and finite census

Boundary: Hako Program(JSON v0) compatibility root -> static receiver route ->
`method_call_handlers.rs` -> existing static target/result terminal. It includes
the Hako compiler helper call that currently stops at
`ParserStringUtilsBox.starts_with/3`; it excludes source-backed cataloged calls,
Math compatibility, Env/Extern, instance `Method`, and unrelated static
families.

| Input row | Existing authority | Required D0 disposition |
| --- | --- | --- |
| Compatibility `ParserStringUtilsBox.starts_with/3` with one exact declaration row | sealed module catalog, but no source-site product | decide whether a complete target/result issuer exists; otherwise named stop |
| Compatibility static receiver with no or multiple catalog rows | none | stop before arguments and effects |
| Source-backed cataloged static call with `Cataloged` source lineage | source-site/catalog publication owner | retain existing selected ingress unchanged |
| Qualified `Math.*` | existing Math compatibility owner | retain outside this row |
| static `this`, receiverless `me`, or unqualified generic static | no exact issuer | retain existing retirement terminal |

## Required design tasks

1. Trace the compatibility call from the Hako entry to the exact route and
   confirm whether the declaration catalog is the same module and same brand
   as the generated static function body.
2. Inspect the existing catalog key, static declaration, draft collector, and
   result/publication owners. A catalog key alone is not enough if result type,
   source site, body ownership, or publication receipt is missing.
3. Decide whether the compatibility contract can consume one target-only
   terminal without reusing `StaticResultPublicationIngressV1::Cataloged`; if
   not, record the row as Retain/Stop and leave the StringBox dynamic probes
   blocked by this baseline.
4. Name the exact old edge and any successor edge. Deleting the retirement
   terminal or re-enabling generic fallback is forbidden until a complete
   issuer and positive/negative route evidence are accepted.
5. Define one positive exact-catalog case, one missing/ambiguous negative, and
   one no-partial-effects guard for the successor I0. Keep all Hako lowerer and
   StringBox membership work outside this design row.

## Acceptance

- one finite compatibility caller and its terminal are named;
- source authority, canonical issuer, result/publication owner, and same-brand
  relation are either proven or explicitly absent;
- the exact `ParserStringUtilsBox.starts_with/3` row is classified without
  relying on name/arity inference or a fallback;
- the successor I0 has a bounded delete tuple and focused positive/negative/
  guard evidence, or the row is sealed as Retain/Stop;
- the existing static retirement tests and Math compatibility remain unchanged;
- README/reference and `CURRENT_STATE.toml` can be synchronized without
  claiming StringBox dynamic acceptance.

## Non-claims

The phase14/16/17 StringBox smokes currently stop at this downstream static
baseline after the raw-loop compatibility recovery. Their red is known
baseline evidence, not a failure of the scope-selection change. No fallback,
retry, or synthetic static target is introduced while this D0 is open.

## Worker audit result (2026-09-14)

The Hako owner is
`lang/src/compiler/parser/scan/parser_string_utils_box.hako:32`, a static
`ParserStringUtilsBox` definition whose `starts_with` helper is also called by
its own `index_of`. The bounded direct-caller inventory is 16 sites:

- `parser_program_box.hako`: 102, 125, 164;
- `parser_box.hako`: 163, 226, 229;
- `parser_control_box.hako`: 82, 146, 238, 281, 363, 492;
- `parser_guard_box.hako`: 26, 92;
- `parser_stmt_box/core.hako`: 347;
- `parser_string_utils_box.hako`: 70 (`index_of` self-call).

Each currently reaches the ordinary `MemberCallRoutePlan::StaticReceiver`
path and `handle_static_method_call_with_descent`. In the compatibility root,
`RawLegacyChildLoweringPortV1` reports static publication ingress as
`Unavailable`, so there is no source/site handoff that can issue a complete
target and result relation. The first observed terminal is therefore the
existing pre-effect
`[freeze:contract][static-call/legacy-fallback-retired]`.

This audit confirms a design boundary, not an implementation permission. The
next D0 must either prove a same-module, same-brand catalog issuer plus a
complete compatibility result/publication terminal, or seal this cohort as
`NoSafeSlice`/Retain. The StringBox lowerer remains outside this row until that
choice is accepted.
