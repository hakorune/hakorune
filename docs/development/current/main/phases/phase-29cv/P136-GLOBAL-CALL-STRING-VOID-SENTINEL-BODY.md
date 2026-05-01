---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P136, generic string-or-void sentinel body target shape
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P128-GLOBAL-CALL-VOID-SENTINEL-RETURN-REASON.md
  - docs/development/current/main/phases/phase-29cv/P129-GLOBAL-CALL-VOID-SENTINEL-BODY-BLOCKER.md
  - docs/development/current/main/phases/phase-29cv/P135-GLOBAL-CALL-STRING-PRINT-SIDE-EFFECT.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P136: Global Call String Void Sentinel Body

## Problem

After P135, the source-execution blocker moved past debug `print` to:

```text
target_shape_reason=generic_string_return_void_sentinel_candidate
target_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
```

P128/P129 intentionally kept this as diagnosis only. The active function now
uses the already-supported generic string body subset and returns either a
string handle or a void/null sentinel.

## Decision

Promote validated string-or-void sentinel candidates to a dedicated lowerable
target shape:

```text
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
```

The same module generic string emitter lowers the body. Void/null sentinel
returns are emitted as i64 zero, which is already how the generic string emitter
represents sentinel constants.

## Rules

Allowed:

- canonical returns that are all string handles or void/null sentinel values
- the same instruction subset accepted by the generic pure string body scan
- direct same-module calls through the new shape/proof only

Forbidden:

- treating `generic_string_return_void_sentinel_candidate` itself as permission
- accepting unsupported method/backend/global body blockers
- changing `.hako` source to avoid the sentinel
- externalizing the same-module target

## Expected Evidence

After this card, calls to
`Stage1InputContractBox.resolve_emit_program_source_text/0` should carry:

```text
target_shape=generic_string_or_void_sentinel_body
return_shape=string_handle_or_null
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q build_mir_json_root_emits_string_or_void_sentinel_route` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-exe ... stage1_cli_env.hako` advances past
  the string-or-void sentinel blocker.
