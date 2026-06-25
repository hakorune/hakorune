# HAKO-COMPILER-TEXT-BUILDER-V0-001

Status: Selected
Date: 2026-06-26
Scope: Docs-only placement card for the first compiler-library landing zone.

## Goal

Fix where the first Hako compiler libraries live so the next implementation
step can stay small and structure-first.

## Placement Decision

The compiler-facing libraries live as ordinary `.hako` modules under:

```text
lang/src/compiler/lib/
```

Initial library files:

```text
lang/src/compiler/lib/text_builder.hako
lang/src/compiler/lib/canonical_json.hako
lang/src/compiler/lib/projection_value.hako
```

## Boundaries

Allowed:

- ordinary Hako library code for compiler meaning, JSON generation, and text
  building
- reuse of existing `StringBox`, `ArrayBox`, and `OrderedMapBox`

Forbidden:

- TypeBox ABI exposure for compiler-library semantics
- host ABI facade for JSON/Text/projector behavior
- distribution/package ABI for the library surface
- new language syntax or spec promotion
- `hako.buf` backing in v0

## Non-Claims

```text
new_type_abi = 0
host_abi_compiler_semantics = 0
package_abi = 0
language_syntax_change = 0
hako_buf_backing = 0
source_selfhost_claim = 0
```

## Acceptance

- the placement doc exists
- `lang/src/compiler/lib/README.md` exists and the parent compiler README
  points at the library landing zone
- the task-order keeps Hako compiler libraries after the Python freeze
- the next implementation card can target the library files above without
  reopening the ABI discussion
