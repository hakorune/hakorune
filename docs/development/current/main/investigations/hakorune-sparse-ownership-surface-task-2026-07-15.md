---
Status: Accepted parked taskboard; production activation 0
Date: 2026-07-15
Decision: Explicit-share, owner-anchored sparse ownership surface
Source semantics SSOT: ../../../../reference/language/ownership.md
Current lane: unchanged; D-prime Binding SSA next-row selection remains active
First executable ownership row when selected: O2-P0a generated initializer census
Related:
  - hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md
  - hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - design-registry-v1-sharded-manifest-task-2026-07-14.md
---

# Hakorune Sparse Ownership Surface Taskboard

## 0. Authority and parking law

This is the bounded execution owner for the accepted ownership reference. It
does not own source semantics and must not restate them differently.

```text
source ownership semantics:
  docs/reference/language/ownership.md

object lifecycle / fini / weak / reclamation:
  docs/reference/language/lifecycle.md
  design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md

implementation order:
  this taskboard

historical evidence and detailed fixture inventory:
  hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md

call-result View branch details:
  hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md
```

`CURRENT_STATE.toml`, `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, and
`10-Now.md` remain unchanged while this board is parked. Selection of an O2 row
requires an explicit current-lane decision. Canonical failure must never retry
the current SharedV1 route.

The 7,000-line design registry is not extended by this work. Its existing
sharded-manifest task owns cleanup and begins with `CLEAN0`; only after that
cutover may this task be registered in the generated/sharded index.

## 1. Closed decision

```text
ordinary local whole-root alias:
  mutable ScopedAlias; owner delta 0

ordinary parameter / receiver:
  noescape alias

ordinary return / owning destination:
  one owner forwarded

independent lifetime:
  explicit share

Shared lane:
  compiler-managed CopyOwned / DestroyOwned allowed

API-only non-default contracts:
  parameter take/share
  result view/share

ordinary-source owned/borrow/clone/move annotations:
  none
```

The current task does not add grammar or change runtime behavior. The accepted
semantics are reference-first; parser, resolver, verifier, Builder, runtime,
and backend activation remain separate rows.

## 2. Dependency map

```text
OWN-REF-D0  reference reconciliation
    |
    v
O2-P0a      generated initializer-shape census
    |
    +--> O2-P0r   exact whole-root eligibility census --+
    |                                                  |
    +--> O2-P0b1  exact callable/signature evidence ---+
                                                        v
O2-P0c      owning-destination / owner-count census
             |
             v
GRAM-SHARE0  passive contextual share expression
             |
             v
GRAM-PARAM0  passive take/share parameter modes
             |
             v
GRAM-RESULT0 passive view/share result modes
             |
             v
O2-A0        passive alias/source-intent carrier
             |
             v
O2-L0        source-CFG Loan Flow
             |
             v
O2-M0        owner availability / forwarding verifier
             |
             v
O2-DIAG0     typed diagnostics + non-owning debug oracle
             |
             v
UBOX-P0 -> UBOX-M0 -> UBOX-I0
             |
             +--> ALIAS-I0 -> ALIAS-CFG0 -> ABI0 -> VIEW0
             |
             +--> SHARE-PLAN0 -> SHARE-I0
                                      |
                                      +--> RESOURCE0 -> WEAK0
                                      +--> SYNC0
