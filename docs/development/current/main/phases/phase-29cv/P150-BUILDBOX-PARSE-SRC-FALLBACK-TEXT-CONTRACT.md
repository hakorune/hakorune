---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P150, BuildBox parse-source fallback text contract
Related:
  - docs/development/current/main/phases/phase-29cv/P149-GLOBAL-CALL-VOID-TYPED-BLOCKER-PROPAGATION.md
  - docs/development/current/main/phases/phase-291x/291x-282-buildbox-parse-source-narrowing-ssot-card.md
  - lang/src/compiler/build/README.md
  - lang/src/compiler/build/build_box.hako
---

# P150: BuildBox Parse-Source Fallback Text Contract

## Problem

`BuildBox._resolve_parse_src/1` is the source-to-parse-source narrowing owner:

```text
BodyExtractionBox.extract_main_body(scan_src)
  -> return extracted Main.main body when present
  -> otherwise fall back to scan_src
```

After P149 exposed the real nested blocker, this owner stayed unsupported
because the fallback returned the raw `scan_src` parameter. The generic string
route correctly refused to infer unknown parameters as strings by default:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
  target_shape_reason=generic_string_return_not_string
```

## Decision

Keep the parser fallback semantics, but make the source-text materialization
explicit at the owner boundary:

```hako
return me._coerce_text_compat(scan_src)
```

This mirrors the existing stage1 source/text contract pattern without widening
Rust-side parameter inference:

- `BuildBox._resolve_parse_src/1` still owns only parse-source narrowing
- `BodyExtractionBox` remains the main-body extraction authority
- fallback still parses the full scan source
- no generic rule now treats unknown parameters as strings
- no backend fallback or by-name exception is introduced

## Evidence

After emitting `lang/src/runner/stage1_cli_env.hako`:

```text
BuildBox._parse_program_json_from_scan_src/1 -> BuildBox._resolve_parse_src/1
  tier=DirectAbi
  target_shape=generic_string_or_void_sentinel_body
  proof=typed_global_call_generic_string_or_void_sentinel

BuildBox._resolve_parse_src/1 -> BodyExtractionBox.extract_main_body/1
  tier=DirectAbi
  target_shape=generic_string_or_void_sentinel_body

BuildBox._resolve_parse_src/1 -> BuildBox._coerce_text_compat/1
  tier=DirectAbi
  target_shape=generic_pure_string_body
```

The full pure-first trace advances to the next real owner boundary:

```text
target_shape_blocker_symbol=BuildBox._parse_program_json/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

## Acceptance

```bash
target/release/hakorune --emit-mir-json /tmp/hakorune_p150_buildbox_fallback.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p150_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
```
