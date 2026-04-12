# Phase 203x: Debug observer policy decision

Status: Landed

Purpose
- land lane C1 after `phase202x`
- fix `Debug` as a mainline DCE observer anchor before any lane C2 widening

Scope
- lock `Debug` as keep-owned observer instruction in mainline DCE
- document that any future debug stripping must live in a separate diagnostic-off lane
- add a focused DCE regression that keeps `Debug` and its observed operand live

Non-goals
- no terminator-adjacent operand/control liveness cleanup yet
- no `Branch` / `Jump` / `Return` deletion logic
- no diagnostic-off or debug-strip mode

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::observer -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `git diff --check`

Result
- `Debug` is now explicitly locked as a permanent observer anchor in mainline DCE
- immediate next is lane C2: terminator-adjacent operand/control liveness cleanup
