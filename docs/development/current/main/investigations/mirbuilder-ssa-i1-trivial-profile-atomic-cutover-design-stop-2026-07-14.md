---
Status: Decision accepted — A′ whole-unit profile routing
Date: 2026-07-14
Blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-TRIVIAL-PROFILE-ATOMIC-CUTOVER-DESIGN-STOP-001
Parent taskboard: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Decision: A′ — seal an exact whole-owner trivial profile before Builder effects
Resolved: 2026-07-15
---

# SSA-I1 Trivial Profile / Atomic Cutover Design Stop

## Executive question

SSA-I1 must make one function-owned `BindingSsaBuilderV1` the only value/PHI
authority for every source unit it accepts. The closed ownership inventory says
the first cutover is trivial-only, but the current canonical A+ route already
accepts untyped parameters, Outbox, String, Void/Null, and implicit Void
completion. Those surfaces do not yet have an executable per-function
representation witness.

Choose whether the first production cutover is:

```text
A′. whole-owner profile routing
    Seal an exact per-function trivial profile before Builder effects.
    Route admitted source units atomically to Binding SSA and keep other
    currently accepted source units on the old canonical A+ owner temporarily.

B. close every current representation first
   Add parameter/Outbox/text/Void/Null representation and ABI witnesses before
   starting SSA-I1, then cut the entire current canonical family atomically.

C. narrow current canonical acceptance
   Reject the unsupported current surfaces and cut only the trivial subset.
   This is a behavior/compatibility change and needs an explicit language or
   compatibility Decision.
```

Accepted decision: **A′**. Route selection happens once for the whole source
unit before Builder effects, never retries after failure, and the old A+ owner
has an explicit bounded retirement sequence. B remains a larger future
compatibility closure; C is rejected because it would narrow accepted behavior.

## Returned decision

```text
executable witness owner:
  src/mir/resolved_value_profile
  VerifiedTrivialCanonicalOwnerV1

route selection:
  whole source unit only
  before prepare_module / Builder effects
  TrivialBindingSsa or CurrentCanonicalAPlus
  no function/body/site-level mixing
  no retry after a selected-route failure

SSA-I1-T ownership boundary:
  production Ownership SSA witness/install/verifier = 0
  CopyOwned / DestroyOwned production activation = 0
  first production Ownership SSA activation remains SSA-I1-O1

terminal law:
  return trivial-value = exact value disposition
  return; = explicit no-value disposition
  implicit fallthrough = implicit no-value disposition

typed profile rejects:
  unsealed parameter/receiver ABI
  Outbox as a value producer
  String / BorrowedText
  Void or Null as a value producer
  mixed-representation If merge profile

temporary A+ law:
  allowed only as the once-selected whole-unit owner for a non-admitted unit
  never synchronized with Binding SSA
  never used as a fallback or retry
  caller-zero retirement is required at SSA-I1-FULL
```

`Void` remains usable here only as a completion disposition. This decision
does not assign it a value representation, and it does not turn Outbox's
Void seed into an admitted value.

## Evidence

### Current production acceptance

`src/mir/compiler/capability.rs` currently accepts:

```text
untyped parameters
Local with an optional initializer slot
empty-seeded Outbox
all AST Literal variants
variable / binary / assignment / BlockExpr
fallthrough statement If, including nested If
final `return value`, final `return`, and implicit Void fallthrough
```

The production fixture in
`src/mir/builder/resolved_lowering/tests.rs` contains one untyped parameter and
one Outbox and is green through the public resolved route.

Focused evidence run on 2026-07-14:

```text
cargo test -q closed_family_uses_resolver_bindings_without_legacy_allocation
  -> green

cargo test -q preflight_owns_nested_if_flow_with_blockexpr_condition_and_optional_else
  -> green
```

### Closed P0 profile

`tools/checks/fixtures/canonical_ownership_production_profile_v1.json` records:

