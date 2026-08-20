---
Status: fast implementation row
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PARSER-SOURCE-HANDOFF-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-parser-source-handoff-d0-2026-08-20.md
ProductionCaller: none; canonical source-plan transport only
ReplacementCell: retain the parser postpass before source-only A
Classification: BoxShape
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PARSER-SOURCE-HANDOFF-I0

## Six-line brief

Decision: retain one parser-issued postpass product from the canonical
`parse_once` boundary through source-plan classification; do not reparse or
reconstruct parser Facts.

Source authority + canonical issuer: the parser's one-shot postpass remains
the sole semantic issuer. `CanonicalParserSourceHandoffV1` is a non-Clone,
move-only transport that co-seals that product with the existing source
profile, read/parse receipt, and UTF-8 digest.

Non-authority: AST-only constructors, paths/display names, digest equality,
`ParsedProgramWithSourceV1` box-only projections, resolver handoff projections,
AST reconstruction, source-plan names, and any canonical-side reparse cannot
issue or replace the handoff.

Fail-fast boundary: parse/postpass, profile, digest, and source identity must
remain one co-sealed handoff before source-plan input is issued. Drop, replay,
foreign, incomplete, or mismatched products fail before A/C/B, Recipe, Join,
Resolver, Builder, or physical work; no compatibility/raw fallback.

Smallest next slice: expose the existing parser postpass at crate scope,
add the frontdoor handoff carrier, and make `PreparedNormalSourcePlanInputV1`
retain that carrier while preserving its AST-only test constructor. Add only
focused transport tests and a reusable structural guard.

Non-claims: no source-only A producer, Script disposition, canonical request
field, Recipe/Join issuance, physical Call/publication/Return, source-admission
change, selected-normal switch, raw/compat retirement, ABI/backend,
performance, or Builder cleanup.

## Scope and ownership

The parser postpass is issued by the existing parser entry and contains the
retained source-backed cohort, build-gate/constructor/parameter evidence, and
coverage. The new frontdoor carrier may expose read-only identity diagnostics,
but only its move operation may transfer the postpass onward. It has no public
raw-parts constructor, no `Clone`, no replay, and no AST-only escape used for
semantic reclassification.

The source-plan input stores either the existing AST-only test fixture or the
parser-backed postpass. The AST-only constructor remains test compatibility;
the canonical frontdoor must use the parser-backed variant. `source()` may
lend the retained AST for the existing plan parser, while the postpass itself
stays owned by the source-plan product until a later A design consumes it.

No-Box/compatibility parser cohorts are retained as explicit parser coverage.
This row must not turn an incomplete cohort into an empty or complete
source-only candidate. A later A issuer decides cohort eligibility.

## Acceptance

Positive:

- one canonical `parse_once` issues one postpass and one handoff;
- the source-plan input owns the same postpass without a second parse or
  resolver pass;
- parser profile, read/parse counts, UTF-8 digest, source identity, and
  postpass coverage remain available at the source-plan boundary;
- existing AST-only source-plan unit fixtures remain unchanged and are marked
  non-canonical test inputs;
- the handoff and postpass cannot be cloned or replayed;
- canonical and raw frontdoors preserve their existing route decisions.

Negative:

- a second `parse_from_string*`/postpass invocation after `parse_once`;
- source path, display name, pointer, ordinal, or digest-only re-pairing;
- missing/foreign/duplicate postpass or profile/digest mismatch;
- a dropped parser product silently becoming compatibility, Raw, or zero
  semantic Facts;
- AST reconstruction of `VerifiedFinalCallableProgramSourceV1`;
- any Resolver, A/C/B, Recipe, Join, Builder, physical, publication, or
  production-caller change in this row.

## Required files and line budget

Expected implementation owners are a new frontdoor child below 300 lines,
the existing `normal_file_vm_frontdoor.rs` and source-plan input with only
thin field/move wiring, and visibility-only parser changes. Before semantic
growth, split any source at 760 lines; 800 is a hard stop. Do not grow the
748-line canonical dispatch, the 762-line parser module, or create a second
source-plan authority.

## Focused evidence

The I0 gate must prove one-read/one-parse transport, parser-backed source-plan
retention, AST-only fixture compatibility, digest/profile identity, and
negative no-reparse/no-clone/no-fallback cases. It must also run the current
pointer guard, `git diff --check`, and the reusable handoff guard. Cargo is
run one process at a time under the repository's resource contract.

## Closeout boundary

This row closes only when the parser-backed carrier and its focused evidence
are green. The next row remains the parked source-only A design; no A/C/B or
physical task may be opened by this I0 closeout alone.
