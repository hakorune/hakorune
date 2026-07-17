---
Status: S0 closed; P0 is next
Date: 2026-07-17
Decision: canonical declaration-order-independent unique bare-static recovery
Baseline: 4524c9675f
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Supersedes: callable-catalog-l0b-bare-static-recovery-design-stop-2026-07-17.md
Scope: complete same-module callable catalog cutover and one narrow recovery widening
---

# Callable catalog L0b canonical unique recovery task

## Current progress

`R0-CALLABLE-CATALOG-L0B-S0` is closed. The disconnected catalog now owns the
exact legacy declaration-store union through namespace-disjoint
`StaticBoxMethod` and `InstanceBoxMethod` rows. Static candidate lookup is a
separate static-only index; exact structured declaration lookup can address
either namespace. Program roots, singleton BoxDeclaration roots, and verified
empty non-declaration roots are sealed without widening nested discovery.

Constructors, top-level functions, ordinary-box static methods, records, and
sync boxes remain excluded. Production catalog producers/consumers and
recovery policy owners remain zero. Focused catalog tests are 7/7, `cargo
check` and quick 66/66 are green, formatting and diff checks are clean, and
every touched source/check file remains below 800 lines.

The next code-facing row is:

```text
R0-BARE-STATIC-RECOVERY0-P0
```

## Decision

Candidate A is accepted.

Bare-static recovery is defined only by the complete immutable declaration
catalog:

```text
exactly one StaticBoxMethod key for source name + checked arity:
  Unique(canonical key)

zero StaticBoxMethod keys:
  NoRecovery(NoCandidate)

two or more StaticBoxMethod keys:
  NoRecovery(Ambiguous)
```

Provider-first and caller-first declaration order therefore have the same
meaning. This is an intentional narrow semantic widening: source that the old
duplicate-registration map rejected after lowering the provider may now
resolve to the same unique callable that already resolved in caller-first
order.

The exact task order is:

```text
R0-CALLABLE-CATALOG-L0B-S0
  -> R0-BARE-STATIC-RECOVERY0-P0
  -> R0-CALLABLE-CATALOG-L0B-CUT0
  -> R0-CALLABLE-CATALOG-L0B-G0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-S0
```

## Authority boundary

### Declaration authority

One non-Clone `VerifiedSameModuleCallableDeclarationCatalogV1` is sealed from
the complete root before declaration-index side effects. Its identity is:

```rust
CanonicalSameModuleCallableKeyV1 {
    namespace,
    owner,
    name,
    arity,
}
```

Its namespaces are exactly:

```text
StaticBoxMethod:
  every FunctionDeclaration in a non-sync, non-record static box

InstanceBoxMethod:
  every non-static method in a non-sync, non-record ordinary box
```

Excluded:

```text
constructors
top-level functions
ordinary-box static methods
record methods
sync-box methods
```

The catalog owns membership, canonical keys, body/header pairing, and the
static candidate inventory. It does not own call-result representation,
runtime behavior, backend admission, argument evaluation, or Call emission.

### Recovery authority

One pure `BareStaticRecoveryDecisionV1` owns exact-one selection. It consumes
only `catalog.static_candidates(name, checked_arity)` and returns one of:

```text
Unique(canonical key)
NoRecovery(NoCandidate)
NoRecovery(Ambiguous)
```

Argument cardinality is converted with a checked `u32` conversion. Overflow
fails before Call effects; it is never truncated.

Both existing unresolved-global call entrypoints are thin production
consumers of this one decision. They derive the MIR symbol in one direction
from the selected canonical key and emit exactly once. If emission fails, the
failure propagates; no second resolver is tried because recovery had already
selected a target.

Caller context is not an admission input. After each entrypoint's existing
earlier owners decline, exact StaticBoxMethod name+arity cardinality alone
decides recovery. This row makes no termination claim for a selected recursive
call.

### Non-authorities

```text
declaration order
lowering order
duplicate registration count
caller box/function identity
first-wins or last-wins
current_static_box
physical MIR symbol parsing
suffix or best-match lookup
runtime type tags
call-result inference
record-helper local snapshots
```

Existing ordinary/direct-module/builtin/extern/qualified-call resolution keeps
its current priority before this narrow recovery step.

## R0-CALLABLE-CATALOG-L0B-S0

```text
production behavior delta: 0
production catalog producers: 0
production catalog consumers: 0
```

Extend the disconnected L0a product to the exact union of the two legacy
stores.

Required API split:

```text
static_candidates(method, arity):
  returns StaticBoxMethod keys only

declaration(namespace, owner, method, arity):
  exact structured declaration lookup
```

Required fixtures:

