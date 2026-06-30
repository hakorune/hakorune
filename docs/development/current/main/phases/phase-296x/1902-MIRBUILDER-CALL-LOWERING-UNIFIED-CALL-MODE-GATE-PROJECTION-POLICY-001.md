# 1902 - MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001
```

## Purpose

Resolve the `UnifiedCallModeGate` subcluster selected by the CallLowering
feature predicate decomposition.

The selected source surface is:

```text
is_unified_call_enabled() -> bool
```

This surface reads the existing `NYASH_MIR_UNIFIED_CALL` config gate through
`src/config/env/builder_flags.rs::builder_unified_call_mode`. It does not own a
Hako projection surface, a new runtime fallback, or a new environment flag.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_unified_call_mode_gate_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_unified_call_mode_gate_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentConfigGate
config_authority =
  src/config/env/builder_flags.rs::builder_unified_call_mode

projection_surface_selected = 0
hako_config_gate_selected = 0
new_env_flag = 0

next_card =
  MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001
```

## Evidence

```text
source_count = 1
source_surface = is_unified_call_enabled
env_var = NYASH_MIR_UNIFIED_CALL
default_on_marker = default ON during development; explicit opt-out supported
opt_out_values = 0 | false | off
```

## Acceptance

```text
policy = KeepParentConfigGate
projection_surface_selected = 0
hako_config_gate_selected = 0
new_env_flag = 0
runtime_or_projection_policy_by_name = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no standalone Hako config gate
no new environment flag
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
