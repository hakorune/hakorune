---
Status: Provisional SSOT
Decision: provisional
Date: 2026-03-30
Scope: canonical declaration metadata surface を `@rune` に寄せ、legacy `@hint` / `@contract` / `@intrinsic_candidate` を compat alias として正規化する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/20-Decisions.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/ir/ast-json-v0.md
  - src/config/env/parser_flags.rs
  - src/parser/statements/helpers.rs
  - src/parser/runes.rs
  - src/tests/parser_opt_annotations.rs
  - lang/src/compiler/parser/parser_box.hako
  - lang/src/compiler/parser/rune/rune_contract_box.hako
  - lang/src/compiler/parser/stmt/parser_stmt_box.hako
  - lang/src/compiler/entry/stageb/stageb_rune_box.hako
---

# Rune v1 Metadata Unification (SSOT)

## Goal

- declaration metadata の canonical surface を `@rune` に統一する。
- legacy `@hint` / `@contract` / `@intrinsic_candidate` は compat alias として受理し、内部では同じ `attrs.runes` に正規化する。
- carrier / Program(JSON v0) / backend scope は widen せず、parser/front-door だけを一本化する。

## Fixed Decisions

| Item | Decision |
| --- | --- |
| canonical surface | `@rune Hint(...)`, `@rune Contract(...)`, `@rune IntrinsicCandidate("...")` |
| compat aliases | declaration-leading `@hint(...)`, `@contract(...)`, `@intrinsic_candidate("...")` |
| internal carrier | declaration-local `attrs.runes` only |
| Program(JSON v0) | no widen |
| direct MIR | existing declaration-local rune metadata pathを再利用 |
| canonical gate | `NYASH_FEATURES=rune` |
| compat gate | `NYASH_FEATURES=opt-annotations` also enables the unified metadata parser path during the compat window |
| statement-position legacy aliases | parse/noop compat keep |
| statement-position canonical optimization runes | fail-fast (`declaration required`) |
| backend semantics | parse/noop only for optimization families in this wave |
| compat removal | post-Array phase only |

## Canonical Surface

Examples:

```hako
static box Main {
  @rune Hint(hot)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("StringBox.length/0")
  main() {
    return 0
  }
}
```

Legacy compat aliases remain accepted on declaration-leading positions:

```hako
static box Main {
  @hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  main() {
    return 0
  }
}
```

Both routes normalize to:

```json
{
  "attrs": {
    "runes": [
      { "name": "Hint", "args": ["hot"] },
      { "name": "Contract", "args": ["no_alloc"] },
      { "name": "IntrinsicCandidate", "args": ["StringBox.length/0"] }
    ]
  }
}
```

## Placement Rules

- declaration-leading canonical `@rune` metadata attaches to the next declaration as before
- declaration-leading legacy aliases normalize to the same pending-rune path
- body-position legacy aliases remain compat/noop and do not produce `attrs.runes`
- body-position canonical optimization runes are rejected because the canonical surface is declaration metadata only
- duplicate family use on the same declaration is fail-fast whether the source used canonical syntax, legacy aliases, or a mix

## Gate Reading

The old split-gate reading is superseded for parser/front-door behavior.

- `NYASH_FEATURES=rune` is the canonical gate and is the only gate shown in new docs/examples
- `NYASH_FEATURES=opt-annotations` remains as a compat alias gate during the migration window
- while the compat window is open, either gate enables the same unified metadata parser path
- this does not mean optimization metadata is backend-active; it remains parse/noop until verifier/registry/backend slices are explicitly promoted

## Current Implementation Status

| Area | Status | Current truth |
| --- | --- | --- |
| Rust parser | landed | canonical `@rune Hint/Contract/IntrinsicCandidate` + legacy alias normalization |
| `.hako` parser | landed | same families, same value contract, same legacy alias normalization |
| `.hako` Stage-B selected-entry keep | landed | declaration-leading legacy aliases are normalized into selected-entry rune JSON |
| AST/direct MIR carrier | landed | still declaration-local `attrs.runes` only |
| Program(JSON v0) | locked | root/body attrs are not widened |
| tests/smokes | landed | dual-route noop + selected-entry rune attr parity cover the unified path |

## Non-Goals

- no backend-active optimization semantics in this wave
- no new metadata carrier beyond `attrs.runes`
- no Program(JSON v0) widening
- no removal of compat aliases before Array phase closes
- no claim that Array/Map owner work depends on this metadata lane

## Acceptance

- `cargo test parser_opt_annotations -- --nocapture`
- `bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh`
- `bash tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`
