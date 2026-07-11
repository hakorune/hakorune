# Phase 200x: dead Load pruning on private carriers

Status: Landed

Purpose
- land lane B1 after `phase199x`
- add the first generic-memory code cut without mixing `Store` or observer/control policy

Scope
- prune dead `Load { dst, ptr }` only when `ptr` is a definitely private carrier root
- keep the first cut limited to `RefNew`-rooted local carriers with copy-only alias propagation
- keep the DCE structure modular by introducing a dedicated `memory.rs` slice

Non-goals
- no overwritten `Store` pruning yet
- no store-to-load forwarding
- no `Debug` / terminator policy change
- no phi-carried or mixed public/private carrier widening

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::memory -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- dead `Load` pruning is now landed for definitely private carrier roots
- immediate next is lane B2: overwritten `Store` pruning on the same private carriers
