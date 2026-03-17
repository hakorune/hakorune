---
Status: SSOT
Decision: accepted
Date: 2026-03-18
Scope: backend-zero の `.hako` policy owner と transport-only C substrate の責務分離を、route profile という canonical object で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - lang/src/shared/backend/README.md
  - lang/src/shared/backend/backend_recipe_box.hako
  - lang/src/shared/backend/llvm_backend_box.hako
  - src/host_providers/llvm_codegen.rs
  - crates/nyash-llvm-compiler/src/boundary_driver.rs
  - lang/c-abi/shims/hako_llvmc_ffi.c
---

# Backend Recipe Route Profile SSOT

## Purpose

- backend-zero の route/policy/compat の判断源を 1 つの `.hako` owner に寄せる。
- `BackendRecipeBox` が compile recipe と compat replay だけでなく、route profile の名前と所有者も返すようにして、C shim へ意味を漏らさない。
- `hako_llvmc_ffi.c` は export / marshal / transport fallback に閉じ、seed classification の増殖を抑える。

## Canonical Route Profile

`BackendRecipeBox.compile_route_profile(json_path)` が返す profile は、少なくとも次のキーを持つ。

- `route_profile`
  - stable label for the current route policy
  - example: `backend-zero/pure-first+harness`
- `policy_owner`
  - canonical `.hako` policy owner name
  - example: `BackendRecipeBox`
- `transport_owner`
  - thin transport substrate owner name
  - example: `hako_llvmc_ffi`
- `json_path`
  - normalized MIR JSON path
- `compile_recipe`
  - caller-facing compile recipe name
  - example: `pure-first`
- `compat_replay`
  - compat replay lane name
  - example: `harness`

## Ownership Split

1. `.hako` policy owner
   - `lang/src/shared/backend/backend_recipe_box.hako`
   - decides route profile name and caller-facing recipe policy
   - prepares link recipe normalization
2. `.hako` caller facade
   - `lang/src/shared/backend/llvm_backend_box.hako`
   - consumes the profile and forwards explicit payload to `env.codegen.*`
3. Rust transport boundary
   - `src/host_providers/llvm_codegen.rs`
   - `crates/nyash-llvm-compiler/src/boundary_driver.rs`
   - mirrors explicit payload only at the handoff
4. C export/transport substrate
   - `lang/c-abi/shims/hako_llvmc_ffi.c`
   - exports stable C symbols
   - replays compat transport only

## Stop Line

- Do not move unsupported-shape classification back into the C shim.
- Do not let `hako_llvmc_ffi.c` become a policy owner for daily route decisions.
- Do not add a new canonical ABI surface for backend-zero.
- Do not promote `native_driver.rs` or `llvmlite` to daily owner through the route profile.

## Current Rule

- Daily `.hako` callers should first ask `BackendRecipeBox` for a route profile.
- `LlvmBackendBox` should validate the returned profile and then stop at `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)`.
- Rust and C layers may mirror the same policy names, but they must not invent new policy names.

## Next Expansion Rule

- Broader seed classification belongs here, not in the C shim, when a new exact fixture proves that the route profile needs more evidence.
- Until then, profile growth should stay at the `.hako` owner level and remain visible in `CURRENT_TASK.md` plus phase-29ck docs.
