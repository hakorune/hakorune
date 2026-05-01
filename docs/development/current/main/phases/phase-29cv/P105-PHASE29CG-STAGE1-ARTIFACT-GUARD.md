---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: guard the remaining phase29cg bridge proof against the reduced run-only stage1-cli artifact.
Related:
  - docs/development/current/main/phases/phase-29cv/P104-BRIDGE-EXE-PROOF-ARCHIVE.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/mainline/build_stage1.sh
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/entry/stage1_cli_env_entry.hako
---

# P105 Phase29cg Stage1 Artifact Guard

## Goal

Prepare the last direct `program_json_mir_bridge_emit()` caller for a clean
replacement without changing the proof target.

The intended replacement is:

```bash
stage1_contract_exec_mode "$STAGE1_BIN" emit-mir "$ENTRY" "$SOURCE_TEXT" > "$TMP_MIR"
```

Then the existing `ny_mir_builder.sh --emit obj` and `opt -passes=verify`
checks can stay unchanged.

## Observation

The script-level replacement is structurally clean, but the current default
artifact is not the right proof source:

```text
target/selfhost/hakorune.stage1_cli
  artifact_kind=stage1-cli
  entry=lang/src/runner/entry/stage1_cli_env_entry.hako
```

That entry is a thin run-only stub. It is not the logical Stage1 env authority
`lang/src/runner/stage1_cli_env.hako`, and it cannot currently emit the
Program(JSON) or MIR payload that `phase29cg_stage2_bootstrap_phi_verify.sh`
expects.

Without a guard, the probe fails later as a marker mismatch, which looks like a
bridge problem even though the selected artifact is the wrong owner.

## Decision

- Keep `phase29cg_stage2_bootstrap_phi_verify.sh` in the bridge capsule for now.
- Add a Stage1 contract helper that reads the artifact entry from
  `<artifact>.artifact_kind`.
- Fail-fast in `phase29cg_stage2_bootstrap_phi_verify.sh` when the selected
  artifact is the reduced run-only `stage1-cli` built from
  `entry/stage1_cli_env_entry.hako`.
- Do not replace `program_json_mir_bridge_emit()` until an emit-capable Stage1
  env artifact proves `stage1_contract_exec_mode ... emit-mir` and the
  downstream LLVM verifier.

## Non-goals

- no `phase29cg` archive
- no `program_json_mir_bridge.sh` deletion
- no public `--emit-program-json-v0` deletion
- no direct MIR dominance fix in `stage1_cli_env.hako`
- no Stage1 artifact kind redesign

## Acceptance

```bash
bash -n tools/selfhost/lib/stage1_contract.sh
bash -n tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
tools/selfhost/mainline/build_stage1.sh --artifact-kind stage1-cli --reuse-if-fresh 1 --timeout-secs 240
set +e
OUT_DIR=/tmp/p105_phase29cg_guard KEEP_OUT_DIR=1 \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh \
  >/tmp/p105_phase29cg_guard.log 2>&1
rc=$?
set -e
test "$rc" -ne 0
grep -F "reduced run-only stage1-cli artifact cannot emit Program/MIR payloads" /tmp/p105_phase29cg_guard.log
bash tools/checks/current_state_pointer_guard.sh
SMOKES_ENABLE_SELFHOST=1 NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
git diff --check
```

## Next

The next clean replacement card must make this command green for a Stage1
artifact, then update `phase29cg_stage2_bootstrap_phi_verify.sh` to consume that
MIR directly:

```bash
stage1_contract_exec_mode "$STAGE1_BIN" emit-mir "$ENTRY" "$SOURCE_TEXT"
```
