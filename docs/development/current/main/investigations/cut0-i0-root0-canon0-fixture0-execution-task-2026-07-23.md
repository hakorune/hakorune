# CUT0-I0 ROOT0-CANON0 CANON-FIXTURE0 実行タスク

Status: **Active — CANON-FIXTURE0-S0 is the next executable row**

Related:

- `cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md`
- `cut0-i0-root0-canon0-fixture0-bridge-design-question-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-recursive0-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Close one real compiler-owned aggregate proof for the four canonical routes.
The positive path must use the same internal terminals that future canonical
ingress will use:

```text
exact preflight plan
-> MirCompiler::bind_canonical_source
-> MirCompiler::begin_canonical_invocation
-> same-session lowering
-> same physical typed collector
-> collector-issued receipt product
-> route-specific completion
```

The fixture is disconnected. It must not activate canonical production
capture, drain, finalization, external commit, fallback, or retry.

## Decision lock: CB-prime fixture scope

The worker audit selects the following split:

```text
CANON-FIXTURE0-S0
  real four-route aggregate success chain

CANON-FIXTURE0-P0
  provenance and typed-admission static proof

CANON-FIXTURE0-C0
  reuse the existing same-collector atomic late-collision proof

CANON-FIXTURE0-G0
  dedicated manifest-backed fixture guard and caller census
```

The aggregate fixture must not fake negative cases with a test token,
post-hoc brand wrapper, or a second package constructor. The exact bridge API
already makes several invalid combinations unrepresentable; those are proved
by static census and focused preflight tests rather than by constructing an
illegal positive-path object.

## CANON-FIXTURE0-S0 — four-route aggregate

Add one focused compiler test module, for example
`src/mir/compiler/canonical_bridge_fixture0_p0.rs`, and register it from the
compiler test module. The module must contain one aggregate matrix over these
exact variants:

```text
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

Each row must run the real chain by value:

```text
plan fixture
-> MirCompiler::bind_canonical_source(plan)
-> MirCompiler::begin_canonical_invocation(package)
-> lower()
-> collect()
-> complete()
```

The fixture must assert, for every row:

```text
compiler issuer is the only producer
token/session/shell/collector/receipt share one brand
the exact route variant is preserved (A+ != trivial != acyclic != recursive)
the completion owns the original non-Clone token and receipt product
recursive has one branded install receipt
acyclic has one branded absence witness
no DRAIN0/finalizer/external commit consumer is reached
```

Use source builders already accepted by the preflight layer. The A+ row must
assert `ExactCanonicalPreflightPlanV1::APlus(_)` explicitly; the trivial row
must assert `BindingSsaTrivial(_)` explicitly. Do not use the old generic
single fixture as evidence for both.

## CANON-FIXTURE0-P0 — invalid combinations are structural non-claims

The bridge must keep these constructors absent:

```text
prepare(token, plan)
caller-supplied family/header/catalog
compiler-token -> Builder-token conversion
ordinal/domain copy adapter
post-hoc receipt rebranding
canonical collector(key, policy, symbol, arity)
```

The guard must census those surfaces and focused tests must retain the
strongest available preflight evidence (`require_same_plan` / family-bound
source sealing). There is no runtime “foreign pairing” fixture on the new
positive path because the API has no legal way to construct the pair.

Canonical synthetic identities remain absent from the typed facade:

```text
FunctionDraftKeyV1::Main               = unrepresentable
FunctionDraftKeyV1::SyntheticConditionFn = unrepresentable
```

Add one positive fixture whose canonical physical symbol is spelled
`condition_fn/N`. It must be accepted because its key/header/catalog authority
is canonical; spelling alone is not a synthetic identity.

## CANON-FIXTURE0-C0 — collision evidence boundary

Do not add a bridge-only post-hoc fault injector merely to manufacture a late
collision. The source-driven callable catalog is unique and the new typed
collector starts empty, so such an injector would be a second authority.

The late-collision contract remains covered by the existing disconnected
callable-batch transaction fixture (its test-only owner is intentionally not
claimed as a compiler-bridge owner chain):

```text
whole-batch preflight
-> late collision
-> collector delta = 0
-> no receipt / completion product
```

The new fixture guard must require that focused atomic-batch test and state
explicitly that it is a disconnected collector proof, not a canonical
production-consumer proof. If a future requirement demands a bridge-native
fault seam, stop for a new design consultation before adding one.

## CANON-FIXTURE0-G0 — dedicated guard

Create `tools/checks/lib/cut0_i0_root0_canon0_fixture0_guard.py`. It must use a
manifest-backed census and check:

```text
aggregate fixture file and module registration = 1
all four route variants asserted = 1 each
condition_fn spelling fixture = 1
recursive/acyclic witness assertions = 1 each
existing atomic late-collision fixture registration = 1
compiler token producer = 1
production bind/begin/lower/collect/complete callers = 0
test-only factory use on the aggregate path = 0
token conversion / post-hoc rebrand = 0
canonical loose-key API = 0
all manifest files < 800 lines
```

Keep the existing bridge guard as the shared identity/owner census. This
fixture guard owns aggregate registration, route coverage, non-claim checks,
and production-caller zero. It must not print fixed claims without measuring
the repository.

## Acceptance

CANON-FIXTURE0 closes only when all of the following are green:

```text
one aggregate four-route real-bridge test
explicit A+ and trivial route discrimination
acyclic/recursive branded witness parity
canonical condition_fn/N acceptance
static proof that foreign pairing and synthetic keys are unrepresentable
existing same-collector late-collision atomic proof
dedicated fixture guard with real production census
production canonical ingress/capture/drain/finalizer/external commit = 0
all touched source/check files < 800 lines
```

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_bridge_fixture0_p0 --lib
RUSTFLAGS='-Awarnings' cargo test -q source_bound_package_p0 --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_physical_completion_p0 --lib
RUSTFLAGS='-Awarnings' cargo test -q callable_batch_collection_p0 --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_bridge_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_fixture0_guard.py
```

## Stop line and explicit non-claims

```text
aggregate fixture != production canonical ingress
completion != DRAIN0 consumption
disconnected late-collision fixture != bridge-native fault injection
static impossibility != runtime foreign-pairing exercise
typed-admission absence != Raw/legacy collector removal
```

Do not begin DRAIN0, finalization, external commit, or atomic CUT0 activation
from this card. Those remain separate rows after CANON-FIXTURE0-G0.
