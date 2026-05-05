---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381CB, source MIR callsite cleanup for BuildBox Program(JSON) route
Related:
  - docs/development/current/main/phases/phase-29cv/P380X-STAGE1-EMIT-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P381CA-UNIFORM-MIR-FUNCTION-CANDIDATE-EMIT.md
  - src/mir/passes/callsite_canonicalize.rs
  - src/mir/extern_call_route_plan.rs
  - lang/src/mir/builder/MirBuilderBox.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P381CB: BuildBox Program(JSON) Extern Canonicalize

## Problem

P381CA made ny-llvmc follow selected same-module
`missing_multi_function_emitter` targets. The next stop moved inside
`BuildBox._parse_program_json/2`, whose live source-owner body creates
`ParserBox` and calls `ParserBox.parse_program2`.

Teaching Stage0 to lower that parser body in C would duplicate compiler
source-owner semantics. The source-only Program(JSON) authority already has a
MIR-owned extern contract from P380X:

```text
BuildBox.emit_program_json_v0(source, null)
  -> nyash.stage1.emit_program_json_v0_h(source)
```

## Decision

Apply the same contract to canonical source MIR callsites. The callsite
canonicalizer rewrites only the exact static-null route:

```text
Call Global BuildBox.emit_program_json_v0/2(source, null-or-void)
  -> Call Extern nyash.stage1.emit_program_json_v0_h(source)
```

This is source-owner cleanup, not a new Stage0 parser body shape.

The MirBuilder source-entry compat path now preserves that source-only contract
at its owner boundary: non-null `opts` fail fast locally, while the raw leaf
calls `BuildBox.emit_program_json_v0(source, null)` directly so canonical MIR
can bind the extern route.

## Rules

Allowed:

- rewrite the explicit source-only BuildBox route with static null/void opts
- keep the route as canonical MIR `Callee::Extern`
- let `extern_call_routes` publish the existing Stage1 Program(JSON) contract

Forbidden:

- accepting non-null opts
- adding ParserBox method lowering to ny-llvmc
- adding another `GlobalCallTargetShape` or C shim body classifier

## Acceptance

```bash
cargo test --release callsite_canonicalize -- --nocapture
target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_stage1_cli_env_parse_probe.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the Stage1 source Program(JSON) callsite is an extern route, so the
BuildBox parser body no longer becomes the next selected same-module callee.

## Result

Accepted. The canonicalizer now rewrites both live source-only Program(JSON)
callsites in the Stage1 CLI MIR:

```text
MirBuilderSourceCompatBox._emit_program_json_from_source_raw/2
  -> nyash.stage1.emit_program_json_v0_h
Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
  -> nyash.stage1.emit_program_json_v0_h
```

The module generic prepass now consumes the shared extern-call view for both
`mir_call` and `externcall` surfaces, so those extern raw leaves can be emitted
as normal same-module definitions.

While validating the advanced path, two emitter ownership issues surfaced and
were fixed in the same owner cleanup:

- planned same-module definitions no longer receive redundant LLVM
  declarations before their definitions
- `generic_i64_body` calls whose target body is a numeric leaf are planned under
  the numeric leaf emitter owner

Validation:

```bash
cargo test --release callsite_canonicalize -- --nocapture
target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_stage1_cli_env_parse_probe.ll \
  target/release/ny-llvmc \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_stage1_cli_env_parse_probe.o
```

The `ny-llvmc` probe now advances past the BuildBox parser body and the Stage1
Program(JSON) raw leaves. The current next blocker is:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=LowerLoopMultiCarrierBox._emit_multi_count_json/7
```
