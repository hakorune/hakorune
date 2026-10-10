# MIRBUILDER-NEGATIVE-INTEGER-RETURN-HOME-S0

Status: CLOSED; source and selected physical acceptance verified
Date: 2026-10-11
Parent Decision: `mirbuilder-static-local-call-bindings-coverage-d0-2026-10-11.md`
Scope: exact negative integer immediate at a Home-tracked `return`
Related: `mirbuilder-static-local-call-bindings-coverage-d0-2026-10-11.md`, `mirbuilder-loop-cond-after-phi-s0-2026-10-11.md`

## Production trigger and authority

The unchanged `lang/src/hako_alloc/memory/size_class_box.hako` has
`return -1` in `bin_size` and `good_size`. Their Home flow records
`ReturnValueNotCovered` for those exact exits, making all sibling exits
unready and leaving already recorded Static local-call binding groups
unconsumed. The source authority is the exact unary-minus integer literal
and checked result already exposed by
`expression_source::negative_integer_immediate`. Reuse the Home terminal
IntegerLiteral issuer, Completion exit, selected Plain/Call exit entry and
existing integer physical emission. Do not synthesize a Call or exit.

## Acceptance and boundary

- Positive: both affected owners have every explicit Home exit ready; their
  local-call groups are attached to the existing selected exit entries.
- Negative: overflow, nonliteral/wrong unary, wrong source site, unready
  sibling exit, owner/group mismatch and duplicate entry remain fail-closed.
- Focused existing Home terminal/entry tests and only missing independent
  cases; original SizeClassBox physical probe must move beyond these two
  owners or expose a later named frontier. Package/physical acceptance is
  not inferred from a source unit test alone.
- The `size_to_bin` search loop is excluded and has separate
  `TerminalNotCovered`/`PrefixNotCovered` evidence. No whole-source PASS is
  claimed by this slice.

After implementation, record exact focused command/results, source
first-stop movement, and any new frontier. Close the row only with those
facts; retain the search-loop D0 independently.

## Current evidence

The exact signed immediate now issues the existing IntegerLiteral relation.
Literal return classification moved into `home_terminal_literal.rs`; the
terminal walk is 772 lines. Focused `ordinary_new_bin_size_words_tests::`
is 3/3 green in quick profile, including exact source all-exit readiness
for both `bin_size` and `good_size`
(`/tmp/hako-negative-integer-home-s0-family-3-20261011.log`).
Existing mutated-source tests now permit only the newly reached named
`static-scalar/incoming-identity` refusal before Home inspection; original
positive and the signed-immediate negative remain strict.

The disposable original SizeClassBox physical probe with protected Eq WIP
now reaches `ordinary-new/local-commit/physical-boundary/phi-in-recorded-block`
before the prior aggregate coverage gate
(`/tmp/hako-negative-integer-home-s0-physical-20261011.log`). This is a
new physical first stop, not a source publication pass. Identify the exact
owner/block and confirm both `bin_size` and `good_size` source flows before
S0 closeout; do not loosen PHI validation as part of this source slice.
Disposable-only diagnosis identifies `SizeClassBox.bin_size/1`, recorded
`BasicBlockId(32)`, leading PHI with one predecessor and no accepted
single-predecessor policy (`/tmp/hako-negative-integer-home-s0-phi-diag-20261011.log`).
The selected LoopCond PHI role was added in a separate S0. Its unchanged
source probe now crosses block 32 and stops at
`IncompleteOrdinaryNewCoverage`
(`/tmp/hako-loop-cond-after-phi-s0-physical-20261011.log`). Disposable
coverage diagnostics show `bin_size` owner 4 and `good_size` owner 7 have
`all_ready=Some(true)` and all local-call sites occur in emitted exits;
`size_to_bin` owner 5 alone has `all_ready=Some(false)` with its call site
absent from exits (`/tmp/hako-loop-cond-after-phi-s0-coverage-20261011.log`).
The selected signed-immediate responsibility is therefore consumed. The
next aggregate stop belongs to the separate search-loop Home D0. Current
checkout `cargo check --profile quick -p nyash-rust --lib` and the existing
three focused signed/negative family tests pass. No whole-source PASS.
