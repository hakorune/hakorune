---
Status: SSOT mirror
Date: 2026-08-09
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Now

## Current

Read the machine-readable pointer first:

```text
CURRENT_STATE.toml
  -> active_lane / work_mode
  -> current_execution_row / current_blocker_token
  -> latest_workstream_card / latest_card_path
  -> current_design_stop / current_execution_design
```

Current mode is `design_stop`. The parser public-AST/postpass, V2 schema, typed
callable syntax carriage, old instance-result/target retirement,
source-handoff D0/I0, resolver declaration/signature I0, and Home callable ABI
D0 design, passive Home relation vocabulary S0, bounded Home ABI0 S0, and
declared Query behavior D0/I0, aggregate D0/I0 are closed. The general
body-source I0 and borrowed Query body-source projection I0 are also closed.
The current execution row is:

```text
CALLABLE-BODY-OWNER-BINDING-D0
```

The Query body-selection D0/I0 is closed. Its aggregate-owned borrowed
selected-Query view and borrowed `<'body,'contract>` sparse projection check
parser provenance and resolver brand, preserve sparse source order, keep the
general catalog reusable, and emit no default non-Query row. The current
design stop is the exact body-source/`VerifiedResolvedFunctionV1` owner
co-seal. No body facts, conformance, target, Recipe/CallSlot, Builder/MIR, or
production work is open.

The preceding I0 deleted the audited caller-zero body-inferred
instance-result/target family and preserved only neutral source-view
primitives. The rich parser now owns a non-Clone ordinary-Rust-Box source seal,
the handoff is consumed once into an AST-free resolver ingress, and the closed
declaration/signature I0 issues one fresh resolver nominal/type catalog with
semantic `I64`/`Unit` classes. Relation0 is closed as a passive vocabulary
slice, and its relation brand is batch provenance only—not the resolver
catalog brand or nominal type identity. The closed ABI0 implementation issues
only one non-`Clone` same-brand/site I64/Unit Home catalog. Query I0 issues
only the typed non-empty Query subset and never duplicates Home relations.
Aggregate I0 implements only their same-brand/site/order relational co-seal;
Home remains the declaration owner. Resolver targets, source-bound CallSlot
relations, ScanWithInit, physical lowering, production selection, and legacy
retirement remain closed. The one-shot parser transaction/provenance bridge,
general direct-cohort body cardinality, and borrowed Query projection are
landed; the FunctionOwner co-seal is the current design stop. Body
conformance must not pair by name, inventory ordinal, Query re-selection, or
MIR facts.

The explicit LoopRecipe V2 wire (`I64|Bool|Unit|Text`, local `CallSlot`, and
`TextEq`) is implemented and its seven-test focused closeout is green. No
fallback or source/physical shortcut is allowed.

## Restart

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Read the active card and workstream named by `CURRENT_STATE.toml`. Do not use
historical loop chronology in this mirror to select S6C, M9, or production
cutover. The ordered path remains:

```text
typed syntax carriage (closed)
-> old instance-result/target retirement (closed bounded I0)
-> declaration / Home ABI / declared Query aggregate (closed)
-> body-source transaction / general AST-free body catalog (closed I0)
-> declared Query body-source projection (closed I0)
-> body-owner co-seal
-> body facts / conformance catalog
-> target / source-bound relation / Recipe CallSlot
-> S6C ScanWithInit
-> M9 parity
-> M10 semantic co-seal and transfer authority
-> production selection / M10b
-> M11/M12 retirement
```

## Rule

This file is only a mirror. Implementation details, acceptance, landed
history, and parked tasks belong in the active card, workstream SSOT, phase
cards, or git history.
