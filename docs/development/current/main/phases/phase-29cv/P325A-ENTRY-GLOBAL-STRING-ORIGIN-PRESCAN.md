---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv source-execution generated EXE runtime dispatch
Related:
  - docs/development/current/main/phases/phase-29cv/P324A-SOURCE-EXE-NYRT-FRESHNESS-LINK-PROOF.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P325A: Entry Global String Origin Prescan

## Problem

P324A proved that source-execution can emit and link an EXE with a fresh NyRT
archive. Running that generated EXE with `NYASH_STAGE1_MODE=emit-program` still
returns `97`.

The generated `ny_main` shows the dispatcher value coming from:

```text
%r45 = call i64 @"Stage1ModeContractBox.resolve_mode/0"()
```

The first empty-string guard is emitted as a string comparison:

```text
%str_eq_56 = call i64 @"nyash.string.eq_hh"(i64 %r45, i64 %r59)
```

But the later mode checks are emitted as raw handle equality:

```text
%r62 = phi i64 [ %r45, %bb0 ]
%r73 = icmp eq i64 %r62, %r75
```

The env value handle and the const `"emit-program"` handle are distinct runtime
objects, so handle equality misses and the dispatcher falls through to `97`.

## Cause

The entry lowering emits the DirectAbi global call result with `ORG_STRING`
during the actual emit pass, but the earlier prescan does not mark the global
call result as a string from the LoweringPlan route.

PHI origin propagation happens during prescan. Therefore the PHI that carries
the mode value is not marked as string, and later comparisons do not use
`nyash.string.eq_hh`.

## Boundary

Do not widen `generic_string_body`, `generic_i64_body`, or collection method
acceptance.

Do not add a new `GlobalCallTargetShape`.

Do not special-case `Stage1ModeContractBox` or mode names.

This is an entry lowering fact-consumption bug: the prescan must consume the
same existing LoweringPlan DirectAbi return-shape facts that the emit pass
already consumes.

## Implementation

Add prescan origin marking for Global calls with existing LoweringPlan direct
routes:

```text
string_handle / string_handle_or_null -> ORG_STRING
array_handle                         -> ORG_ARRAY_STRING_BIRTH when static string array
map_handle                           -> ORG_MAP_BIRTH for existing map constructor shapes
```

This keeps Stage0 small. The backend is not learning new `.hako` semantics; it
is carrying already-recorded route facts early enough for PHI and compare
lowering.

## Acceptance

Rebuild the C shim:

```text
bash tools/build_hako_llvmc_ffi.sh
```

Regenerate the source-execution EXE:

```text
NYASH_LLVM_ROUTE_TRACE=1 NYASH_EMIT_EXE_NYRT=target/release \
  target/release/hakorune --emit-exe /tmp/hakorune_p325a.exe \
  lang/src/runner/stage1_cli_env.hako
```

The generated `ny_main` must compare dispatcher mode strings with
`nyash.string.eq_hh`, not raw `icmp eq i64`, after the mode PHIs.

Running the generated EXE with the Stage1 emit-program env contract must advance
past return code `97`. Any later return code or blocker is a separate card.

## Result

The C shim rebuild succeeds:

```text
bash tools/build_hako_llvmc_ffi.sh
```

Source-execution EXE generation succeeds:

```text
NYASH_LLVM_ROUTE_TRACE=1 NYASH_EMIT_EXE_NYRT=target/release \
  target/release/hakorune --emit-exe /tmp/hakorune_p325a.exe \
  lang/src/runner/stage1_cli_env.hako
```

The generated object now has string-compare calls immediately after the mode
resolver call:

```text
R_X86_64_PLT32 Stage1ModeContractBox.resolve_mode/0-0x4
R_X86_64_PLT32 nyash.string.eq_hh-0x4
R_X86_64_PLT32 nyash.string.eq_hh-0x4
R_X86_64_PLT32 Main._run_emit_program_mode/0-0x4
```

Running the generated EXE with the Stage1 emit-program env contract advances
past the previous `97` fallthrough:

```text
rc=0
stdout:
23
```
