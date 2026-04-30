# P6: direct MIR run loader proof

Scope: prove whether `selfhost_build.sh --run` can leave the
Program(JSON v0) keeper route and execute the direct MIR(JSON v1) artifact.

## Why

P4 pinned the current blocker:

```text
v1: unsupported instruction 'newbox' in function 'main'
v0: unsupported op 'field_get' in mir_json_v0 loader
```

The emitted artifact already has `schema_version = 1.x`, so the owner is the
direct MIR(JSON v1) loader, not the v0 fallback. The minimum loader vocabulary
needed for current direct source MIR is:

- `newbox`
- `field_get`

## Decision

Add only the v1 loader support needed for those two emitted instructions, with
a parser unit test before changing the selfhost `--run` route.

The proof is green: direct MIR(JSON v1) execution accepts both minimal return
and binop fixtures after `newbox` / `field_get` loader support. Normal
`selfhost_build.sh --run` now uses direct source -> MIR(JSON) ->
`--mir-json-file`.

`--run --keep-tmp` and raw snapshot diagnostics stay on the Stage-B
Program(JSON v0) artifact route because those modes explicitly request the
old artifact.

## Files

- `src/runner/json_v1_bridge/parse/mod.rs`
- `src/runner/json_v1_bridge/parse/tests.rs`
- `tools/selfhost/lib/selfhost_build_run.sh`
- `tools/selfhost/lib/selfhost_build_route.sh`

## Acceptance

```bash
cargo test --release parse_v1_accepts_newbox_and_field_get -- --nocapture
cargo build --release --bin hakorune
target/release/hakorune --backend mir --emit-mir-json /tmp/run.mir.json /tmp/run.hako
target/release/hakorune --mir-json-file /tmp/run.mir.json
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_binop_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `cargo test --release parse_v1_accepts_newbox_and_field_get -- --nocapture`
  passed.
- Direct probes for `return 7` and `1 + 2 * 3` emitted MIR(JSON) and executed
  with rc=7 through `target/release/hakorune --mir-json-file`.
- `SMOKES_ENABLE_SELFHOST=1` quick return/binop `--run` smokes passed.
