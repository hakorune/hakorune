# Failure/Outcome semantic manifest — MAINT0 execution task

Status: **Executable mechanical prerequisite — separate commit**
Date: 2026-07-24
Consumer row: `RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION0-S0`

## Purpose

Restore the existing min-gate generated-artifact parity before PUBLICATION0
closeout. This task changes no Rust behavior, route, owner, semantic
classification policy, or publication authority.

Current measured drift:

```text
tracked evidence occurrences = 605
current evidence occurrences = 625
tracked semantic sites        = 55
current semantic sites        = 56
missing_argument_zero pending = 0 -> 0
```

The delta consists of current source/test evidence additions and moved line
anchors, including weak-upgrade and ConstValue carrier observations. The
pending baseline does not widen.

## Exact operation

```bash
python3 tools/docs/failure_outcome_semantic_site_graph.py --write
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
```

Review the generated diff before commit:

```text
new unresolved semantic class          = 0
pending baseline increase              = 0
manual JSON edit                        = 0
Rust/source behavior delta              = 0
publication source edit                 = 0
```

Run the focused generator tests:

```bash
python3 -m unittest \
  tools.docs.test_failure_outcome_semantic_site_graph \
  tools.docs.test_failure_outcome_exhaustiveness
git diff --check
```

Commit this generated-artifact refresh separately from PUBLICATION0 Rust
changes.

## Non-claims

```text
new Failure/Outcome semantic decision = 0
pending-site acceptance widening      = 0
runtime/provider behavior change      = 0
Raw publication implementation        = 0
public ingress / CUT0                  = 0
```

## Proof budget

```text
ceremony_tier = T0 mechanical generated-artifact refresh
sunset_id = none
new proof authority = 0
budget_repayment_evidence = generator --check green with pending baseline unchanged
```
