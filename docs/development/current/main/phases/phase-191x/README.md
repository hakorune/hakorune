# Phase 191x: loop-carried same-root local field pruning

Status: Landed

Purpose
- lock the first `phase190x` lane-A cut by proving that dead local `FieldGet` / `FieldSet` operations still disappear when their base travels through a one-round backedge-carried same-root local phi
- keep this cut contract-only: no new generic memory reasoning, no loop overwrite propagation, no mixed-root merge widening

Scope
- focused DCE unit contracts for loop-carried same-root local roots
- pointer/docs sync for the landed A1 slice

Non-goals
- no overwritten-write pruning across loop rounds
- no generic `Store` / `Load`
- no `Debug` / terminator cleanup
- no mixed-root or multi-round loop dataflow

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- `phase190x` lane A1 is now fixed as a supported contract: loop-carried same-root local phi carriers already participate in dead local `FieldGet` / `FieldSet` pruning
