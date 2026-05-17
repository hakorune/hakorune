# 293x-581 GUARD-MANIFEST-007 OSVM Closeout Thin Wrappers

Status: selected current
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-007` is the implementation row selected by
`GUARD-MANIFEST-006`.

The selected target is the OSVM fast-path closeout pair. Public
`k2_wide_*` names stay stable; thick bodies move behind manifest-owned
implementation commands.

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

