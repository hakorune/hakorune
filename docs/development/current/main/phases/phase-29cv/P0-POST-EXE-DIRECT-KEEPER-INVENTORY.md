# P0: post-EXE-direct keeper inventory

Scope: cut the new phase after `phase-29ci` P26 and pin the remaining
`Program(JSON v0)` work as keeper buckets before deleting more surface.

## Why

P26 changed the normal selfhost EXE route from:

```text
source -> Stage-B Program(JSON v0) -> MIR(JSON) -> ny-llvmc -> EXE
```

to:

```text
source -> MIR(JSON) -> ny-llvmc -> EXE
```

That means the old phase-29ci "public compat retirement" lane is no longer the
right active pointer. The remaining `Program(JSON v0)` surface is mostly
keeper-owned infrastructure, and each bucket needs a delete/archive condition.

## Current Inventory

Repository search for the live spellings still shows four meaningful buckets:

- Stage-B run/diagnostic route:
  `tools/selfhost/lib/selfhost_build_stageb.sh`,
  `tools/selfhost/lib/selfhost_build_direct.sh`, and
  `tools/selfhost/lib/program_json_mir_bridge.sh`.
- Stage1 contract route:
  `tools/selfhost/lib/stage1_contract.sh`.
- JoinIR/MirBuilder fixture route:
  `tools/smokes/v2/lib/stageb_helpers.sh` and the
  `phase29bq_hako_mirbuilder_*` smoke family.
- Rust compat/delete-last route:
  `--emit-program-json-v0`, the deprecation warning, the Stage1 bridge
  compat cluster, and `src/stage1/program_json_v0*`.

## First Delete Slice

The old helper-local `emit_exe_from_program_json_v0_with_mir_tmp()` wrapper has
no in-repo callers. The context-owning helper
`emit_exe_from_program_json_v0_with_context()` remains as the diagnostic
Program(JSON)->MIR->EXE consumer, while the normal `--exe` route uses direct
source MIR.

Delete the stale wrapper definitions from both selfhost helper files and update
the selfhost README so it no longer describes `--exe` as a Program(JSON)
consumer.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_direct.sh \
  tools/selfhost/lib/selfhost_build_exe.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

