# Phase 199x: generic memory DCE docs-facts phase

Status: Landed

Purpose
- land `phase190x` lane B0 before any generic `Load` / `Store` pruning
- fix observer/owner contract for generic memory DCE

Scope
- define lane-B vocabulary and private-carrier contract
- fix the split between lane A (`FieldGet` / `FieldSet`), lane B (`Load` / `Store`), and lane C (`Debug` / terminators)
- move current pointers from B0 to B1

Non-goals
- no DCE code widening
- no `Load` pruning yet
- no `Store` pruning yet
- no `Debug` / terminator policy change

Acceptance
- docs-only
- `git diff --check`

Result
- generic memory `Load` / `Store` now has a fixed observer/owner contract
- lane B first cut is now B1: dead `Load` pruning on definitely private carrier roots
