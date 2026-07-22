# CUT0-I0 ROOT0-DRAIN0 実行タスク

Status: **Active — ROOT0-DRAIN0-PHYSICAL0 is the next executable row**

Related:

- `cut0-i0-root0-drain0-design-question-2026-07-23.md`
- `cut0-i0-root0-canon0-fixture0-execution-task-2026-07-23.md`
- `cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md`
- `cut0-i0-root0-design-stop-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Implement the first canonical one-shot drain without connecting production
ingress, finalization, external commit, fallback, retry, or the Raw route.
The only authorized chain is:

```text
CanonicalPhysicalCompleteInvocationV1
-> source-derived exact manifest
-> mutation-free physical preflight
-> PreparedCanonicalDrainV1
-> infallible one-shot drain
-> CanonicalDrainedInvocationV1::{Single, Callable}
```

## Decision lock: Candidate D-prime-r1

The consultation is closed with these decisions:

```text
Q1 terminal owner  = compiler completion-owned prepare_drain(self)
Q2 inventory       = neutral policy SSOT + source-derived exact manifest
Q3 physical unpack = one Builder-internal prepared physical terminal
Q4 output          = new canonical Single/Callable drained products
Q5 failure         = fallible prepare, infallible drain, no retry capability
Q6 lifetime        = retained header/catalog only; no re-observation
```

The old `ModuleLoweringInvocationDrainOwnerV1`, caller-authored inventory,
`ConditionFnPolicyV1::Optional`, `DrainedModuleCandidateV1`, and
`canonical_root_completion.rs` are not canonical authorities and must not be
connected or expanded.

## ROOT0-DRAIN0-POLICY0 — neutral policy SSOT

Add a neutral `crate::mir` policy product (keep it in a new file below the
line limit, for example `src/mir/module_invocation_policy.rs`):

```text
ModuleInvocationPolicyV1
  family
  inventory authority
  root policy
  condition policy
  fallback policy
```

Canonical policy is fixed as:

```text
A+ / BindingSsaTrivial:
  ExactCanonicalOwner, synthetic root forbidden, condition forbidden

BindingSsaAcyclic / BindingSsaRecursive:
  ExactCallableCatalog, synthetic root forbidden, condition forbidden
```

Replace compiler-local `CanonicalRoutePolicyV1` with this neutral policy
without adding concrete function rows. Keep `RouteOwnedInvocationInventoryV2`
as a thin policy/evidence wrapper; do not duplicate policy decisions.

Acceptance:

```text
policy authority = 1
compiler-local duplicate route policy = 0
caller policy/fallback/condition constructor = 0
Raw policy behavior unchanged
production drain consumer = 0
```

## POLICY0 closeout — 2026-07-23

`ROOT0-DRAIN0-POLICY0` is closed. A neutral `crate::mir` policy SSOT now
derives family, inventory authority, root, condition, and fallback policy for
all five families. Canonical source continuations use it instead of the old
compiler-local route policy. The Builder route wrapper delegates to the same
policy product and retains only route-matrix/source-symbol evidence; it no
longer carries a second policy decision table.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib                         green
RUSTFLAGS='-Awarnings' cargo test -q drain_policy_p0 --lib         2 passed
RUSTFLAGS='-Awarnings' cargo test -q route-owned policy fixtures    green
git diff --check                                                   green
bash tools/checks/current_state_pointer_guard.sh                   green
```

The source, manifest, and focused-test files remain below 800 lines. No
concrete function rows, physical drain, canonical production consumer, Raw
route change, finalizer, external commit, retry, or fallback was added.

The next executable row is `ROOT0-DRAIN0-MANIFEST0`.

## ROOT0-DRAIN0-MANIFEST0 — exact source projection

Create a separate canonical manifest product. It is not a second catalog and
never accepts caller-authored rows:

```text
CanonicalDrainIdentityV1
  ResolvedOwner(FunctionOwnerIdV1)
  Callable(CanonicalCallableKeyV1)

CanonicalDrainRowV1
  semantic identity
  physical symbol
  arity
  sealed canonical reject-duplicate / inserted disposition

CanonicalDrainManifestV1
  Single { brand, family, policy, row }
  Callable { brand, family, policy, sorted rows }
```

Projection rules:

```text
single  = retained exact owner header
callable = retained exact verified catalog
callable row order = CanonicalCallableKeyV1 order
expected inventory = source manifest, never receipt/collector/module map
```

The manifest is projected exactly once during drain preparation and becomes a
non-Clone drained inventory witness. No source re-resolution or catalog
reacquisition is allowed.

## MANIFEST0 closeout — 2026-07-23

`ROOT0-DRAIN0-MANIFEST0` is closed. The canonical drain manifest now projects
expected rows only from the retained source continuation: one owned header for
single routes and the verified callable catalog for callable routes. Callable
rows follow the catalog's canonical-key order and carry semantic identity,
physical symbol, arity, and a type-sealed canonical inserted disposition.

