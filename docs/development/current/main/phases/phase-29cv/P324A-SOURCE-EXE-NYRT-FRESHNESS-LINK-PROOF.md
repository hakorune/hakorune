---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv source-execution link-stage proof after P323A
Related:
  - docs/development/current/main/phases/phase-29cv/P323A-MODULE-GENERIC-MAP-KEYS-DECLARE.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P324A: Source EXE NyRT Freshness Link Proof

## Problem

P323A moved the source-execution probe past LLVM IR verification:

```text
opt -S -passes=mem2reg /tmp/hako_p323_probe.ll -o /tmp/hako_p323_mem2reg.ll
```

The next observed stop was object link failure:

```text
Error: /usr/bin/ld: /tmp/hakorune_p323.o: in function `BuilderProgramJsonInputContractBox._program_json_header_present/1':
ny-llvmc failed with status: Some(1)
```

The generated object carried unresolved runtime helper symbols such as:

```text
nyash.map.keys_h
nyash.array.slot_append_hh
nyash.runtime_data.get_hh
nyash.stage1.emit_program_json_v0_h
nyash.string.substring_hii
```

This looked like a new linker/runtime blocker, but the helper definitions
already exist in `crates/nyash_kernel`.

## Boundary

Do not widen Stage0 body classifiers for this issue.

This is not a `generic_string_body` or collection-semantics blocker. The
backend emitted valid helper calls, and `opt` accepted the module. The remaining
condition is that the NyRT static archive linked by `--emit-exe` must be fresh
enough to contain the helper symbols referenced by the current lowering plan.

## Action

Rebuild the NyRT archive before the source-execution link proof:

```text
cargo build -p nyash_kernel --release
```

Then verify the runtime symbol surface with `llvm-nm`:

```text
llvm-nm -g --defined-only target/release/libnyash_kernel.a
```

Relevant symbols now resolve:

```text
T nyash.array.birth_h
T nyash.array.slot_len_h
T nyash.box.from_i8_string_const
T nyash.map.birth_h
T nyash.map.keys_h
T nyash.runtime_data.get_hh
T nyash.stage1.emit_program_json_v0_h
T nyash.string.len_h
```

Manual link of the P324 object with the fresh archive succeeds:

```text
cc -no-pie -Wl,--gc-sections -o /tmp/hakorune_p324_manual.exe \
  /tmp/hakorune_p324.o target/release/libnyash_kernel.a -ldl -lpthread -lm
```

## Result

The normal source-execution EXE route now reaches executable output:

```text
NYASH_LLVM_ROUTE_TRACE=1 NYASH_EMIT_EXE_NYRT=target/release \
  target/release/hakorune --emit-exe /tmp/hakorune_p324b.exe \
  lang/src/runner/stage1_cli_env.hako
```

Observed result:

```text
[ny-llvmc] executable written: /tmp/hakorune_p324b.exe
EXE written: /tmp/hakorune_p324b.exe
```

## Guardrail

Future source-execution link failures after successful `opt` verification
should first check NyRT archive freshness before adding backend acceptance
surface:

```text
cargo build -p nyash_kernel --release
llvm-nm -g --defined-only target/release/libnyash_kernel.a | rg '<missing helper>'
```

Only treat the issue as a code blocker if the helper is genuinely absent from
the current `crates/nyash_kernel` runtime surface.
