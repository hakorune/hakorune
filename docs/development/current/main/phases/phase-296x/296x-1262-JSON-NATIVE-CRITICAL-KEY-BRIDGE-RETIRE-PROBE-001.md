---
Status: Done
Decision: rejected
Date: 2026-06-19
Scope: Probe whether the json_native critical object-key bridge can be retired after generic entry-table lookup work.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1261-JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001.md
  - apps/rust-subset-to-hako/probes/regression/json_unknown_key_materialization_probe.hako
  - apps/rust-subset-to-hako/probes/regression/json_object_key_materialization_probe.hako
  - apps/rust-subset-to-hako/STATUS.md
---

# JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-PROBE-001

## Decision

Reject full retirement of `JsonObjectKeyMaterializer` for this row.

Unknown object keys already work through JsonNode's generic entry-table lookup,
but RustSubset critical-key routes still require the parser-contextual bridge.

## Evidence

With the bridge enabled:

```text
probe=apps/rust-subset-to-hako/probes/regression/json_unknown_key_materialization_probe.hako
result=green

probe=apps/rust-subset-to-hako/probes/regression/json_object_key_materialization_probe.hako
result=green
```

With the bridge removed:

```text
json_unknown_key_materialization_probe=green
json_object_key_materialization_probe=bad-kind
```

This means:

```text
generic_unknown_key_lookup_green=1
critical_key_bridge_retire_allowed=0
```

The remaining failure is not caused by generic object entry-table lookup. The
next probe should isolate critical-key string value publication:

```text
JSON-NATIVE-CRITICAL-STRING-VALUE-PUBLICATION-PROBE-001
```

## Stop Lines

```text
do not remove JsonObjectKeyMaterializer until critical-key regression is green without it
do not widen the bridge from object-key context into JSON string values
do not add converter-side schema key special cases
do not treat unknown-key success as proof that critical RustSubset keys are stable
```

## Contract

```text
output_contract=json-native-critical-key-bridge-retire-probe-v0

unknown_key_generic_lookup_green=1
critical_key_without_bridge_green=0
critical_key_bridge_retire_allowed=0
json_object_key_materialization_probe_green_with_bridge=1
next_task=JSON-NATIVE-CRITICAL-STRING-VALUE-PUBLICATION-PROBE-001
implementation_allowed=0

summary=ok
```