```text
static and instance exact rows
static/instance same-name same-arity namespace separation
singleton static and instance BoxDeclaration roots
scalar/non-Program root -> sealed empty catalog
params / ParamDecl / return spelling / body pairing
declaration reorder parity
constructor/top-level/ordinary-static/record/sync exclusion
duplicate owner/key and parameter drift typed errors
instance rows returned by static_candidates = 0
physical-symbol reverse parsing = 0
```

This row does not connect the catalog to production and does not implement
bare-static selection.

## R0-BARE-STATIC-RECOVERY0-P0

```text
production behavior delta: 0
production recovery consumers: 0
```

Add the pure canonical recovery decision and disconnected normalized proofs.

Add one tracked HMI-independent proof application:

```text
apps/bare-static-recovery-proof/
  README.md
  provider_first.hako
  caller_first.hako
  ambiguous.hako
  instance_control.hako
  test.sh
```

The checker runs independent cases so an expected unresolved case cannot hide
the rest of the matrix.

Required fixtures:

```text
provider-first same-box call -> Unique
caller-first same-box call -> same Unique key
cross-box calls in both declaration orders
box and method reorder parity
zero-argument call
same name with different arity
wrong arity -> NoCandidate
two static owners with same name/arity -> Ambiguous
instance same-name row does not contaminate static candidates
qualified/direct/builtin paths remain outside the decision
app and script source surfaces
text-merged/using source surface
record-helper structured static and instance lookups
```

Historical baseline behavior remains evidence, not parity:

```text
baseline provider-first:
  unresolved

baseline caller-first:
  resolves

Candidate A normalized result:
  both resolve to the same canonical key
```

## R0-CALLABLE-CATALOG-L0B-CUT0

This is one atomic production cutover and the only semantic row.

```text
production behavior delta:
  provider-first unique bare-static recovery is activated

broader callable grammar delta:
  0

call-result representation delta:
  0
```

One commit performs all of the following:

```text
1. clear the previous-root catalog during module preparation
2. seal a complete candidate catalog before declaration-index mutations
3. install it into the candidate Builder exactly once
4. connect both static-recovery consumers to the pure decision
5. migrate record-helper body reads to structured catalog queries
6. remove static_method_index and all accessors/writers
7. remove lowered_method_asts / LoweredMethodAst and all accessors/writers
8. remove declaration-index/module-lifecycle/exprs duplicate registrations
9. activate provider-first and caller-first production fixtures
10. verify old authority/caller counts are zero
```

Record-helper setter handling preserves its existing diagnostic and work
ordering:

```text
structured (owner, method, arity) allowlist
  -> exact catalog declaration lookup
  -> consumer-local ephemeral body snapshot
  -> mutable Builder work
```

Looking up or cloning the body before the allowlist check is forbidden.

No landed production commit may contain both the old maps and the new catalog
as active authorities.

Root sealing preserves the existing Builder carrier surfaces:

```text
Program:
  complete catalog

standalone BoxDeclaration:
  catalog for that root

other expression root:
  empty catalog
```

No nested Program/Box discovery is added. This root handling matches the
existing declaration-index root surface and is compatibility plumbing, not a
new source grammar claim. Catalog absence is never an old-map fallback.

`prepare_module` may clear the prior-root slot, but every `lower_root` installs
exactly one sealed catalog, including a verified empty catalog for a
non-declaration root. Query-before-install and duplicate install are internal
typed failures. A missing catalog must not be converted to `NoCandidate`;
recovery and body queries return `Result`, and only an installed catalog may
produce an ordinary no-match result.

Atomicity:

```text
catalog seal failure:
  catalog publication = 0
  declaration-index mutation = 0
  body lowering = 0

later lowering failure:
  candidate module is discarded
  current compiler publication = 0

zero or ambiguous recovery:
  Call mutation = 0

successful recovery:
  exactly one canonical target is emitted

duplicate source canonical key:
  typed seal error; never silent deduplication
```

Required production fixtures:

```text
provider-first and caller-first compile to the same target
cross-box order parity
app/script and debug/release normalized parity
zero/unique/ambiguous/wrong-arity matrix
qualified static call regression
same-name instance method regression
record-helper static and instance body lookup parity
failed malformed/ambiguous compile followed by valid compiler reuse
canonical target emitted exactly once
result-contract publication remains zero
```

## R0-CALLABLE-CATALOG-L0B-G0

