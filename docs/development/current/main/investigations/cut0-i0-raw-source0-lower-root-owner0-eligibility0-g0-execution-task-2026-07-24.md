# RAW-SOURCE0 LOWER ROOT0 OWNER0 — ELIGIBILITY0-G0 evidence task

Status: **Closed — S0 evidence closeout green**
Date: 2026-07-24

## Boundary

This is a small evidence-only closeout for
`RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0`. It does not widen the
eligibility grammar or open any physical owner.

## Required additions

Add focused fixtures proving:

```text
valid runtime args/safepoint survive bind into RawSourceContinuationV1
malformed runtime input is rejected before token issuance
plain static Main with a helper method has an exact eligible catalog
scalar order/cardinality reaches eligibility without a second body scan
nested Lambda and instance/constructor shapes reject before physical effects
```

Strengthen the S0 guard only for the new evidence:

```text
runtime capture producer is one
bind path has a positive and malformed fixture
eligibility fixture registration is explicit
physical/production consumers remain zero
all touched files remain below 800 lines
```

The existing source plan remains the sole body classifier. Catalog validation
may inspect declaration shape only; it must not re-run the ScalarControl0
body classifier. No retry, fallback, session, shell, collector, ledger,
tracker, root lowering, finalization, postprocess, or commit is allowed.

## Evidence

```bash
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_ --lib -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_owner0_eligibility0_s0_guard.py
git diff --check
```

After this row is green, close S0/G0 and advance the pointer to
`RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PHYSICAL0`.
