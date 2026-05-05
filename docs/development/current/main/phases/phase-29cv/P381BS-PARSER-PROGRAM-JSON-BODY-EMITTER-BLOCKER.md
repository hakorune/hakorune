---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: lock the parser Program(JSON) dedicated body-emitter deletion blocker before further `.inc` cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - docs/development/current/main/phases/phase-29cv/P381BR-MODULE-GENERIC-SELECTED-KIND-REGISTRY.md
  - docs/development/current/main/phases/phase-29cv/P152-PARSER-PROGRAM-JSON-BODY-RECIPE.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BS: Parser Program(JSON) Body Emitter Blocker

## Problem

After P381BR, parser Program(JSON) selected-set planning is unified with the
module generic symbol registry. The next tempting cleanup is deleting:

```text
emit_parser_program_json_function_definition
```

That deletion is not safe yet.

The historical parser direct contract was written for a narrow one-argument
owner body:

```text
BuildBox._parse_program_json/1
  ParserBox.birth()
  ParserBox.stage3_enable(1)
  ParserBox.parse_program2(parse_src)
```

The live `lang/src/compiler/build/build_box.hako` owner is now a two-argument
body:

```text
BuildBox._parse_program_json(parse_src, scan_src)
  ParserBox.birth()
  ParserBox.stage3_enable(1)
  ParserBox.set_enum_inventory_from_source(scan_src)
  ParserBox.parse_program2(parse_src)
```

## Probe

The live MIR JSON probe shows the current owner drift:

```bash
target/release/hakorune \
  --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
```

Observed route:

```text
target_symbol=BuildBox._parse_program_json/2
tier=Unsupported
reason=missing_multi_function_emitter
target_shape_blocker_symbol=ParserBox.parse_program2
target_shape_blocker_reason=generic_string_unsupported_known_receiver_method
```

## Decision

Do not delete the dedicated parser body emitter in this slice.

The next valid cleanup is either:

- source-owner cleanup that makes parser Program(JSON) flow through the existing
  Stage1 extern route, or
- a MIR-owned two-argument parser contract with a matching C body-emission
  contract.

Until one of those exists, deleting the body emitter would only remove an older
contract path without making the live owner lowerable.

The parser-specific Stage1 extern declaration check is safe to remove now:
parser Program(JSON) definitions are recorded in the unified planned generic
symbol registry, so `module_has_planned_generic_string_definition()` already
covers the declaration need.

## Acceptance

```bash
cargo test --release parser_program_json -- --nocapture
cargo test --release stage1_emit_program_json_extern_route -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