The manifest owns no collector, receipt, `MirModule`, or Builder state. It is
non-`Clone`; its constructors are restricted to the compiler projection path,
and the caller cannot provide keys, symbols, arities, or publication policy.
The manifest family is derived from the neutral policy SSOT rather than stored
as a second route authority. A package-level projector checks the resulting
family against the package token before returning the manifest.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib                              green
RUSTFLAGS='-Awarnings' cargo test -q canonical_drain_manifest_p0 --lib  2 passed
RUSTFLAGS='-Awarnings' cargo test -q drain_policy_p0 --lib              2 passed
git diff --check                                                        green
bash tools/checks/current_state_pointer_guard.sh                        green
```

No physical collector/receipt validation, shell mutation, completion-owned
drain, old drain connection, production consumer, finalizer, external commit,
retry, fallback, or Raw route change was added. All touched source and check
files remain below 800 lines.

The next executable row is `ROOT0-DRAIN0-PHYSICAL0`.

## ROOT0-DRAIN0-PHYSICAL0 — prepared physical terminal

Add one Builder-internal terminal for each existing collected physical product,
in a new small module rather than expanding the near-limit owner files:

```text
CollectedCanonicalSinglePhysicalV1::prepare_drain(manifest)
CollectedCanonicalCallablePhysicalV1::prepare_drain(manifest)
```

The terminal must validate before any shell mutation:

```text
manifest brand == shell/collector/receipt brand
shell function map is empty
source row count == collector row count == receipt row count
identity, symbol, arity, policy, replacement exact
missing/surplus/duplicate row = 0
```

Success returns prepared physical products. Their only consuming terminal is
an infallible `drain(self)` which moves drafts into the empty shell in manifest
order. The result retains an opaque drained module, the original collector
receipt, and a non-Clone inventory witness. Bare `MirModule` or loose receipt
outputs are forbidden.

## ROOT0-DRAIN0-I0 — completion-owned one-shot drain

Add the only canonical entry:

```rust
CanonicalPhysicalCompleteInvocationV1::prepare_drain(self)
```

It must:

```text
consume complete by value
project the exact manifest from retained continuation
call the prepared physical terminal
return PreparedCanonicalDrainV1 or rejected complete owner
```

Then add:

```rust
PreparedCanonicalDrainV1::drain(self)
  -> CanonicalDrainedInvocationV1::{Single, Callable}
```

The drained variants retain the original non-Clone token, Builder session,
source continuation, opaque drained physical product, and callable
acyclic/recursive capability witness. The collector is consumed and does not
survive the drain, avoiding a second physical function owner.

## ROOT0-DRAIN0-P0 — focused proof matrix

Required disconnected fixtures:

```text
success: A+, trivial, acyclic, recursive
condition_fn/N canonical spelling accepted
synthetic Main/SyntheticConditionFn constructor absent
deterministic callable declaration reorder parity
shell already published -> rejected before mutation
missing/surplus/duplicate row -> rejected before mutation
receipt/collector mismatch -> rejected before mutation
foreign brand -> rejected before mutation
prepare failure leaves live Builder unchanged
second drain/retry/fallback terminal absent
recursive witness retained; acyclic absence witness retained
```

Raw remains outside this canonical row and keeps its closed RAW0 owner chain.

## ROOT0-DRAIN0-G0 — guard and production boundary

Create a manifest-backed DRAIN0 guard. It must measure:

```text
CanonicalPhysicalCompleteInvocationV1::prepare_drain consumer = 1
PreparedCanonicalDrainV1::drain terminal = 1
old drain constructors canonical callers = 0
InvocationDrainExpectation caller inventory = 0
require_main / ConditionFnPolicyV1::Optional = 0
DrainedModuleCandidateV1 canonical callers = 0
current_module/module-map expected-inventory reads = 0
receipt clone/rebrand/reacquisition = 0
production drain/finalizer/external commit = 0
all manifest files < 800 lines
```

The guard must distinguish shell-empty invariant reads from forbidden
expected-inventory reconstruction. Do not edit `canonical_root_completion.rs`;
retain it only as disconnected legacy evidence until a later retirement row.

## Failure and one-shot law

Every preparation mismatch returns a rejected complete owner before shell
mutation. The rejected owner exposes no retry, replacement manifest, or
re-entry-to-complete terminal. `PreparedCanonicalDrainV1` and drained products
are consuming and non-Clone. Failure never reaches drain, finalizer, external
commit, fallback, or retry.

## Stop line

This card closes only the disconnected canonical drain product and its proof.
Production canonical ingress, finalization, external commit, Raw convergence,
and atomic CUT0 activation remain separate later rows.

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q drain0_p0 --lib
python3 tools/checks/lib/cut0_i0_root0_drain0_guard.py
```
