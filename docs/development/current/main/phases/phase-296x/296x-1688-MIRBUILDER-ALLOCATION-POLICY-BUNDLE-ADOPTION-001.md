Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-ALLOCATION-POLICY-BUNDLE-ADOPTION-001

# MirBuilder Allocation Policy Bundle Adoption

## Scope

Adopt the generated prepared-state `next_value_id` policy kernel into the
MirBuilder ordered-map context bundle as explicit artifact membership.

This is a membership and smoke-execution slice only. It does not promote the
kernel to mainline source authority and does not claim full `MirBuilder`
transport.

## Authority

```text
prepared-state policy kernel artifact
  -> explicit bundle membership
  -> bundle-level smoke
```

The bundle consumes the kernel artifact manifest and
`VerifiedFamilyArtifactContractV1` reference. It does not copy the kernel's
selected methods, semantic transports, denials, or source policy facts.

## Landed Evidence

```text
ordered_map_crate_bundle_generated_artifact=unchanged
output_contract=rust-lifecycle-mirbuilder-easy-v0-ordered-map-bundle-v4
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
```

The bundle now includes:

```text
mirbuilder_next_value_id_prepared_state_kernel
```

and exercises:

```text
MirBuilderAllocationPolicy.prepared_state_next_value_id
```

## Non-Claims

```text
full MirBuilder object transport = 0
ScopeContext conversion = 0
CompilationContext conversion = 0
parameter compatibility fallback = 0
formal INVALID sentinel exclusion = 0
overflow parity = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
mainline_selected = 0
source selfhost claim = 0
```

## Next

```text
MIRBUILDER-MINIMAL-EXECUTION-PATH-SELECTION-001
```

Select the first generated context call graph beyond standalone artifacts.
