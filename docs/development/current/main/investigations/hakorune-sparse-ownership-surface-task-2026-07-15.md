---
Status: Accepted parked taskboard; production activation 0
Date: 2026-07-15
Decision: Explicit-move/share, owner-anchored sparse ownership surface
Source semantics SSOT: ../../../../reference/language/ownership.md
Current lane: unchanged; follow CURRENT_STATE.toml
First executable ownership row when selected: OWN-GRAM-REJECT0 exact inactive-syntax fail-fast
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
  return forwards implicitly; an existing source binding otherwise uses move

independent lifetime:
  explicit share

Shared lane:
  explicit share creates every additional owner
  compiler-managed DestroyOwned closes each owner

API-only non-default contracts:
  parameter move/share
  result view/share

ownership expressions:
  move forwards owner 1 -> 1
  share adds one same-identity owner

copy/clone names:
  ordinary calls; no ownership authority
```

The current task does not add grammar or change runtime behavior. The accepted
semantics are reference-first; parser, resolver, verifier, Builder, runtime,
and backend activation remain separate rows.

## 2. Dependency map

```text
OWN-REF-D0  initial reference reconciliation
    |
    v
OWN-REF-D1  move/share/copy vocabulary reconciliation
    |
    v
OWN-GRAM-REJECT0  exact inactive ownership-result syntax fail-fast
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
GRAM-MOVE0   passive contextual move expression
             |
             v
GRAM-SHARE0  passive contextual share expression
             |
             v
GRAM-PARAM0  passive move/share parameter modes
             |
             v
GRAM-RESULT0 passive view/share result modes
             |
             v
REF-GRAM0    live syntax/reference reconciliation
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
             +--> MOVE-STORE0
             |
             +--> SHARE-PLAN0 -> SHARE-I0
                                      |
                                      +--> REF-RUNTIME0
                                      +--> RESOURCE0 -> WEAK0
                                      +--> SYNC0

parked independent rows:
  COPY-PROTOCOL0  verified Copyable/FreshIdentityWitness
  CLONE-LINT0     optional non-semantic style lint
```

No row after a branch point may start merely because its type names exist.
Every activation row consumes the sealed products from its prerequisites.

## 3. Documentation closeout — OWN-REF-D0 / OWN-REF-D1

Objective: establish one normative source contract before code.

State: closed. D0 introduced the sparse surface; D1 renamed consuming source
spelling to `move`, made every same-identity owner acquisition explicit as
`share`, and removed `clone`/method-name policy from ownership authority.
Behavior, parser grammar, backend, and current-lane deltas remain zero.

Artifacts:

- `docs/reference/language/ownership.md`
- reconciled `variables-and-scope.md`, `lifecycle.md`, and
  `scope-exit-semantics.md`
- reference navigation pointers
- B-prime narrowed to the Shared/resource runtime lane
- old root/view taskboards marked as evidence/subtasks of this board
- `stage-profiles.md` records `move/share/view` as accepted-but-parser-inactive

Acceptance:

```text
source ownership SSOT count = 1
current production activation delta = 0
CURRENT_STATE active-lane delta = 0
EBNF/parser accepted-row delta = 0
design/INDEX.md line delta = 0
```

After OWN-REF-D1, the next ownership action is the bounded
`OWN-GRAM-REJECT0` conformance repair, followed by the generated O2-P0a
artifact. Do not insert another ownership consultation or docs-only card.

## 4. Evidence prerequisite — O2-P0a/P0r/P0b1/P0c

Reuse the detailed classifiers and preliminary counts in the historical root
alias taskboard. Materialize machine-readable, reproducible artifacts for:

1. initialized local kind: whole root / projection / rvalue / call / other;
2. exact eligible owner-root and alias-chain sites;
3. callable result provenance and current signature transport;
4. destination kind: noescape use / owner forward / independent lifetime /
   unknown boundary;
5. current source identifiers that collide with contextual `move`, `share`, or
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

## 5. Grammar and passive transport — GRAM-MOVE0 / SHARE0 / PARAM0 / RESULT0

### OWN-GRAM-REJECT0 — inactive syntax must not become a type name

Before any ownership grammar is activated, both current parsers must reject
exact lookalike result forms such as `: view Node` and `: share Service` with
one stable contract tag. Today the return-type scanners may discard whitespace
and coalesce these spellings into ordinary names such as `viewNode` or
`shareService`; that is a known conformance gap, not compatibility behavior.

Required closure:

```text
Rust parser exact reject fixtures
Hako parser exact reject fixtures
shared stable reason/tag
ordinary type names `view` / `share` remain legal where unambiguous
ordinary calls move(...) / share(...) remain unchanged
grammar/AST/resolver/Builder/runtime activation = 0
silent type-name coalescing = 0
```

This guard is the first executable ownership row when the parked lane is
selected. It only makes the documented inactive-syntax failure boundary true;
it does not make `move`, `share`, or `view` parser-live.

Exact target grammar is activated as contextual keywords, one durable row at a
time:

```ebnf
ownership_expr := ('move' | 'share') unary_expr

