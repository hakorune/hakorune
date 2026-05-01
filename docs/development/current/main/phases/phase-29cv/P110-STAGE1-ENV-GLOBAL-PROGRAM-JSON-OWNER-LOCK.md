---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify the next full Stage1 env pure-first stop after P108/P109.
Related:
  - docs/development/current/main/phases/phase-29cv/P106-PHASE29CG-MIR-FIRST-REPLACEMENT-BLOCKER.md
  - docs/development/current/main/phases/phase-29cv/P108-ENV-GET-LOWERING-PLAN-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P109-KEEPALIVE-PURE-FIRST-NOOP.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/selfhost/mainline/build_stage1.sh
---

# P110 Stage1 Env Global Program JSON Owner Lock

## Goal

Keep the next pure-first stop from turning into a ny-llvmc raw global matcher.

After P108 and P109, the Rust direct diagnostic MIR for
`lang/src/runner/stage1_cli_env.hako` gets past:

- plan-backed `extern env.get/1`
- canonical MIR `keepalive`

The next stop is a global user call:

```text
[llvm-route/trace] stage=mir_call result=seen reason=enter extra=ii=7 dst=684 recv=0 ctype=Global bname=- mname=BuildBox.emit_program_json_v0/2 a0=685 a1=688
[llvm-pure/unsupported-shape] recipe=pure-first first_block=15020 first_inst=7 first_op=mir_call owner_hint=backend_lowering reason=mir_call_no_route
unsupported pure shape for current backend recipe
```

This is not an `EnvGet`-style missing runtime ABI row. It is the current
source-to-Program(JSON v0) authority surface inside the full Stage1 env source.

## Current Role Split

`tools/selfhost/mainline/build_stage1.sh --artifact-kind stage1-cli` uses the
thin run-only bootstrap entry:

```text
lang/src/runner/entry/stage1_cli_env_entry.hako
```

That entry carries no CLI policy and no source-to-Program(JSON v0) authority.
It remains the right reduced artifact entry. A diagnostic pure-first compile of
that entry is green:

```bash
target/release/hakorune \
  --emit-mir-json /tmp/p110_stage1_cli_env_entry.mir.json \
  lang/src/runner/entry/stage1_cli_env_entry.hako

HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc \
  --in /tmp/p110_stage1_cli_env_entry.mir.json \
  --emit obj \
  --out /tmp/p110_stage1_cli_env_entry.o
```

Observed:

```text
functions=13
MIR bytes=62206
object bytes=920
```

The full env source is different:

```text
lang/src/runner/stage1_cli_env.hako
```

It intentionally carries the Stage1 authority cluster, including
`Stage1SourceProgramAuthorityBox` and its checked
`BuildBox.emit_program_json_v0(...)` handoff. That full source is still needed
as the logical owner for exact Stage1 env emit contracts and for the remaining
`phase29cg` bridge replacement proof.

## Decision

- Do not add a raw ny-llvmc matcher for `BuildBox.emit_program_json_v0/2`.
- Do not treat `BuildBox.emit_program_json_v0/2` as a new generic
  `ColdRuntime` helper just to make the full Stage1 env compile.
- Do not read the reduced run-only entry as an emit-capable Stage1 env
  replacement.
- Keep `BuildBox.emit_program_json_v0(...)` as the sole source-to-Program(JSON
  v0) authority until the Program(JSON v0) capsule is deleted or split.
- Treat this stop as an owner/route split problem:
  - reduced bootstrap artifact entry: keep thin and run-only
  - full Stage1 env source: authority cluster, currently not pure-first backend
    proof
  - MIR-first bridge replacement: still waits for an emit-capable Stage1 env
    artifact or a narrower emit-MIR-only owner that does not carry Program(JSON)
    authority into the backend proof

## Next Clean Options

Only these are acceptable next roots:

1. add a dedicated emit-MIR-only Stage1 env owner/entry if the remaining
   `phase29cg` proof needs an artifact with payload emission but not
   `emit-program`;
2. introduce a typed user/global-call LoweringPlan family if the compiler lane
   decides to lower same-module static user calls broadly;
3. keep the bridge capsule until option 1 or 2 is proven.

The rejected shortcut is:

```text
if callee == "BuildBox.emit_program_json_v0/2" in .inc, lower something
```

That would reopen Program(JSON v0) as backend-local policy.

## Acceptance

```bash
target/release/hakorune \
  --emit-mir-json /tmp/p110_stage1_cli_env_entry.mir.json \
  lang/src/runner/entry/stage1_cli_env_entry.hako

HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc \
  --in /tmp/p110_stage1_cli_env_entry.mir.json \
  --emit obj \
  --out /tmp/p110_stage1_cli_env_entry.o

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
