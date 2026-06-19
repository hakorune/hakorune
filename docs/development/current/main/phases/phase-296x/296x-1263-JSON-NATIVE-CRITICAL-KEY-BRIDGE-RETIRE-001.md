# JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001

Date: 2026-06-19
Status: rejected-after-full-smoke
Scope: rust-subset-to-hako json_native hardening

## Decision

Do not retire `JsonObjectKeyMaterializer` yet.

The previous retire probe rejected removal because unknown object keys were green
through generic entry-table lookup, while critical RustSubset keys such as
`kind` failed without the parser-contextual dictionary. A focused probe showed
that the token key and token string value were already correct:

```text
key=kind
value=RustSubsetModule
```

The remaining owner is object-key equality across scanner-derived keys and
literal lookup keys. Adding direct string equality before the substring fallback
fixes the focused `kind` probe, but full converter parity still fails without
the bridge because unrelated same-length keys can false-positive through the
fallback path.

## Implementation

```text
JsonParser.parse_object:
  key = key_token.get_value()

JsonNode.object_key_equals:
  null guard
  direct a == b equality
  length + char-by-char fallback
```

Kept:

```text
apps/lib/json_native/core/key_materializer.hako
JsonObjectKeyMaterializer parser import
RustSubset critical-key dictionary in json_native
```

## Evidence

```text
apps/rust-subset-to-hako/probes/investigations/json_dynamic_kind_key_and_value_probe.hako
  expected: json.dynamic.kind=ok

apps/rust-subset-to-hako/probes/regression/json_unknown_key_materialization_probe.hako
  expected: unknown.key.materialization=ok

apps/rust-subset-to-hako/probes/regression/json_object_key_materialization_probe.hako
  expected: key.materialization=ok
```

Full app-front regression smoke is the decisive gate and rejects retirement in
this row:

```bash
RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

Observed failure without the bridge:

```text
expected: record Point { ... }
actual:   // TODO: Point
```

## Stop Lines

```text
do not remove the parser-contextual bridge until full converter parity is green
do not move schema-specific lookup into converter callsites
do not infer keys by source/fixture name
do not replace json_native with a host JSON DLL
do not treat focused key probes as sufficient full-app acceptance
```

## Report

```text
output_contract=json-native-critical-key-bridge-retire-retry-v0
json_object_key_materializer_removed=0
parser_contextual_key_dictionary_enabled=1
focused_critical_key_probe_green_without_bridge=1
full_converter_parity_green_without_bridge=0
selected_owner=json_object_key_equality_false_positive
next_task=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
schema_specific_json_library_semantics=0
converter_core_changed=0
summary=rejected
```
