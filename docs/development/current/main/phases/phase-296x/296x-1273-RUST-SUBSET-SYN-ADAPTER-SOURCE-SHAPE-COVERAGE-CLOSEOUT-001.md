# RUST-SUBSET-SYN-ADAPTER-SOURCE-SHAPE-COVERAGE-CLOSEOUT-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front source-shape batch closeout

## Decision

Close the current RustSubset syn-adapter source-shape transport batch.

The batch has enough coverage to stop adding source-shape rows by default and
return to the next concrete blocker:

```text
next_lane=json_native_object_key_equality_owner_selection
next_task=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
```

Further RustSubset source-shape work should reopen only after a new real input
front exposes a missing shape. Do not continue adding Rust syntax coverage from
speculation alone.

## Closed Source-Shape Coverage

Supported transport shapes:

```text
tail expression return
if
assign
while
vec literal
else-if as recursive If
returnless void body
Vec method calls
loop without break/continue as While true
explicit unit return -> void
```

Explicit unsupported handoff shapes:

```text
trait item
match expression
for-loop expression
```

Compiler-only backlog, not app-front work:

```text
break
continue
multi-stage scanner exits
read_next_number_literal full shape
loop-carried PHI scanner shape
nested break/continue interactions
return/break/continue interaction
```

## Evidence

Latest full app-front gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 \
  bash apps/rust-subset-to-hako/smoke.sh
```

Result:

```text
summary=ok
```

## Next Lane

Return to JSON native hardening:

```text
JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
  classify why full fixture key lookup false-positives without JsonObjectKeyMaterializer
  keep JsonObjectKeyMaterializer temporary until full converter parity is stable
  implementation_allowed=0 in owner-selection row
```

Reason:

```text
critical key bridge retirement remains rejected by full converter parity
focused dynamic-key probes pass without the bridge
full converter parity can still misroute same-length keys through JsonNode.object_key_equals
```

## Stop Lines

```text
do not add more RustSubset source-shape rows without a real input blocker
do not implement break/continue in the rust-subset app-front lane
do not remove JsonObjectKeyMaterializer during this closeout
do not implement JSON key equality fixes in the closeout row
do not reopen VM product route
```

## Report

```text
output_contract=rust-subset-syn-adapter-source-shape-coverage-closeout-v0
source_shape_batch_closed=1
supported_transport_shape_count=10
unsupported_handoff_shape_count=3
compiler_recipe_backlog_preserved=1
latest_full_smoke_summary=ok
next_lane=json_native_hardening
next_task=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
implementation_allowed=0
summary=ok
```
