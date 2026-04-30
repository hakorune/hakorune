# Legacy Stage1 CLI Helpers

These scripts are archived historical helpers, not active Stage1 entrypoints.

- `stage1_debug.sh`
- `stage1_minimal.sh`

They predate the current Stage1 shell contract and keep old mode names such as
`emit-program-json` / `emit-mir-json` / `run-vm`.

Current active entrypoints:

- `tools/selfhost/lib/stage1_contract.sh`
- `tools/selfhost/compat/run_stage1_cli.sh`
- `tools/selfhost/mainline/build_stage1.sh`

Use `tools/dev/phase29ch_program_json_compat_route_probe.sh` for explicit
Program(JSON v0) compatibility proof.
