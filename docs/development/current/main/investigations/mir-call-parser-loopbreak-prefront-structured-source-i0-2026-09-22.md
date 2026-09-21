---
Status: closeout__2026-09-22__PrefrontStructuredSourceRows1to4
Task: MIR-CALL-PARSER-LOOPBREAK-PREFRONT-STRUCTURED-SOURCE-I0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-prefront-source-consumer-d0-2026-09-22.md
Implementation permission: false; the finite pre-front structured source owner and its focused guards are closed
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
---

# Parser pre-front structured LoopBreak source I0

## Six-line brief

```text
Decision: admit the finite single-root structured LoopBreak rows through the
  existing source Facts/Recipe and Parts/LoopV0 owner.
Source authority + canonical issuer: resolver forest/exit/item ledger,
  existing static publication owner, and existing LoopBreak Recipe/physical
  envelope; no new semantic issuer.
Non-authority: route names, AST rescans, LoopRouteContext, generic fallback,
  VM, target-only rows, empty/default item lists, and package reordering.
Fail-fast boundary: before Parts allocation, co-seal root/forest/Recipe and an
  ordered per-item `SelectedStatic` or `CoreMethod` disposition batch.
Smallest next slice: one structured candidate path plus mixed-item positive and
  negative guards for the four finite pre-front parser rows.
Non-claims: I3 publication acceptance, caller switch, old-edge deletion,
  unrelated LoopBreak shapes, backend parity, or warning cleanup.
```

## Accepted input boundary

The existing direct three-statement LoopBreak candidate remains unchanged.
The existing structured source projection/Recipe/Parts/LoopV0 owner is widened
only to accept a one-member forest when its role tree contains the resolver-
issued explicit break/branch relation. Multi-member composite rows retain their
current path. A projection or Recipe failure remains a typed reject; it does
not fall through to `SupportedNonCandidate` or the legacy composer.

The source item batch is issued from existing source/target owners and keeps
source order. Each row is exactly one of:

* `SelectedStatic` — the existing publication owner supplies the one-shot
  handoff, which is consumed once and validated for the existing requirement;
* `CoreMethod` — the resolver-issued bound-receiver item is covered by the
  existing source expression port and consumes no publication row.

Mixed batches are allowed. Every resolver item must occur once, have one
disposition, remain under the root site, and be consumed before physical
allocation. A static target with no selected handoff, a foreign/duplicate or
missing item, an unknown call family, and any leftover publication row are
named rejects. No item may be silently dropped because the route is not a
publication candidate.

## Ordered implementation rows

1. **Structured candidate:** reuse the existing composite projection and Recipe
   owner for one-member structured roots; retain direct/multi-member identity
   and add a focused single-root positive/negative matrix.
2. **Item disposition:** add one co-sealed source-item batch view that joins
   resolver items with existing selected publication rows or core-method rows;
   preserve exact order and residual checks.
3. **Physical consume:** carry that batch through the existing composite
   physical envelope and validate all relations before Parts/LoopV0 effects.
4. **Finite parser guard:** cover `trim/1`, `to_int/1`,
   `_parse_delegate/3`, and `ParserRecordDeclarationBox.parse/3` source rows;
   classify any unsupported shape as the named terminal and keep direct and
   existing composite tests green.

After these rows, return the pointer to I3 task 4. Do not switch the caller or
delete the old edge in this I0.

## Verification requirements

Focused positive evidence must show one-member structured Recipe construction,
mixed static/core item coverage, source-port lowering, and cleanup. Negative
evidence must cover missing, foreign, duplicate, out-of-root, target-only,
unknown-family, residual, and second-take rows. Existing route/package/raw
guards must remain green, all touched Rust files stay below the 760/800 line
limits, and the warning cohort remains paused at I147.

## Implementation checkpoint — 2026-09-22

Rows 1–4 are implemented in the existing LoopBreak owner. The composite
projection no longer discards a valid one-member structured forest; the source
route now consumes one source-order disposition per resolver item, combining
the existing selected-static handoff with the existing CoreMethod owner; and
the physical envelope validates that batch before Parts/LoopV0 allocation.
No new semantic issuer, fallback, VM route, or publication claim was added.

Focused evidence is green:

* `normal_callable_loop_source` route tests: 35/35
* `loop_break_source` package tests: 19/19
* `single_structured_projection_is_admitted_by_composite_owner`: 1/1
* `finite_parser_loopbreak_sources_retain_structured_candidates`: 1/1
* merged parser lifecycle: 1/1, reaching the named
  `[freeze:contract][callable-loop/static-publication/no-selected-handoff]`
  frontier
* `cargo check --profile quick --lib`: pass with the I147 warning baseline
  (`1671` generated warnings)
* `cargo fmt --all -- --check`, `git diff --check`, and the current-state
  pointer guard: pass

The four finite parser source rows (`trim/1`, `to_int/1`,
`_parse_delegate/3`, and `ParserRecordDeclarationBox.parse/3`) retain their
structured candidates and exact-site consumption. A join-bearing nested `if`
that is outside the four exit-bearing contract shapes remains an opaque
`Stmt`, so the existing source statement owner handles it without a new
semantic contract. The merged parser lifecycle now reaches the existing I3
publication boundary. I3 task 4 is therefore the next bounded slice; caller
switch, old-edge deletion, and warning cleanup remain queued.
