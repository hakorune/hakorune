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
  - daily example: `backend-zero/pure-first+none`
  - explicit keep example: `backend-zero/pure-first+harness-keep`
- `policy_owner`
  - canonical `.hako` policy owner name
  - example: `BackendRecipeBox`
- `transport_owner`
  - thin transport substrate owner name
  - example: `hako_llvmc_ffi`
- `acceptance_policy`
  - stable label for the current pure/compat acceptance basis
  - example: `boundary-pure-seed-matrix-v1`
- `acceptance_case`
  - shape-specific evidence row owned by `.hako`
  - example: `runtime-data-array-get-missing-v1`
- `json_path`
  - normalized MIR JSON path
- `compile_recipe`
  - caller-facing compile recipe name
  - example: `pure-first`
- `compat_replay`
  - compat replay lane name
  - daily example: `none`
  - explicit keep example: `harness`
- `legacy_daily_allowed`
  - stable daily-demotion guard mirrored through the Rust bridge
  - daily `.hako ll emitter` examples: `no`
  - legacy C / compat keep examples: `yes`

## Grouped Evidence Buckets

- `acceptance_case` の個々の名前は transport trivia ではなく、`BackendRecipeBox.compile_route_profile(...)` が owned する grouped evidence bucket だと読む。
- `legacy_daily_allowed` is route-policy truth, not transport-local trivia; Rust/C may mirror it but must not invent or widen it.
- 現在の evidence bucket は次の粒度で維持する。
  - seed / pure-first baseline
    - `ret-const-v1`
    - `hello-simple-llvm-native-probe-v1`
  - string evidence
    - `array-string-indexof-branch-v1`
    - `array-string-indexof-cross-block-select-v1`
    - `array-string-indexof-interleaved-branch-v1`
    - `array-string-indexof-interleaved-select-v1`
    - `array-string-indexof-select-v1`
    - `string-length-ascii-v1`
    - `string-indexof-ascii-v1`
  - runtime-data evidence
    - `runtime-data-array-get-missing-v1`
    - `runtime-data-string-length-ascii-v1`
    - `runtime-data-array-length-v1`
    - `runtime-data-array-push-v1`
    - `runtime-data-map-size-v1`
    - `runtime-data-array-has-missing-v1`
    - `runtime-data-map-has-missing-v1`
    - `runtime-data-map-get-missing-v1`
  - compat keep evidence
    - `method-call-only-small-compat-v1`
- 新しい bucket は、新しい exact fixture が classification family の不足を証明したときだけ追加する。
- `method-call-only-small-compat-v1` is evidence ownership only; it does not promote the seed into the supported pure-first lane.

## Ownership Split

1. `.hako` policy owner
   - `lang/src/shared/backend/backend_recipe_box.hako`
   - decides route profile name, caller-facing recipe policy, explicit keep profile naming, and visible acceptance evidence rows
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

- Daily `.hako` callers should first ask `BackendRecipeBox.compile_route_profile(...)` for the mainline route profile.
- Root-first daily callers may ask `BackendRecipeBox.compile_root_profile(...)` when they already hold a hydrated MIR root.
- Explicit compat keep callers should ask `BackendRecipeBox.compile_keep_profile(..., "harness")`.
- `LlvmBackendBox` should validate the returned profile field values against `BackendRecipeBox` owner names and route evidence, then:
  - compile `hako_ll_emitter` daily profiles through `root -> facts -> ll text -> env.codegen.compile_ll_text(...)`
  - keep explicit legacy/compat callers on the explicit compat helper (`src/host_providers/llvm_codegen/legacy_mir_front_door.rs::compile_object_from_legacy_mir_json(...)`)
  - keep link handoff on `env.codegen.link_object(...)`
- `LlvmBackendBox` should mirror `acceptance_case`, `transport_owner`, and `legacy_daily_allowed` through env only at the backend handoff; bridge/provider layers must treat them as read-only payload.
- Rust and C layers may mirror the same policy names, but they must not invent new policy names.
- `acceptance_case` growth must stay grouped at the `.hako` policy owner; do not add per-case transport ownership in Rust/C.

## Next Expansion Rule

- Broader seed classification belongs here, not in the C shim, when a new exact fixture proves that the route profile needs another evidence bucket.
- current state is close-synced: the route profile is no longer an active widening front.
- until a fresh exact blocker appears, profile growth stays frozen at the current grouped evidence rows and the next backend-zero front moves to boundary fallback reliance reduction.

## Final Shape

1. `.hako` policy owner
- `BackendRecipeBox`
  - owns compile recipe, compat replay, route-profile naming, narrow pure-seed acceptance policy, and shape-specific acceptance-case naming
2. `.hako` caller facade
   - `LlvmBackendBox`
   - validates the route profile and stops at `env.codegen.*`
3. Rust glue
   - payload decode, symbol selection, boundary-call glue only
4. C substrate
   - `extern "C"` export, allocator/error ownership, loader/process/path glue, and compat transport execution only

## Clean Stop Line For This Wave

- Stop after `BackendRecipeBox` is the only visible owner for route profile and recipe naming.
- Stop after daily mainline is visibly `pure-first + compat_replay=none`, while `harness` remains an explicit keep-only profile.
- Stop after `BackendRecipeBox` is also the only visible owner for acceptance-policy naming and acceptance-case naming.
- Stop after `LlvmBackendBox` reads the profile and transport layers only mirror it.
- Current result:
  - this stop line is landed for the current evidence rows, including `method-call-only-small-compat-v1`
  - reopen only for a new narrow evidence row proven by a real `phase-29ck` blocker
- Do not keep thinning `hako_aot_shared_impl.inc` without a fresh exact blocker.
- Do not move broad CFG/pattern acceptance logic out of `hako_llvmc_ffi.c` in the same wave as policy handoff.
- Do not mix one more pure-seed widening with transport refactor unless the route profile needs that evidence.

## Explicit Non-Goals Right Now

- Replacing `hako_llvmc_ffi.c` with `.hako`
- Promoting `native_driver.rs` to daily owner
- Turning `boundary_driver.rs` or `llvm_codegen.rs` into shape-policy owners
- Adding a new ABI surface or transport contract
