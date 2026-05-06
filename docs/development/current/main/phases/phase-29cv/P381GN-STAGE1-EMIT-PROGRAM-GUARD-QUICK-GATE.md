# P381GN Stage1 Emit Program Guard Quick Gate

Date: 2026-05-06
Scope: wire the Stage1 emit-program runtime-helper guard into the daily quick gate.

## Context

P381GM repaired the public Stage1 Program(JSON v0) lowering seam:

```text
BuildBox.emit_program_json_v0(source, null)
  -> nyash.stage1.emit_program_json_v0_h(source)
```

It also added a focused guard:

```bash
bash tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
```

The guard was documented in the check-script index, but it was still only a
manual check. That made the repaired seam easier to regress than the neighboring
Program(JSON) surface guards already covered by `dev_gate.sh quick`.

## Change

- Added `tools/checks/stage1_emit_program_json_runtime_helper_guard.sh` to the
  `quick` profile list in `tools/checks/dev_gate.sh --list`.
- Added the guard to `run_quick()` immediately after the Stage0 shape inventory
  guard and before broader Program(JSON) surface guards.

## Result

The daily quick gate now checks that the repaired public Stage1 emit-program
runtime-helper route remains consumable by Stage0:

```text
current-state pointer
Stage0 shape inventory
Stage1 emit-program runtime-helper
Program(JSON) surface guards
...
```

This is smoke organization only. It does not delete smokes, change lowering
semantics, or widen Stage0 acceptance.

## Validation

```bash
bash tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
