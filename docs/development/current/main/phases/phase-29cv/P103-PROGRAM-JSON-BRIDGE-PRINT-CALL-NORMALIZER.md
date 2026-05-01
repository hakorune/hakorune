---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: normalize Program(JSON)->MIR bridge console print output at the backend boundary.
Related:
  - docs/development/current/main/phases/phase-29cv/P102-SELFHOST-EXE-STAGEB-DIRECT-DEFAULT.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - src/host_providers/mir_builder.rs
  - src/host_providers/mir_builder/backend_shape.rs
  - src/host_providers/mir_builder/handoff.rs
  - src/runtime/mirbuilder_emit.rs
  - tools/selfhost_exe_stageb.sh
---

# P103 Program(JSON) Bridge Print Call Normalizer

## Goal

Fix the explicit `stageb-delegate` bridge failure without widening ny-llvmc.

Observed failure:

```text
[llvm-pure/unsupported-shape] recipe=pure-first first_block=0 first_inst=1 first_op=externcall owner_hint=mir_normalizer reason=unknown_op
unsupported pure shape for current backend recipe
```

First unsupported instruction:

```json
{"op":"externcall","func":"nyash.console.log","args":[1],"dst":null}
```

The direct route emits the same source-level print as:

```json
{"op":"mir_call","dst":null,"mir_call":{"callee":{"type":"Global","name":"print"},"args":[...],"effects":["IO"],"flags":{}}}
```

## Decision

- Keep ny-llvmc fail-fast for raw `externcall`.
- Keep `Program(JSON v0)` itself unchanged.
- Add a narrow bridge backend-shape normalizer that rewrites only console print
  externcalls in the emitted MIR JSON:

```text
externcall nyash.console.log/env.console.log
  -> mir_call Global print
```

This puts the cleanup at the owner hinted by the diagnostic:

```text
owner_hint=mir_normalizer
```

The normalizer is owned by `host_providers::mir_builder` and is called by both
direct Program(JSON) handoff and runtime-side `env.mirbuilder.emit`. This keeps
the bridge cleanup in one place instead of adding another route policy in
ny-llvmc.

## Non-goals

- no generic `externcall` acceptance in ny-llvmc
- no broad Program(JSON v0) schema widening
- no archive/delete of bridge probes in this card
- no new env toggle

## Acceptance

```bash
cargo test -q -p nyash-rust --lib host_providers::mir_builder::backend_shape
cargo test -q -p nyash-rust --lib env_mirbuilder_emit_normalizes_console_print_for_backend_boundary
bash -n tools/selfhost_exe_stageb.sh
cargo build --release --bin hakorune
timeout --preserve-status 180s env \
  HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  NYASH_LLVM_ROUTE_TRACE=1 \
  bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako \
  -o /tmp/p103_stageb_delegate.exe
NYASH_NYRT_SILENT_RESULT=1 /tmp/p103_stageb_delegate.exe
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected route trace includes:

```text
stage=generic_walk result=seen reason=mir_call extra=ii=1 dst=0 op=mir_call
stage=mir_call result=seen reason=enter extra=ii=1 dst=0 recv=0 ctype=Global bname=- mname=print
```
