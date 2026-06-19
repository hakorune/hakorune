---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select and fix the JSON object-key equality owner, then retire the temporary critical-key bridge.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1262-JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1263-JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1273-RUST-SUBSET-SYN-ADAPTER-SOURCE-SHAPE-COVERAGE-CLOSEOUT-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001

## Decision

The full fixture failure without `JsonObjectKeyMaterializer` was caused by
`JsonNodeInstance.object_key_equals` treating distinct same-length
scanner-derived dynamic keys as equal.

The owner is:

```text
owner=JsonNodeInstance.object_key_equals
failure_mode=same_length_dynamic_key_false_positive
```

The temporary RustSubset critical-key dictionary is no longer needed after the
generic key equality fix.

## Evidence

The focused single-key negative probe is green:

```text
probe=apps/rust-subset-to-hako/probes/investigations/json_object_same_length_key_negative_probe.hako
result=same_length_key=ok
```

The multi-key owner probe reproduced the prior full-fixture failure before the
fix:

```text
probe=apps/rust-subset-to-hako/probes/investigations/json_object_same_length_pair_lookup_probe.hako
before_fix=same_length_pair=bad-kind
observed=kind=Point
```

After the fix, the same probes are green:

```text
same_length_pair=ok
same_length_key=ok
```

A stable regression probe now guards the behavior:

```text
probe=apps/rust-subset-to-hako/probes/regression/json_object_same_length_key_lookup_probe.hako
result=same_length_key_regression=ok
```

Full rust-subset smoke with regression probes is green without the bridge:

```bash
RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

```text
summary=ok
```

## Implementation

`JsonNodeInstance.object_key_equals` no longer uses one-character substring
comparison as the decisive same-length fallback. It now uses full-string
ordering comparisons after length and direct equality checks.

Object keys are also materialized generically at the entry-table boundary:

```text
object_set:
  key -> object_key_materialize(key)

object_get:
  key -> object_key_materialize(key)
```

This is generic JSON key publication, not a RustSubset schema-key dictionary.

The following bridge is retired:

```text
removed=apps/lib/json_native/core/key_materializer.hako
removed_parser_import=JsonObjectKeyMaterializer
removed_parser_call=JsonObjectKeyMaterializer.materialize(...)
```

## Stop Lines

```text
do not restore RustSubset schema-key dictionaries in json_native
do not move schema-specific lookup into converter call sites
do not replace json_native with a host JSON DLL
do not run smoke/regression commands that rebuild libhako_llvmc_ffi.so in parallel
```

## Report

```text
output_contract=json-native-object-key-equality-owner-selection-v0
selected_owner=JsonNodeInstance.object_key_equals
same_length_dynamic_key_false_positive_reproduced=1
same_length_dynamic_key_false_positive_fixed=1
json_object_key_materializer_removed=1
parser_contextual_key_dictionary_removed=1
regression_probe_added=1
full_smoke_with_regression_green=1
next_blocker=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
summary=ok
```
