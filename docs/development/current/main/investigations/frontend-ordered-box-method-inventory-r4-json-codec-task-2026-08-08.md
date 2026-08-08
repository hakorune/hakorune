---
Status: closed — ordered JSON v2 codec landed with legacy compatibility boundary
Date: 2026-08-08
Decision: ordered roundtrip JSON v2; legacy JSON v1 is compatibility-only
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R4

## Goal

Give the explicit AST roundtrip codec one ordered Box-method v2 schema that
preserves every inventory row without reconstructing source truth. Keep the
existing JoinIR/legacy JSON shape as a compatibility transport whose imports
produce only `CompatibilityOnly(LegacyJsonV1)` rows.

## Authority split

```text
ast_json_roundtrip_v2:
  selected declaration order
  exact declaration
  selected method ordinal
  diagnostic span
  ExplicitSource Direct/SelectedBuildGate path
  Generated Property/Delegate/MacroOrImport provenance

legacy/joinir JSON v1:
  compatibility method payload only
  decode -> CompatibilityOnly(LegacyJsonV1)
  never upgrade or infer source provenance
```

Schema mode is selected exactly once at the public root decoder:

```text
schema=ast_json_roundtrip_v2 + schema_version=2:
  strict recursive v2 decode

schema=ast_json_roundtrip_v1 + schema_version=1, or no schema marker:
  legacy compatibility decode

partial, mismatched, or unknown schema marker:
  reject at root
```

After root selection, every child stays in the same decode mode. A malformed
nested v2 node or Box rejects the whole root; it never falls back to v1 and is
never silently removed by `filter_map`.

The v2 decoder validates the complete method array before constructing the
inventory. Duplicate names, non-contiguous selected ordinals, malformed gate
paths, declaration-name mismatches, invalid provenance payloads, and ordinal
overflow reject the whole Box. Partial insertion and rollback repair are
forbidden.

## Required structure

Split the Box inventory codec from the current large roundtrip facade before
adding the v2 responsibility. The shared AST recursion may be borrowed, but
the compatibility encoder/decoder must not become the v2 semantic authority.

```text
roundtrip Box codec:
  encode_inventory_v2
  decode_inventory_v2 (prepare all -> one commit)

roundtrip root facade:
  emit schema/version on every encoded root object
  choose decode mode once
  use strict child collection in v2 mode

joinir_compat Box codec:
  existing payload shape
  explicit legacy compatibility import
```

## Acceptance

```text
v2 Direct roundtrip preserves order/site/span/provenance
v2 nested SelectedBuildGate roundtrip preserves outer-to-inner path
v2 Property and Delegate provenance roundtrip
v2 MacroOrImport provenance roundtrip
v2 duplicate/malformed/ordinal mismatch rejects atomically
partial/mismatched/unknown root schema rejects without fallback
malformed nested v2 Box rejects the whole root
v1 order/name payload imports only CompatibilityOnly(LegacyJsonV1)
v1 payload cannot spell or promote ExplicitSource
nested Box declarations use the same v2 codec recursively
all touched source files < 800 lines
```

## Stop lines

```text
no resolver-grade source seal
no CallableContract parser or issuer
no Hako parser parity claim
no Builder consumer migration (R5)
no changes to constructor ownership
no source order reconstructed from method names
no silent v2-to-v1 fallback after a v2 marker is observed
```

The implementation commit updates the AST/codec owner README,
`docs/reference/language/callable-contracts.md`, this card, the ordered task
map, and `CURRENT_STATE.toml`. Reference updates are mandatory in the same
slice; they are not deferred to the final callable-contract implementation.

## R4-I0 landed receipt

The frontend AST now exposes one descriptive roundtrip-v2 reconstruction
transaction:

```text
BoxMethodInventoryRoundtripRowV2[]
  -> PreparedBoxMethodInventoryRoundtripV2::try_new
  -> complete name/declaration/duplicate/ordinal/path preflight
  -> infallible commit
  -> BoxMethodInventoryV1
```

It preserves exact provenance and diagnostic span but issues no resolver
capability. Focused AST tests cover mixed provenance, duplicate names,
declaration mismatch, ordinal gaps, and empty selected-gate paths.

## R4 landed receipt

The codec is split into a small root facade, a strict recursive decoder, and a
Box-inventory v2 codec:

```text
ast_json_roundtrip_v2 root marker
  -> one DecodeMode selection
  -> recursive children stay in that mode
  -> complete Box method rows preflight
  -> one inventory commit
```

Legacy JSON remains an explicit compatibility transport. It is selected only
by the v1 marker or by an unmarked legacy payload and produces
`CompatibilityOnly(LegacyJsonV1)` rows. A partial, mismatched, or unknown
marker rejects at the root; malformed nested v2 nodes set a decoder failure
receipt and reject the complete root instead of being silently removed by a
collection helper.

Focused coverage now includes v2 schema/provenance preservation, legacy
compatibility provenance, schema mismatch rejection, malformed nested-box
rejection, and the existing AST transport negative matrix. The codec remains a
descriptive transport only: no resolver capability, source promotion, Builder
consumer, or fallback was opened.
