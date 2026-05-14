---
Status: Landed
Date: 2026-05-14
Scope: BoxShape cleanup for duplicated AST JSON metadata decode helpers.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-293x/293x-285-GEN-001-GENERIC-TYPE-ANNOTATION-METADATA-CAPSULE.md
  - src/macro/ast_json/shared.rs
  - src/macro/ast_json/joinir_compat.rs
  - src/macro/ast_json/roundtrip.rs
---

# 293x-286 AST JSON Metadata Helper Cleanup

## Result

Landed as a BoxShape-only cleanup after GEN-001 and before GEN-002.

This card does not add a new language feature, parser acceptance shape, or
semantic checker. It only removes duplicated metadata transport helpers and
normalizes small formatting drift around recent metadata fields.

## Cleanup

- Moved duplicated AST JSON decode helpers into `src/macro/ast_json/shared.rs`:
  - `json_to_contract_clauses_with`
  - `json_to_string_array`
  - `json_to_transition_decls`
- Updated both AST JSON decoders to use the shared helpers:
  - `src/macro/ast_json/joinir_compat.rs`
  - `src/macro/ast_json/roundtrip.rs`
- Normalized `uses` / `contracts` field formatting in constructor and macro
  generated function declarations.
- Extracted `consume_generic_close` inside `src/parser/common/type_refs.rs` to
  keep `>` / `>>` handling in one local helper.

## Non-Goals

- no GEN-002 arity checking
- no generic semantic checks
- no Stage0/Stage1 language behavior change
- no Program JSON schema change
- no new guard entrypoint

## Proof

Passed locally:

```bash
cargo check -q
bash tools/checks/k2_wide_contract_syntax_metadata_guard.sh
bash tools/checks/k2_wide_transition_metadata_capsule_guard.sh
bash tools/checks/k2_wide_uses_metadata_capsule_guard.sh
bash tools/checks/k2_wide_generic_type_annotation_metadata_guard.sh
git diff --check
```

## Next

Resume `GEN-002 generic arity check` as the next BoxCount/semantic row.
