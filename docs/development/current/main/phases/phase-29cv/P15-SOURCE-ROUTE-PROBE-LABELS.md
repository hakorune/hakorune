# P15: source-route probe labels

Scope: make source-route diagnostic probe output distinguish helper build route
from the source-route behavior being observed.

## Why

`tools/archive/legacy-selfhost/engineering/phase29ch_source_route_direct_probe.sh`
and
`tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh`
build a temporary
helper executable through `tools/selfhost_exe_stageb.sh`. That helper build
defaulted to `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate` when this label card
landed. P102 changed the helper default to `direct`; explicit
`stageb-delegate` remains a bridge compat capsule.

Their diagnostic labels previously read like the helper artifact itself was the
direct source route. That is easy to misread while cleaning Program(JSON v0)
keepers.

## Decision

Print two separate facts:

- `helper-build=selfhost_exe_stageb route=<stageb-delegate|direct>`
- `probe-target=<the MirBuilderBox source-route observation>`

This is output-label cleanup only. It does not change helper build routing or
probe acceptance.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/phase29ch_source_route_direct_probe.sh \
  tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
