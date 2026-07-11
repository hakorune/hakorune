---
Status: Landed
Date: 2026-07-05
Scope: anti-debt policy for MirBuilder Rust ASTNode to token snapshot adapters.
---

# MIRBUILDER-TOKEN-SNAPSHOT-ADAPTER-SUNSET-POLICY-001

## Decision

Rust `ASTNode` to token snapshot code is a temporary projection ABI for
MirBuilder-first migration. It is not an authority layer.

```text
layer_kind=temporary_projection_abi
authority_owner=HHako facade DTO owner
non_authority_owner=Rust ASTNode/token projector
sunset_target=HHako ProgramJSON to snapshot/facade input during parser integration
migration_order=mirbuilder_first_parser_later
```

The current mixed route is allowed only while parser integration is later than
MirBuilder authority migration:

```text
current_route=RHako parser -> Rust ASTNode -> Rust token snapshot -> HHako facade -> DTO
target_route=HHako parser -> ProgramJSON -> HHako snapshot/facade -> DTO
```

## Rules

- Token/projector code may extract fields from the current Rust authority input,
  but must not invent new semantic policy.
- Token names must mirror existing Rust authority concepts. Do not create a new
  hidden taxonomy in adapter strings.
- Each new MirBuilder facade card must name its input contract and keep the
  projector as non-authority.
- Prefer direct structured snapshot/facade input over adding a new bridge layer.
- Once the HHako `ProgramJSON` route can provide the same snapshot, the Rust
  token projector is removable.

## Non-Claims

```text
source_selfhost_claim=0
parser_integration_done=0
program_json_facade_input_done=0
rust_astnode_removed=0
new_backend_route=0
new_semantic_layer=0
```

## Next

```text
MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-001
```
