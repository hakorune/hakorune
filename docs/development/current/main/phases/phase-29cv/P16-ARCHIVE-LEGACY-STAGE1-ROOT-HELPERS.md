# P16: archive legacy Stage1 root helpers

Scope: remove stale Stage1 helper entrypoints from active `tools/` root.

## Why

`tools/stage1_debug.sh` and `tools/stage1_minimal.sh` were early Stage1 CLI
wrappers. They carried their own mode vocabulary:

- `emit-program-json`
- `emit-mir-json`
- `run-vm`

Current Stage1 shell truth now lives in:

- `tools/selfhost/lib/stage1_contract.sh`
- `tools/selfhost/compat/run_stage1_cli.sh`
- `tools/selfhost/mainline/build_stage1.sh`

Keeping the old root helpers active makes Stage1 mode ownership look split,
especially while Program(JSON v0) is being quarantined.

## Decision

Move the two old helpers to `tools/archive/legacy-selfhost/stage1-cli/`.

Do not change the active Stage1 contract or the current compatibility runner.

## Active Entry

Use:

```bash
tools/selfhost/compat/run_stage1_cli.sh --bin <stage1-cli> emit mir-json <source.hako>
```

For explicit Program(JSON) compat proof, use:

```bash
tools/dev/phase29ch_program_json_compat_route_probe.sh --bin <stage1-cli> <source.hako>
```

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/stage1-cli/stage1_debug.sh \
  tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh \
  tools/selfhost/compat/run_stage1_cli.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
