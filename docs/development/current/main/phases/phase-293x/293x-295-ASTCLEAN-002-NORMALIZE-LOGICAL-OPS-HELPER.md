# 293x-295 ASTCLEAN-002 normalize logical ops helper

Status: complete

## Decision

Decision: accepted.

`||` / `&&` source pre-tokenization normalization remains a parser entry concern, but the implementation must have one helper owner.

## Scope

- Move duplicated `normalize_logical_ops` local functions into one private parser helper.
- Keep parse behavior unchanged for strings, line comments, block comments, `#` comments, `|>`, `||`, and `&&`.
- Keep both public parser entrypoints as thin callers of the helper.

## Non-goals

- No tokenizer grammar change.
- No logical operator semantic change.
- No Stage1 / MIR / backend change.

## Guard

- `tools/checks/k2_wide_astclean_normalize_logical_ops_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_normalize_logical_ops_guard.sh` passed locally.
