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
source-handoff D0/I0, resolver declaration/signature I0, Home callable ABI
D0 design, passive Home relation vocabulary S0, bounded Home ABI0 S0, declared
Query behavior D0/I0, aggregate D0/I0, general body-source I0, borrowed Query
body-source projection I0, bounded Query body-facts, complete-evidence design,
bounded evidence I0, bounded exact Query conformance catalog I0, and the
bounded Call+Return effect/control I0 are closed. The current design stop is:

```text
CALLABLE-BODY-HOME-FLOW-D0
```

The Call+Return row landed one private borrowed receipt from one
`ResolvedFunctionBodyShapeProductV1` for exact root-direct
`return me.invoke()` (Call + ordinary Return + exact source relations), with
five focused tests. Home-flow is not an implementation row yet: the source
Home event/grammar issuer, Home-demand ABI issuer, and CFG-complete ownership
witness are absent. Keep Home flow at `NoSafeSlice`; target, Recipe/CallSlot,
Builder/MIR, publication, fallback, and production remain closed.

The Query body-selection D0/I0 is closed. Its aggregate-owned borrowed
selected-Query view and borrowed `<'body,'contract>` sparse projection check
parser provenance and resolver brand, preserve sparse source order, keep the
general catalog reusable, and emit no default non-Query row. The carrier
D0/I0 is closed. Its permitted route was the parser transaction keeping
AST-backed syntax only inside a single
transaction-scoped callback/lease, the resolver constructs
`FunctionSyntaxViewV1` there, and only AST-free carrier/catalog products escape.
The carrier retains exact source identity, parser/resolver brands, nominal Box,
owner-bearing forest/function, root/body receipt, and body coverage. Owner-
binding D0/I0 is closed: the non-`Clone` owner-link consumes only the selected
Query body projection and existing resolver-issued carrier/catalog; the carrier
root is the exact resolved-function input and no second owner issuer exists.
The owner-link I0 is closed, and the resolved-shape D0 is accepted. The body-
facts audit found that `VerifiedResolvedFunctionV1` and its source-site
inventory lack neutral expression/statement kind, field/method identity,
return-value relation, and complete effect/control evidence. The current I0
extends the existing parser-private syntax lease and shadow traversal with
one neutral AST-free shape inventory, bounded to receiver lexical reads and
ordinary returns. The carrier keeps that shape from the same owner-tree walk
and co-seals it with the declaration catalog's parser provenance and resolver
brand. Direct field/state authority remains closed. The shape I0, bounded
Query body-facts I0, and bounded structural-safety/Home no-transfer evidence
I0 are now closed. The evidence aggregate and conformance catalog prove only
the exact `return me` cohort. General evidence now stops at one design
boundary that must define complete body coverage, neutral effect events,
neutral control/exit events, and a language Home-flow event authority before
one same-root co-seal can be issued. Existing body-shape effects are partial
(Print/IO and unsupported control are not fully recorded), and
`VerifiedHomeAbi` remains declaration-only. No general conformance catalog,
target, Recipe/CallSlot, Builder/MIR, or production work is open; broader
bodies remain `NoSafeSlice`.
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
landed; the resolver-issued FunctionOwner carrier is the current bounded fast
row before the FunctionOwner co-seal. Body conformance must not pair by name,
inventory ordinal, Query re-selection, FunctionOrigin, or MIR facts.

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
-> bounded body facts / conformance (closed I0)
-> general body execution evidence D0 (current design stop)
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
