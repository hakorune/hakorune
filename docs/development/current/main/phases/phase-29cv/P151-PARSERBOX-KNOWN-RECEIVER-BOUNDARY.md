---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P151, ParserBox known-receiver boundary triage
Related:
  - docs/development/current/main/phases/phase-29cv/P150-BUILDBOX-PARSE-SRC-FALLBACK-TEXT-CONTRACT.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/compiler/build/build_box.hako
  - src/mir/global_call_route_plan.rs
---

# P151: ParserBox Known-Receiver Boundary

## Current Stop

After P150, the source-execution pure-first trace reaches the parser authority:

```text
target_shape_blocker_symbol=BuildBox._parse_program_json/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The body is not a plain global string helper. It constructs a `ParserBox` and
uses known-receiver methods:

```text
new ParserBox()
ParserBox.stage3_enable(1)
ParserBox.parse_program2(parse_src)
```

## Decision

Do not hide this behind a raw `BuildBox._parse_program_json/1` or
`ParserBox.parse_program2` backend name matcher.

The next implementation must keep the boundary structural:

- observe the unsupported `newbox + known-receiver method` shape explicitly
- keep the parser authority in `BuildBox._parse_program_json/1`
- do not infer arbitrary object-return methods as strings
- do not reopen Program(JSON v0) as a mainline proof route
- if acceptance is added, pin one narrow recipe with a fixture/test first

## First Implementation Slice

Before adding a lowering path, add MIR-owned blocker vocabulary for known
receiver parser-method boundaries. The immediate goal is better stop-line
truth:

```text
generic_string_unsupported_known_receiver_method
target_shape_blocker_symbol=ParserBox.parse_program2
```

This is BoxShape cleanup. It must not make the parser route lowerable by
itself.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_parser_known_receiver_method_blocker
cargo test -q global_call_routes
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p151_parser_boundary.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p151_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```

## Evidence

`BuildBox._parse_program_json_from_scan_src/1` now reports the parser method
boundary directly:

```text
BuildBox._parse_program_json/1
  target_shape_reason=generic_string_unsupported_known_receiver_method
  target_shape_blocker_symbol=ParserBox.parse_program2
  target_shape_blocker_reason=generic_string_unsupported_known_receiver_method
```

The full pure-first trace propagates the same blocker to the top source
execution stop:

```text
target_shape_blocker_symbol=ParserBox.parse_program2
target_shape_blocker_reason=generic_string_unsupported_known_receiver_method
```

No `ParserBox` method was made lowerable in this card.
