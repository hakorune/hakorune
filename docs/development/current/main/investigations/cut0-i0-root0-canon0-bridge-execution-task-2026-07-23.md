# CUT0-I0 ROOT0-CANON0 CANON-BRIDGE0 実行タスク

Status: **Active — CB-prime selected; CANON-FIXTURE0 is the only executable row**

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

### CANON-BRIDGE0-OWNER0 — closed

Add one private `MirCompiler` terminal that consumes the source-bound package
by value and opens the actual `ModuleBuilderInvocationSessionV1`, shell, and
canonical collector carrying the same token. The package is route-typed and
has no public split/getter/Clone/Arc path.

The physical owner opens before lowering. The old standalone
`CanonicalModuleLoweringSessionV1` remains only as an explicitly quarantined
legacy ingress until atomic CUT0; the new bridge must not open it.

Failure law: package/physical-open failure retains the unpublished package,
leaves the live Builder and collector unchanged, and permits no retry.

Closeout evidence: the compiler-owned `begin_canonical_invocation` terminal
now opens one shared-brand session, function-empty shell, and branded
collector before invoking the existing draft-only LOWER0 seam. The focused
`canonical_source_binding_owner0` fixture and lane guard are green. OWNER0
intentionally does not merge the shell module with the candidate Builder or
collect drafts; those are COLLECT0 responsibilities.

### CANON-BRIDGE0-COLLECT0-PHYSICAL — closed

Make canonical admission typed and source-driven:

```text
single  = exact owner header -> CanonicalResolvedOwner key/symbol/arity
batch   = exact callable catalog -> CanonicalCallable rows
```

The caller cannot supply `FunctionDraftKeyV1`, policy, symbol, arity, or raw
entries. `Main` and `SyntheticConditionFn` are therefore not constructible on
the canonical path. A physical symbol spelled `condition_fn/N` remains valid
when its canonical key/header/catalog authority is exact.

The compiler-owned physical owner now derives both admission forms from source
authority and issues the exact receipt product from the same branded collector.
The collector and receipt are retained together in the collected physical
owner; the next subrow will move that product into route-specific completion.

Closeout evidence (2026-07-23):

```text
single: exact VerifiedResolvedOwnerHeaderV1 -> CanonicalResolvedOwner key,
       header symbol, and header arity
batch: exact VerifiedResolvedCallableModuleV1 catalog -> CanonicalCallable
       rows with whole-batch preflight before collector mutation
receipt: collector-issued brand matches the compiler token brand
canonical_source_binding_collect0 = 2 passed (single + acyclic batch)
RUSTFLAGS='-Awarnings' cargo test -q source_bound_package --lib = 3 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib = passed
cut0_i0_root0_canon0_bridge_guard.py = passed
current_state_pointer_guard.sh = passed
git diff --check = passed
all touched source/check files < 800 lines
```

### CANON-BRIDGE0-COLLECT0-COMPLETION — closed

Move the collected physical product into a route-specific completion owner
without loosening the source authority:

```text
CollectedCanonicalPhysicalInvocationV1
-> canonical single / callable-batch completion product
-> exact receipt retained by value through completion
-> recursive/acyclic witness co-sealed
```

The compiler-side completion terminal consumes the collected owner by value.
No loose receipt argument, receipt rebranding, collector re-acquisition, or
legacy Main-state reuse is allowed. The new product keeps the original token,
session, source continuation, opaque physical shell/collector/receipt product,
and callable capability witness together.

Closeout evidence (2026-07-23):

```text
new completion module = src/mir/compiler/canonical_physical_completion.rs
completion consumer = CollectedCanonicalPhysicalInvocationV1::complete (1)
route products = Single / Callable
single receipt + token/session/physical brand = retained by value
acyclic capability absence + receipt = same brand/family
recursive install receipt + receipt = same brand/family
canonical_physical_completion_p0 = 3 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib = passed
cut0_i0_root0_canon0_bridge_guard.py = passed
current_state_pointer_guard.sh = passed
git diff --check = passed
all touched source/check files < 800 lines
```

The old Builder-only `canonical_root_completion.rs` remains disconnected and
is not converted or imported by the new compiler completion path.

### CANON-FIXTURE0 — active

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
CANON-FIXTURE0 is now executable, but its aggregate proof is not yet closed
production CUT0 activation is not part of this task
```

## Required evidence for the active row

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q source_bound_package --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_physical_completion_p0 --lib
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_identity --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_bridge_guard.py
```

The active fixture row closes only when the four route proofs use the real
compiler-owned bridge in one aggregate fixture. DRAIN0, finalization, and
external commit remain forbidden until that fixture and its guard are green.
