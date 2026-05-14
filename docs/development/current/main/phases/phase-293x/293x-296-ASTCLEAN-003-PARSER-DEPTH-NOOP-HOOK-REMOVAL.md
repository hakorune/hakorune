# 293x-296 ASTCLEAN-003 parser depth no-op hook removal

Status: complete

## Decision

Decision: accepted.

Parser depth tracking hooks that no longer carry state are removed from the common advance path. Cursor/newline policy remains unchanged.

## Scope

- Remove `update_depth_before_advance` / `update_depth_after_advance` from `ParserUtils`.
- Remove calls to the hooks from `advance`.
- Remove `NyashParser` no-op implementations if present.

## Non-goals

- No cursor policy change.
- No newline or semicolon skip behavior change.
- No token stream or grammar change.

## Guard

- `tools/checks/k2_wide_astclean_parser_depth_noop_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_parser_depth_noop_guard.sh` passed locally.
