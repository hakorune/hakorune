# Phase 206x: simplification-handoff wording lock

Status: Landed

Purpose
- land lane `C2c` after `phase205x`
- make the handoff boundary to the later simplification bundle explicit in code and docs

Scope
- keep DCE focused on liveness and seed ownership
- keep `Branch` / `Jump` / `Return` deletion out of mainline DCE
- separate control-anchor liveness from future CFG simplification work

Non-goals
- no block merge
- no branch/jump rewriting
- no terminator deletion
- no `Debug` policy change

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests::liveness -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `git diff --check`

Result
- the DCE / SimplifyCFG handoff is now explicit
- immediate next is the next layer step after lane C closes
