# 296x-1680: CoreContext Artifact Contract Projection

Status: Complete
Date: 2026-06-25
Token: CORE-CONTEXT-ARTIFACT-CONTRACT-PROJECTION-001

## Decision

Insert one synchronization-cost reduction slice before opening
`MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001`.

The selected structure is:

```text
VerifiedHakoFamilyIR
  + stable Deny results
  + artifact identity
    -> VerifiedFamilyArtifactContractV1
       -> manifest
       -> verifier expectation
       -> guard consumer
```

This keeps `VerifiedHakoFamilyIR` as the semantic authority. Generated `.hako`
and artifact manifests remain derived outputs, not semantic/edit authority.

## Scope

First pilot family:

```text
CoreContext
```

Selected objective:

```text
semantic capability added = 0
bundle started = 0
generated .hako behavior changed = 0

CoreContext manifest and verifier expectation are projected from the same
verified family artifact contract.
```

## Authority

`VerifiedHakoFamilyIR` owns:

```text
selected method semantics
behavior recipe
field/state semantics
receiver / argument / return shape
semantic transports
typed information needed by the emitter
```

Stable Deny results own:

```text
denied method
deny reason
deny detail
```

Artifact identity owns only:

```text
family_id
artifact_path
manifest_path
schema_version
stable output ordering policy
```

## Contract Shape

Minimal conceptual shape:

```python
VerifiedFamilyArtifactContractV1:
  family_id
  method_universe
  verified_ir
  denials
  artifact_identity
```

Derived properties, not copied second truths:

```text
selected_methods = verified_ir.selected_methods
semantic_transports = verified_ir.semantic_transports
denied_methods = denials.method ids
selected_body_count = len(selected_methods)
```

## Required Invariants

```text
selected_methods ∩ denied_methods = ∅
selected_methods ∪ denied_methods = method_universe
every selected method has a recipe
every selected method references declared semantic transports
semantic transport IDs are unique
same physical lane must not merge distinct semantic transport IDs
denied methods are not emitted into the artifact
```

The partition invariant is required to prevent silent method drops.

## Implementation Boundary

Allowed:

```text
add immutable Python contract dataclass / helper
make CoreContext manifest renderer consume the contract
make CoreContext verifier consume the contract
remove duplicated selected/excluded/transport literals from the guard path
```

Not allowed:

```text
facts extraction behavior change
behavior recipe semantic change
emitter change
generated .hako behavior change
backend route change
ABI change
runtime fallback
bundle artifact change
task-order generation
phase card generation
```

## Verification Model

Avoid self-verifying generated outputs.

Bad:

```text
contract -> expected manifest
contract -> actual manifest
compare expected to actual
```

Required:

```text
Rust facts / stable denials -> verified contract
verified contract -> manifest serialization
actual generated .hako -> independent structural observation
verify(actual observation, verified contract)
```

## Acceptance

```text
CoreContext has VerifiedFamilyArtifactContractV1
selected / denied / method universe partition is checked
semantic transports feed manifest and verifier from the same contract
manifest is generated from the contract
verifier observes actual .hako and checks it against the contract
guard does not duplicate method / transport expectation arrays
selected/excluded method list two-truth management is removed
generated core_context.hako is byte-identical
at least one drift probe fails closed
converter matrix green
no backend route / no ABI / no runtime fallback
```

## Focused Evidence

```text
VerifiedFamilyArtifactContractV1=implemented
CoreContext manifest projection=green
CoreContext verifier projection=green
CoreContext guard consumes contract verifier=green
generated core_context.hako=unchanged
contract drift probes=green
```

Validated:

```text
python3 tools/rust_lifecycle/generate_core_context_artifact.py --check
python3 tools/rust_lifecycle/verify_core_context_artifact_contract.py --drift-probes
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_binding_context_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
git diff --check
```

## Closeout Note

Full converter matrix closeout depended on the existing BindingContext
same-module route edge being closed first. That edge was fixed separately by
carrying the generic `MapGet` return shape through the generated `.get()` body,
without a BindingContext/lookup name branch, C-side fallback, or runtime
fallback.

```text
binding_context_lookup_return_shape=MixedRuntimeI64OrHandle
callee_name_branch=0
C_side_fallback=0
runtime_fallback=0
full_converter_matrix=green
```

Suggested drift probes:

```text
manifest ValueIdAsI64 changed to BasicBlockIdAsI64 -> fail
generated .hako missing peek_next_block -> fail
denied method emitted into artifact -> fail
```

## Non-Claims

```text
all-family contract migration = 0
generic schema DSL = 0
task-order auto-generation = 0
phase-card auto-generation = 0
bundle v1 implementation = 0
mainline_selected = 0
source selfhost claim = 0
```

## Next

After this contract projection is green, return to
`MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001` design and keep bundle ownership
limited to explicit family membership. Bundle manifests must reference family
contracts; they must not copy selected methods, transports, or denials.
