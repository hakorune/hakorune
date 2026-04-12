# Phase 198x: root pointer compression

Status: Landed

Purpose
- return `CURRENT_TASK.md` and `05-Restart-Quick-Resume.md` to pointer-only docs
- remove long landed-history ledger content from the root restart surfaces

Scope
- compress root pointer docs to current lane / next lane / read order only
- keep historical evidence in phase docs and investigations

Non-goals
- no code change
- no optimization roadmap reorder
- no DCE widening

Result
- root restart docs now point directly at lane B0 and current owners
- landed history stays in phase docs instead of the root pointer