parameter := IDENT (':' TYPE_REF)?
           | 'move' IDENT ':' TYPE_REF
           | 'share' IDENT ':' TYPE_REF

result_spec := ':' TYPE_REF
             | ':' 'view' TYPE_REF view_anchor?
             | ':' 'share' TYPE_REF

view_anchor := 'from' ('me' | IDENT)
```

Constraints:

- `move`, `share`, and `view` are not globally reserved identifiers.
- Prefix `move/share` is an ownership expression only when it is not followed
  by `(`; `move(...)` and `share(...)` remain ordinary identifier calls.
- In a parameter list, `move/share IDENT ':'` is a modifier; `move: T` and
  `share: T` remain parameters named `move`/`share`.
- In a result spec, `view/share` is a modifier only when followed by a
  `TYPE_REF`; a declaration of a type literally named `view` or `share`
  remains distinguishable by the following delimiter.
- The first anchor grammar admits only receiver/parameter WholeObject roots.
  Field paths, static anchors, and named domains are later rows.
- return type and return ownership are distinct AST/schema fields.
- `MoveExpression` and `ShareExpression` are separate dedicated
  AST/source-carrier rows. Resolver/Lower string matching is forbidden.
- `@rune Ownership(...)` is not used as source ownership authority.
- Rust parser, Hako parser, grammar registry, AST JSON, macro/source carrier,
  and corpus fixtures close together.
- unsupported backends/routes fail before effects.

Existing bindings passed to consuming/owning destinations use `move`; fresh
Owned rvalues and terminal `return` do not repeat it. Local `owned`/`borrow`
annotations and mandatory `clone` spelling do not belong to the baseline.

### Reference activation tasks

`REF-GRAM0` closes only when the grammar rows above are parser-live and updates
these live-syntax authorities together:

```text
docs/reference/language/EBNF.md
docs/reference/language/quick-reference.md
docs/reference/language/stage-profiles.md
grammar registry / parser support matrix
```

Before `REF-GRAM0`, `ownership.md` is the accepted target contract while EBNF
and quick-reference continue to report the syntax as unsupported. It is a
failure to document parser-live syntax before activation or to activate syntax
without updating all four surfaces.

## 6. Resolved intent and Loan Flow — O2-A0 / O2-L0

Passive products:

```text
ResolvedBindingInitIntentV2:
  OwnRvalue
  ScopedAlias(root, loan)
  MoveOwner(root, expression_site)

ShareOwnershipPlanV1:
  PromoteUniqueRootAndAcquire(root, expression_site)
  AcquireFromSharedRoot(root, expression_site)
  PromoteFreshRvalueAndForward(expression_site)

VerifiedCallableOwnershipAbiV1:
  parameters = Alias | Move | Share
  result     = Trivial | Owned | View(anchor/domain) | Share

VerifiedScopedLoanFlowV1:
  creation / root / uses / last-use frontier / conflicts
```

Laws:

- Binding SSA remains the only `BindingRef -> ValueId` authority.
- Loan Flow carries permission/liveness only; it is not a second value map.
- aliases flatten to one exact whole-root binding;
- Move intent forwards one token and normally emits no MIR ownership opcode;
- Share plans seal Unique-vs-Shared-vs-fresh materialization before Builder
  effects and carry no ValueId/BasicBlockId;
- owner move/rebind/fini/rehome/share is permitted only with a sealed
  no-live-loan witness;
- alias/View escape, suspension, PHI, reassignment, and unknown ABI reject;
- runtime type, method name, pointer equality, RC count, and map diff are not
  intent authority.
- `copy()` and `clone()` calls remain ordinary callable ABI rows; their names
  never create Move/Share intent or fresh-identity proof.

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
- fixes: narrow scope, use `move owner` after last-use, or acquire an
  independent owner with `share owner` after last-use;
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
alias last-use before explicit owner move/drop
no call escape, projection, Shared, resource, weak, dyn, plugin, FFI, task,
or suspension
```

Required claims:

```text
ScopedAlias owner-token delta = 0
ScopedAlias CopyOwned = 0
ScopedAlias DestroyOwned = 0
Move ownership runtime opcode = 0
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
- consuming `move` parameter;
- Shared parameter;
- default Owned/Trivial result;
- exact static/final callee only.

ABI conversion law:

```text
Owned binding -> move parameter:
  call site uses `move actual`; consume after no-live-loan proof

fresh Owned rvalue -> move parameter:
  forward directly without a redundant move marker

Unique -> share parameter:
  reject unless the actual expression is explicit `share owner`

Shared -> share parameter:
  `share actual` adds an owner, or `move actual` transfers an existing owner

Alias/View -> share parameter:
  reject

Unique -> share result:
  callee return expression must cross explicit `share`
```

Unknown dyn/interface/plugin/FFI ABI remains a typed preflight error. Interface
and plugin metadata become later exact-match rows, never inferred adapters.

`MOVE-STORE0` is a separate owning-destination row after the first local Box
family. It accepts one exact strong field/store family:

```text
field = move owner:
  forward one token; source becomes unavailable; CopyOwned 0

field = fresh_owned_rvalue:
  forward the temporary token; extra move marker 0

replace occupied field:
  materialize next -> commit store -> DestroyOwned previous
```

Collection, registry, closure capture, outbox compatibility, plugin/FFI, and
unknown destinations remain separate. `field = owner` must not silently move
or share an existing binding; diagnostics offer `move owner` or `share owner`
according to the intended lifetime.

## 10. Anchored views — VIEW0

The existing Anchored View taskboard owns the detailed sub-DAG. The first
production row is an instance-method receiver WholeObject anchor,
straight-line, same-task, and noescape. Parameter anchors follow before field,
static, named-domain, or temporary-anchor rows.

`VIEW0` is a branch-selection umbrella, not a code-facing implementation row.
Its code-facing sequence is `PROJ-S0` through `PROJ-I0` in the Anchored View
taskboard. Selecting `VIEW0` must not skip the parent evidence, grammar,
Loan-Flow, first-Box, or callable-ABI prerequisites.

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

`share` is the only source operation that adds a same-identity owner. On an
eligible Unique root with no active loan, it rehomes the source binding as a
still-usable Shared owner and yields one independent Shared owner. On an
already-Shared root, it acquires one additional owner without rehome. On a
fresh rvalue, only the result remains source-visible. Optimizers may remove
redundant count traffic but cannot change this availability law.

Each `CopyOwned` must carry `ExplicitShare` provenance from an exact source
site or an equally explicit verified boundary ABI. Ordinary Shared assignment
does not silently acquire an owner. `DestroyOwned` remains compiler-managed.
The physical first slice may use a correctness-first count strategy, but the
source contract does not expose atomic/non-atomic mode.

Must reject:

- hidden Unique-to-Shared promotion;
- `share` with live aliases/views;
- `share` on a scoped alias/view operand or a trivial value;
- Share on weak/raw/unsupported/unknown representation;
- Shared owner copies on an unsupported backend;
- cross-thread publication without the later synchronization capability.

B-prime resource tombstones, weak generation, host handles, plugins, and
cross-thread Shared begin only after SHARE-I0 is correct.

`REF-RUNTIME0` then reconciles runtime/Box reference manuals with the landed
representation and retires language-authority wording around
`clone_box`/`share_box`/`clone_or_share`. It must not edit those manuals ahead
of runtime activation merely to describe target behavior as current.

`COPY-PROTOCOL0` is independent and non-blocking. It may define a verified
`Copyable` protocol and `FreshIdentityWitness`, but ordinary `value.copy()` is
just an Owned-returning call until that witness exists. `CLONE-LINT0` may warn
that `clone` has no ownership meaning; it must never change lowering.

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
- alias last-use followed by owner return/`move`/fini/share;
- stable aliases in nested If/Loop;
- default noescape call;
- Owned result forwarding;
- explicit share entry followed by Shared independent lifetime;
- already-Shared `share` adds exactly one owner with exact provenance;
- `copy()`/`clone()` method names do not alter ownership intent;
- zero ownership bookkeeping for alias/View;
- debug oracle does not change release lifetime.

Must reject:

- live-loan owner consume/rebind/fini/share/rehome;
- alias/View escape, capture, suspension, reassignment, or PHI;
- projection alias in the V1 profile;
- implicit Unique promotion;
- implicit Shared owner acquisition on ordinary assignment/store/call;
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
hidden Shared owner acquisitions = 0
CopyOwned without explicit source/boundary provenance = 0
runtime ownership-mode decisions = 0
callee-name ownership decisions = 0
silent raw fallbacks = 0
profile retry/fallback = 0
```

## 14. May claim / must not claim

After OWN-REF-D1 only, may claim:

```text
the target source ownership contract is accepted
ordinary local spelling is preserved
move is the explicit existing-owner transfer
share is the only same-identity owner-acquisition boundary
copy/clone method names have no ownership authority
the implementation order and fail-fast boundaries are parked
production behavior has not changed
```

Must not claim before the corresponding activation row:

```text
current parser accepts move/share/view ownership syntax
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
2. requires routine `owned`, `borrow`, or `clone` annotations outside the
   exact `move`/`share` ownership operations;
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
15. inserts `CopyOwned` for an ordinary Shared assignment without exact
    `share` or verified-boundary provenance.
16. treats `copy()` or `clone()` spelling as fresh-identity, retain, or
    ownership-mode authority.
