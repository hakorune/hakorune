# 293x-483 VMHAKO-EXTERN-SPEC-001 Subset Externcall Spec Validator

Status: landed
Date: 2026-05-16

## Decision

`VMHAKO-EXTERN-SPEC-001` is the BoxShape cleanup selected by
`MIR-EXTERN-SPEC-002`.

The vm-hako subset validator currently repeats accepted legacy `externcall`
symbol knowledge for route-backed rows such as `env.get` and hako OSVM
reserve/commit/decommit/unreserve calls. `MIR-EXTERN-SPEC-001` introduced
`ExternCallRouteSpec` as the route constant owner, so this row makes the
subset validator consume that table for those already-accepted route-backed
legacy `externcall` rows.

## Scope

- Add a vm-hako subset helper that validates legacy `externcall` shapes from
  `ExternCallRouteSpec`.
- Preserve existing error labels and arity/dst shape behavior.
- Keep hako_intrin, `hako_osvm_page_size_i64`, hako TLS diagnostics, GC
  barrier, and other legacy-only externcalls on their existing local
  validation path unless they already have an `ExternCallRouteSpec` row.
- Keep `mir_call` extern validation unchanged.

## Stop Lines

- Do not add, remove, or rename extern routes.
- Do not broaden vm-hako accepted legacy `externcall` symbols.
- Do not add hako_intrin or `hako_osvm_page_size_i64` to
  `ExternCallRouteSpec` in this row.
- Do not change backend lowering, C shim behavior, pure-first preflight, or
  allocator behavior.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `VMEXT.1` | Document owner and stop lines. | Current points to this card. | no code before docs |
| `VMEXT.2` | Add a spec-backed externcall shape validator. | Existing subset externcall tests pass. | no new accepted symbols |
| `VMEXT.3` | Replace duplicated env/OSVM route-backed ladder arms. | OSVM route-backed rows read `ExternCallRouteSpec`. | leave legacy-only rows local |
| `VMEXT.4` | Verify focused and quick gates. | Required evidence is green. | no backend/provider activation |

## Required Evidence

```text
cargo test -q --lib subset_accepts_externcall
cargo test -q --lib subset_rejects_externcall
cargo test -q extern_call_route_plan
bash tools/checks/k2_wide_osvm_first_row_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when route-backed legacy `externcall` validation consumes
`ExternCallRouteSpec`, behavior remains unchanged, and current moves to the
next row selection card.

## Landed Implementation

```text
owners:
  src/runner/reference/vm_hako/subset_check/externcalls.rs
  src/runner/reference/vm_hako/subset_check/mod.rs
  src/runner/reference/vm_hako/tests/subset_control_misc_parts/externcalls.rs
  tools/checks/k2_wide_osvm_first_row_guard.sh
```

The legacy vm-hako subset `externcall` path now validates already accepted
route-backed rows through `ExternCallRouteSpec` for:

```text
env.get
hako_osvm_reserve_bytes_i64
hako_osvm_commit_bytes_i64
hako_osvm_decommit_bytes_i64
hako_osvm_unreserve_bytes_i64
```

Legacy-only rows such as `hako_osvm_page_size_i64`, hako intrinsics, TLS
diagnostics, GC barrier, and print remain on their existing local paths.
`nyash.env.get/1` remains rejected by the legacy subset path so this row does
not broaden accepted symbols.

Evidence:

```text
cargo test -q --lib subset_accepts_externcall
cargo test -q --lib subset_rejects_externcall
cargo test -q extern_call_route_plan
bash tools/checks/k2_wide_osvm_first_row_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

Closeout:

```text
current blocker moves to VMHAKO-EXTERN-SPEC-002 post-subset-validator row selection.
```
