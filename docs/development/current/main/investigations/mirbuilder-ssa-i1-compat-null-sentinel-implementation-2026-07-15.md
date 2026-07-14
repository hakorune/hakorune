# SSA-I1-COMPAT-N0a NullSentinel Implementation Evidence

Status: Closed — exact Null local flow uses production Binding SSA

Date: 2026-07-15

Decision: preserve exact source `null` as `NullSentinel` in the resolved value
profile while reusing the existing `ConstValue::Null` / `MirType::Void` /
runtime no-value representation. This is a local-flow compatibility row, not
a general Void or return ABI.

Parent taskboard:
`mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`

Selection card:
`mirbuilder-ssa-i1-compat-null-sentinel-selection-2026-07-15.md`

## Closed production boundary

The existing whole-owner preflight now admits exact Null only in this closed
grammar:

```text
Null literal
local declaration / read / assignment
BlockExpr tail
homogeneous one-sided, two-sided, and nested fallthrough If
Null == Null / Null != Null -> InlineBool
terminal result = InlineBool or existing no-value completion
```

`NullSentinel` is exact profile identity only. MIR materialization stays:

```text
LiteralValue::Null
  -> TrivialRepresentationV1::NullSentinel
  -> ConstValue::Null + MirType::Void
  -> existing runtime no-value
```

No new opcode, runtime tag, backend lane, ownership token, or call ABI was
introduced.

## Authority and negative boundary

```text
resolved value profile:
  exact Null source/value/definition/merge coverage

Binding SSA:
  sole BindingRef -> ValueId and demand-driven PHI authority

runtime/backend:
  existing no-value execution only

non-authority:
  names, runtime tags, StorageClass inference, legacy retry
```

The following remain whole-unit A+ selections before Builder effects:

```text
Null terminal return
explicit Void value
Outbox
BorrowedText
parameter / receiver owners
mixed Null/non-Null merge
```

The selected Binding-SSA route still skips legacy RC insertion and never
retries A+ after a lowering error.

## Machine inventories

The canonical ownership production inventory now has 18 rows. Exact trivial
rows increase from three to four by adding only
`origin.literal.null = literal_shape_null_sentinel`; the independent Void row
remains a typed rejection.

The trivial-owner profile guard now fixes 12 focused profile fixtures and the
Null admission/rejection split. The legacy `ReleaseStrong` ledger was refreshed
for already-landed and current D-prime evidence/guard surfaces and now closes
124 surfaces and 275 exact occurrences without changing runtime authority.

## Validation evidence

```text
cargo fmt --all -- --check
git diff --check
  -> green

cargo check -q
  -> green (pre-existing warnings only)

cargo test -q --lib mir::resolved_value_profile::tests
  -> 12/12 green

cargo test -q --features vm-reference --lib \
  mir::builder::resolved_lowering::null_tests
  -> 3/3 green

cargo test -q --features vm-reference --lib \
  mir::builder::resolved_lowering
  -> 86/86 green

cargo test -q --lib mir::compiler::capability_tests
  -> 5/5 green

three exact compiler finish/publication tests
  -> 3/3 green

resolved ownership production profile
  -> 18/18 green

resolved trivial-owner profile
  -> 12 fixtures green

legacy ReleaseStrong inventory
  -> 124 surfaces / 275 occurrences green

bash tools/checks/resolved_region_flow_authority_guard.sh
  -> green

bash tools/checks/dev_gate.sh quick
  -> PASS 66/66

cargo build --release --bin hakorune
  -> green (pre-existing warnings only)

largest modified source/check file
  -> 736 lines
```

One broad `mir::compiler::tests` filter still exposes the unrelated
exact-numeric box-field refresh fixture failure. The N0a-specific compiler
finish/publication tests and required quick gate are green; this row does not
change that contract-refresh family.

## May claim

```text
exact local-flow Null values use the production trivial Binding-SSA route
Null values and homogeneous Null PHIs retain MirType::Void
Null comparison produces an exact Bool terminal
selected Null route ownership operations and legacy RC insertion are zero
negative compatibility families remain preflight-separated
```

## Must not claim

```text
general Void value support
Outbox support
Null return ABI
nullable Box or mixed-representation PHIs
parameter / receiver ABI
BorrowedText
Ownership SSA activation
Loop activation
SSA-I1-FULL
```

## Next decision stop

Return to SSA-I1-COMPAT row selection. Choose exactly one remaining semantic
row before implementation. Exact typed parameters remain separate from the
receiver owner family; explicit Void disposition remains separate from Outbox;
BorrowedText still requires its own lifetime and ABI decision.
