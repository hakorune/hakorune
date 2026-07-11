# 293x-581 GUARD-MANIFEST-007 OSVM Closeout Thin Wrappers

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-007` is the implementation row selected by
`GUARD-MANIFEST-006`.

The selected target is the OSVM fast-path closeout pair. Public
`k2_wide_*` names stay stable; thick bodies move behind manifest-owned
implementation commands.

Next selected row:

```text
GUARD-MANIFEST-008
```

`GUARD-MANIFEST-008` should decide the final hako_alloc closeout wrapper
migration target for `reclaim_scalar_lane_closeout` and `reuse_proof_closeout`.

## Selected Scripts

```text
tools/checks/k2_wide_hako_alloc_osvm_fast_path_route_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_closeout_guard.sh
```

## Stop Lines

- No deletion or rename of public top-level guard entrypoints.
- No broad generator rewrite.
- No helper extraction in this row.
- No dev-gate / allocator-wide manifest pilot wiring.
- No allocator `.hako` behavior.
- No compiler acceptance change.
- No backend `.inc` route or matcher change.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM007.1` | Move thick OSVM closeout bodies behind implementation commands. | implementation commands run old checks. | public names stay |
| `GM007.2` | Add manifest rows. | `run_row_guard.sh --only <id>` runs each row. | no schema change |
| `GM007.3` | Thin public wrappers. | public scripts delegate to row runner only. | no body duplication |
| `GM007.4` | Extend wrapper no-growth guard. | selected wrappers are checked. | no dev_gate wiring |

## Required Evidence

```text
tools/checks/run_row_guard.sh --only hako-alloc-osvm-fast-path-route-closeout
tools/checks/run_row_guard.sh --only hako-alloc-osvm-fast-path-unreserve-closeout
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added manifest rows for the two selected OSVM closeout guards.
- Moved thick command bodies to `tools/checks/impl/`.
- Kept public `tools/checks/k2_wide_*_closeout_guard.sh` paths as stable thin
  wrappers around `run_row_guard.sh --only <id>`.
- Extended `tools/checks/k2_wide_manifest_wrapper_guard.sh` to cover the OSVM
  closeout wrappers.

