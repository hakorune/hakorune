---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Probe whether json_native can restore the read_next_number_literal-style NUMBER payload route after the staged loop/break canary passed planner_required.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1243-COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001.md
  - apps/lib/json_native/lexer/scanner.hako
  - apps/lib/json_native/lexer/tokenizer.hako
  - apps/lib/json_native/core/number_materializer.hako
  - apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
---

# COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001

## Decision

Do not add new Recipe/CorePlan code for the captured staged loop/break shape.

The compiler-side staged loop/break canary is already accepted under
planner_required. The json_native restoration blocker is narrower:

```text
JsonToken NUMBER dynamic payload stability after tokenizer ArrayBox publication
```

Therefore:

```text
new_recipe_acceptance_required=0
json_native_route_restore_complete=0
next_owner=json_native_number_token_payload_stability
```

## Current Shape

`JsonTokenizer.tokenize_number()` already calls the scanner route:

```text
scanner.read_number()
```

The active tokenizer still materializes small numeric payloads through:

```text
JsonNumberTextMaterializer.materialize(number_str)
```

This bridge is intentional and temporary. It keeps accepted fixture values
stable while scanner-derived NUMBER payloads are not yet generally stable
through token storage and parser publication.

## Evidence

The accepted regression probe remains:

```text
apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
```

It covers small materialized values used by the active v0 converter fixtures.

The stronger out-of-materializer payload probe is captured as an investigation:

```text
apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
```

Command:

```bash
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_number_scanner_payload_probe \
  apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
/tmp/hako_json_number_scanner_payload_probe
```

Observed result:

```text
json.number.scanner=bad-value
Result: 2
```

This means `123` did not survive as an integer payload through the current
tokenizer/parser path.

## Interpretation

The previous blocker was not caused by the minimal loop/break recipe shape.
The remaining owner is token payload stability:

```text
scanner-derived numeric string
  -> JsonToken("NUMBER", value)
  -> token ArrayBox
  -> parser token.get_value()
  -> JsonNode.create_int_from_string()
  -> JsonNode.as_int()
```

The small dictionary in `JsonNumberTextMaterializer` is still required for the
accepted v0 fixture set.

## Next Row

Proceed to a token-payload-focused row before removing the number materializer:

```text
JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-001
```

Required output:

```text
dynamic_number_payload_survives_token_array=1
number_materializer_retire_allowed=1
```

Only after that row is green may the hardening queue item below be reopened:

```text
JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001
```

## Stop Lines

```text
do not add Recipe/CorePlan code for the green staged loop/break canary
do not retire JsonNumberTextMaterializer from an investigation failure
do not widen the small numeric dictionary as the primary fix
do not special-case read_next_number_literal by name
do not move schema-specific number handling into converter_core.hako
do not put expected-fail probes under probes/regression
```

## Contract

```text
output_contract=coreplan-loop-break-json-native-restore-probe-v0

scanner_read_number_route_active=1
planner_required_loop_break_canary_green=1
new_recipe_acceptance_required=0
accepted_small_number_regression_green=1
out_of_materializer_number_probe_green=0
json_native_route_restore_complete=0
number_materializer_still_required=1
next_owner=json_native_number_token_payload_stability
next_task=JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-001

summary=ok
```
