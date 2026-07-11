# Phase 205x: legacy control-anchor seed cleanup

Status: Landed

Purpose
- land lane `C2b` after `phase204x`
- remove legacy in-instruction-list control-anchor seeding from mainline DCE

Scope
- stop seeding `Branch` / `Jump` / `Return` operands from `block.instructions`
- keep control-anchor liveness owned only by `block.terminator` plus reachable edge args
- preserve `Return.value`, `Branch.condition`, and reachable edge args via the focused `C2a` contracts

Non-goals
- no control-instruction deletion
- no CFG simplification
- no branch/jump rewriting
- no `Debug` policy change

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::liveness -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `git diff --check`

Result
- legacy instruction-list control-anchor seed ownership is removed
- immediate next is `C2c` simplification-handoff wording lock
