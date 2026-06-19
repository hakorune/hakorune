---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the next owner for the scan-methods loop blocker after the
  ambiguous loop-var freeze moved to a multi-candidate loop shape.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1302-COREPLAN-SCAN-LOOP-COMMA-CLOSE-OWNER-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1289-COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1288-COREPLAN-LOOP-RESOLVER-SHADOW-001.md
---

# COREPLAN-SCAN-METHODS-MULTI-CANDIDATE-LOOP-OWNER-SELECTION

## Current Task Snapshot

Current status:

```text
original_loop_correctness_freeze=advanced
current_failure_mode=compile_time_timeout
focused_direct_timeout=1
focused_gate_green=0
current_owner=typed_object_box_origin_value_origin_lookup_cost
current_blocker_token=TYPED-OBJECT-BOX-ORIGIN-VALUE-ORIGIN-CONTEXT-OWNER-001
```

Focused command:

```bash
timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

Immediate task queue:

```text
1. TYPED-OBJECT-BOX-ORIGIN-VALUE-ORIGIN-CONTEXT-OWNER-001
   Reduce BoxOriginQueryContext copy-origin lookup cost by sharing
   function-local copy-origin resolution. Preserve origin semantics.

2. Re-run focused direct command.
   If still timed out, sample the new owner and update this card plus
   CURRENT_STATE.toml before further implementation.

3. COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
   Use only if ownership keeps moving across unrelated passes or a new
   pass-level ledger is needed before more optimizer/typed-object edits.
```

Current stop lines:

```text
do not commit this row as closed while focused_direct_timeout=1
do not reopen legacy loop route perfection from compile-time-owner evidence
do not add source/function-name workarounds
do not merge box-origin and storage inference ownership while reducing lookup
cost
```

## Decision

The current blocker is not a PHI SSOT issue and not a safe one-line
`generic_loop_v1` candidate preference patch.

The failing function is:

```text
ParserBox.static_const_bitand/2
```

The visible loop shape is:

```hako
loop(a > 0 || b > 0) {
  ...
  a = a / 2
  b = b / 2
  bit = bit * 2
}
```

`generic_loop_v1` currently sees both `a` and `b` as valid loop-var
candidates and freezes under strict/planner_required:

```text
[plan/freeze:ambiguous] multiple loop_var candidates matched
```

Two drive-by implementations were rejected during local investigation:

```text
1. treating ambiguity as Ok(None)
   result=compile progressed, then ParserBox import / focused gate timed out or stack-overflowed

2. selecting the preferred left-side candidate
   result=unit-level extraction can pass, but the focused gate still times out/stack-overflows
```

Therefore, the next owner is a small owner-selection row for multi-candidate
loop semantics, not immediate lowering.

## B-lite Boundary

The current B-lite code remains a legacy registry observer:

```text
legacy_matched
legacy_effective
legacy_suppressed
legacy_selected
```

It is not yet an independent semantic resolver because its decision is still
derived from the existing registry candidate set. Do not promote it to route
selection.

The desired split remains:

```text
legacy_registry_observer.rs
  observe legacy matched/effective/suppressed/actual-selected routes

loop_resolver.rs
  frozen loop evidence -> Allow/Deny
  does not read ENTRIES or collect_candidates

loop_resolution_diagnostics.rs
  compare legacy observation with resolver decision
  feedback_to_resolver=0
```

## Historical Selection Context

```text
COREPLAN-MULTI-CANDIDATE-LOOP-OWNER-SELECTION-001
```

Purpose:

```text
Decide whether `a > 0 || b > 0` with independent updates is owned by:
  A. a conservative multi-carrier generic-loop recipe
  B. LoopCondBreak / flowbox adoption
  C. a source-level selfhost rewrite (rejected unless language semantics require it)
  D. deferred unsupported shape with explicit fixture expectation
