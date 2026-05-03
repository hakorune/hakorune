---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: make `tools/selfhost_exe_stageb.sh` default to the MIR-first direct route.
Related:
  - docs/development/current/main/phases/phase-29cv/P101-PROGRAM-JSON-V0-CAPSULE-CALLER-INVENTORY.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost_exe_stageb.sh
  - tools/selfhost/README.md
  - tools/archive/legacy-selfhost/engineering/phase29ch_source_route_direct_probe.sh
  - tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh
---

# P102 selfhost_exe_stageb Direct Default

## Goal

Retire the implicit Program(JSON v0) bridge default from
`tools/selfhost_exe_stageb.sh`.

Before this card:

```text
default -> stageb-delegate -> Program(JSON v0) -> env.mirbuilder.emit -> MIR
```

After this card:

```text
default -> direct -> --emit-mir-json -> MIR
explicit HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate -> bridge compat capsule
```

## Evidence

Representative input:

```text
apps/tests/hello_simple_llvm.hako
```

Current result:

| Route | Result | Reading |
| --- | --- | --- |
| `HAKORUNE_STAGE1_EMIT_ROUTE=direct` | EXE generated and executed, rc=0 | default candidate |
| `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate` | rc=4, `unsupported pure shape for current backend recipe` | not a replacement proof |

This means the bridge capsule should stay explicit, not default.

## Decision

- Change `tools/selfhost_exe_stageb.sh` default route to `direct`.
- Keep `stageb-delegate` available through explicit
  `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate`.
- Update docs and diagnostic probes whose route label mirrored the old default.
- Do not delete `program_json_mir_bridge.sh` or `stageb_program_json_capture.sh`.

## Non-goals

- no bridge capsule deletion
- no backend pure-first shape widening
- no `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh` archival
- no Stage1 contract or public CLI deletion

## Acceptance

```bash
bash -n tools/selfhost_exe_stageb.sh \
  tools/archive/legacy-selfhost/engineering/phase29ch_source_route_direct_probe.sh \
  tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh
timeout --preserve-status 180s bash tools/selfhost_exe_stageb.sh \
  apps/tests/hello_simple_llvm.hako -o /tmp/p102_direct_default.exe
NYASH_NYRT_SILENT_RESULT=1 /tmp/p102_direct_default.exe
set +e; timeout --preserve-status 180s env HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate \
  bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako \
  -o /tmp/p102_stageb_delegate.exe >/tmp/p102_stageb_delegate.log 2>&1; test "$?" -ne 0; \
  grep -F "unsupported pure shape for current backend recipe" /tmp/p102_stageb_delegate.log; \
  rm -f /tmp/p102_direct_default.exe /tmp/p102_stageb_delegate.exe /tmp/p102_stageb_delegate.log
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
