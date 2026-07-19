---
Status: Decision closed
Date: 2026-07-20
Scope: PHI0 predecessor-row readiness before any production facade connection
Related:
  - docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
  - src/mir/builder/phi_completion/
  - src/mir/builder/emission/phi_lifecycle.rs
  - src/mir/builder/builder_emit.rs
---

# PHI predecessor-readiness consultation

## Decision frontier

`PHI0-S0` sealed a disconnected `PhiDraftV1` that requires caller-supplied
exact predecessor rows. `PHI0-M0` proves that the current four Builder entries
cannot all obtain those rows from one existing authority. Consequently,
`PHI0-I0` must not connect that product yet.

```text
PHI0-M0 (closed evidence)
  -> PHI0-PRED0-D0 (closed decision)
  -> PHI0-PRED0-S0 / P0
  -> PHI0-I0 generic input/type connection
  -> PHI0-CFGREADY0 route-owned activation
```

The question is not whether a PHI type decision is shared: it already is. The
question is whether *generic input replacement* and *CFG-ready PHI completion*
are one operation at every current entry.

## Observed authority matrix

| Entry | Existing predecessor readiness | Why a generic current-CFG scan is invalid |
| --- | --- | --- |
| raw emission | current emitted CFG at the immediate call site | no shared route witness is passed today |
| complete final insert | caller route / current CFG | physical inputs are rematerialized live |
| provisional patch | sometimes intentionally unsealed | an existing test patches a block before predecessors are published |
| batch | route-owned `header_pred_policy` and candidate CFG | a legal host-entry predecessor can precede its terminator |
| Binding SSA adapter | `VerifiedPredecessorsV1` | this proof is Binding-SSA scoped, not generic Builder truth |

The expected predecessor set may not be derived from the incoming rows
themselves: that would validate only sorting/duplication and would not detect a
missing or phantom CFG row.

## Fixed non-authorities

`PHI0-PRED0` must not introduce any of the following:

```text
a persistent global predecessor/CFG table
a blanket lowering-time call to final verifier or compute_predecessors
logical inputs used as their own expected predecessor authority
patch-time rematerialization
new global rollback for raw/final live materialization residuals
origin-policy merge or value_origin_newbox widening
function-level transient type publication
Binding SSA proof widened into generic Builder authority
```

Final verification remains the whole-function backstop, not a source of
lowering-time predecessor truth.

## Candidate directions

### A — force one current-CFG authority

Rejected. It invalidates legal unsealed patch and future-edge batch paths.

### B — retain S0 exact-row validation but let each caller pass its inputs

Rejected. This is self-certification and makes the exact-row contract empty.

### C — install a new global predecessor-readiness table

Rejected. It duplicates CFG authority and violates PHI0's BoxShape boundary.

### D′ — split semantic boundaries before I0

Selected. The semantic split is:

```text
generic PHI input replacement / type publication
  from
route-owned CFG-ready predecessor validation
```

The former is the common four-entry transaction. The latter is consumed only
where an existing route already owns a sealed predecessor witness. A
provisional patch must not pretend to be CFG-ready just because it has
received inputs. This does not accept a new source shape and does not alter the
final verifier's authority.

`PHI0-I0` means generic input/type completion only. A future `PHI0-CFGREADY0`
may initially admit only canonical resolved-If
`VerifiedIfMergePredecessorsV1` and the existing CorePlan select-as-PHI proof.
Binding SSA, `exprs_peek`, generic loop, JoinIR exit, header batch, and legacy
paths require their own route-local proof or remain generic. No entry is
silently promoted from dominance, cached predecessors, or caller input rows.

## Proposed task order

```text
PHI0-PRED0-D0
  closed: Candidate D′ and its owner inventory are fixed

PHI0-PRED0-S0
  split only the selected private capability/vocabulary
  production consumers = 0

PHI0-PRED0-P0
  prove unsealed patch, future-edge batch, route-owned ready cases, and
  final-verifier parity

PHI0-I0
  connect all four entries only to generic input/type completion

PHI0-CFGREADY0
  activate only explicitly route-owned readiness consumers

PHI0-G0
  prove generic completion bypass and partial type publication are zero
```

`PHI0-PRED0-D0` is a real design stop, not permission to write an adapter.
The first code change is allowed only after its terminology makes every
consumer's precondition mechanically checkable.

## Claims after the decision

Until `PRED0` is green, PHI0 may claim only that a disconnected product exists
and that the existing type decision is shared. It must not claim that all four
entries share one CFG/predecessor validation authority, that provisional patch
is CFG-ready, or that final verifier facts are available during lowering.
