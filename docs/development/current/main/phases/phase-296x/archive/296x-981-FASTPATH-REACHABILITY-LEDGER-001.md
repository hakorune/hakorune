# 296x-981 FASTPATH-REACHABILITY-LEDGER-001

Status: Landed
Date: 2026-06-17
Scope: tooling / docs only

## Purpose

Make fast-path route selection visible before adding more backend consumers.

The substring-concat closeout showed a recurring ambiguity:

```text
backend consumer code exists
MIR metadata candidate may or may not exist
an old exact seed may select the executable route first
```

This row adds an observation-only ledger so follow-on work can see the active
front's selected route, reachable consumer, and preempted candidates without
guessing from backend source files.

## Implementation

Added:

```text
tools/hako_check/fastpath_reachability_ledger.py
tools/hako_check/tests/test_fastpath_reachability_ledger.py
```

Updated:

```text
tools/hako_check/README.md
docs/tools/check-scripts-index.md
```

The tool reads an existing MIR JSON artifact. It does not emit MIR, rewrite
source, alter route priority, force backend reachability, retire exact seeds, or
make benchmark winner claims.

## V0 Surface

The first ledger surface is intentionally narrow:

```text
exact_seed_backend_route:
  explicit selected route

string_dead_text_region_plans:
  generic metadata-path candidates
```

Candidate existence means the active MIR front emitted the metadata. The tool
does not infer candidates from helper names, backend consumer files, benchmark
names, or source names.

If an explicitly selected exact seed and a generic metadata-path candidate both
exist, the generic candidate is reported as preempted and
`winner_claim_allowed=0`.

If only an unselected candidate exists, it is not treated as reachable.

## Evidence

Unit fixture with both an exact seed and a generic metadata candidate:

```text
output_contract=hako-fastpath-reachability-ledger-v0
front=kilo_micro_substring_concat
candidate_count=2
selected_route=substring_concat_loop_ascii
selected_route_owner=function_level_exact_seed
selected_backend_consumer=substring_concat_loop_ascii
new_consumer_exists=1
new_consumer_reachable=0
old_exact_seed_selected=1
preemption_detected=1
forced_reachability_allowed=0
winner_claim_allowed=0
summary=ok
```

Current real `bench_kilo_micro_substring_concat.hako` MIR report:

```text
output_contract=hako-fastpath-reachability-ledger-v0
front=kilo_micro_substring_concat
candidate_count=1
selected_route=substring_concat_loop_ascii
selected_route_owner=function_level_exact_seed
selected_backend_consumer=substring_concat_loop_ascii
new_consumer_exists=0
new_consumer_reachable=0
old_exact_seed_selected=1
preemption_detected=0
forced_reachability_allowed=0
winner_claim_allowed=1
summary=ok
```

Reading:

```text
old exact seed route is explicitly selected
string_dead_text_region_plans are not emitted for this active MIR front
there is no generic metadata-path winner claim from this front
```

## Stop Line

This row does not:

```text
change backend lowering
change route priority
force generic metadata-path reachability
retire substring_concat_loop_ascii
add benchmark/source/helper-name branches
claim a new performance winner
```

## Next

```text
FASTPATH-UNREACHABLE-CONSUMER-GUARD-001
```

The next row should prevent new backend consumers from being counted as wins
unless they are either reachable in the active front or explicitly marked as
unreachable scaffolding with `winner_claim_allowed=0` and a named follow-up.
