# MIRBUILDER-ROOT-EXIT-GRAPH-PORT-R0

Status: closed behavior-preserving slice
Date: 2026-10-11
Scope: behavior-preserving raw root graph writer extraction
Related: `mirbuilder-root-exit-graph-writer-d0-2026-10-11.md`

## Contract

Extract the existing root cleanup graph algorithm and terminal Call ingress
behind one private writer port under `ordinary_new_admission/selected/`.
The initial raw writer delegates to the current builder methods. It owns
no source classification, new cleanup order, or selected Static admission.

Source authority: existing Home root order and terminal Call packet.
Canonical issuer: current root payload and its ledger recording.
Target production caller: the existing normal root Plain/Call/MapGet payload.
Replaced responsibility: its direct raw block/instruction writes. The
unchanged graph and binding order must remain byte-for-byte equivalent.
Fail-fast: duplicate/missing blocks, wrong site, wrong frame, malformed
binding and edge graphs retain their existing refusals.

## Acceptance

- Existing root Plain and terminal Call physical tests, both cleanup paths,
  and root graph/order negative tests pass. Reuse the family; add no
  implementation-mirroring test or row guard.
- A before/after physical MIR comparison for one root Plain and one terminal
  Call source has the same instructions, blocks, edges and binding order.
- The unchanged SizeClassBox first stop remains `root-exit-unconsumed`.
  No Static V2 root consumption or EXE pass is claimed.
- `cargo check --profile quick -p nyash-rust --lib`, focused tests,
  `rustfmt --check`, `git diff --check`, and pointer guard pass.

The port is retained for the later canonical CFG writer. If that writer
cannot consume it without a second graph algorithm, return to D0 rather
than minting a parallel emitter.

## Closeout evidence

At `53655dfd33` before extraction and this slice after extraction, the same
temporary Plain and terminal Call sources produced identical physical MIR
after sorting debug-print block entries by block ID. Plain had 8 blocks
(`94efd7c2294de85595a48b64000669ac5219e2938e99e162f3aa905c5e500d47`);
Call had 13 (`1c7c0f922165a1c66fb27fc91650bfd2f89455a5a5e4dfd94b0a2b28d4584fef`).
The root-exit ledger binding sections were byte-identical as well (Plain
`51a36358700cc7fb9576ff24effbd0870bf52aea1a66bd2762b065b6c2a8af1f`,
Call `b70928380e7a6de67f3fea66d8b8571d70b12033f3811c0e93aabcec8b9d037d`).
The observation tests were removed after comparison; no permanent test or
guard was added.

`cargo check --profile quick -p nyash-rust --lib` passed. Existing terminal
Call cleanup positive and graph mutation negatives passed 1/1; root child
field finalization, instance receiver mutation negative, and static direct
return each passed 1/1 using the same test binary. A temporary unchanged
`size_class_box.hako` physical probe passed 1/1 and confirmed the first stop
remains `root-exit-unconsumed`. That probe was removed. `rustfmt --check`,
`git diff --check`, and current-state pointer guard passed. The new writer
port delegates existing raw block, terminator, projection, and binding writes;
no selected Static root consumption or EXE result is claimed.

Next: `MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-D0`, to specify canonical CFG
write order and exact selected two-exit integration before semantic cutover.
