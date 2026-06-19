---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Inventory candidate fixes for scanner-derived JSON NUMBER payload stability without widening accepted app-front behavior.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1244-COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
  - apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
---

# JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-INVENTORY-001

## Decision

Do not implement a drive-by app-level fix for arbitrary JSON NUMBER payloads in
this row.

The current accepted route remains:

```text
JsonNumberTextMaterializer small table
```

The stronger scanner-derived payload probe is still an investigation, not a
regression gate:

```text
apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
```

## Evidence

Accepted route:

```bash
bash tools/build_hako_llvmc_ffi.sh >/dev/null
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_nonzero_number_probe \
  apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
/tmp/hako_json_nonzero_number_probe
```

Result:

```text
json.number=ok
Result: 0
```

Stronger investigation:

```bash
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_number_scanner_payload_probe \
  apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
/tmp/hako_json_number_scanner_payload_probe
```

Result:

```text
json.number.scanner=bad-value
Result: 2
```

## Non-Keeper Candidates

The following app-level candidates were tried and rejected because they either
did not fix the stronger probe or broke the accepted regression:

```text
1. Parser source-span recovery from token start/end
   result=nonkeeper
   reason=object-path NUMBER still failed or backend pure shape regressed

2. JsonToken primitive integer sidecar
   result=nonkeeper
   reason=accepted small-number regression read number_value as 0 after token storage

3. JsonNode source-span / object int sidecar
   result=nonkeeper
   reason=accepted route regressed or object-path NUMBER still failed

4. Character-by-character number materializer
   result=nonkeeper
   reason=accepted small-number regression regressed; concatenated payload was not stable enough
```

These candidates are not present in the accepted code path.

## Interpretation

The remaining owner is below the rust-subset converter core:

```text
scanner-derived StringBox / token payload publication stability
```

This is likely a lower substrate issue around dynamic string payloads crossing:

```text
JsonToken
  -> ArrayBox token storage
  -> parser readback
  -> JsonNode int payload
  -> object field storage
```

Do not widen `JsonNumberTextMaterializer` with more whole-number entries. That
would hide the substrate owner and grow a dictionary by data value.

## Next Row

Proceed to an owner-selection row before implementation:

```text
JSON-NATIVE-NUMBER-PAYLOAD-OWNER-SELECTION-001
```

Candidate owners:

```text
StringBox dynamic substring publication
JsonToken text_value storage semantics
ArrayBox object/value publication
MapBox/object field readback
C ABI same-module dynamic string route
```

The next row must pick exactly one owner and one seam before code changes.

## Stop Lines

```text
do not expand JsonNumberTextMaterializer with per-number entries
do not put expected-fail probes under probes/regression
do not alter converter_core.hako for JSON NUMBER stability
do not claim arbitrary JSON integer support from the accepted small-number regression
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-number-token-payload-stability-inventory-v0

accepted_small_number_regression_green=1
scanner_payload_probe_green=0
app_level_candidate_keeper=0
number_materializer_retire_allowed=0
next_owner_selection_required=1
next_task=JSON-NATIVE-NUMBER-PAYLOAD-OWNER-SELECTION-001

summary=ok
```
