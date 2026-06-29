# 1817 - MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002

## Token

```text
MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002
```

## Purpose

Select the next native source owner seed pilot target after ReturnEmission has
been adopted as a narrow native Hako leaf owner.

This is a selection card only. It does not materialize native source and does
not run the selected family's HakoAdopted decision.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-native-owner-seed-pilot-target-selection-v1.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_native_owner_seed_pilot_target_selection_002_guard.sh
```

## Result

```text
decision = SelectNativeOwnerSeedPilotTarget
selected_target = FunctionRegionStackPop
next_card = MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001
```

## Acceptance

```text
ReturnEmission adoption evidence consumed
ReturnEmission excluded as AlreadyAdopted
FunctionRegionStackPop promotion evidence consumed
SlotRegistryRelease promotion evidence consumed
selected_target = FunctionRegionStackPop
stable_priority_selection = 1
manual_family_selection = 0
native_source_owner_materialized = 0
family_adoption_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Non-Claims

```text
no HakoAdopted decision
no generated artifact edit authority
no Source Selfhost claim
no seed materialization in this card
```
