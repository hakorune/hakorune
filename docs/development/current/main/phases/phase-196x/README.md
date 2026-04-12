# Phase 196x: loop-roundtrip overwritten local field-set pruning

Status: Landed

Purpose
- land `phase190x` lane A2 as the first semantic widening beyond the loop-carried same-root contract lock
- prune an earlier local `FieldSet` when the same carried local root is overwritten immediately after one backedge roundtrip at loop-header entry

Scope
- one-roundtrip loop-header overwrite pruning for definitely non-escaping local roots
- focused DCE tests for prune/keep behavior
- pointer/docs sync from lane A2 to lane B0

Non-goals
- no mixed-root phi merge widening
- no multi-round loop dataflow
- no generic `Store` / `Load`
- no `Debug` / terminator cleanup

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- lane A2 is now landed: predecessor-local loop-body `FieldSet` writes disappear when the next loop-header entry overwrites the same root/field before any same-field read or escape use
- the next DCE lane is now B0: generic memory `Store` / `Load` docs/facts phase