```text
declaration catalog definitions = 1
production catalog producers = 1
catalog installs per root = 1
query-before-install acceptance = 0
duplicate catalog install acceptance = 0
missing catalog -> NoCandidate conversions = 0

bare-static recovery decision owners = 1
bare-static recovery production consumers = 2
StaticBoxMethod candidate index owners = 1
InstanceBoxMethod rows in static candidates = 0
InstanceBoxMethod result-contract consumers = 0

static_method_index definitions/reads/writes = 0
register_static_method/get_static_method_candidates = 0
lowered_method_asts definitions/reads/writes = 0
LoweredMethodAst definitions = 0
register_lowered_method_ast/lowered_method_ast = 0
duplicate post-lowering registrations = 0

lowering-order/multiplicity authority = 0
physical-symbol reverse parsing = 0
second persistent AST/body store = 0
callee-first/re-lowering/retry/fallback = 0
source rewrite/automatic qualification = 0

call-result representation production consumers = 0
GenericLoop behavior delta = 0
source/check files >= 800 lines = 0
```

Only after G0 is green may `R0-CALLABLE-RESULT-I64-CATALOG0-S0` resume.

## Validation entrypoints

```bash
cargo test -q --lib mir::builder::callable_declaration_catalog::tests
cargo test -q --lib mir::builder::record_helper_args::tests
bash apps/bare-static-recovery-proof/test.sh
python3 tools/checks/lib/same_module_call_result_representation_inventory.py . --check-reference
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
bash tools/checks/dev_gate.sh quick
```

The historical same-module inventory is updated when the old stores disappear;
it must require catalog seal/install before declaration indexing and old-store
counts zero. The phase-296x pinned helper guard is not an acceptance authority
while its unrelated required historical document remains absent.

## WIP stash law

The pre-decision implementation is evidence only:

```text
immutable stash commit:
  994dbc762565e6b0c27878f370c190ba8640eaa4

parent:
  31bab44f650cf9d6a92bce902dd871637b60dc63

label:
  wip/callable-catalog-l0b (bare-static recovery behavior delta)
```

Forbidden:

```text
stash apply/pop/restore
wholesale cherry-pick
stashed current docs recovery
stashed source as implementation authority
mutable stash@{n} as a durable pointer
```

Permitted:

```text
read-only git show/diff by immutable hash
implementation-feasibility evidence
historical test-result comparison
```

Implementation starts from clean HEAD. Each selected structure is rewritten or
selectively reconstructed and re-proven by its current row fixtures.

The evidence stash may be deleted only after CUT0 and G0 land, current pointers
are green, old stores/callers are zero, provider/caller order fixtures plus
`cargo check` and quick are green, and the worktree is clean. Stash deletion is
a separate explicit housekeeping action, not part of G0.

## Implementation may claim

```text
one complete immutable same-module callable declaration authority
static and instance declaration rows remain namespace-disjoint
bare-static recovery uses one canonical exact-one decision
unique recovery is declaration- and lowering-order independent
provider-first and caller-first unique calls resolve identically
zero and ambiguous candidate sets do not recover
record-helper body inspection uses structured identity
old partial declaration/body stores are retired atomically
```

## Implementation must not claim

```text
general callable resolution
general overload resolution
method-call or receiver widening
call-result representation or inference
runtime or backend widening
source compatibility through automatic qualification
constructor/top-level/record/sync callable support
result-contract support for InstanceBoxMethod
fallback, retry, callee-first lowering, or re-lowering
```

## Stop conditions

Stop before CUT0 if any of the following is required:

1. Record-helper lookup cannot receive structured identity without symbol parsing.
2. An InstanceBoxMethod row enters static recovery or result inference.
3. An old map, compatibility fallback, or second persistent catalog must remain.
4. Ambiguity requires first/last/declaration/box preference or a new overload law.
5. Existing argument evaluation order must move.
6. A cross-namespace physical projection collision requires new identity semantics.
7. Catalog seal/install cannot precede declaration-index side effects atomically.
8. Existing direct-module/builtin/extern/qualified resolution priority must change.
9. Result typing, runtime/backend behavior, source migration, or HMI-specific policy
   is required by the cutover.
10. A source/check file reaches 800 lines.

## Final decision lock

> Candidate A is selected. One complete immutable declaration catalog owns the
> exact StaticBoxMethod and InstanceBoxMethod declaration union, while one pure
> recovery decision sees only StaticBoxMethod candidates and selects a target
> only when source name plus checked arity has exactly one canonical key.
> Provider-first and caller-first order therefore become intentionally
> equivalent. `R0-CALLABLE-CATALOG-L0B-S0` first extends the disconnected
> namespace without production effects; `R0-BARE-STATIC-RECOVERY0-P0` seals the
> zero/unique/ambiguous policy; `R0-CALLABLE-CATALOG-L0B-CUT0` atomically
> installs the catalog, connects both recovery consumers, migrates structured
> body lookup, activates the narrow provider-first case, and deletes both old
> partial authorities; G0 then guards the boundary before result-contract work
> resumes. The pre-decision stash is read-only evidence and is never restored.