```

No row after a branch point may start merely because its type names exist.
Every activation row consumes the sealed products from its prerequisites.

## 3. Documentation closeout — OWN-REF-D0

Objective: establish one normative source contract before code.

State: closed by the documentation series that introduced this taskboard;
behavior, grammar, backend, and current-lane deltas are zero.

Artifacts:

- `docs/reference/language/ownership.md`
- reconciled `variables-and-scope.md`, `lifecycle.md`, and
  `scope-exit-semantics.md`
- reference navigation pointers
- B-prime narrowed to the Shared/resource runtime lane
- old root/view taskboards marked as evidence/subtasks of this board

Acceptance:

```text
source ownership SSOT count = 1
current production activation delta = 0
CURRENT_STATE active-lane delta = 0
EBNF/parser accepted-row delta = 0
design/INDEX.md line delta = 0
```

After OWN-REF-D0, the next O2 action must be the generated O2-P0a artifact,
not another ownership consultation or docs-only card.

## 4. Evidence prerequisite — O2-P0a/P0r/P0b1/P0c

Reuse the detailed classifiers and preliminary counts in the historical root
alias taskboard. Materialize machine-readable, reproducible artifacts for:

1. initialized local kind: whole root / projection / rvalue / call / other;
2. exact eligible owner-root and alias-chain sites;
3. callable result provenance and current signature transport;
4. destination kind: noescape use / owner forward / independent lifetime /
   unknown boundary;
5. current source identifiers that collide with contextual `share`, `take`, or
   `view` parsing.

Required outputs:

```text
tools/checks/fixtures/ownership_v2/initializer_census.json
tools/checks/fixtures/ownership_v2/callable_abi_census.json
tools/checks/fixtures/ownership_v2/destination_census.json
```

The row is evidence-only: parser changes, AST rewrites, ownership inference,
and production routing are zero. Projection frequency decides whether a later
explicit projection-view row is urgent; it does not widen `ScopedAliasV1`.

## 5. Grammar and passive transport — GRAM-SHARE0 / PARAM0 / RESULT0

Exact target grammar is activated as contextual keywords, one durable row at a
time:

```ebnf
ownership_expr := 'share' unary_expr

parameter := IDENT (':' TYPE_REF)?
           | 'take' IDENT ':' TYPE_REF
           | 'share' IDENT ':' TYPE_REF

result_spec := ':' TYPE_REF
             | ':' 'view' TYPE_REF view_anchor?
             | ':' 'share' TYPE_REF

