---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: parser annotation dual-route smoke の Rust-side raw Program(JSON) emit を AST JSON emit へ移す。
Related:
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
---

# P11 Parser Dual Route AST Migration

## Goal

`parser_opt_annotations_dual_route_noop.sh` は parser annotation の no-op 契約を
確認する smoke だが、Rust parser route の観測に raw
`--emit-program-json-v0` を使っていた。

Rust parser の観測は AST JSON が本線なので、Rust-side を
`--emit-ast-json` に移し、raw Program(JSON) caller を削る。

## Decision

- Rust route: `--emit-ast-json` で AST JSON を emit し、`attrs` を除いた構造が
  baseline / annotated で一致することを確認する。
- Hako route: 既存 wrapper の Program(JSON) stdout を維持し、`body` が baseline /
  annotated で一致することを確認する。
- Current env で Hako parser route が annotated fixture 未達の場合は明示 SKIP。
  raw Rust Program(JSON) route で未達を隠さない。
- raw `--emit-program-json-v0` はこの smoke から削除する。

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh
rg -n -- '--emit-program-json-v0' tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh
bash tools/checks/current_state_pointer_guard.sh
```
