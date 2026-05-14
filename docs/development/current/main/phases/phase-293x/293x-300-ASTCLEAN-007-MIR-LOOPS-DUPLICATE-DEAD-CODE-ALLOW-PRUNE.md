# 293x-300 ASTCLEAN-007 MIR loops duplicate dead_code allow prune

Status: complete

## Decision

Decision: accepted.

`src/mir/builder/loops.rs` had duplicate adjacent `#[allow(dead_code)]` attributes on loop utility helpers. This row removes only redundant duplicate attributes and does not change warning suppression for the staged helpers themselves.

## Scope

- Remove duplicate adjacent `#[allow(dead_code)]` attributes from `current_header`, `in_loop`, and `depth`.
- Keep one allowance on each staged loop helper.
- Preserve MIR builder loop utility behavior.

## Non-goals

- No MIR builder API deletion.
- No loop lowering or planner behavior change.
- No broad MIR utility cleanup.

## Guard

- `tools/checks/k2_wide_astclean_mir_loops_duplicate_dead_code_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_mir_loops_duplicate_dead_code_guard.sh` passed locally.
