# Gate1 bin_size returned Mul Home S0

Status: selected; implementation pending
Date: 2026-10-10
Scope: MIRBUILDER-GATE1-BIN-SIZE-RETURN-MUL-HOME-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-bin-size-words-local-mul-s0-2026-10-10.md
  - src/mir/resolved_semantics/README.md

## Decision

The unchanged `SizeClassBox.bin_size` first-stop is Body11
`return words * me.word_size()`, now `ReturnValueNotCovered`. Resolver exact
binary and ordered child sites supply source authority. The current Home walk
proves `words: I64`; the existing exact zero-argument CurrentOwner static
call claim supplies the right operand class and call target. The Home terminal
owner must join those facts and the NormalInteger Mul envelope, stage the one
call observation only after the whole expression is proved, then issue the
existing I64 scalar terminal relation.

The old borrowed-formal `IntegerMulReturn` gate does not apply to a post-loop
local. Keep that gate unchanged and fail closed if the new whole-expression
proof is absent. Do not reclassify `words` as a borrowed formal, infer from
source names, or let Home success grant physical Loop Mul.

## Bounded acceptance

- Original source crosses the Body11 Home Return value boundary; inspect the
  actual next first-stop rather than assuming result or physical completion.
- Reject wrong owner/site/order, non-I64 `words`, wrong or missing static call
  claim, extra child, and wrong operator without partial call publication.
- Reuse focused `real_bin_size_`, terminal scalar/borrowed Mul tests and
  existing guard before adding a distinct uncovered case. Record source and
  binary provenance, selected tests, line cap, and failures.

Non-claims: result contract, physical Loop Mul, EXE, old-edge retirement,
CALLEE-RETURN-OUTCOME-S0 and full goal completion remain open.

## Handoff

Body10 words S0 closed at `2b87d1c98f`: original first-stop is
`ReturnValueNotCovered(Body11)`, with `words` installed as I64 in Home.
Read-only review located the borrowed-formal gate at
`ordinary_new_borrowed_integer_return_source.rs` and the explicit physical
Recipe Mul rejection at `pure_operation_emitter.rs`. This row changes only
the Home terminal scalar meaning.