```

Required before implementation:

```text
minimal fixture without ParserBox import
focused evidence for compile route and runtime behavior
no route-name / function-name special case
no preferred-candidate lowering without runtime proof
```

Separate cleanup row:

```text
COREPLAN-LOOP-ROUTE-RETIRE-001
target=registry_candidate_suppression
first_branch=loop_cond_continue_only_redundant_suppression
```

This cleanup is still valid but must not be confused with the active
multi-candidate loop blocker.

## Stop Lines

```text
do not select a loop-var candidate by variable name
do not lower multi-candidate loops through generic_loop_v1 without runtime proof
do not treat B-lite legacy observer parity as independent resolver proof
do not add route suppression to force the focused fixture through
do not rewrite ParserBox.static_const_bitand as a workaround
```

## Report

```text
output_contract=coreplan-scan-methods-multi-candidate-loop-owner-selection-v0
active_fixture=selfhost_blocker_scan_methods_loop_min
observed_function=ParserBox.static_const_bitand/2
failure=ambiguous_loop_var_candidates
phi_ssot_reopened=0
preferred_candidate_patch_rejected=1
ok_none_patch_rejected=1
b_lite_shadow_is_legacy_observer=1
resolver_selection_owner_enabled=0
next_task=COREPLAN-MULTI-CANDIDATE-LOOP-OWNER-SELECTION-001
summary=ok
```

## Current Implementation Probe

Local implementation work progressed the original ambiguous-loop freeze, but
the focused gate is not green yet.

Implemented probe slices in the current worktree:

```text
1. loop_cond owner retention
   - keep LoopCondBreakContinue ownership when the shape is a ConditionalUpdate
     loop-cond route
   - avoid invoking generic_loop_v1 multi-candidate hint in that owner case

2. self-referential collection-origin guards
   - typed object collection field-key inference returns unknown on PHI cycles
   - generic method route origin inference returns unknown on PHI cycles

3. PHI edge rematerialization expansion
   - allow RuntimeDataBox/StringBox.substring rematerialization for PHI
     predecessor edges
   - share def-block/dominator analysis within one PHI input materialization
     pass

4. string-kernel origin cache probe
   - add scan-local ValueOriginCache with copy-chain path compression
```

Observed result:

```text
original_freeze_advanced=1
stack_overflow_advanced=1
focused_gate_green=0
```

The remaining failure mode is compile-time timeout, not the original
multi-candidate freeze. GDB samples show the current hot owner in MIR
semantic refresh / string kernel plan refresh:

```text
owner=string_kernel_plan_refresh_work_explosion
path=semantic_refresh::refresh_function_semantic_metadata
path=string_kernel_plan::refresh_function_string_kernel_plans
path=string_kernel_plan::infer_string_kernel_text_consumer
path=string_kernel_plan::record_text_consumer_use
symptom=plan-value-by-plan-value full-function rescans with long copy-origin chains
```

`--dump-mir --no-optimize` also times out before output, so this is not a VM
execution issue and not an optimizer-only issue.

Next structural task:

```text
COREPLAN-STRING-KERNEL-REFRESH-WORK-OWNER-001
```

Purpose:

```text
Measure and reduce string-kernel refresh work for the focused scan-methods
fixture before committing the implementation probe as a green slice.
```

Likely fix direction:

```text
do not derive text-consumer facts by rescanning the whole function once per
string-kernel plan value. Build a function-local consumer/origin observation
once, then answer per-plan queries from that observation.
```

Stop line:

```text
do not disable string-kernel refresh globally
do not add function-name/source-name bypasses
do not commit the current code slice as closed while the focused gate times out
```

## Follow-up Implementation Probe 2026-06-19

The current worktree further reduced several compile-time work owners, but the
focused gate still times out.

Additional implemented probe slices:

```text
5. string-kernel consumer analysis modularization
   - split consumer analysis out of string_kernel_plan.rs
   - keep all touched files below 800 lines
   - derive function-local consumer analysis once for refresh

6. string-kernel verifier batch consumer inference
   - infer expected text consumers for verifier-owned plans in one function scan
   - keep single-plan API as a thin wrapper for tests/callers

7. typed-object box-origin query context
   - share box-origin memo/visiting sets across one param inference pass
   - keep existing storage inference semantics unchanged

8. generic method string route dispatch probe
   - dispatch route matchers by method surface before calling family-specific
     matchers
   - share GenericPureStringFlowAnalysis across string route matchers in one
     function refresh
```

Focused direct command remains:

```bash
timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

Current result:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

GDB owner progression:

```text
previous_owner=string_kernel_plan_refresh_work_explosion
previous_owner_closed_by=function_local_consumer_analysis

next_owner=string_kernel_verifier_consumer_rescan
next_owner_closed_by=batch_expected_text_consumer_inference

next_owner=typed_object_box_origin_interproc_rescan
next_owner_partially_closed_by=BoxOriginQueryContext_shared_per_param_pass

current_owner=semantic_refresh_def_map_rebuild_work_explosion
current_path=semantic_refresh::refresh_function_pre_fixpoint_routes
current_path=direct_array_extent_fact::refresh_function_direct_array_extent_facts
current_path=value_origin::build_value_def_map
```

