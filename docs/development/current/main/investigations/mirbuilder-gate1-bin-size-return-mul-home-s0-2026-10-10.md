# Gate1 bin_size returned Mul Home S0

Status: CLOSED (Home terminal S0)
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

## S0 evidence and closeout

The terminal owner now tries the existing borrowed-formal Mul proof first.
Only when that proof is unavailable does it try the ordered local-I64 ×
zero-argument CurrentOwner static-I64 proof. The call observation is staged
only after the complete returned expression passes. The original unchanged
`size_class_box.hako` (SHA-256
`ac6513ccd595a664bf7bd4a47402baff1377152a93ef2a059566826b13d642b0`)
now exits its Body11 Home walk with the existing I64Scalar terminal relation.
The focused table rejects swapped operands, a non-I64 left local and a
non-call right child at Body11 without a terminal publication. Existing
malformed earlier bodies keep their precise fail-closed stops.

`CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib bin_size
--quiet` passed 8/8 in 326.55 s total (test execution 0.04 s). Test binary
SHA-256: `cc8602379af3e91eb128bfd1fc67926f18d6a946e75193c744b77de247e15ecb`.
The same binary passed `scalar_expression` 17/17, `home_new_prefix` 14/14,
and `borrowed_mul` 4/4. The quick CLI build passed in 247.56 s; binary
SHA-256: `b9079fd6ee018343cca3f7433536f6282df4100b0dfa474c69e667e018dcedde`.

The unchanged `apps/mimalloc-lite/main.hako` pure-first EXE probe exits 1
before EXE emission at the independent
`ordinary-new/borrowed-entry/source-only-object-actuals` boundary. That
first-stop does not prove result-contract or physical Loop Mul readiness for
`bin_size`; those remain separate tasks. No source rewrite or compatibility
replay was used.