| Origin | Current status | First profile |
| --- | --- | --- |
| Integer / Bool / Float literal | accepted exact | trivial exact |
| Local / binary / variable / assignment / PHI / BlockExpr / Return | accepted derived | derived trivial-only |
| untyped parameter | accepted | typed preflight reject |
| Outbox | accepted Void | typed preflight reject |
| String literal | accepted BorrowedText | typed preflight reject |
| Void / Null literal | accepted | typed preflight reject |

P0 is a guarded inventory. It does not publish a
`VerifiedTrivialCanonicalOwnerV1` (or equivalent) proving every located value,
binding definition, representation-only If merge profile, and terminal
disposition in one exact source owner. PHI placement remains Binding SSA.

### Ownership activation ambiguity

The taskboard says both:

```text
SSA-I1:
  trivial-only when exact BoxRef producer count is zero
  CopyOwned / DestroyOwned production activation = 0

SSA-I1-O1:
  first exact BoxRef Ownership SSA production activation
```

but the SSA-I1 acceptance list also requires ownership-managed definitions,
assignment `DestroyOwned`, scope ownership cleanup, and Ownership SSA
verification. The boundary must be one of:

```text
recommended for trivial SSA-I1:
  per-function trivial witness = installed and verified
  Owned/Borrowed values = 0
  CopyOwned/DestroyOwned = 0
  production Ownership SSA witness/verifier = 0

SSA-I1-O1 and later:
  exact ownership witness installed
  Ownership SSA verifier required before publication
```

If an all-`None` Ownership SSA witness is instead required at SSA-I1, that must
be selected explicitly and the current production-caller-zero guard changed.

## Authority boundary that must remain fixed

```text
pre-Builder:
  exact source coverage
  ScopeId / RegionId topology
  control ports and exact targets
  cleanup obligations
  selected executable representation profile

function-owned Binding SSA:
  sole BindingRef -> reaching ValueId authority
  PHI placement and forwarding from CFG predecessors

family CFG boxes:
  If block layout and edge emission

Ownership SSA:
  token consumption only after an exact ownable representation/ABI witness
```

P0 inventory, `MirType`, `StorageClass`, runtime `VMValue`, Span, pointer,
name, or post-Lower value inspection must not be promoted into the executable
profile proof.

## Option A′ details

Add a pre-Builder sealed product, name provisional:

```rust
pub struct VerifiedTrivialCanonicalOwnerV1 {
    owner: FunctionOwnerIdV1,
    values: Box<[VerifiedLocatedTrivialValueV1]>,
    terminals: VerifiedTrivialTerminalProfileV1,
    coverage: VerifiedTrivialProfileCoverageV1,
}
```

It is co-sealed with the existing function input/control/completion products.
It proves:

```text
every admitted literal has exact InlineI64/InlineBool/InlineF64 representation
every local/read/assignment/binary/BlockExpr result preserves one exact profile
every If merge profile has homogeneous exact trivial predecessor profiles
PHI existence, placement, and result ValueId remain Binding SSA authority
`return;` and implicit fallthrough are explicit None/no-value dispositions
every located value and terminal is accounted exactly once
parameters, Outbox, String, Void/Null values, and mixed merge profiles are absent
```

Route selection law:

```text
whole source unit only
before prepare_module / Builder effects
TrivialBindingSsa profile or CurrentCanonicalAPlus profile
no function/body/site-level mixing
no canonical failure retry
unsupported by both profiles -> typed error
```

Required task split after accepting A′:

```text
SSA-I0-PROFILE:
  disconnected exact trivial owner profile + coverage + rejection fixtures

SSA-I1-T:
  atomic trivial-profile Binding SSA cutover
  old A+ remains only for non-admitted whole source units

SSA-I1-COMPAT rows:
  seal parameter ABI, Outbox/Void disposition, BorrowedText, and Null policy
  one representation family per row

SSA-I1-FULL:
  all current canonical source units use Binding SSA
  old A+ production caller zero

SSA-R1:
  physical old effect/join/value-owner retirement
```

The `BindingRef -> ValueId` authority count is therefore profile-scoped during
the transition and becomes globally one only at `SSA-I1-FULL`.

