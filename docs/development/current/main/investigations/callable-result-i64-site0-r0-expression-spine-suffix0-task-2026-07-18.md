---
Status: SUFFIX0-P0 closed; SUFFIX0-I0 next
Date: 2026-07-18
Decision: positive exact-body-suffix classification before any raw router view
Parent: callable-result-i64-site0-r0-expression-spine-task-2026-07-18.md
Scope: behavior-neutral proof substrate; production consumers remain zero
---

# Callable-result SITE0-R0 expression-spine SUFFIX0 task

## Decision

The located BLK0 path must never expose `body[index..]` to the legacy suffix
router merely because a proof attempt returned an error. Active suffixes are a
normal classification result, not a failure and not a retry boundary.

```text
LocatedLegacyBodyV1 + checked start
  -> LocatedLegacyBodySuffixV1
  -> caller ledger exact classification
       Inactive(VerifiedCallableResultInactiveBodySuffixV1)
       Active { first exact row }
```

The exact membership authority is:

```text
domain_parent + SourceBodyKindV1 + start item index
```

A row belongs to the suffix iff stripping the exact domain parent leaves an
item segment owned by that body kind and its canonical item index is greater
than or equal to `start`. All descendants of that item belong to the suffix.

The semantic body-root carrier, AST equality, span, names, enum ordering, and
repeated statement-prefix probes are not suffix authority.

## Authority split

| Concern | Authority |
| --- | --- |
| exact borrowed body slice | existing located body carrier |
| body family and domain parent | existing BODYDOMAIN0 carrier |
| body item variant and ordinal | `SourceBodyKindV1::owned_item_index` |
| active/inactive suffix decision | caller ledger |
| exact first active witness | activation rows in canonical coverage order |
| raw suffix routing | existing BLK0 driver/router, unchanged in S0 |
| Loop/JoinIR recipe and consumed count | existing normalization owners |

The inactive proof is non-Clone and retains the exact borrowed slice, caller,
body family, domain parent, start, and plan lifetime. The unverified suffix
carrier exposes no raw statement slice. Only the verified inactive product may
implement the future read-only slice view.

## Exact classification law

```text
start conversion:
  checked usize -> u32

accepted bounds:
  0 <= start <= body.len

start == len:
  Inactive with exact empty suffix

row at current or later item:
  Active { first }

rows only before start:
  Inactive

row in sibling body or condition:
  outside this suffix domain

foreign plan/caller, unlocated body, overflow, start > len:
  typed error
```

Classification is read-only. It never claims a row, writes the claim set,
constructs a source path from AST syntax, or catches `RowsUnderPrefix`.

## Task order

```text
SUFFIX0-S0
  -> SUFFIX0-P0
  -> SUFFIX0-I0
  -> LOOP0-D0
```

### SUFFIX0-S0 — disconnected classifier

Production behavior delta: 0. Production consumers: 0.

Add:

```text
SourceBodyKindV1::owned_item_index
LocatedLegacyBodySuffixV1
VerifiedCallableResultInactiveBodySuffixV1
CallableResultBodySuffixDecisionV1
VerifiedCallableResultCallerLedgerV1::classify_body_suffix
```

`owns_item_segment` delegates to the new index authority. No second body-kind
match table is allowed in the ledger.

Required evidence:

```text
full/inactive root suffix and exact borrowed-node identity
earlier row outside suffix
current and later rows active with exact first witness
mixed earlier+later rows do not stop at the earlier row
nested If and Loop descendants belong to their enclosing root item
IfThen/IfElse sibling isolation and condition exclusion
empty body and end suffix
overflow and out-of-bounds distinction
foreign plan and foreign caller
active classification followed by exact claims and successful finish
Builder/CFG/router/claim-state delta = 0
```

S0 closeout:

```text
checked located suffix carriers = 1
positive Active/Inactive classifiers = 1
non-Clone exact borrowed inactive proofs = 1
body-item inverse-index authorities = 1

actual empty body / end suffix = green
condition / If sibling / nested If / direct Loop separation = green
foreign plan/caller / unlocated / checked bounds = green
classification then exact claims and finish = green

production classifier/constructor consumers = 0
Builder / BLK0 / router / claim-state delta = 0

focused suffix fixtures = 6/6
caller-ledger fixtures = 14/14
callable-result fixtures = 65/65
public EXPR0-SPINE0 structural guard = green
cargo check --all-targets = green
release build = green
quick = 66/66 in 71.09s
worker authority/code/test audits = 3 GO
modified source/check files >= 800 lines = 0
```

