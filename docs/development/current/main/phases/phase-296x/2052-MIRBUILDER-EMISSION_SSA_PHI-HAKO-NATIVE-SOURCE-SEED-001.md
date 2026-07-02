# 2052 - MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native `.hako` source seed for `mirbuilder::emission_ssa_phi`
from `DerivedArtifactSeedDraftInput`.

This creates the native source seed and module export only. It does not run a
HakoAdopted decision and does not claim Source Selfhost.

## Result

```text
native_source_seed_path =
  lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako

module_export = lib.mirbuilder.emission_ssa_phi_native_seed
native_seed_materialization = 1
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0

decision = HakoAdoptionDecisionDeferred
reason_token = EmissionSsaPhiNativeSourceSeedMaterialized
selected_next_card =
  MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_emission_ssa_phi_hako_native_source_seed_guard.sh
```
