# 293x-272 LOOP-002 Stage0 LoopRange Parser Capsule

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

LOOP-002 adds the Stage0 parser capsule for loop-only range headers.

## Landed files

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/loop-range-parser-capsule-ssot.md` | LOOP-002 parser capsule SSOT. |
| `src/parser/statements/control_flow.rs` | Parses `loop i in start..end`, `loop(i in start..end)`, and paren-less `loop cond`. |
| `src/macro/ast_json/joinir_compat.rs` | Transports LoopRange metadata through AST JSON. |
| `src/macro/ast_json/roundtrip.rs` | Decodes LoopRange metadata for roundtrip JSON. |
| `src/stage1/program_json_v0/lowering.rs` | Emits LoopRange metadata in Program JSON v0. |
| `src/tests/parser_loop_scan_range_shape.rs` | Parser fixture for LoopRange and paren-less condition loop headers. |
| `tools/checks/k2_wide_loop_range_parser_capsule_guard.sh` | Local guard for the parser capsule and stop lines. |

## Stop lines

LOOP-002 does not add Stage1 semantics:

```text
no range lowering
no read-only index enforcement
no continue-safe step insertion
no bounds facts
no array iteration
no custom step
no inclusive range
no for keyword as canonical syntax
```

## Verification

```bash
bash tools/checks/k2_wide_loop_range_parser_capsule_guard.sh
```

## Next blocker

```text
LOOP-003 Stage1 LoopRange lowering
```