### SUFFIX0-P0 — disconnected route parity

Production behavior delta: 0.

Add a cfg(test)-only associated-input reference proving that a verified
nonempty inactive Loop suffix reaches the existing router, while an Active
decision supplies no router input and continues statement descent. Always-None
located routing is explicitly non-parity. Keep environment changes under the
existing test lock/restore law.

P0 closeout:

```text
test-only classified suffix ports = 1
existing production block driver calls from the reference = 1
production port/driver/router deltas = 0

inactive Loop:
  proof-backed selected route demand = [0]
  selected statement descent = [1]
  raw production block normalized MIR parity = exact

active suffix:
  exact first witness = Body(1).Value
  router input = none
  statement descent = [0, 1]

always-none control:
  statement descent = [0, 1]
  selected inactive route parity = explicitly false

focused route parity = 3/3
callable-result fixtures = 68/68
block-driver fixtures = 5/5
BLK0 and public EXPR0-SPINE0 guards = green
cargo check --all-targets = green
release build = green
quick = 66/66 in 208s
worker authority/code/test audits = 3 GO
modified source/check files >= 800 lines = 0
```

The raw and always-none lanes happen to emit equivalent MIR for the minimal
`loop(true) { break }` fixture. P0 therefore fixes route non-parity with the
exact statement-descent witness, while normalized MIR equality remains the
selected-versus-raw production parity authority.

### SUFFIX0-I0 — behavior-neutral block seam cutover

Generalize `LegacyBlockDescentPortV1` with one associated suffix-view type.
The raw port remains the sole raw-slice constructor; a future located port can
use only the inactive proof. The central driver remains the sole environment,
router, function-name, prefix-variable, consumed-index, termination, and scope
owner.

I0 does not add a production located root or replace the existing whole-body-
inactive `RecursiveChildLoweringPortV1::lower_body` path. The later C0 root
connector and LOOP0 body connector remain explicit separate rows.

## Counters and guards

```text
body item-index authorities = 1
suffix source carriers = 1
suffix classifiers = 1
inactive suffix proof owners = 1
active decision witnesses = 1

unverified raw-slice accessors = 0
claim writes/calls inside classifier = 0
RowsUnderPrefix catch-based selection = 0

S0 production classifier consumers = 0
S0 block-port/router/Builder deltas = 0
production located roots = 0

AST rewalk/path reconstruction/name/span heuristics = 0
retry/fallback = 0
source/check files >= 800 lines = 0
```

## Non-claims

```text
general raw suffix safety outside the future typed port
Loop site carriage or Loop activation
JoinIR/CorePlan recipe widening
statement consumption or ledger claim ordering
CFG, PHI, scope, termination, or result authority
active branch widening under the closed IF0 route
production callable-result publication
runtime/backend/ownership widening
```

## Stop conditions

Stop if any row needs:

1. `RowsUnderPrefix` or another error caught as active/inactive selection.
2. A literal second match table for Body/IfThen/IfElse/Loop item variants.
3. AST rewalk, span/name lookup, enum ordering, or reconstructed statement paths.
4. Only the start item checked while later suffix items are ignored.
5. A sibling body or condition treated as part of the selected body domain.
6. An unverified located suffix exposing `&[ASTNode]`.
7. The proof discarded before the raw slice reaches the future port.
8. Plan/view/ledger/proof stored in `MirBuilder`.
9. Router/environment/recipe/consumed-index policy duplicated outside BLK0.
10. Existing `lower_body` globally replaced or active IF branches widened.
11. Retry, fallback, production root activation, or source/check file >=800 lines.

## Final lock

> SUFFIX0 selects one positive exact-body-suffix classifier. The source view
> constructs a checked, non-raw located suffix carrier; the caller ledger uses
> only the existing domain parent, one `SourceBodyKindV1` item-index authority,
> and the start index to return either a non-Clone inactive proof or the exact
> first active row. Active is a normal result, never a caught proof error.
> S0 changes no Builder, block driver, suffix router, claim state, production
> caller, grammar, or runtime behavior. P0 proves route parity and I0 later
> cuts the existing BLK0 port seam over without adding a located production
> root. Loop carriage remains parked at LOOP0-D0.

docs_only_closeout = forbidden