## Option B details

Before any Binding SSA production activation, add executable witnesses for:

```text
untyped parameter caller/callee ABI or replace it with a typed signature
Outbox seed/result disposition
BorrowedText copy/escape/destruction law
Void and Null representation/disposition
mixed branch/PHI rejection or forwarding
explicit empty Return and implicit fallthrough result law
```

Then retain one `SSA-I1` whole-family atomic commit. This avoids transitional
dual canonical production owners, but it couples Binding SSA activation to
multiple representation and ABI decisions.

## Guard migration already identified

When implementation is authorized:

```text
canonical_ssa_seam_inventory_v1.json:
  freeze its current SHA-256 as historical evidence
  do not rewrite the 92-row fixture as a live caller ledger

resolved_binding_ssa_builder.py / mir_adapter.py / cfg.py:
  allow exactly one named production function-SSA session owner

resolved_if_lowering_contract.sh:
  require canonical effect queries, join rows, branch snapshots,
  manual join PHIs, and manual publication to be zero

resolved_ownership_legacy_release_inventory.py:
  keep the canonical lowerer row as an exact zero-occurrence retirement row
  make this the sole live canonical legacy-release caller-zero owner

canonical compile finalization:
  use an explicit schedule policy so the Binding-SSA canonical route skips
  optional legacy RC insertion; prove this under `rc-insertion-minimal`
```

The public guard facade stays below 800 lines; new checks remain in bounded
private helpers.

## Resolved decision output

The returned decision fixes:

1. **A′** is accepted;
2. `VerifiedTrivialCanonicalOwnerV1` under
   `src/mir/resolved_value_profile` owns the executable per-function proof;
3. production Ownership SSA remains zero through SSA-I1-T and first activates
   only at SSA-I1-O1;
4. explicit empty Return and implicit fallthrough are distinct no-value
   dispositions, while Outbox, Void values, and Null values reject;
5. temporary A+ is permitted only through pre-Builder whole-unit selection,
   with no unit-internal mixing or failure retry;
6. the order is SSA-I0-PROFILE -> SSA-I1-T -> compatibility rows ->
   SSA-I1-FULL -> SSA-R1, with SSA-I1-O1 independently gated by one exact
   BoxRef producer and ABI witness;
7. fixtures and guards must preserve the authority and stop conditions below.

## Nonclaims

```text
SSA-I1-T production activation
current canonical acceptance narrowing
parameter/return ownership ABI closure
String/Null ownership closure
Ownership SSA production activation
old A+ caller zero or retirement
Loop production support
default source route cutover
ProgramV0, Lambda/capture, REPL, or Hako Lower parity
```

## Stop conditions

Do not implement or publish SSA-I1-T if a proposal:

```text
uses the P0 inventory as an executable proof
infers representation from MirType, StorageClass, VMValue, Span, pointer, name,
or a value observed after Builder effects
silently rejects a currently accepted canonical source unit
mixes A+ and Binding SSA inside one source unit or owner
retries A+ after a Binding SSA failure
keeps a flat value map synchronized with Binding SSA
activates owned operations without an exact BoxRef producer and ABI witness
calls legacy optional RC insertion on the Binding-SSA canonical route
installs or verifies Ownership SSA vacuously without a selected contract
publishes a function before coverage, CFG, Binding SSA, PHI, and applicable
ownership verification all finish
```

## Next action

The consultation stop is resolved and the disconnected SSA-I0-PROFILE product
is closed by
`mirbuilder-ssa-i0-trivial-owner-profile-2026-07-15.md`. Proceed to SSA-I1-T
as one atomic trivial-profile production cutover. Until SSA-I1-T lands:

```text
production Binding SSA sessions = 0
production Ownership SSA witness/install/verifier calls = 0
CopyOwned / DestroyOwned production callers = 0
current A+ production behavior = unchanged
```

SSA-I1-T must preserve whole-unit route selection, no mixing, no retry, and
must skip legacy `insert_rc_instructions` on the selected Binding-SSA route.
