---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AL, MIR JSON phi incoming i64 seed cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AK-RETURN-LOGICAL-TEXT-CARRIERS.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P381AL: MIR JSON PHI Incoming I64 Seeds

## Problem

After P381AK, the direct Stage1 env EXE route reaches:

```text
MirJsonEmitBox._emit_phi_incoming_rec/3
%r68 = phi i64 [ %r56, ... ], [ %r58, ... ]
%r58 defined with type i1 but expected i64
```

The emitter seeds `value_id` and `block_id` with literal `0` and later merges
those locals with ArrayBox item payloads. The pure-first path infers the literal
seed as Bool in the missing-item branch.

## Decision

Use explicit i64 seeds for PHI incoming numeric carriers.

## Boundary

Allowed:

- make `value_id`/`block_id` seeds explicit i64 values
- keep emitted MIR JSON unchanged

Not allowed:

- add backend i1-to-i64 PHI repair
- change PHI incoming shape
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381al_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `MirJsonEmitBox._emit_phi_incoming_rec/3` no longer merges Bool seeds with
  i64 incoming ids

## Result

Implemented. `value_id` and `block_id` now seed with explicit i64 zero. The
direct Stage1 env EXE route progressed past
`MirJsonEmitBox._emit_phi_incoming_rec/3` and exposed the next shared
Compare-side carrier issue in `PatternUtilBox.find_local_bool_before/3`.
