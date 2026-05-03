---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: remeasure the phase29cg MIR-first replacement blocker after Program(JSON) caller guards
Related:
  - docs/development/current/main/phases/phase-29cv/P106-PHASE29CG-MIR-FIRST-REPLACEMENT-BLOCKER.md
  - docs/development/current/main/phases/phase-29cv/P372A-PROGRAM-JSON-MIR-BRIDGE-CALLER-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P373A-STAGEB-PROGRAM-JSON-CAPTURE-CALLER-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P374A-STAGE1-PROGRAM-JSON-COMPAT-CALLER-GUARD.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/selfhost_exe_stageb.sh
---

# P375A: Phase29cg MIR-First Direct-Verify Remeasure

## Intent

Lock the next owner before touching implementation.

P372A-P374A narrowed active Program(JSON) bridge callers. The remaining
`phase29cg` replacement proof is still blocked, but the blocker is not an
argument for adding another Program(JSON) shell route or widening a Stage0 body
shape.

## Current Evidence

The bridge proof still stops before proving an emit-capable Stage1 env
artifact:

```bash
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p375_phase29cg_probe \
  bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
```

Observed:

```text
rc=1
[FAIL] phase29cg_stage2_bootstrap_phi_verify: reduced run-only stage1-cli artifact cannot emit Program/MIR payloads
artifact_entry=lang/src/runner/entry/stage1_cli_env_entry.hako
required: emit-capable Stage1 env artifact for lang/src/runner/stage1_cli_env.hako
```

The direct full Stage1 env EXE build still fails MIR verification:

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p375_stage1_cli_env.exe
```

Observed:

```text
rc=1
[freeze:contract][emit-mir/direct-verify] route=mir errors=24
```

This is improved from P106's `errors=32`, but still not green.

## Residual Dominance Clusters

A strict no-verify MIR capture maps the residual verifier failures to these
source owners:

```text
BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
JsonFragBox.read_float_from/2
JsonFragNormalizerBox._canonicalize_f64_str/1
JsonFragNormalizerBox._read_num_token/2
JsonNumberCanonicalBox.canonicalize_f64/1
JsonNumberCanonicalBox.read_num_token/2
ParserControlBox.parse_if/4
```

The common symptom is a value produced inside a branch of a short-circuit or
conditional expression being used from a non-dominating block. The next owner is
therefore the MIR/JoinIR value-flow or PHI construction boundary, not a bridge
caller cleanup.

## Rejected Cleanup

A broad source-side split of compound conditions in
`lang/src/mir/builder/internal/fallback_authority_box.hako` was tested as a
diagnostic only.

Result:

```text
isolated strict emit timed out at 60s before direct verify
```

Decision:

- do not keep broad `.hako` condition splitting as the repair
- do not treat source rewrites as the primary owner for this blocker
- fix the narrow compiler owner that creates non-dominating incoming values

## Decision

- keep the Program(JSON)->MIR bridge proof capsule until the P106 replacement
  gate is green
- do not add new Program(JSON) shell routes
- do not add another Stage0 body shape or C shim body emitter for this blocker
- proceed with a narrow MIR/JoinIR dominance repair, starting from the
  short-circuit/conditional value-flow owner

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
