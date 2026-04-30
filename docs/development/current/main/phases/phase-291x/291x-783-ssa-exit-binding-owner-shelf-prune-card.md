# 291x-783 SSA Exit-Binding Owner Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/mod.rs`
- `src/mir/builder/control_flow/joinir/route_entry/mod.rs`
- `src/mir/builder/control_flow/FOLDERIZATION_MAP.md`
- `src/mir/builder/control_flow/ssa/`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

After `291x-581` deleted the zero-use `plan::exit_binding*` wrappers, the
remaining `control_flow::ssa::exit_binding*` files were still only a staged
owner seam:

- repository search found no live non-test callers under `src/` or `tests/`
- active exit-binding materialization already happens in
  `normalization::execute_box`
- boundary construction already lives under
  `join_ir::lowering::inline_boundary_builder` and inline-boundary types

Keeping the `control_flow::ssa` surface around only preserved local
`#[allow(dead_code)]` shelves and duplicated an owner path that no longer exists
in runtime code.

## Decision

Delete the retired `control_flow::ssa::exit_binding*` family and point the
folderization doc at the durable owner path. This is BoxShape-only cleanup; it
does not change JoinIR lowering semantics.

## Landed

- Removed the unused `control_flow::ssa::exit_binding*` files and the now-empty
  `control_flow::ssa` owner surface.
- Dropped the `control_flow::mod.rs` `mod ssa` declaration.
- Removed stale route-entry commentary that still presented the retired seam as
  active.
- Updated `FOLDERIZATION_MAP.md` to mark the `ssa/` surface retired and record
  the real owner path.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The MIR structural vocabulary queue stays open, but the exit-binding owner seam
is no longer part of it. Remaining candidates stay in the `cond_profile /
hints / phi_query / LocalSSA / extractor-detector` families.

## Proof

- `rg -n "control_flow::ssa|ssa::exit_binding|ExitBindingBuilder" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
