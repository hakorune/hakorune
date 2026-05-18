# 293x-656 PURE-FIRST-BRAND-CONSTRUCT-001 Brand Constructor MIR Acceptance

Status: landed
Date: 2026-05-18

## Decision

Add the smallest pure-first MIR acceptance seam needed before allocator scalar
ID brands can be piloted in `hako_alloc`.

The direct MIR route currently treats `BlockId(7)` / `PageId(1)` /
`SegmentId(1)` as ordinary function calls and fails with unresolved function
diagnostics. This row should recognize declared brand constructors as a
transparent wrapper over their single underlying value for MIR lowering.

## Scope

- Collect top-level `BrandDeclaration` names in the MIR source route.
- Lower `BrandName(value)` as the lowered `value` when `BrandName` is declared
  and exactly one argument is present.
- Reject invalid brand constructor arity with a stable diagnostic.
- Keep `BrandName.unwrap(value)` behavior out unless it is already naturally
  routed by the current parser/builder path; otherwise select a follow-up.
- Add focused tests/probes for:
  - accepted brand constructor in direct MIR,
  - unresolved ordinary function still fails,
  - invalid brand constructor arity fails fast.

## Non-Goals

- No full brand type checker in MIR builder.
- No field/return/typed-local/cross-module brand inference.
- No backend representation change.
- No allocator behavior.
- No broad type-alias semantics.
- No provider activation or host allocator replacement.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `001.1` | Find the MIR builder entry that resolves `FunctionCall`. | owner function is identified. | no behavior change |
| `001.2` | Add declared-brand constructor recognition. | `BrandId(value)` lowers as `value`. | no unresolved-function fallback |
| `001.3` | Add focused tests / guard evidence. | direct MIR accepts brand constructor and still rejects non-brand unresolved calls. | no allocator behavior |
| `001.4` | Return to HAKO-ALLOC-ID-BRAND pilot selection. | next row is explicit. | no broad type system bundle |

## Required Evidence

```text
cargo test -q stage1_program_json_v0::tests::basics_and_enums::source_to_program_json_v0_accepts_matching_brand_method_arg
cargo test -q parser_brand_surface
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

Implemented direct MIR recognition for declared brand constructors:

```text
brand BlockId: i64
local block = BlockId(7)
```

Implementation:

- `declaration_indexer` registers top-level `BrandDeclaration` rows into
  `CompilationContext`.
- `build_function_call()` lowers declared `BrandName(value)` calls as the
  lowered `value`.
- invalid constructor arity fails fast with `[brand/constructor-arity]`.
- undeclared function calls still fail through the normal unresolved-function
  path.

Focused evidence:

```text
cargo test -q mir_brand_constructor
cargo test -q parser_brand_surface
cargo test -q source_to_program_json_v0_accepts_matching_brand_method_arg
NYASH_DISABLE_PLUGINS=1 NYASH_FEATURES=rune NYASH_USING_AST=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  cargo run -q --bin hakorune -- --backend mir --emit-mir-json /tmp/brand.mir.json /tmp/brand_probe.hako
```

Selected next row:

```text
HAKO-ALLOC-ID-BRAND-002
  allocator scalar ID brand first pilot
```
