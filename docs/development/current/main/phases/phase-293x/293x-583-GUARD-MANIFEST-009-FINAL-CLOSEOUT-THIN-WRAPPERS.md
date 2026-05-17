# 293x-583 GUARD-MANIFEST-009 Final Closeout Thin Wrappers

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-009` is the implementation row selected by
`GUARD-MANIFEST-008`.

The selected target is the final public hako_alloc closeout wrapper pair:

```text
tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh
```

Next selected row:

```text
GUARD-MANIFEST-010
```

`GUARD-MANIFEST-010` should close the manifest-wrapper cleanup burst and decide
whether to return to the mimalloc allocator row queue or do one final manifest
helper extraction.

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
| `GM009.1` | Move thick final closeout bodies behind implementation commands. | implementation commands run old checks. | public names stay |
| `GM009.2` | Add manifest rows. | `run_row_guard.sh --only <id>` runs each row. | no schema change |
| `GM009.3` | Thin public wrappers. | public scripts delegate to row runner only. | no body duplication |
| `GM009.4` | Extend wrapper no-growth guard. | selected wrappers are checked. | no dev_gate wiring |

## Required Evidence

```text
tools/checks/run_row_guard.sh --only hako-alloc-reclaim-scalar-lane-closeout
tools/checks/run_row_guard.sh --only hako-alloc-reuse-proof-closeout
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added manifest rows for the final two public hako_alloc closeout wrappers.
- Moved thick command bodies to `tools/checks/impl/`.
- Kept public `tools/checks/k2_wide_*_closeout_guard.sh` paths as stable thin
  wrappers around `run_row_guard.sh --only <id>`.
- Extended `tools/checks/k2_wide_manifest_wrapper_guard.sh` to cover all
  manifest-backed public hako_alloc closeout wrappers.

