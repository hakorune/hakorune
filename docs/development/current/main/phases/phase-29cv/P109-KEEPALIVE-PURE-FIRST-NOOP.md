---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume canonical MIR `keepalive` as a pure-first no-op.
Related:
  - docs/reference/language/lifecycle.md
  - docs/reference/mir/INSTRUCTION_SET.md
  - docs/development/current/main/phases/phase-29cv/P108-ENV-GET-LOWERING-PLAN-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
  - apps/tests/mir_shape_guard/keepalive_noop_min_v1.mir.json
---

# P109 KeepAlive Pure-First No-Op

## Goal

Move past the next pure-first stop after P108:

```text
first_op=keepalive
reason=unknown_op
```

`KeepAlive` is canonical MIR lifecycle intent. It has no runtime side effect in
execution backends; it exists to preserve liveness for analysis and DCE.

## Decision

- Treat `op=keepalive` as a no-op in ny-llvmc generic pure lowering.
- Keep `release_strong` behavior unchanged.
- Add a minimal MIR shape fixture proving pure-first accepts `keepalive`.

## Non-goals

- no lifecycle policy redesign
- no DCE change
- no `release_strong` expansion
- no hidden compat replay

## Follow-up Observation

With P108 and P109 applied, the full Stage1 env direct MIR moves past
`env.get/1` and `keepalive`. The next pure-first stop is:

```text
first_op=mir_call
callee=Global BuildBox.emit_program_json_v0/2
reason=mir_call_no_route
```

That is not part of this card; it needs a separate owner decision because the
callee name is a compat Program(JSON v0) surface.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/keepalive_noop_min_v1.mir.json \
  --out /tmp/p109_keepalive_noop.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
