# Phase 202x: observer/control docs inventory

Status: Landed

Purpose
- land lane C0 before any `Debug` or terminator-adjacent operand/control liveness cleanup code change
- fix observer/control ownership and split C0 / C1 / C2 clearly

Scope
- inventory current DCE ownership for `Debug`
- inventory current DCE ownership for `Branch` / `Jump` / `Return`
- fix pointers from B2 to C1

Non-goals
- no code changes
- no `Debug` policy decision yet
- no terminator-adjacent operand/control liveness cleanup yet
- no exception/control widening

Acceptance
- docs-only
- `git diff --check`

Result
- lane C inventory is now fixed
- immediate next is `C1 Debug policy decision`
