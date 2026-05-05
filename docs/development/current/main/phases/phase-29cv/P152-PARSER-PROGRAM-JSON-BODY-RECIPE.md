---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P152, exact parser Program(JSON v0) body recipe
Related:
  - docs/development/current/main/phases/phase-29cv/P151-PARSERBOX-KNOWN-RECEIVER-BOUNDARY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/compiler/build/build_box.hako
  - src/mir/global_call_route_plan.rs
  - crates/nyash_kernel/src/exports/stage1.rs
---

# P152: Parser Program(JSON v0) Body Recipe

## Problem

P151 made the source-execution stop-line precise:

```text
target_shape_blocker_symbol=ParserBox.parse_program2
target_shape_blocker_reason=generic_string_unsupported_known_receiver_method
```

The immediate owner is not arbitrary `ParserBox` lowering. It is the narrow
authority body:

```hako
BuildBox._parse_program_json(parse_src)
  new ParserBox()
  ParserBox.birth()
  ParserBox.stage3_enable(1)
  ParserBox.parse_program2(parse_src)
  return program_json
```

## Decision

Add one MIR-owned direct parser Program(JSON) contract. The original P152
landing used a temporary target-shape variant; P381BN retired that variant, so
the current route evidence is proof/return-contract based:

```text
proof=typed_global_call_parser_program_json
return_shape=string_handle
value_demand=runtime_i64_or_handle
target_shape=null
```

This recipe accepts only the exact single-block parser authority body:

- one source parameter
- one `newbox ParserBox`
- `birth()` on that receiver
- `stage3_enable(1)` on that receiver
- `parse_program2(source_param)` on that receiver
- return the parse result
- copy aliases are allowed

The backend does not match raw `BuildBox._parse_program_json/1` or
`ParserBox.parse_program2` by name. C lowering consumes only the MIR proof and
emits a same-module wrapper that calls:

```text
nyash.stage1.emit_program_json_v0_h(i64 source_text_handle) -> i64 program_json_handle
```

`ParserBox.parse_program2` itself is still not generally lowerable.

## Evidence

The MIR JSON route for the exact body is direct:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._parse_program_json/1
  tier=DirectAbi
  proof=typed_global_call_parser_program_json
  target_shape=null
```

The top pure-first source-execution stop moved past `ParserBox.parse_program2`
to the next owner boundary:

```text
target_shape_blocker_symbol=StringHelpers.starts_with/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

## Acceptance

```bash
cargo test -q parser_program_json --lib
cargo test -q -p nyash_kernel stage1_emit_program_json_v0_h --lib
cargo fmt --check
cargo build --release -p nyash_kernel
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p152_parser_program_json.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p152_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
