# 296x-1741 HAKO-COMPILER-LIBRARY-LANDING-001

Status: Landed
Date: 2026-06-26
Scope: First compiler library landing zone under `lang/src/compiler/lib/`.

## Goal

Materialize the compiler-facing Hako library surface as ordinary library code
without reopening ABI or syntax boundaries.

## Landed Surface

```text
lang/src/compiler/lib/text_builder.hako
lang/src/compiler/lib/projection_value.hako
lang/src/compiler/lib/canonical_json.hako
lang/src/compiler/hako_module.toml
```

## Boundary

Allowed:

- ordinary Hako library code for compiler-facing text building and canonical
  JSON emission
- reuse of existing `StringBox`, `ArrayBox`, `MapBox`, and related collection
  helpers

Forbidden:

- TypeBox ABI exposure for compiler-library semantics
- host ABI facades for JSON/Text/projector behavior
- distribution/package ABI for the library surface
- new language syntax or spec promotion
- `hako.buf` backing in v0

## Evidence

- the landing zone files exist under `lang/src/compiler/lib/`
- compiler module exports now point at the new library modules
- the library placement README already points at this directory as the
  compiler-facing Hako library home
- the new library surface stays on ordinary Hako modules with no ABI boundary
  added

## Non-Claims

```text
new_type_abi = 0
host_abi_compiler_semantics = 0
package_abi = 0
language_syntax_change = 0
hako_buf_backing = 0
source_selfhost_claim = 0
```

## Next

```text
MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001
```
