---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv source-execution generated EXE emit-program stdout
Related:
  - docs/development/current/main/phases/phase-29cv/P325A-ENTRY-GLOBAL-STRING-ORIGIN-PRESCAN.md
  - docs/development/current/main/phases/phase-29cv/P135-GLOBAL-CALL-STRING-PRINT-SIDE-EFFECT.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P326A: Global Print String Handle Marshal

## Problem

P325A moves the generated `stage1_cli_env.hako` EXE through mode dispatch and
the emit-program validator:

```text
rc=0
stdout:
23
```

The return code shows that `Stage1ProgramResultValidationBox` accepted the
Program(JSON v0) text, but stdout contains the runtime handle id instead of the
string payload.

The active `Global print` lowering always emits an integer printf surface:

```text
printf("%ld\n", value)
```

For string handles this prints the handle number.

## Boundary

Do not add a new body classifier or `GlobalCallTargetShape`.

Do not make `print` a user/global function route.

Do not teach Stage0 Program(JSON) semantics.

This is the already-supported backend global `print` surface from P135. The
missing piece is argument marshalling: when existing origin facts say the
argument is a string handle, call the NyRT console handle helper instead of
printing the numeric handle.

## Implementation

For `Global print`:

```text
string origin / string scan origin -> nyash.console.log_handle(handle)
otherwise                         -> existing printf("%ld\n", scalar)
```

Declare `nyash.console.log_handle(i64) -> i64` when the print surface is needed.

This keeps scalar debug prints unchanged and fixes string payload printing for
the generated source-execution EXE.

## Acceptance

Rebuild the C shim:

```text
bash tools/build_hako_llvmc_ffi.sh
```

Regenerate and run the source-execution EXE in emit-program mode:

```text
NYASH_LLVM_ROUTE_TRACE=1 NYASH_EMIT_EXE_NYRT=target/release \
  target/release/hakorune --emit-exe /tmp/hakorune_p326a.exe \
  lang/src/runner/stage1_cli_env.hako
```

Expected runtime behavior:

```text
rc=0
stdout starts with {"version":0,"kind":"Program"
stdout is not a small numeric handle id
```

## Result

The C shim rebuild succeeds:

```text
bash tools/build_hako_llvmc_ffi.sh
```

Source-execution EXE generation succeeds:

```text
NYASH_LLVM_ROUTE_TRACE=1 NYASH_EMIT_EXE_NYRT=target/release \
  target/release/hakorune --emit-exe /tmp/hakorune_p326a.exe \
  lang/src/runner/stage1_cli_env.hako
```

The generated object uses `nyash.console.log_handle` for string-origin print
arguments while retaining `printf` for scalar prints:

```text
R_X86_64_PLT32 nyash.console.log_handle-0x4
R_X86_64_PLT32 printf-0x4
```

Running the generated EXE in emit-program mode now prints Program(JSON v0)
payload text instead of a handle id:

```text
rc=0
stdout:
{"body":[{"expr":{"type":"Int","value":0},"type":"Return"}],"kind":"Program","user_box_decls":[{"field_decls":[],"fields":[],"name":"Main","type_parameters":[]}],"version":0}
```
