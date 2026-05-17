# 293x-577 GUARD-MANIFEST-003 Segment Closeout Thin Wrappers

Status: selected current
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-003` is the implementation row selected by
`GUARD-MANIFEST-002`.

The first `k2_wide_*` migration target is the recent hako_alloc segment
closeout family. The top-level script names must remain stable for humans and
docs, but the executable body should move behind manifest-owned implementation
commands.

## Scope

- Add manifest rows for the three selected segment closeout guards.
- Keep top-level `k2_wide_*_closeout_guard.sh` scripts as thin wrappers that
  call `tools/checks/run_row_guard.sh --only <id>`.
- Move thick guard bodies to an implementation-only location.
- Add or extend a guard so the selected top-level wrappers cannot regrow
  embedded guard bodies.

## Selected Scripts

```text
tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh
tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh
```

## Stop Lines

- No deletion or rename of public top-level guard entrypoints.
- No broad generator rewrite.
- No dev-gate / allocator-wide manifest pilot wiring.
- No allocator `.hako` behavior.
- No compiler acceptance change.
- No backend `.inc` route or matcher change.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM003.1` | Move thick bodies behind implementation commands. | implementation commands run the old checks. | top-level names stay |
| `GM003.2` | Add `guard_rows.toml` rows. | `run_row_guard.sh --only <id>` runs each row. | no schema change |
| `GM003.3` | Thin public wrappers. | public scripts delegate to row runner only. | no body duplication |
| `GM003.4` | Add wrapper no-growth guard. | selected wrappers cannot call guard helpers directly. | no dev_gate wiring |

## Required Evidence

```text
tools/checks/run_row_guard.sh --only hako-alloc-segment-arena-bitmap-closeout
tools/checks/run_row_guard.sh --only hako-alloc-segment-lifecycle-closeout
tools/checks/run_row_guard.sh --only hako-alloc-segment-page-membership-closeout
bash tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

