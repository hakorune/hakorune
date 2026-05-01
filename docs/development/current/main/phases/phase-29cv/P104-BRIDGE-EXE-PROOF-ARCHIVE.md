---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the standalone Program(JSON)->MIR->EXE proof after replacement proof went green.
Related:
  - docs/development/current/main/phases/phase-29cv/P63-BRIDGE-EXE-PROOF-ARCHIVE-HOLD.md
  - docs/development/current/main/phases/phase-29cv/P103-PROGRAM-JSON-BRIDGE-PRINT-CALL-NORMALIZER.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost_exe_stageb.sh
  - tools/archive/legacy-selfhost/engineering/phase29ci_selfhost_build_exe_consumer_probe.sh
---

# P104 Bridge EXE Proof Archive

## Goal

Remove one live Program(JSON v0) bridge caller now that the replacement proof is
green.

P63 kept:

```text
tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
```

because the closest replacement,

```text
HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate tools/selfhost_exe_stageb.sh
```

still failed with `unsupported pure shape for current backend recipe`.

P103 fixed the first unsupported bridge shape by normalizing console print
output at the shared Program(JSON)->MIR bridge boundary. The replacement proof
now reaches ny-llvmc and writes an EXE.

## Decision

- Archive the standalone bridge-to-EXE dev probe under
  `tools/archive/legacy-selfhost/engineering/`.
- Use explicit `tools/selfhost_exe_stageb.sh`
  `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate` as the bridge-to-EXE compat
  proof.
- Keep `tools/selfhost/lib/program_json_mir_bridge.sh` live because
  `phase29cg_stage2_bootstrap_phi_verify.sh` still calls it directly.
- Do not change `stageb-delegate` back into a default route.

Archive metadata:

```text
original_path: tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
archived_on: 2026-05-01
archived_by_card: P104-BRIDGE-EXE-PROOF-ARCHIVE
last_known_owner: phase-29cv Program(JSON)->MIR bridge capsule
replacement: HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate tools/selfhost_exe_stageb.sh
delete_after: bridge capsule has no direct Program(JSON)->MIR callers
restore_command: git mv tools/archive/legacy-selfhost/engineering/phase29ci_selfhost_build_exe_consumer_probe.sh tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
```

## Non-goals

- no `program_json_mir_bridge.sh` deletion
- no `phase29cg_stage2_bootstrap_phi_verify.sh` replacement
- no public `--emit-program-json-v0` deletion
- no ny-llvmc acceptance widening

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/phase29ci_selfhost_build_exe_consumer_probe.sh
bash -n tools/selfhost_exe_stageb.sh
timeout --preserve-status 180s env \
  HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako \
  -o /tmp/p104_stageb_delegate_replacement.exe
test ! -e tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
rg -n "tools/dev/phase29ci_selfhost_build_exe_consumer_probe" \
  docs/development/current/main/design/json-v0-route-map-ssot.md \
  docs/development/current/main/phases/phase-29cv/README.md \
  tools/selfhost/README.md docs/reference/environment-variables.md && exit 1 || true
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