Current interpretation:

```text
The original correctness freeze advanced. The blocker is now compile-time work
explosion in semantic_refresh, with many per-function passes rebuilding
ValueDefMap independently. Further patching one pass at a time is likely to
repeat the same owner. The next clean task should introduce a semantic-refresh
per-function analysis context or a smaller def_map sharing seam, then migrate
the hot pre-fixpoint producers in a controlled series.
```

Recommended next task:

```text
COREPLAN-SEMANTIC-REFRESH-FUNCTION-CONTEXT-001
```

Purpose:

```text
Create a read-only per-function refresh context that owns ValueDefMap and other
cheap shared observations for semantic refresh producers. Start with direct
array extent / range-index adjacent producers, then migrate only measured hot
producers. Do not change route semantics in the same row.
```

Stop line:

```text
do not keep adding per-producer memo patches after this point
do not mix semantic-refresh context creation with new acceptance shapes
do not commit this probe as a closed green slice while the focused command
still times out
```

## Follow-up Implementation Probe 2026-06-19b

The focused command is still not green, but the compile-time owner moved through
several structural seams. No additional loop route acceptance shape was added.

Implemented compile-time reductions:

```text
9. semantic refresh shared def-map seam
   - reuse one ValueDefMap across sum placement / dead-text / range-index /
     direct-array-extent refresh producers where already measured hot
   - behavior_changed=0

10. DCE local-field early return and PHI query memo hardening
   - skip overwritten local-field analysis when no local field candidates exist
   - memoize PHI copy-origin roots inside PhiBaseQueryContext
   - cycle remains deny/unknown, not fallback-to-fast

11. DCE and memory-effect worklist used-value propagation
   - replace repeated whole-function propagation scans with dst->uses worklists
   - preserve existing dead-use semantics

12. value-consumer clone-free used-value walker
   - avoid MirInstruction::used_values() clone path for value-consumer fact
     collection
   - keep local helper private to value_consumer.rs

13. simplify_cfg reachable-set owner reduction
   - pass computed reachable blocks into branch-thread and jump-merge finders
   - recompute after each CFG mutation, so no stale reachability is reused
```

Focused direct command:

```bash
timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

Current result after the reductions:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
commit_allowed=0
```

Latest sampled owner:

```text
current_owner=dce_local_fields_phi_query_origin_resolution
current_path=passes::string_corridor_sink::apply_string_corridor_transforms
current_path=passes::dce::elimination::eliminate_dead_code_in_function
current_path=passes::dce::local_fields::analyze_local_reads
current_path=phi_query::PhiBaseQueryContext::query
current_path=value_origin::resolve_value_origin
```

Interpretation:

```text
The original multi-candidate loop correctness blocker has advanced into
compiler construction cost. Repeated owner samples now show optimizer /
semantic-refresh passes rather than route selection itself. Continuing to
perfect legacy named loop routes is not the clean path. The clean next slice is
to finish compile-time owner reduction around DCE/local_fields PHI base queries,
or to introduce a narrow per-function query context for local-field analysis.
```

Next task options:

```text
Option A:
  COREPLAN-DCE-LOCAL-FIELDS-PHI-QUERY-CONTEXT-001
  Scope: share/cap PHI base queries inside analyze_local_reads and avoid
  repeated copy-origin traversal for the same function.

Option B:
  COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
  Scope: stop implementation, preserve all green local tests, and design a
  measured pass-level compile-time ledger before more optimizer edits.
```

Stop line:

```text
do not commit as closed while focused_direct_timeout=1
do not reopen old LoopSimpleWhile / named-route perfection from this evidence
do not add source/function-name bypasses
```

## Worker Audit / SSOT Decision 2026-06-19c

Two independent audits were run after the focused timeout remained. The result
changes the next task from ad-hoc debugging to a small SSOT repair.

### Value origin / def-map audit

Observed duplication:

```text
build_value_def_map_call_sites ~= 100
resolve_value_origin_call_sites ~= 223
custom_copy_chain_helpers ~= 39
```

Current owner boundaries:

