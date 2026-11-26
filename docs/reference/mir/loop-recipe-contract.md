# Loop Recipe V2 contract

The compiler uses Loop Recipe V2 to describe a loop's typed operations and
logical control flow. The contract is defined in [the V2 schema](../../../src/mir/loop_recipe_contract/schema_v2.rs)
and checked by [`LoopRecipeVerifierV2`](../../../src/mir/loop_recipe_contract/typed_schema_v2.rs).
The Loop Recipe V1 wire and its parity fixtures are retired.

## Artifact and recipe

The artifact has four fields.

| Field | Contents |
|---|---|
| `schema_version` | Must be 2 for `verify_artifact`. |
| `provenance` | The producer identifier. |
| `source_binding` | Structural claims linking recipe keys to source coordinates. |
| `recipe` | The typed logical loop representation. |

The recipe identifies a `root_loop` and holds arrays for `loops`, `blocks`,
`items`, `bindings`, `values`, `inputs`, `carriers`, `read_only_inputs`,
and `exits`. An omitted `read_only_inputs` means an empty list. The schema
rejects unknown fields on its records and tagged variants. Logical keys
are scoped to their own domain, and keys in the keyed arrays must be
contiguous in array order starting at zero. The root loop has key zero.

A loop names a body block and has either an always-true condition or a
predicate with a block and a Bool value. Blocks belong to a loop and
sequence operation, branch, nested loop, and exit items. Operation
variants specify their operand and result classes. An input supplies a
value to the recipe. A carrier relates a binding to the values that can
flow across loop boundaries. A read-only input relates an immutable
binding to its input value and is excluded from the carrier roles.

## Verification

`verify` checks the logical recipe and returns a `VerifiedLoopRecipeV2` or
a typed rejection. It checks these relations.

- Keys and references resolve in their declared domains. Inputs and
  definitions are unique, and value uses must have available definitions
  of the required class.
- Operations respect the operand and result rules for their variants.
  Predicates are Bool, and branch result selection follows the two Normal
  arms of its source branch.
- Carrier bindings and entry values agree with the declared classes and
  scope. Read-only input relations agree with the binding and input tables.
- Block, item, loop, and exit ownership agree. Nested loop parentage and
  exit targets follow the control structure, and unreachable items are
  rejected. [Control structure checks](../../../src/mir/loop_recipe_contract/typed_schema_v2_structure.rs)

`verify_artifact` checks the version, verifies the recipe, and binds
structural source claims to that exact verified recipe.
`bind_verified_artifact` consumes a recipe that has already passed
`verify` and checks the same source claims without repeating recipe
verification. The claim checker validates coverage and consistency
against recipe keys. Establishing that a claim refers to the original
source is the responsibility of the source-bound compiler path.
[Source claim checker](../../../src/mir/loop_recipe_contract/source_binding.rs)

The compiler issues V2 recipes through the source-loop, typed scan, and
Dynamic full-body producers. Physical placement, MIR generation, and
verification before publication belong to the later compiler stages.
[Source loop](../../../src/mir/loop_recipe_contract/source_loop_v2.rs)
[Typed scan](../../../src/mir/loop_recipe_contract/s6c_scan_with_init.rs)
[Dynamic full body](../../../src/mir/compiler/dynamic_full_body_recipe/mod.rs)
