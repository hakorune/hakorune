# Gate1 bin_size words local Mul S0

Status: CLOSED; Body10 Home I64 passed, first-stop Body11 ReturnValueNotCovered
Date: 2026-10-10
Scope: MIRBUILDER-GATE1-BIN-SIZE-WORDS-LOCAL-MUL-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-bin-size-loop-home-after-s0-2026-10-10.md
  - src/mir/resolved_semantics/README.md

## Decision

The unchanged `SizeClassBox.bin_size` first-stop is Body10
`local words = (5 + top) * scale`. The resolver's exact initializer and
ordered binary-child relations are the source authority; the current Home
walk's scalar classes are the I64 authority. The existing
`home_new_prefix_scalar_expression` is the sole issuer for this local
initializer. Its current root-Mul rejection is intentional for broader
unproven cases, so this slice admits the bounded pure I64 shape with an
ordered Add left child and a live I64 local right child, after checking the
whole expression, exact source paths and NormalInteger Mul envelope. It does
not create a second source parser, Recipe, or physical operation owner.

Construction begins at the selected Home scanner's Body10 local initializer
caller. The old `None` from the scalar preflight is replaced only for this
proven shape. Fail closed on foreign owner/site, wrong operator/order,
non-I64 operand, invalid literal/extra child, or unavailable operator
envelope; reject the whole initializer without a partial class install.

## Bounded acceptance

- The original source's first-stop moves from Body10 to Body11, with `words`
  installed as I64 by the selected Home walk.
- Focused source variants reject wrong ordered shape and operand classes at
  Body10. Existing malformed initializer cases still stop at Body5 and
  invalid Loop cases at Body9.
- Reuse the existing `real_bin_size_` family and scalar-expression owner tests
  before adding an independent test. Verify pointer, selected formatting,
  source caps and diff; record source and binary provenance.

Non-claims: physical Loop Mul, Body11 returned Mul, EXE, old-edge retirement,
CALLEE-RETURN-OUTCOME-S0 and the full goal remain open.

## Handoff

Home After S0 closed at `c6e9368099`. Read-only review confirmed
`home_new_prefix_scalar_expression.rs` currently admits only a guarded inner
local-times-integer-literal Mul, while root Mul returns `None`. The same
review confirmed resolver expression rows are passive and physical Loop Mul
is a separate unavailable boundary.

## S0 evidence and closeout

`home_new_prefix_scalar_expression` now admits only a pure local-initializer
root Mul with an exact Add child on the left (integer literal then I64 local),
an I64 local on the right, and the existing NormalInteger Mul envelope. The
recursive preflight checks both children before installing the target local;
no AST reparse, source-name test, new Recipe, or physical operation was added.

The unchanged `size_class_box.hako` SHA-256 is
`ac6513ccd595a664bf7bd4a47402baff1377152a93ef2a059566826b13d642b0`.
`CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib bin_size
--quiet` passed 7/7 in 320.28 s total (test execution 0.03 s; the rest is
combined build/link overhead). Its binary `nyash_rust-6da50fb535f1fa27`
has SHA-256 `a9356e706d8bc5e98c13f29ba8b4b27a229c34ee6fa7f1ee6efb8c456f387d44`.
The same binary passed `scalar_expression` 17/17 and `home_new_prefix` 14/14.

The original source's first-stop is now Body11 `ReturnValueNotCovered`.
The focused table rejects swapped root operands, Bool literal or right value,
an extra Add child, and wrong root operator at Body10. Existing malformed
divide and nested-Mul initializers remain rejected at Body4/5. Initial test
placement failed Rust privacy checks; moving it into the package's private
test owner resolved that. The first runtime test run exposed stale
`PrefixNotCovered` expectations after the source advanced; the tests now
assert the precise new Return boundary. No current red remains.

Pointer guard, selected rustfmt check, diff check and 800-line cap pass.
Physical Loop Mul and returned Mul remain unavailable; this is Home class
evidence only. Read-only audit recommends a separate Body11 Home return row
next, using resolver ordered binary source, current `words: I64`, the existing
zero-argument CurrentOwner static `word_size()` claim, and the terminal owner.
It must not reuse the borrowed-formal-only `IntegerMulReturn` gate for `words`.
