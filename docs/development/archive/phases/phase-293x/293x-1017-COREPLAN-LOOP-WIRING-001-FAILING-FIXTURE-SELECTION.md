# COREPLAN-LOOP-WIRING-001: failing fixture selection

Status: Landed
Date: 2026-06-14
Scope: select the next concrete compiler expressivity blocker after E1 closeout.

## Selected Fixture

```text
case_id=selfhost_parse_loop_min
fixture=apps/tests/phase29bq_selfhost_blocker_parse_loop_min.hako
function=Main.parse_loop_min/3
```

## Failure

The fixture parses and reaches MIR verification, then fails with a dominator
violation in PHI input wiring:

```text
[freeze:contract][mir/verify:dominator_violation]
fn=Main.parse_loop_min/3
kind=phi_input
```

Observed command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
```

Focused row:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_parse_loop_min
```

## Classification

This is not a remaining legacy-v0 route problem. Active routed `loop_*_v0`
count is already zero.

Initial classification:

```text
selected_failure_kind=dominator_violation
selected_owner_family=loop_wiring_phi_inputs
box_count_or_shape=undecided
implementation_started=0
```

## Non-goals

- Do not add a new `loop_*_v0` box.
- Do not change the fixture expected result in the TSV to accept the failure.
- Do not add a fallback route.
- Do not implement the fix in this selection card.

## Next

```text
COREPLAN-LOOP-WIRING-002
```
