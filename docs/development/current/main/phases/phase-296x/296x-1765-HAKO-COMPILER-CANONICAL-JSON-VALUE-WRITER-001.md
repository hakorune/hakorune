---
Status: Landed
Date: 2026-06-28
Scope: Docs-only placement card for the compiler-facing canonical JSON writer.
---

# HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-001

## Goal

Keep the canonical JSON writer as ordinary Hako compiler-library code so the
next implementation step stays small, structure-first, and ABI-free.

## Placement Decision

The canonical JSON writer lives as ordinary `.hako` library code under:

```text
lang/src/compiler/lib/
```

Landed surface for this placement:

```text
lang/src/compiler/lib/canonical_json.hako
lang/src/compiler/lib/projection_value.hako
lang/src/compiler/lib/README.md
```

The writer is allowed to own canonical JSON emission concerns:

```text
string escaping
i64 / bool / null serialization
array serialization
object key ordering
stable whitespace policy
```

## Boundaries

Allowed:

- ordinary Hako library code for compiler-facing canonical JSON emission
- reuse of existing `StringBox`, `ArrayBox`, and `OrderedMapBox`

Forbidden:

- TypeBox ABI exposure for compiler-library semantics
- host ABI facades for JSON/Text/projector behavior
- distribution/package ABI for the library surface
- new language syntax or spec promotion
- `hako.buf` backing in v0

## Evidence

- the landing zone files already exist under `lang/src/compiler/lib/`
- the library placement README already points at this directory as the
  compiler-facing Hako library home
- the existing `canonical_json.hako` and `projection_value.hako` modules keep
  compiler-facing canonical JSON work inside the ordinary library lane

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
