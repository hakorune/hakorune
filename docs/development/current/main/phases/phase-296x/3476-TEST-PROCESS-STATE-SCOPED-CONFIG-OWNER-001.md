# 3476 - TEST-PROCESS-STATE-SCOPED-CONFIG-OWNER-001

## Status

Complete. The Language v1 grammar closeout evaluation proves that the current
22-row registry is not exhaustive: loops, weak surfaces, records, and current
literal surfaces required by `LANGV1-GRAMMAR-001` are still absent. 3477 owns
the resulting consolidated grammar-surface decision stop.

Decision: accepted by 3471 Decision D.

Implementation: complete.

## Selected Contract

```text
single test-isolation owner:
  scoped config injection

subprocess role:
  baseline/contamination classification oracle only

production defaults:
  unchanged

global lock:
  not an ownership substitute
```

Tests must not rely on process order, parallel scheduling, ambient environment,
or a production-default change. Scoped state must restore on success, failure,
and panic unwind.

## Structural Implementation

1. Inventory process-global writes in parser feature gates, plugin loader state,
   and MIR strictness controls. Record the owning config API, not every caller.
2. Add one reusable before/after process-state snapshot boundary to the test
   harness.
3. Classify the five suspect route tests with a fresh-subprocess oracle and the
   pre-3472 baseline evidence.
4. Select the highest-impact leaking family and inject one typed scoped config
   object. Do not migrate unrelated families in the same slice.
5. Make restoration RAII/unwind-safe and fail fast on leaked state.
6. Keep subprocess cases as classification fixtures; normal unit tests should
   consume scoped config directly.

## Classification Law

```text
fails on pre-3472 and in a fresh subprocess:
  baseline expectation drift candidate

passes in a fresh subprocess but fails after another same-process test:
  process-global contamination

fails only under parallel scheduling with snapshot drift:
  process-global interference

parallel/serial aggregate difference without state evidence:
  insufficient proof
```

## Stable Tags

```text
test/global_state_leak
test/env_contamination_detected
test/process_global_write_without_scope
test/baseline_expectation_drift
test/subprocess_isolation_mismatch
test/parallel_only_global_interference
test/production_default_change_forbidden
test/scoped_config_missing
test/plugin_loader_state_leak
test/parser_feature_gate_state_leak
test/mir_strictness_state_leak
```

## Forbidden Designs

```text
production default change to make tests green
manual test-order dependency
ignore list as isolation
environment mutation without restoration
global lock as the only owner
serial-only success claim
subprocess-only test architecture
fixture-specific runtime branch
runtime or backend fallback
```

## Fixture Matrix

```text
scoped environment/config write and restore
  -> snapshot unchanged

unscoped environment write
  -> test/env_contamination_detected

plugin loader global mutation leak
  -> test/plugin_loader_state_leak

MIR strictness mutation leak
  -> test/mir_strictness_state_leak

route test fails in fresh subprocess and pre-3472
  -> test/baseline_expectation_drift classification

route test passes fresh but fails after another test
  -> test/process_global_write_without_scope classification

scoped config panic/unwind
  -> original state restored
```

## Acceptance

```text
test_isolation_owner_count = 1
test_isolation_owner = ScopedTestConfig
process_state_snapshot_guard = 1
fresh_subprocess_classification_oracle = 1
first_high_impact_family_scoped = 1
scope_restores_on_unwind = 1
production_default_changed_for_tests = 0
test_order_dependency = 0
source_over_800_lines = 0
docs_only_closeout = forbidden
```

Implemented evidence:

```text
single root test owner:
  src/test_support.rs::ScopedTestConfig

snapshot/drift owner:
  ProcessStateSnapshot / ProcessStateDrift

stable leak tag:
  test/env_contamination_detected

scope restoration tests:
  absent value = green
  existing value = green
  panic unwind = green
  follow-on scope after poisoned panic = green

first always-on migrated family:
  src/tests/mir_locals_ssa.rs

additional legacy family migrated:
  mir_funcscanner_skip_ws / mir_funcscanner_skip_ws_min

direct env writes in migrated files = 0
ScopedTestConfig source lines = 192
production default changes = 0
```

Fresh subprocess baseline classification:

```text
global_call_route_plan:
  refresh_module_global_call_routes_accepts_map_handle_child_field_get_string_body
  refresh_module_global_call_routes_rejects_arbitrary_unknown_map_return_as_box_type_inspector
  refresh_module_global_call_routes_rejects_non_static_array_factory

user_box_method_route_plan:
  refresh_module_user_box_method_routes_accepts_loop_carried_nullable_object_return
  refresh_module_user_box_method_routes_refines_void_placeholder_object_route_result

classification:
  all five fail in fresh processes with the pre-3472 values
  -> test/baseline_expectation_drift
```

`mir_locals_copy_instructions_emitted` also retains an unrelated baseline test
bug: it treats `%1` reuse in different functions as one SSA namespace. This
card does not change that expectation merely to make the suite green.

Verification must include focused snapshot/scoped-config tests, the classified
route-test matrix in fresh and same-process modes, relevant parser/plugin/MIR
tests, the current-state pointer guard, and `git diff --check`.

## Non-Claims

```text
all_process_global_state_removed = 0
full_lib_test_isolation_closeout = 0
production_default_changed_for_tests = 0
language_v1_grammar_closeout = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

The scoped owner and full current grammar corpus are green. Do not claim
`LANGV1-GRAMMAR-001` closeout from that green corpus: it covers 22 rows across
11 spellings, while the workstream explicitly requires loops, weak, records,
and current literal surfaces too. Continue at 3477 and classify those missing
families before extending the registry.
