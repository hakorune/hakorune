# 293x-626 GUARD-MANIFEST-011 Pure-First Route Thin Wrapper Pilot

Status: selected current
Date: 2026-05-18

## Decision

`GUARD-MANIFEST-011` is a BoxShape cleanup row selected before continuing
`ROUTE-DIAG-VOCAB-001`.

The guard/proof surface has grown again. Existing infrastructure already has a
safe path:

```text
tools/checks/run_row_guard.sh
  -> tools/checks/guard_rows.toml
  -> tools/checks/lib/manifest_runner.py
  -> tools/checks/impl/<guard>.sh
```

This row applies that path to one recent pure-first route guard while keeping
the public `tools/checks/k2_wide_*` command stable.

## Scope

- Move one thick pure-first route guard body to `tools/checks/impl/`.
- Keep the public guard path as a thin wrapper.
- Add a `guard_rows.toml` manifest row.
- Do not introduce a new YAML/generator path.

Selected pilot:

```text
tools/checks/k2_wide_pure_first_same_module_static_helper_global_call_guard.sh
```

## Stop Lines

- No all-at-once guard rewrite.
- No generated shell checked in.
- No shell `eval`.
- No `shell=True` subprocess dispatch.
- No allocator behavior.
- No compiler route behavior.
- No proof app source change.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM11.1` | Add manifest row for the pilot guard. | `run_row_guard.sh --only ...` reaches the impl command. | no new runner |
| `GM11.2` | Move thick body to `tools/checks/impl/`. | public wrapper is thin and executable. | no command rename |
| `GM11.3` | Verify wrapper and manifest entry. | public guard and manifest runner both pass. | no gate wiring |
| `GM11.4` | Update current pointers. | pointer guard and diff check pass. | no bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_row_guard.sh --only pure-first-same-module-static-helper-global-call
bash tools/checks/k2_wide_pure_first_same_module_static_helper_global_call_guard.sh
git diff --check
```
