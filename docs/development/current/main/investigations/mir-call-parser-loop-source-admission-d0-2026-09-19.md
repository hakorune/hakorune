---
Status: selected__design_stop__parser_loop_source_admission__2026-09-19
Task: MIR-CALL-PARSER-LOOP-SOURCE-ADMISSION-D0
Date: 2026-09-19
Parent: mir-call-static-compatibility-catalog-target-i0-2026-09-19.md
Implementation permission: false; this card is a read-only authority and shape decision
Classification: BoxCount; one finite source-loop admission shape
---

# Parser loop source admission D0

## Six-line brief

```text
Decision: decide whether one existing source-aware loop authority can admit the finite ParserProgramBox.parse/2 outer loop, or retain its typed NoSafeSlice terminal.
Source authority + canonical issuer: parser-branded RawInvocationSourceContextV1 plus the existing CallableGenericLoopSourceFactsIssuerV1; any extension must co-seal with the existing Facts/Recipe/JoinSig/physical owner.
Non-authority: parser line numbers, method names, AST/MIR rescans, the legacy LoopRouteContext, VM/compatibility fallback, and a new parser-specific issuer.
Fail-fast boundary: outer cont_prog == 1 loop, its break exits, nested static_semis == 1 and loop(true) loops, and the exact source-site relation must be admitted or rejected before effects.
Smallest next slice: inventory the source loop/exit relations and map them to existing LoopBreak/GenericLoop Facts, Recipe, JoinSig, and physical adapters; name the first missing owner.
Non-claims: no static catalog/publication, resolver If support, loop implementation, compatibility retirement, production switch, or whole-parser acceptance.
```

## Finite shape under review

The only selected shape is the outer loop in `ParserProgramBox.parse/2`:

```text
condition = cont_prog == 1
body      = skip whitespace -> declaration/static/statement parse
exits     = i >= n, max_prog guard, explicit break, and return on contract error
nested    = static_semis == 1 and loop(true) statement-semicolon scan
target    = ParserStringUtilsBox.starts_with/3 in the declaration branch
```

The target call is evidence for the parent static tuple, not a loop authority.
The loop must be admitted first through source relations; it must not be
reconstructed from the target's name, arity, or Hako line number.

## Existing-owner census

| Existing owner | What it can prove | Boundary for this D0 |
| --- | --- | --- |
| `CallableGenericLoopSourceFactsIssuerV1` | source-located GenericLoop facts, exact raw `[GenericLoopV1]` selection, and the existing semantic/physical adapter | source context rejects nested lowering as `UnsupportedFirstCohort`; current parser result is `GenericLoopV1NotSelected` |
| `LoopBreakRecipe` route | legacy break facts and a Recipe tree through `LoopRouteContext`/`MirBuilder` | no source-lineage co-seal or callable source handoff; it cannot be re-entered from this card |
| `loop_cond_break_continue` projection | test-only one-body `if` with break/continue shape | `#![cfg(test)]`; not a production consumer and does not cover this parser body |
| static publication owners | selected source static-result publication after loop admission | not reachable until the loop terminal is resolved |

The census shows no reusable source-aware owner for this shape today. The
missing relation is the source-bound loop/exit contract, not a missing static
target lookup.

## Source inventory receipt — 2026-09-19

The resolver already exposes the required source authority: `loop_sites`,
`resolved_loop_source_context`, `resolved_loop_source_forest`, and the sealed
`resolved_exits` inventory. The existing callable handoff consumes only the
parent loop site, direct condition/body child sites, and binding rows. Its
`classify_suffix` check explicitly rejects any descendant loop or nested body
segment as `nested-loop-profile-not-admitted`, and the prepared source-facts
payload has no exit-relation field.

This closes design tasks 1 and 2 at the boundary level. The missing owner is
precise: one source-bound product must co-seal the outer loop identity, its
direct/nested loop forest, and every resolved break/return transfer before a
Recipe or physical adapter can consume the body. Reusing only the existing
binding schedule would leave exits and nested ownership unproved, so it is not
a safe promotion.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Source relation inventory | Record parent, condition, body, every break/return exit, nested-loop identity, and the selected target site from resolver-owned source context. |
| 2 | Existing-product mapping | For each relation, identify the existing Facts, Recipe, JoinSig, and physical adapter field that would consume it; missing fields stay typed gaps. |
| 3 | Authority decision | Choose one: extend the existing source-aware loop authority as one co-sealed contract, or retain `NoSafeSlice` and keep the static I0 terminal. Do not create a second issuer. |
| 4 | Follow-up boundary | If extension is accepted, write a separate implementation I0 with positive/negative ownership guards. If no safe extension exists, record the typed terminal and leave the static tuple retained. |
| 5 | Pointer/closeout | Update this card, the static parent card, and `CURRENT_STATE.toml`; run pointer/diff checks. No Cargo or backend CI is required for this design-only card. |

## Acceptance and non-claims

Acceptance is an auditable authority matrix plus one named decision for the
finite shape. A green local loop test cannot substitute for source relation,
co-seal, or physical-owner evidence. Until an accepted source terminal exists,
the parent static card remains before Cataloged/Selected and retains its old
compatibility edge.

No parser fallback, AST rewrite, MIR scan, VM retry, synthetic source site,
static publication row, or production caller switch is permitted by this card.
