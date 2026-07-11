# Phase 204x: control-anchor operand liveness contracts

Status: Landed

Purpose
- start lane C2 with the narrowest safe cut
- lock control-anchor operand liveness before any broader control cleanup discussion

Scope
- fix `C2a` as control-anchor operand liveness only
- keep `Return.value`, `Branch.cond`, and reachable edge args live in mainline DCE
- extract the seeding path so control-anchor ownership is explicit in code
- add focused regressions for return and branch-condition operand liveness

Non-goals
- no `Branch` / `Jump` / `Return` deletion
- no block merge or CFG simplification
- no jump-threading
- no `Debug` policy change

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::liveness -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `git diff --check`

Result
- lane `C2a` is landed
- immediate next is `C2b` legacy in-instruction-list control-anchor seed cleanup
