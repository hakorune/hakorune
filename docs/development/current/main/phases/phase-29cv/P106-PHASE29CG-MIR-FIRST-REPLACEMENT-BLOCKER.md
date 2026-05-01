---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: lock the MIR-first replacement blockers for the remaining phase29cg Program(JSON)->MIR bridge proof.
Related:
  - docs/development/current/main/phases/phase-29cv/P105-PHASE29CG-STAGE1-ARTIFACT-GUARD.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost_exe_stageb.sh
---

# P106 Phase29cg MIR-First Replacement Blocker

## Goal

Prevent the last `phase29cg` bridge cleanup from turning into an ad hoc
backend acceptance patch.

The structurally clean replacement for the bridge section in
`tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` is still:

```bash
SOURCE_TEXT="$(stage1_contract_source_text "$ENTRY")"
stage1_contract_exec_mode "$STAGE1_BIN" emit-mir "$ENTRY" "$SOURCE_TEXT" > "$TMP_MIR"
```

Then the existing `ny_mir_builder.sh --emit obj` and `opt -passes=verify`
checks can stay as the proof body.

## Evidence

The replacement is not green yet. The blockers are outside the
Program(JSON)->MIR bridge helper itself.

### 1. Reduced Stage1 artifact is not a payload source

P105 locked this with a fail-fast guard:

```text
target/selfhost/hakorune.stage1_cli
  artifact_kind=stage1-cli
  entry=lang/src/runner/entry/stage1_cli_env_entry.hako
```

That entry is a run-only bootstrap stub. It is not an emit-capable Stage1 env
artifact for `lang/src/runner/stage1_cli_env.hako`.

### 2. Stage-B mainline-only route does not emit this MIR yet

```bash
mkdir -p /tmp/p106_mainline
timeout --preserve-status 180s \
  bash tools/hakorune_emit_mir_mainline.sh \
  lang/src/runner/stage1_cli_env.hako \
  /tmp/p106_mainline/stage1_cli_env.mir.json
```

Observed:

```text
rc=1
[FAIL] Stage-B failed under mainline-only mode (compat fallback disabled)
```

### 3. Rust direct MIR is useful diagnosis, not Stage1 proof

```bash
timeout --preserve-status 180s env \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  "$PWD/target/release/hakorune" \
  --emit-mir-json /tmp/p106_stage1_cli_env_emit_mir_json.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed:

```text
rc=0
MIR bytes=89239849
```

This proves the Rust binary can materialize MIR for diagnosis. It does not
replace the Stage1 artifact proof.

### 4. Pure-first rejects the first unplanned runtime env call

```bash
NYASH_LLVM_BACKEND=crate \
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
timeout --preserve-status 180s \
  bash tools/ny_mir_builder.sh \
  --in /tmp/p106_stage1_cli_env_emit_mir_json.mir.json \
  --emit obj -o /tmp/p106_stage1_cli_env.o --quiet
```

Observed:

```text
rc=4
[llvm-route/trace] stage=mir_call result=seen reason=enter extra=ii=3 dst=48 recv=0 ctype=Extern bname=- mname=env.get/1 a0=49 a1=0
[llvm-route/trace] stage=mir_call_string_extern result=miss reason=unsupported extra=kind=0 argc=1
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=3 first_op=mir_call owner_hint=backend_lowering reason=mir_call_no_route
unsupported pure shape for current backend recipe
```

The root fix is not a new raw `.inc` matcher. `env.get/1` needs a
LoweringPlan/CoreOp/runtime ABI contract, likely as explicit `ColdRuntime`
while it is not a hot proof.

Post-P108/P109 update:

```text
env.get/1: plan-backed consumer landed
keepalive: pure-first no-op landed
next stop: mir_call Global BuildBox.emit_program_json_v0/2
```

P110 classifies that next stop as Stage1 authority surface, not a backend raw
matcher target.

### 5. Direct EXE build of the full Stage1 env still fails MIR verify

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p106_stage1_cli_env.exe
```

Observed:

```text
rc=1
[freeze:contract][emit-mir/direct-verify] route=mir errors=32
[emit-mir/direct-verify] route=mir detail=Value %604 used in block bb4489 but defined in non-dominating block bb4487
```

This is a MIR/JoinIR/value-flow owner issue. It is not evidence that the
Program(JSON)->MIR bridge should be kept as a hidden fallback.

## Decision

- Keep `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` in the bridge
  capsule until an emit-capable Stage1 env artifact proves the MIR-first
  replacement command above.
- Do not replace the Stage1 proof with Rust direct `--emit-mir-json` output.
- Do not add a ny-llvmc raw MIR matcher for `env.get/1`.
- Treat the next root work as separate owner slices:
  1. Stage1 env artifact capability: produce MIR through
     `stage1_contract_exec_mode ... emit-mir`.
  2. LoweringPlan coverage: represent `env.get/1` as an explicit plan-backed
     runtime ABI call, not compat replay.
  3. MIR dominance: fix the `stage1_cli_env.hako` direct route verifier
     failures in the MIR/JoinIR owner lane.

## Replacement Gate

`phase29cg_stage2_bootstrap_phi_verify.sh` can drop
`program_json_mir_bridge_emit()` only when this full proof is green:

```bash
SOURCE_TEXT="$(stage1_contract_source_text "$ENTRY")"
stage1_contract_exec_mode "$STAGE1_BIN" emit-mir "$ENTRY" "$SOURCE_TEXT" > "$TMP_MIR"

NYASH_LLVM_BACKEND=crate \
NYASH_LLVM_SKIP_BUILD=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
bash tools/ny_mir_builder.sh --in "$TMP_MIR" --emit obj -o "$TMP_OBJ" --quiet

opt -passes=verify "$TMP_IR" -disable-output
```

## Non-goals

- no `phase29cg` bridge replacement in this card
- no `program_json_mir_bridge.sh` deletion
- no `env.get/1` backend matcher
- no `HAKO_BACKEND_COMPAT_REPLAY=harness` promotion
- no direct MIR dominance repair

## Acceptance

```bash
bash -n tools/selfhost_exe_stageb.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
