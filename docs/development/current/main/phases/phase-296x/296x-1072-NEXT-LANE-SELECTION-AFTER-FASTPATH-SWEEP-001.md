Status: Design Consultation
Date: 2026-06-17
Scope: lane selection after exact-AOT fastpath sweep closeout
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1071-FRESH-EXACT-AOT-PERF-SWEEP-AFTER-FASTPATH-GAP-CLOSEOUT-001.md

# NEXT-LANE-SELECTION-AFTER-FASTPATH-SWEEP-001

## Purpose

Stop at the lane-selection decision point after the fastpath sweep found no
fresh exact-AOT compiler optimization owner.

This is intentionally a design consultation card. It does not select a new
implementation owner by itself.

## Current Evidence

The latest exact-AOT sweep succeeded on all 16 candidate fronts:

```text
successful_front_count=16
aot_failed_front_count=0
fresh_compiler_optimization_owner_selected=0
selected_perf_owner=none
selected_perf_owner_confidence=none
```

The user-box method fastpath gap is also closed as a false gap:

```text
focused_known_receiver_direct_method_route_count=19
focused_known_receiver_direct_method_thin_entry_covered_count=19
focused_known_receiver_direct_method_uncovered_count=0

whole_known_receiver_direct_method_route_count=184
whole_known_receiver_direct_method_thin_entry_covered_count=184
whole_known_receiver_direct_method_uncovered_count=0
```

The only Hako-slower successful exact front is tiny-floor residue:

```text
front=kilo_micro_substring_views_only
c_kernel_instr=1301
ny_kernel_instr=3100
ratio_kernel_instr=0.42
ratio_kernel_cycles=0.44
classification=tiny_floor_rejected
```

The nearest non-tiny mixed result is not a strong owner:

```text
front=kilo_meso_substring_concat_array_set
ratio_kernel_instr=1.59
ratio_kernel_cycles=0.93
classification=near_parity_cycles_not_fresh_owner
```

## Options

### A. Pause exact-AOT fastpath optimization

Return to compiler construction / selfhost compiler work.

```text
recommended=1
reason=no fresh exact-AOT owner selected
reason=fastpath metadata gaps closed
reason=continuing optimization now risks ratio-chasing or tiny-floor chasing
```

This keeps the compiler architecture cleaner because new implementation rows
remain owner-first and evidence-first.

### B. Open wider perf-owner discovery

Run a wider benchmark search before choosing a new implementation owner.

```text
recommended=0
allowed=1
condition=user explicitly wants optimization lane to continue
condition=benchmark set is widened before implementation
```

This is safe only as measurement work. It must not start backend changes until
a new Hako-slower owner appears.

### C. Deep-dive near-parity substring_concat_array_set

Investigate the small cycles gap in `kilo_meso_substring_concat_array_set`.

```text
recommended=0
allowed=conditional
condition=perf annotate/asm finds a concrete hot owner
not_allowed=ratio-only implementation
```

This is lower priority because the front already has fewer Hako instructions
and no clear owner from the sweep.

## Recommendation

Choose A unless the user explicitly wants to continue optimization discovery:

```text
recommended_next_lane=compiler_construction
exact_aot_fastpath_lane_pause=1
next_optimization_requires_fresh_owner=1
```

This matches the current state:

```text
no selected perf owner
no uncovered user-box fastpath gap
no AOT coverage blocker
no strong Hako-slower non-tiny exact front
```

## Contract

```text
output_contract=next-lane-selection-after-fastpath-sweep-v0

fresh_compiler_optimization_owner_selected=0
user_box_fastpath_gap_closed=1
exact_aot_candidate_sweep_done=1

recommended_next_lane=compiler_construction
exact_aot_fastpath_lane_pause_recommended=1
wider_perf_owner_discovery_allowed=1
near_parity_deep_dive_allowed_only_with_hot_owner=1

implementation_started=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0

requires_user_or_owner_decision=1
summary=needs_decision
```

## Stop Lines

```text
do not start another fastpath implementation without a fresh owner
do not optimize tiny-floor fronts
do not optimize near-parity fronts from ratio alone
do not reopen user-box LocalFastPathFact producer while uncovered_count=0
do not treat this card as completion of compiler construction work
```
