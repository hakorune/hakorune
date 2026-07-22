# CUT0-I0 ROOT0-CANON0 CANON-BRIDGE0 実行タスク

Status: **Active — CB-prime selected; OWNER0 is the only executable row**

Related:

- `cut0-i0-root0-canon0-fixture0-bridge-design-question-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-recursive0-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Connect the closed SOURCE-BIND0/LOWER0 vocabulary to the closed ROOT0 physical
owner without creating a second identity, second Builder session, or test-only
bridge. The final chain must be one non-Clone invocation owner:

```text
exact plan
-> compiler-issued token/package
-> one active session + shell + canonical collector
-> same-session plan-consuming lowering
-> collector-issued receipt product
-> route-specific complete invocation
-> source-derived DRAIN0
```

The first implementation row is identity vocabulary only. No canonical
production ingress, capture, drain, finalizer, external commit, fallback, or
retry is authorized until the later rows close.

## Decision lock

Candidate CB-prime is selected:

```text
bridge owner       = MirCompiler private one-shot terminal
identity           = one shared process-domain + local-ordinal kernel
lowering handoff   = physical owner opens before plan consumption
canonical admission = typed facade; Main/SyntheticConditionFn unrepresentable
aggregate fixture  = after IDKERNEL/OWNER0/COLLECT0 only
```

Compiler remains the sole production identity issuer. A compiler token is not
converted into a Builder token; the shared identity value is carried directly
through the phase transition. Ordinal copying, post-hoc rebranding,
`TestInvocationPreflightFactoryV1`, `Arc`, `Clone` authority copies, and loose
token/plan/header/catalog/receipt arguments are forbidden on the new path.

## Row order

### CANON-BRIDGE0-IDKERNEL — closed

Move only the identity value vocabulary to one neutral `crate::mir` module:

```text
ModuleInvocationBrandV1
  = process-scoped compiler domain + compiler-local invocation ordinal

ModuleInvocationIdV1
  = non-Clone owner of the brand

ModuleInvocationTokenV1
  = non-Clone route-bearing owner
```

Keep issuance in `MirCompiler::InvocationIdentityIssuerV1`. Replace the two
parallel compiler/Builder value types without changing semantic route policy
or activating production callers.

Acceptance:

```text
shared brand/token definitions = 1 each
production issuer = MirCompiler only
compiler-token -> Builder-token conversion = 0
ordinal/domain copy adapter = 0
canonical TestInvocationPreflightFactory caller = 0
foreign domain/local ordinal pair rejects before mutation
same compiler ordinals are monotonic and never reused
all touched source/check files < 800 lines
```

Closeout evidence (2026-07-23):

```text
shared module_invocation_identity.rs owns the single family/brand/id/token vocabulary
MirCompiler::InvocationIdentityIssuerV1 is the only production from_issued caller
compiler and Builder duplicate value definitions/conversion paths = 0
module_invocation_identity_idkernel_p0 = 3 passed
source_bound_package = 6 passed
module_invocation_identity = 7 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib = passed
cut0_i0_root0_canon0_bridge_guard.py = passed
```

### CANON-BRIDGE0-OWNER0 — active

Add one private `MirCompiler` terminal that consumes the source-bound package
by value and opens the actual `ModuleBuilderInvocationSessionV1`, shell, and
canonical collector carrying the same token. The package is route-typed and
has no public split/getter/Clone/Arc path.

The physical owner opens before lowering. The old standalone
`CanonicalModuleLoweringSessionV1` remains only as an explicitly quarantined
legacy ingress until atomic CUT0; the new bridge must not open it.

Failure law: package/physical-open failure retains the unpublished package,
leaves the live Builder and collector unchanged, and permits no retry.

### CANON-BRIDGE0-COLLECT0 — later

Make canonical admission typed and source-driven:

```text
single  = exact owner header -> CanonicalResolvedOwner key/symbol/arity
batch   = exact callable catalog -> CanonicalCallable rows
```

The caller cannot supply `FunctionDraftKeyV1`, policy, symbol, arity, or raw
entries. `Main` and `SyntheticConditionFn` are therefore not constructible on
the canonical path. A physical symbol spelled `condition_fn/N` remains valid
when its canonical key/header/catalog authority is exact.

The collector and exact receipt remain one by-value product through completion;
recursive and acyclic capability witnesses retain the same token brand/family.

### CANON-FIXTURE0 — later

Only after the bridge rows close, add the real four-route aggregate fixture:

```text
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

The fixture uses the compiler-owned bridge, never a test token or post-hoc
brand. It covers success, foreign pairing, late batch collision,
condition_fn/N canonical spelling, synthetic-key rejection, and recursive vs
acyclic witness parity. DRAIN0 consumes the complete product only afterward.

## Shared guard policy

Use one reusable CANON-BRIDGE0 lane guard rather than one guard per row. It
must census definitions, production callsites, forbidden conversion/rebrand
patterns, focused fixture registration, and every touched source/check file.
The old broad CANON0 text-presence guard is historical evidence only and must
not be used as proof of the new bridge.

## Explicit non-claims

```text
the existing SOURCE-BIND0 and completion fixtures are not one aggregate proof
green disconnected boxes do not prove a cross-layer owner chain
builder test tokens are not production identity
CanonicalModuleLoweringSessionV1 is not the new physical owner
CANON-FIXTURE0 is not executable before OWNER0/COLLECT0
production CUT0 activation is not part of this task
```

## Required evidence for the active row

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q source_bound_package --lib
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_identity --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_bridge_guard.py
```

The active row closes only when the shared identity kernel is real and the
Builder/compiler duplicate value definitions and conversion paths are gone.