```text
src/mir/value_origin.rs:
  owns ValueDefMap and copy-chain traversal rules

src/mir/phi_query.rs:
  owns PHI base-relation semantics
  currently contains a private cached copy-origin resolver

src/mir/string_kernel_plan.rs:
  currently contains a private ValueOriginCache

semantic_refresh.rs:
  partially shares ValueDefMap across some producers
```

Decision:

```text
The next clean task is not more loop-route patching. It is to make
value_origin.rs the reusable query-context SSOT for copy-chain origin
resolution, then migrate the current hot PHI/DCE consumer first.
```

### Reachability / CFG audit

Decision:

```text
compute_reachable_blocks remains the pure primitive owner in
verification/utils.rs. Reachability caching should stay pass-local because
mutating passes own invalidation timing. This is cleanup/reduction, not the
current focused timeout owner.
```

## Next Task: VALUE-ORIGIN-QUERY-CONTEXT-SSOT-001

Purpose:

```text
Introduce a reusable read-only ValueOriginQueryContext in value_origin.rs so
copy-chain traversal rules and memoization have one owner.
```

Scope:

```text
1. Add ValueOriginQueryContext to src/mir/value_origin.rs.
   - owns &MirFunction
   - owns &ValueDefMap
   - memoizes origin(value)
   - preserves current cycle behavior

2. Migrate PhiBaseQueryContext to consume ValueOriginQueryContext.
   - phi_query.rs keeps only PHI base-relation semantics
   - remove phi_query private resolve_value_origin_cached helper

3. Wire only the current hot DCE/local_fields path if needed.
   - construct one origin context for analyze_local_reads
   - do not migrate all origin call sites in this row
```

Acceptance:

```text
cargo test -q infer_phi_base_relation
cargo test -q dce
cargo build -q --bin hakorune --features vm-reference

focused direct command is re-measured:
timeout 90s env NYASH_DISABLE_PLUGINS=1 NYASH_VM_USE_FALLBACK=0 \
  HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
  target/debug/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako
```

Non-goals:

```text
do not migrate all 223 resolve_value_origin call sites
do not add global caches to MirFunction
do not change loop route semantics
do not add new accepted loop shapes
do not touch string_kernel_plan ValueOriginCache in the same slice unless the
focused owner still points there after PHI/DCE migration
```

Report vocabulary:

```text
output_contract=value-origin-query-context-ssot-v0
value_origin_query_context_enabled=1
copy_chain_traversal_owner=value_origin.rs
phi_query_private_origin_resolver_enabled=0
global_mirfunction_origin_cache_enabled=0
loop_route_semantics_changed=0
focused_direct_timeout=<0|1>
summary=<ok|blocked>
```

## Implementation Result: VALUE-ORIGIN-QUERY-CONTEXT-SSOT-001

Implemented:

```text
value_origin_query_context_enabled=1
copy_chain_traversal_owner=value_origin.rs
phi_query_private_origin_resolver_enabled=0
global_mirfunction_origin_cache_enabled=0
loop_route_semantics_changed=0
```

Code boundary:

```text
src/mir/value_origin.rs:
  adds ValueOriginQueryContext
  keeps resolve_value_origin as compatibility wrapper

src/mir/phi_query.rs:
  consumes ValueOriginQueryContext
  no longer owns private copy-origin traversal
```

Verified:

```text
cargo test -q value_origin
cargo test -q infer_phi_base_relation
cargo test -q dce
cargo test -q memory_effect
cargo build -q --bin hakorune --features vm-reference
```

Focused direct command remains timed out:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

Latest sampled owner after this SSOT repair:

```text
current_owner=dce_liveness_membership_cost
current_path=passes::string_corridor_sink::apply_string_corridor_transforms
current_path=passes::dce::elimination::eliminate_dead_code_in_function
current_path=used_values.contains(dst)
symptom=HashSet<ValueId> membership dominates DCE elimination scan
```

Interpretation:

```text
The value-origin SSOT issue is repaired for the hot PHI/DCE path. The focused
timeout moved from PHI query copy-origin traversal to DCE liveness membership.
The next task should not reopen loop route design. It should either improve the
DCE liveness set representation locally or stop for a pass-level compile-time
ledger before further optimizer edits.
```

Next task options:

```text
Option A:
  DCE-LIVENESS-MEMBERSHIP-OWNER-001
  Scope: reduce used_values/base_used_values membership cost in
  eliminate_dead_code_in_function without changing liveness semantics.

Option B:
  COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
  Scope: stop implementation and add a pass-level compile-time ledger before
  more optimizer edits.
```

