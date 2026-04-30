# P14: Stage-B Program(JSON v0) capture SSOT

Scope: remove duplicate shell/Python extraction logic for Stage-B
`Program(JSON v0)` stdout capture.

## Why

After P13, `selfhost_build.sh` no longer owns Stage-B artifact output. The next
remaining keepers are explicit bridge/delegate routes:

- `tools/hakorune_emit_mir.sh`
- `tools/selfhost_exe_stageb.sh`

Both had their own copy of the same bracket-balancing extractor for finding the
`{"kind":"Program",...}` JSON object in Stage-B stdout. That duplicate is a
small but real route-policy split: future diagnostics or stricter extraction
would have to be patched twice.

## Decision

Add `tools/selfhost/lib/stageb_program_json_capture.sh` as the shell-side SSOT
for Stage-B Program(JSON v0) stdout extraction.

Keep both caller scripts' route behavior unchanged. They source the shared
helper and keep their existing local call shape through a thin wrapper.

## Non-goals

- Do not change `HAKORUNE_STAGE1_EMIT_ROUTE` defaults.
- Do not move `build_stage1.sh` off `stageb-delegate`.
- Do not delete `tools/selfhost_exe_stageb.sh`; it remains a Stage1 mainline
  dependency until direct route dominance issues are fixed.

## Acceptance

```bash
bash -n tools/selfhost/lib/stageb_program_json_capture.sh \
  tools/hakorune_emit_mir.sh \
  tools/selfhost_exe_stageb.sh
bash tools/smokes/v2/profiles/integration/core/phase2231/hakorune_emit_mir_return42_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
