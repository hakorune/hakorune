# SSA-I0-PROFILE Exact Trivial Owner Evidence

Status: Closed — disconnected exact whole-owner trivial profile implemented

Date: 2026-07-15

Decision: A′ whole-unit profile routing. Production Binding SSA and Ownership
SSA activation remain zero in this row.

Parent taskboard:
`mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`

Resolved design stop:
`mirbuilder-ssa-i1-trivial-profile-atomic-cutover-design-stop-2026-07-14.md`

## Physical owner

The disconnected executable proof is isolated under:

```text
src/mir/resolved_value_profile/
  README.md
  mod.rs
  analyzer.rs
  coverage.rs
  error.rs
  operator.rs
  product.rs
  tests.rs
```

Its sealed result is `VerifiedTrivialCanonicalOwnerV1`. The product contains
only exact owner/binding/source identities, the closed
`InlineI64`/`InlineBool`/`InlineF64` vocabulary, and value/definition/If-merge/
terminal coverage. It contains no `ValueId`, `BasicBlockId`, `MirType`,
`StorageClass`, runtime value, Builder state, or MIR instruction.

The analyzer co-seals its result against both
`VerifiedFunctionCompletionV1` and the disconnected carrier-free
`VerifiedResolvedFunctionIfControlV1`. The latter remains control-only: its
exact rows and coverage are consumed as witnesses, while the value-profile
product records only homogeneous representation at an If merge. It never
decides PHI placement.

## Sealed contract

```text
literal source authority:
  Integer / TypedInteger -> InlineI64
  Bool                   -> InlineBool
  Float                  -> InlineF64

derived exact profile:
  local initializer and declaration
  binding read
  assignment value and binding definition
  closed binary operand/result law
  BlockExpr tail/result
  homogeneous fallthrough If merge profile
  exact Return value

no-value terminal dispositions:
  explicit return;
  implicit function fallthrough

typed rejects:
  receiver or unsealed parameter ABI
  Outbox value seed
  missing local initializer
  String / BorrowedText
  Void or Null value
  non-Bool If condition
  mixed If merge profile
  unsupported statement/expression/operator
```

Profile rejection is a pre-Builder route-selection result. It is not a
fallback after Builder effects. A source unit may later select either the
trivial Binding-SSA route or the temporary current A+ route exactly once; the
two authorities may not mix inside an owner or source unit.

## Coverage law

Every admitted exact expression value, binding definition, If merge profile,
and terminal disposition is claimed exactly once in deterministic source order.
If merge profiles use the exact If statement plus `BindingRefV1`; they do not
fabricate an expression site or precompute a PHI row. Foreign owners, missing
resolver facts, duplicate claims, missing reaching profiles, and terminal
cardinality mismatches fail with a typed contract error.

## Guard integration

The implementation is checked by one bounded private helper beneath the
existing Binding-SSA authority facade:

```text
tools/checks/fixtures/canonical_trivial_owner_profile_v1.json
tools/checks/lib/resolved_trivial_owner_profile.py
tools/checks/lib/resolved_trivial_owner_profile_contract.sh
tools/checks/lib/resolved_binding_ssa_contract.sh
tools/checks/resolved_region_flow_authority_guard.sh
```

No new public guard is introduced. The private guard must prove:

```text
profile source files exist and stay below 800 lines
forbidden Builder/MIR/runtime representation imports = 0
profile production callers = 0
Binding SSA production callers = 0
Ownership SSA witness/install/verifier calls = 0
CopyOwned / DestroyOwned production callers = 0
route and accepted-grammar delta = 0
```

## Validation evidence

Recorded evidence:

```text
cargo test -q mir::resolved_value_profile
  -> 10/10 green

private resolved_trivial_owner_profile validator
  -> green
  -> manifest 8/8
  -> production callers 0
  -> historical 92-row SHA-256 exact

bash tools/checks/resolved_region_flow_authority_guard.sh
  -> green

cargo test -q mir::resolved_control_flow::if_control
  -> 9/9 green through the public authority guard

cargo test -q mir::compiler::capability
  -> 4/4 green through the public authority guard

cargo check -q
  -> green (pre-existing warnings only)

cargo build --release --bin hakorune
  -> green (pre-existing warnings only)

tools/checks/dev_gate.sh quick
  -> PASS 66/66

profile manifest
  -> 8/8 files present

largest profile source / private check helper
  -> 610 / 468 lines
```

Current-pointer and diff validation:

```text
bash tools/checks/current_state_pointer_guard.sh
  -> green

git diff --check
  -> green
```

## May claim after validation

```text
one disconnected exact whole-owner trivial representation proof exists
all admitted value/definition/merge-profile/terminal subjects are covered exactly once
unsupported representation and ABI surfaces reject before Builder effects
production Binding SSA and Ownership SSA callers remain zero
the current A+ production route and accepted grammar are unchanged
```

## Must not claim

```text
SSA-I1-T production routing or Binding SSA activation
production Ownership SSA or CopyOwned / DestroyOwned activation
parameter, Outbox, BorrowedText, Void-value, or Null-value representation
old A+ caller zero or retirement
Loop, exit-family, Lambda/capture, ProgramV0, REPL, or Hako parity support
```

## Next row

Advance the current blocker to `SSA-I1-T`: one whole-source-unit atomic
cutover for the sealed trivial profile. Non-admitted current units may remain
on the temporary A+ route only through one pre-Builder selection; retry and
unit-internal mixing remain forbidden. The selected Binding-SSA route must
also bypass legacy `insert_rc_instructions`; disconnected ownership fixtures
do not authorize production ownership insertion.
