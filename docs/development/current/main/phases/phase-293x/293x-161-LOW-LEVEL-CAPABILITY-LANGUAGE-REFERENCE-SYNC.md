---
Status: Complete
Date: 2026-05-11
Scope: docs/reference language sync for mimalloc-driven low-level capabilities.
Related:
  - docs/reference/language/low-level-capabilities.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/types.md
  - docs/reference/ir/ast-json-v0.md
  - docs/reference/runtime/substrate-capabilities.md
  - docs/reference/mir/rune-profile-registry.md
  - docs/reference/mir/metadata-facts-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
---

# 293x-161 Low-Level Capability Language Reference Sync

## Goal

Reflect the language-facing result of the mimalloc lane in `docs/reference`.

The current capability work proves that `.hako` can express low-level allocator
policy through narrow capability modules, static tables, Rune metadata,
verifier-backed contracts, and EXE proof apps. It still does not activate host
allocator replacement.

## Updated Contract

- `docs/reference/language/low-level-capabilities.md` is the language-facing
  entry for allocator-grade low-level `.hako` code.
- Runtime method detail remains in
  `docs/reference/runtime/substrate-capabilities.md`.
- Rune profile detail remains in `docs/reference/mir/rune-profile-registry.md`.
- EBNF reflects `Profile` / `Lowering` and dotted rune args.
- MIR/ABI references list the narrow substrate route/native leaf boundary
  without creating a third ABI.
- The mimalloc default lane remains `.hako` / `hako_alloc`; provider M104+
  stays optional future host-replacement support.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
