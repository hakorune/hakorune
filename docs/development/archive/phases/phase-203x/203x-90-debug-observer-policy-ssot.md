# 203x-90 Debug Observer Policy SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-203x`

## Purpose

- close lane C1 before any lane C2 terminator-adjacent operand/control liveness cleanup work
- keep observer semantics out of generic no-dst pure cleanup and generic memory reasoning

## Decision

- `Debug { value, message }` stays a permanent observer anchor in mainline DCE
- mainline DCE must keep both the `Debug` instruction and the observed operand live
- `Debug` is not eligible for generic no-dst pure pruning
- any future debug stripping must live behind a separately documented diagnostic-off lane and must not reuse mainline DCE ownership implicitly

## Why

- `Debug` is an observer contract, not a dataflow-only artifact
- mixing `Debug` into generic pure cleanup would blur lane B memory ownership and lane C observer ownership
- lane C2 should reason about terminator-adjacent operand/control liveness, not about removing observer instructions

## Non-goals

- this cut does not change `Branch` / `Jump` / `Return`
- this cut does not add diagnostic flags or strip modes
- this cut does not reopen `Safepoint` or `KeepAlive`

## Acceptance

1. pointer docs agree that `phase203x` lands C1 and moves immediate next to C2
2. focused DCE regression proves `Debug` and its observed operand stay live
3. `git diff --check` is clean
