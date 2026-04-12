# Phase 201x: overwritten Store pruning on private carriers

Status: Landed

Purpose
- land lane B2 after `phase200x`
- add the first overwritten `Store` cut without widening into cross-block memory reasoning

Scope
- prune earlier `Store { value, ptr }` when a later same-block `Store` targets the same definitely private carrier root
- allow copy-only alias propagation on the carrier side
- keep the cut local to the new DCE `memory.rs` seam

Non-goals
- no cross-block store reasoning
- no store-to-load forwarding
- no dead-store elimination on public/shared carriers
- no `Debug` / terminator policy change

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::memory -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- overwritten `Store` pruning is now landed for same-block definitely private carrier roots
- immediate next is lane C0: observer/control docs-only inventory