Stop line:

```text
do not commit as closed while focused_direct_timeout=1
do not treat value-origin SSOT repair as focused gate success
do not reopen legacy loop route perfection from DCE membership evidence
```

## Implementation Result: DCE-LIVENESS-MEMBERSHIP-OWNER-001

Implemented:

```text
dce_live_value_set_enabled=1
dce_dense_uses_by_dst_enabled=1
dce_local_fields_dense_parent_map_enabled=1
liveness_semantics_changed=0
loop_route_semantics_changed=0
```

Code boundary:

```text
src/mir/passes/dce.rs:
  owns private LiveValueSet
  owns private UsesByDst
  avoids HashSet/HashMap membership on dense ValueId liveness propagation

src/mir/passes/dce/local_fields.rs:
  owns private DenseParentMap for local field alias parents
  does not change generic ParentMap in value_origin.rs
```

Verified:

```text
cargo test -q dce
cargo build -q --bin hakorune --features vm-reference
```

Focused direct command remains timed out:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

Latest sampled owner after DCE dense liveness:

```text
current_owner=string_corridor_use_count_hashmap_cost
current_path=passes::string_corridor_sink::apply_string_corridor_transforms
current_path=string_corridor_sink::shared::build_use_counts
symptom=HashMap<ValueId, usize> entry/update dominates use-count construction
```

Interpretation:

```text
The DCE liveness membership owner is reduced. The focused timeout moved to
string corridor use-count construction. This is still compile-time work
explosion, not loop route semantics. Further old-route patching is misaligned.
```

Next task options:

```text
Option A:
  STRING-CORRIDOR-USE-COUNT-OWNER-001
  Scope: reduce build_use_counts membership/update cost without changing string
  corridor transform semantics.

Option B:
  COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
  Scope: stop implementation and add a pass-level compile-time ledger before
  more optimizer edits.
```

Stop line:

```text
do not commit as closed while focused_direct_timeout=1
do not reopen legacy loop route perfection from string-corridor cost evidence
do not widen DCE dense containers into global MIR storage without a separate
SSOT design row
```

## Implementation Result: STRING-CORRIDOR-USE-COUNT-OWNER-001

Implemented:

```text
string_corridor_dense_use_counts_enabled=1
string_corridor_use_count_hashmap_enabled=0
string_corridor_transform_semantics_changed=0
typed_object_param_def_map_cache_enabled=1
typed_object_collection_def_map_cache_enabled=1
typed_object_value_origin_context_memo_enabled=1
loop_route_semantics_changed=0
```

Code boundary:

```text
src/mir/passes/string_corridor_sink/shared.rs:
  owns private UseCounts dense vector
  preserves the historical get(...).unwrap_or(0) query surface
  does not introduce global MIR storage

src/mir/typed_object_plan/storage_inference/param_inference.rs:
  builds FunctionDefMaps once per immutable module inference pass
  shares the map across param box-origin and param storage inference

src/mir/typed_object_plan/storage_inference/collection_storage.rs:
  keeps the historical wrapper
  adds a cached-def-map entry used by param inference only

src/mir/typed_object_plan/storage_inference/value_analysis.rs:
  adds value-origin memoization inside BoxOriginQueryContext
  keeps backend/storage decisions unchanged
```

Verified:

```text
cargo test -q dce
cargo test -q memory_effect
cargo test -q value_consumer
cargo test -q storage_inference
cargo build -q --bin hakorune --features vm-reference
```

Known non-focused local check:

```text
cargo test -q string_corridor_sink
result=fail
failing_tests=2
interpretation=existing benchmark-shape string-corridor expectations are not
the active focused gate; this row did not change string-corridor selection
logic, but the failure should be handled before a string-corridor closeout
claim.
```

Focused direct command remains timed out:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

Latest sampled owner after dense use-count and typed-object def-map reuse:

```text
current_owner=typed_object_storage_query_context_missing
current_path=typed_object_plan::storage_inference::value_analysis::storage::storage_for_function_returns
current_path=value_origin::build_value_def_map
symptom=storage_for_value creates a fresh storage query context per value;
return-storage analysis rebuilds callee def maps and storage/value-origin
memos do not span the param-inference scan.
```

Interpretation:

```text
The previous HashMap insertion owners were reduced step by step. The remaining
timeout is still typed-object compile-time work explosion. The next aligned
slice is a BoxShape cleanup: introduce/reuse a StorageQueryContext for
storage_for_value so param inference can share storage_memo,
return_storage_memo, and return def maps across a scan. This should not open
new accepted loop shapes or alter route ownership.
```

