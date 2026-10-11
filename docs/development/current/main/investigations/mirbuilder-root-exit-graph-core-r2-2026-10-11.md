# MIRBUILDER-ROOT-EXIT-GRAPH-CORE-R2

Status: closed; selected graph core ready for S0
Date: 2026-10-11
Scope: behavior-preserving extraction of the sole root Home graph algorithm
Related: `mirbuilder-root-exit-static-two-exit-s0-2026-10-11.md`, `mirbuilder-root-exit-canonical-writer-r1-2026-10-11.md`

## Decision

Source authority + canonical issuer: the existing root Home order and fault
sequences remain ledger-issued; `ordinary_new_admission/selected.rs` remains
the sole physical graph algorithm. Move the graph writer body into one
builder-private core that accepts the R1 writer port and returns exact graph
bindings. The raw caller continues to begin/record root progress immediately.
This extraction must not select the Static V2 caller or publish pending rows.

Non-authority: a second cleanup emitter, deriving Home order from MIR, or
changing source completion or final physical validation.

Fail-fast boundary: preserve missing-origin, frame, ingress, and writer
errors and exact binding order. Do not add fallback or alter raw Plain/Call/
MapGet semantics.

Smallest next slice: extract the graph core and raw wrapper without changing
its observed MIR. Verify focused raw Plain/Call/MapGet positive and negative
cases, physical binding order, and compile; compare unchanged SizeClassBox
first stop. Then return selection to S0 for the selected canonical cutover.

Non-claims: R2 does not consume selected Static exits or make the original
source pass `root-exit-unconsumed`.

## Acceptance

- Existing raw root Plain/Call/MapGet physical MIR and binding order agree
  before and after; focused fail-fast negatives retain their rejection.
- The only root graph algorithm is the extracted core, used by the raw wrapper
  and callable by the already-landed canonical writer. No selected-caller
  behavior changes in this slice.
- Required focused tests, pointer guard, and unchanged-source first-stop are
  recorded before closeout.

## Closeout evidence

The graph's block reservation, clean Return, fault terminal, reverse Normal
cleanup loop, shared Fault suffixes, optional ingress, and Plain Jump now
execute in `emit_root_graph_v1` through the same writer port. The raw wrapper
still begins the same ledger exit and records the same Call/MapGet/Plain
receipt immediately. Construction of the owned ingress operation moved
before graph emission; cloning its Call/key does not allocate MIR IDs. The
writer operations and binding append order remain the same. No selected
Static caller was switched in R2.

`cargo check --profile quick -p nyash-rust --lib` passed. Focused raw Call
physical finishing and final handoff passed 1/1 each; root cleanup order
passed 5/5 and its foreign/missing/cycle/duplicate negative passed 1/1.
MapGet positives passed 3/3. The broader MapGet family was 12/13: the
non-literal-key case fails in package issue with
`static-scalar/incoming-veto: NoIncoming`, before graph emission. The exact
test also fails with the same diagnostic class at unchanged baseline
`b6a5b69ae2` in an isolated worktree, so this is classified as baseline
debt, not an R2 regression. No other selected red remains.

An unchanged original `size_class_box.hako` import plus trivial Main reached
the same `ordinary-new/local-commit/root-exit-unconsumed` first stop as R1.
The current CLI build/link/probe took 254.06s; rerunning the built binary
took 0.03s. `rustfmt --check` on the selected source, `git diff --check`,
and the current-state pointer guard passed. The graph-core source is 748
lines, below the 760-line split threshold. Its `clean` receipt currently
produces one dead-code warning until S0 consumes it; no warning suppression
was added.

Next: `MIRBUILDER-ROOT-EXIT-STATIC-TWO-EXIT-S0` consumes this single graph
core through the canonical writer and implements atomic post-collect root
publication. R2 alone claims no selected progress or EXE success.
