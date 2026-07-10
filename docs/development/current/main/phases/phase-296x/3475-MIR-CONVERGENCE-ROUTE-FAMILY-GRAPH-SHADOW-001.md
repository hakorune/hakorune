# 3475 - MIR-CONVERGENCE-ROUTE-FAMILY-GRAPH-SHADOW-001

## Status

Complete code-facing BoxShape card after 3474 closes Match/record delimiter
ownership.

Decision: accepted by 3471 Decision C.

Implementation: complete.

## Selected Contract

```text
single convergence owner:
  typed route-family dependency graph

changed-function worklist:
  graph-derived mechanism

post-canonicalization invalidation:
  graph-derived local mechanism

current compile authority:
  existing full refresh retained

graph role in this card:
  shadow observation and parity only
```

The graph must describe semantic dependencies, never helper names, fixture
names, source paths, or wall-clock thresholds. Family-specific planners retain
their acceptance and materialization rules.

## Structural Implementation

1. Keep `route_fixpoint.rs` as the public convergence facade and place graph
   types/validation in a small dedicated module.
2. Define a closed typed `RouteFamily` set for the families currently sequenced
   by the fixpoint owner.
3. Define explicit directed dependency edges and validate missing nodes,
   duplicate edges, cycles where prohibited, and deterministic ordering.
4. Project changed families/functions into a sorted deterministic shadow
   worklist. Do not use function or helper names as policy.
5. Record post-canonicalization invalidation through the same graph edge model.
6. Compare shadow coverage/order with the existing full-refresh execution and
   fail fast on missing family coverage or stale dependency reads.
7. Emit stable count/hash observations through the existing compile-timing
   boundary; logs remain opt-in through `NYASH_MIR_COMPILE_TRACE`.

## Shadow Evidence

```text
convergence_epoch_count
dirty_function_count
recomputed_function_count
unchanged_function_recompute_count
route_family_recompute_count
stale_metadata_read_count = 0
full_refresh_parity_mismatch_count = 0
worklist_determinism_hash
dependency_edge_count
post_canonicalization_invalidation_count
```

Wall-clock time remains secondary evidence. The primary regression contract is
deterministic graph/worklist behavior and parity with the retained full refresh.

## Forbidden Designs

```text
helper-name shortcut
fixture-only cache
source-path dependency
stale metadata tolerance
iteration-cap reduction as the fix
wall-clock-based termination
unordered worklist
route acceptance change
compile authority switch
runtime or backend fallback
```

## Fail-Fast Tags

```text
mir/convergence_dependency_missing
mir/convergence_stale_metadata_read
mir/convergence_full_refresh_parity_mismatch
mir/convergence_nondeterministic_worklist
mir/convergence_unbounded_epoch
mir/convergence_dirty_edge_missing
mir/convergence_helper_name_shortcut_forbidden
mir/convergence_fixture_cache_forbidden
mir/convergence_iteration_cap_only_fix_forbidden
```

## Fixture Matrix

```text
no source or metadata change
  -> empty dirty set

one function body change
  -> only graph-dependent function/families dirty

canonicalization changes route metadata
  -> local graph invalidation recorded

one route-family row change
  -> dependent functions/families dirty

unrelated helper name change
  -> no dependency without a semantic graph edge

same input repeated
  -> identical sorted worklist and determinism hash

injected missing edge or stale read
  -> stable fail-fast

shadow result vs full refresh
  -> parity
```

## Acceptance

```text
route_family_dependency_graph_owner_count = 1
route_family_graph_shadow_only = 1
full_refresh_authority_retained = 1
convergence_authority_switch = 0
deterministic_worklist = 1
stale_metadata_read_count = 0
full_refresh_parity_mismatch_count = 0
helper_name_shortcut_count = 0
fixture_cache_count = 0
source_over_800_lines = 0
docs_only_closeout = forbidden
```

Implemented evidence:

```text
typed graph owner:
  src/mir/route_dependency_graph.rs

retained execution owner with shadow trace:
  src/mir/route_fixpoint.rs

route family count = 8
dependency edge count = 12
full refresh authority retained = 1
parity mismatch count = 0

50/100/250 method probes:
  deterministic worklist hash = 16134699973845251250
  shadow contract error = empty

merged grammar adapter first refresh:
  function count = 298
  dirty function count = 0
  unchanged function recompute count = 298
  outer iterations = 4
  family recomputes = 23

merged grammar adapter post-canonicalization refresh:
  function count = 298
  dirty function count = 0
  unchanged function recompute count = 298
  outer iterations = 2
  family recomputes = 19
```

The graph models deterministic family closure and sorted opaque function
worklists. Function names are identities only and cannot select affected
families. This card intentionally does not switch execution authority or claim
compile-time reduction.

Verification must include focused graph/unit tests, existing semantic refresh
tests, MIR compile-scaling tests, a measured grammar-adapter compile trace, the
current-state pointer guard, and `git diff --check`.

## Non-Claims

```text
convergence_authority_switch = 0
changed_function_worklist_authority = 0
post_canonicalization_graph_authority = 0
route_acceptance_changed = 0
stale_metadata_allowed = 0
iteration_cap_only_fix = 0
runtime_backend_fallback = 0
language_v1_grammar_closeout = 0
selfhost_claim = 0
```

## Next

After this shadow card is green, proceed directly to the accepted
`TEST-PROCESS-STATE-SCOPED-CONFIG-OWNER-001` task. Do not switch convergence
authority or open a rerun-only card.
