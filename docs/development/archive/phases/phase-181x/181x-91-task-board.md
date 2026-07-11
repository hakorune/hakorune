# Phase 181x Task Board

Status: Landed
Date: 2026-04-12

## Goal

Land the first generic no-dst pure cleanup slice.

## Tasks

- [x] docs/pointer lock
  - `phase181x` as landed DCE cleanup lane
  - `phase180x` stays landed as the prior string seam cleanup corridor
- [x] Safepoint candidate slice
  - remove `Safepoint` as the first no-dst pure cleanup target
  - keep `Debug` and terminators out of scope
- [x] regression fixed
  - add a focused DCE unit for `Safepoint` removal
  - keep `KeepAlive` and reachable-only behavior unchanged
- [x] closeout
  - update the current pointer docs and move back to the broader DCE backlog

## Exit

- the first generic no-dst pure cleanup slice is landed
- broader no-dst / effect-sensitive DCE remains separate backlog