Next task options:

```text
Option A:
  TYPED-OBJECT-STORAGE-QUERY-CONTEXT-OWNER-001
  Scope: make typed-object storage inference reuse a storage query context
  across param-inference scans without changing storage semantics.

Option B:
  COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
  Scope: stop implementation and add a pass-level compile-time ledger before
  more optimizer/typed-object edits.
```

Stop line:

```text
do not commit as closed while focused_direct_timeout=1
do not reopen legacy loop route perfection from typed-object storage cost
evidence
do not turn StorageQueryContext into a new route/planner owner
```

## Implementation Result: TYPED-OBJECT-STORAGE-QUERY-CONTEXT-OWNER-001

Implemented:

```text
typed_object_storage_query_context_enabled=1
storage_query_context_shared_in_param_inference=1
storage_query_return_def_map_memo_enabled=1
storage_query_return_storage_memo_shared=1
storage_query_storage_memo_shared=1
storage_semantics_changed=0
loop_route_semantics_changed=0
```

Code boundary:

```text
src/mir/typed_object_plan/storage_inference/value_analysis_storage.rs:
  owns StorageQueryContext
  keeps storage_for_value wrapper as compatibility entry
  memoizes return def maps as Rc<ValueDefMap>
  does not become a route/planner owner

src/mir/typed_object_plan/storage_inference/param_inference.rs:
  creates one StorageQueryContext per birth/call param-storage scan
  shares storage_memo / return_storage_memo / return_def_map_memo across
  storage_for_value calls in the same immutable scan
```

Verified:

```text
cargo check -q --lib
cargo test -q storage_inference
cargo build -q --bin hakorune --features vm-reference
```

Focused direct command remains timed out:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

Latest sampled owner after StorageQueryContext:

```text
current_owner=typed_object_box_origin_value_origin_lookup_cost
current_path=typed_object_plan::storage_inference::param_inference::infer_call_param_box_origins
current_path=typed_object_plan::storage_inference::value_analysis::BoxOriginQueryContext::box_origin_for_value
current_path=value_analysis::cached_value_origin
current_path=value_origin::resolve_value_origin
symptom=BoxOriginQueryContext still resolves many distinct copy origins through
ValueOriginQueryContext, which repeatedly hits function.blocks HashMap while
following copy chains.
```

Interpretation:

```text
StorageQueryContext reduced the previous return-storage def-map owner. The
remaining timeout has moved back to box-origin inference. The next aligned
slice is still BoxShape/compile-time only: make BoxOriginQueryContext use a
function-local copy-origin index or parent-map cache instead of calling
resolve_value_origin for each distinct value.
```

Next task options:

```text
Option A:
  TYPED-OBJECT-BOX-ORIGIN-VALUE-ORIGIN-CONTEXT-OWNER-001
  Scope: share copy-origin resolution inside BoxOriginQueryContext, preferably
  through a function-local parent/index cache, without changing origin
  semantics.

Option B:
  COREPLAN-COMPILE-TIME-OWNER-CHECKPOINT-001
  Scope: stop implementation and add a pass-level compile-time ledger before
  more typed-object edits.
```

Stop line:

```text
do not commit as closed while focused_direct_timeout=1
do not add source/function-name workarounds
do not merge box-origin and storage inference ownership while reducing lookup
cost
```

## Implementation Result: TYPED-OBJECT-BOX-ORIGIN-VALUE-ORIGIN-CONTEXT-OWNER-001

Implemented:

```text
box_origin_function_local_copy_origin_index_enabled=1
box_origin_resolve_value_origin_call_on_hot_path=0
origin_semantics_changed=0
storage_semantics_changed=0
loop_route_semantics_changed=0
```

Verified:

```text
cargo check -q --lib
cargo test -q storage_inference
cargo build -q --bin hakorune --features vm-reference
```

Focused direct command remains timed out:

```text
focused_direct_timeout=1
focused_direct_timeout_seconds=90
focused_gate_green=0
```

Latest sampled owner moved to `user_box_method_route_plan::origin_inference`
copy-origin lookup. Continue in:

```text
296x-1304-USER-BOX-METHOD-ROUTE-ORIGIN-INFERENCE-VALUE-ORIGIN-CONTEXT-OWNER-001
```
