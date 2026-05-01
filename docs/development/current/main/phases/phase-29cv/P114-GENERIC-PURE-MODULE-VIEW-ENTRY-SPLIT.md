---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: make ny-llvmc generic pure read a module view while still emitting only
  the selected entry function.
Related:
  - docs/development/current/main/phases/phase-29cv/P113-GLOBAL-CALL-TARGET-CONTRACT-INVENTORY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P114 Generic Pure Module View Entry Split

## Stop Line

P113 made same-module global user calls diagnose as
`missing_multi_function_emitter`. The next implementation must not jump
straight into a `Stage1ModeContractBox.*` matcher. The generic pure emitter has
to learn module structure first.

Before this card, `hako_llvmc_generic_pure_program_view` carried only:

```text
selected fn
selected fn metadata
selected fn blocks
```

That made the implementation shape read as "one MIR function exists". The
actual input is a MIR module with `functions[]`, and entry selection is only one
view over that module.

## Contract

Generic pure now reads:

```text
functions[]
function_count
selected entry fn
entry_index
selected fn metadata
selected fn blocks
```

This is behavior-preserving. P114 does not emit helper functions and does not
make `UserGlobalCall` lowerable. It only gives the next card a stable owner seam
for module-wide prepass and per-function body emission.

## Next Implementation Boundary

The multi-function emitter card should build on this order:

```text
module view
  -> module-wide declaration/need scan
  -> per-function generic pure state reset
  -> function body emission
  -> plan-first UserGlobalCall call sites
```

The entry function remains the only public runtime entry until the entry ABI is
explicitly widened. Same-module functions must use quoted LLVM symbol names
derived from the plan `symbol` / `callee_name`, not backend-local name policy.

## Acceptance

```bash
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/p114_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
NYASH_LLVM_ROUTE_TRACE=1 \
target/release/ny-llvmc --in /tmp/p114_stage1_cli_env.mir.json \
  --emit obj --out /tmp/p114_stage1_cli_env.o
```

The last command still fails by design:

```text
reason=missing_multi_function_emitter
```

