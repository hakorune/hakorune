---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380M, phase29cg explicit OUT_DIR guard
Related:
  - docs/development/current/main/phases/phase-29cv/P380K-STAGE3-SAME-RESULT-BUILD-SEED.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
---

# P380M: phase29cg OUT_DIR Guard

## Problem

After P380K, an emit-capable full Stage1 env seed can be built at:

```text
target/selfhost/hakorune.stage1_cli_env_seed
```

Remeasuring the bridge keeper with an explicit output directory failed before
the real proof:

```text
tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh: line 61: /tmp/p380m_phase29cg/stage1_cli_env.program.json: No such file or directory
```

The script uses `mktemp -d` by default, but does not create a caller-provided
`OUT_DIR`.

## Decision

Create `OUT_DIR` before deriving output files. This is a probe hygiene fix only.

## Non-Goals

- no route change
- no Program(JSON)->MIR bridge retirement
- no Stage0 route/classifier change
- no C shim change

## Acceptance

```bash
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380m_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: helper reaches the real bridge/LLVM result instead of failing on a
missing output directory.

## Result

Implemented.

The helper now creates caller-provided `OUT_DIR` before deriving output files.
Remeasuring with an explicit directory no longer fails on a missing
`stage1_cli_env.program.json` path.

The probe now reaches the real bridge keeper boundary and fails there:

```text
[phase29cg] env.mirbuilder.emit wrapper failed (rc=1)
[ERROR] [vm/error] Invalid instruction: [freeze:contract][mirbuilder/delegate-forbidden] env.mirbuilder.emit blocked (HAKO_SELFHOST_NO_DELEGATE=1)
```

This keeps the bridge keeper honest: `OUT_DIR` hygiene is fixed, while the
remaining blocker is the explicit Program(JSON)->MIR bridge delegate boundary,
not output directory setup.