view_anchor := 'from' ('me' | IDENT)
```

Constraints:

- `take`, `share`, and `view` are not globally reserved identifiers.
- Prefix `share` is an ownership expression only when it is not followed by
  `(`; `share(...)` remains an ordinary call to an identifier named `share`.
- In a parameter list, `take/share IDENT ':'` is a modifier; `take: T` and
  `share: T` remain parameters named `take`/`share`.
- In a result spec, `view/share` is a modifier only when followed by a
  `TYPE_REF`; a declaration of a type literally named `view` or `share`
  remains distinguishable by the following delimiter.
- The first anchor grammar admits only receiver/parameter WholeObject roots.
  Field paths, static anchors, and named domains are later rows.
- return type and return ownership are distinct AST/schema fields.
- `ShareExpression` is a dedicated AST/source-carrier row.
- `@rune Ownership(...)` is not used as source ownership authority.
- Rust parser, Hako parser, grammar registry, AST JSON, macro/source carrier,
  and corpus fixtures close together.
- unsupported backends/routes fail before effects.

No ordinary call-site `take`, local `owned`/`borrow`, or mandatory `clone`
spelling belongs to the baseline grammar.

## 6. Resolved intent and Loan Flow — O2-A0 / O2-L0

Passive products:

```text
ResolvedBindingInitIntentV2:
  OwnRvalue
  ScopedAlias(root, loan)

VerifiedSharePlanV1:
  RehomeRootAndAcquire(root, expression_site)
  AcquireFromFreshRvalue(expression_site)

VerifiedCallableOwnershipAbiV1:
  parameters = Alias | Take | Share
  result     = Trivial | Owned | View(anchor/domain) | Share

VerifiedScopedLoanFlowV1:
  creation / root / uses / last-use frontier / conflicts
```

Laws:

- Binding SSA remains the only `BindingRef -> ValueId` authority.
- Loan Flow carries permission/liveness only; it is not a second value map.
- aliases flatten to one exact whole-root binding;
- Share plans seal root-vs-fresh availability before Builder effects and carry
  no ValueId/BasicBlockId;
- owner consume/rebind/fini/rehome/share is permitted only with a sealed
  no-live-loan witness;
- alias/View escape, suspension, PHI, reassignment, and unknown ABI reject;
- runtime type, method name, pointer equality, RC count, and map diff are not
  intent authority.

O2-L0 starts with disconnected synthetic CFG fixtures. Builder connection and
runtime ownership operations are zero.

## 7. Owner availability and diagnostics — O2-M0 / O2-DIAG0

O2-M0 verifies one owner token across straight-line flow, branches, loops,
returns, cleanup edges, and errors. It consumes the existing function-owned
Binding SSA/Ownership SSA contracts instead of inventing another environment.

O2-DIAG0 golden-tests:

- owner/alias creation sites;
- conflicting consume/escape/invalidation site;
- next reachable loan use;
- fixes: narrow scope, move after last-use, or enter Shared using `share`;
- stable machine-readable reason/fix identifiers.

The debug oracle is non-owning. It may count loan records or poison/quarantine
retired cells, but it must never retain the object or change reclamation time.

## 8. First real Box — UBOX-P0 / UBOX-M0 / UBOX-I0

First production grammar:

```text
one exact BoxRef representation
straight-line local owner
whole-root ScopedAlias
read/write through owner and alias
alias last-use before owner forward/drop
no call escape, projection, Shared, resource, weak, dyn, plugin, FFI, task,
or suspension
```

Required claims:

```text
ScopedAlias owner-token delta = 0
ScopedAlias CopyOwned = 0
ScopedAlias DestroyOwned = 0
owner/alias sequential mutation parity = green
loan verifier + ownership verifier = green
unsupported representation/backend preflight = before Builder effects
```

The family activates atomically. A failed sparse-ownership route never retries
SharedV1.

## 9. Structured aliases and callable ABI — ALIAS-CFG0 / ABI0

ALIAS-CFG0 adds only:

- a stable alias whose definition dominates an `if`/`loop`;
- branch-local/loop-local aliases that close locally;
- cleanup/error edges in last-use verification.

Alias PHIs and alias reassignment remain zero.

ABI0 then activates:

- default noescape receiver/parameter;
- consuming `take` parameter;
- Shared parameter;
- default Owned/Trivial result;
- exact static/final callee only.

ABI conversion law:

```text
Unique -> take parameter:
  normal call; consume after no-live-loan proof

Unique -> share parameter:
  reject unless the actual expression is explicit `share owner`

Shared -> share parameter:
  normal call; compiler forwards/copies a Shared token

Alias/View -> share parameter:
  reject

Unique -> share result:
  callee return expression must cross explicit `share`
```

Unknown dyn/interface/plugin/FFI ABI remains a typed preflight error. Interface
and plugin metadata become later exact-match rows, never inferred adapters.

## 10. Anchored views — VIEW0

The existing Anchored View taskboard owns the detailed sub-DAG. The first
production row is an instance-method receiver WholeObject anchor,
straight-line, same-task, and noescape. Parameter anchors follow before field,
static, named-domain, or temporary-anchor rows.

View requirements:

```text
ordinary call result = Owned/Trivial
View only from verified callable ABI
method-name/runtime inference = 0
View CopyOwned/DestroyOwned = 0
live anchor/domain invalidation = typed error
```

Named container domains, temporary-anchor extension, projection loans, and
same-anchor ViewPhi are separate later decisions.

Arena ownership/escape/drop/promotion is also a separate Decision. It may be
prioritized later from selfhost performance evidence, but it is not a hidden
prerequisite of UBOX, alias, View, or Shared correctness and must not be bundled
into those activation rows.

## 11. Explicit Shared lane — SHARE-PLAN0 / SHARE-I0

`share` is the only Unique-to-Shared source boundary. On an eligible owned root
with no active loan, it rehomes the source binding as a still-usable Shared
owner and yields one independent Shared owner as the expression result. On a
fresh rvalue, only the result remains source-visible. Optimizers may remove
redundant count traffic but cannot change this availability law.

After this boundary only, the compiler may insert `CopyOwned` and
`DestroyOwned` for independent Shared owners. The physical first slice may use
a correctness-first count strategy, but the source contract does not expose
atomic/non-atomic mode.

Must reject:

- hidden Unique-to-Shared promotion;
- `share` with live aliases/views;
- redundant `share` on an already Shared value, a scoped alias/view operand,
  or a trivial value in the first profile;
- Share on weak/raw/unsupported/unknown representation;
- Shared owner copies on an unsupported backend;
- cross-thread publication without the later synchronization capability.

B-prime resource tombstones, weak generation, host handles, plugins, and
cross-thread Shared begin only after SHARE-I0 is correct.

## 12. Migration and sunset

SharedV1 and sparse ownership may coexist only as whole-source-unit input
profiles normalized into one compiler/runtime authority.

Required bridge law:

- cross-profile callable ABI is explicit and verified;
- no raw value or implicit owner-policy handoff;
- no failure retry under the other profile;
- no second MIR/runtime ownership implementation.

Retirement counters:

```text
SharedV1 source units = 0
LegacyImplicitShare producers = 0
LegacyImplicitShare MIR rows = 0
cross-profile bridge copies = 0
legacy name/value ownership maps = 0
profile fallback attempts = 0
```

Only after every counter is zero may SharedV1 source semantics be removed.

## 13. Required guard matrix

Must pass:

- owner and alias reads/mutations;
- alias chain root flattening;
- alias last-use followed by owner return/store/take/fini/share;
- stable aliases in nested If/Loop;
- default noescape call;
- Owned result forwarding;
- explicit share entry followed by Shared independent lifetime;
- zero ownership bookkeeping for alias/View;
- debug oracle does not change release lifetime.

Must reject:

- live-loan owner consume/rebind/fini/share/rehome;
- alias/View escape, capture, suspension, reassignment, or PHI;
- projection alias in the V1 profile;
- implicit Unique promotion;
- runtime/name/refcount ownership inference;
- unknown callable ABI;
- unsupported representation/backend;
- SharedV1 retry.

Authority counters:

```text
second BindingRef -> ValueId map = 0
ScopedAlias ownership tokens = 0
View ownership tokens = 0
hidden Unique-to-Shared promotions = 0
runtime ownership-mode decisions = 0
callee-name ownership decisions = 0
silent raw fallbacks = 0
profile retry/fallback = 0
```

## 14. May claim / must not claim

After OWN-REF-D0 only, may claim:

```text
the target source ownership contract is accepted
ordinary local spelling is preserved
share is the explicit independent-lifetime boundary
the implementation order and fail-fast boundaries are parked
production behavior has not changed
```

Must not claim before the corresponding activation row:

```text
current parser accepts share/take/view ownership syntax
current local assignment is ScopedAlias
production Box ownership/Loan Flow is active
all Box values have C-like cost
Shared uses non-atomic RC
projection aliases or alias/View PHIs work
Any/dyn/interface/plugin/FFI ownership ABI is solved
resource tombstones, weak, host handles, or sync are cut over
Arena allocation/escape semantics are accepted or active
SharedV1 is retired
current Arc runtime is retired
```

## 15. Stop conditions

Stop implementation/publication if a row:

1. silently promotes Unique to Shared;
2. requires routine `owned`, `borrow`, `move`, or `clone` annotations;
3. infers ownership from runtime type, method name, pointer, RC count, or map
   differences;
4. creates an ownership token or destroy operation for ScopedAlias/View;
5. consumes/rebinds/finalizes an owner while a loan is live;
6. retains objects in the debug oracle;
7. introduces alias/View PHIs without their dedicated verifier row;
8. accepts a projection as a whole-root loan;
9. assumes unknown ABI is noescape/View/Owned;
10. creates a second reaching-value or runtime ownership authority;
11. retries SharedV1 after sparse-profile failure;
12. widens grammar/backend behavior in a passive/schema row;
13. updates the giant design index instead of using its sharding task;
14. changes the current D-prime lane merely because this roadmap exists.
