# RAW-SOURCE0 LOWER ROOT0 OWNER0 — ELIGIBILITY0-S0 execution task

Status: **Closed — implementation and S0-G0 evidence green**
Date: 2026-07-24

## Decision

Decision: ELIGIBILITY-prime-r1

`ELIGIBILITY-prime-r1` is locked by the consultation closeout:

```text
Q1 CaptureOnce
Q2 NarrowExhaustive / ScalarControl0
Q3 NarrowReject
Q4 closure and static data typed-reject
Q5 process-global slot typed-reject
```

The single executable row is:

```text
RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0
```

This row seals eligibility only. It does not open a Builder session or any
physical owner.

```text
physical effects = 0
production consumers = 0
```

## Owner products

The consuming terminal is compiler-owned:

```rust
impl SourceBoundRawRootPackageV1 {
    fn prepare_eligibility(
        self,
    ) -> Result<
        EligibleSourceBoundRawRootPackageV1,
        RejectedRawRootEligibilityV1,
    >;
}
```

Success keeps the whole PACKAGE0 owner intact:

```rust
struct EligibleSourceBoundRawRootPackageV1 {
    package: SourceBoundRawRootPackageV1,
    proof: RawRootEligibilityV1,
}
```

Failure keeps the same package by value:

```rust
struct RejectedRawRootEligibilityV1 {
    owner: SourceBoundRawRootPackageV1,
    stage: RawRootEligibilityStageV1,
    error: RawRootEligibilityErrorV1,
}
```

The rejection surface is inspection plus `discard(self)` only. No
`into_owner`, retry, resume, fallback, replacement plan, or second eligibility
terminal is allowed.

## Q1 — runtime inputs

Capture both Raw runtime inputs once before token issuance:

```text
NYASH_SCRIPT_ARGS_JSON > HAKO_SCRIPT_ARGS_JSON > absent
NYASH_BUILDER_SAFEPOINT_ENTRY
```

Store the values in `RawSourceContinuationV1`, not generic Builder config:

```text
script args:
  absent -> Absent
  valid string array (including []) -> Present(values)
  malformed/non-array/non-string -> typed ingress rejection

entry safepoint:
  unset/0/false/off -> Disabled
  1/true/on -> Enabled
  other present value -> typed ingress rejection
```

Capture occurs before token issuance. The new Raw chain performs no ambient
re-read. Existing legacy readers may remain until CUT0, but they are not
allowed in the new eligibility/owner path.

## Q2 — ScalarControl0 classifier

Replace the wildcard source authority with one recursive, explicit classifier.
The first eligible grammar is:

```text
Expr0 = Literal | Variable
      | Unary(Minus | Not | BitNot, Expr0)
      | Binary(current BinaryOperator, Expr0, Expr0)

Stmt0 = Expr0
      | Print(Expr0)
      | Assignment(Variable, Expr0)
      | CompoundAssignment(Variable, BinaryOperator, Expr0)
      | Local(cardinality-matched, untyped, optional Expr0)
      | If(Expr0, Stmt0*, optional Stmt0*)
      | Loop(Expr0, Stmt0*)
      | LoopRange(name, Expr0, Expr0, Stmt0*)
      | Return(None | Expr0)
      | Break/Continue only inside a loop
      | ScopeBox(Stmt0*)
```

Every other AST surface receives an explicit typed disposition. In particular,
Using/Import/BuildGate, Lambda, Call/Field/Index, async/task/context/match,
New, and unknown/future variants never enter a wildcard `RuntimeStatement`
lane. Nested bodies are classified recursively; a top-level-only scan is not
enough.

## Q3 — narrow complete catalog

The only eligible callable shapes are:

```text
Empty Script:
  no top-level declarations or callable rows

Plain static-Main App:
  exactly one static Main box
  main/0 plus zero-or-more static helper methods
  no fields, constructors, static_init, delegates, traits, sync/record/
  interface properties, or sibling boxes
```

For the admitted App, every method row must prove exact name, source locator,
role, symbol, arity, cardinality, and duplicate-free correspondence. The root
Main locator is root authority; helper rows are `MainStaticChild`; callable
Main selection remains owned by the sealed continuation, not a boolean row.

Reject before physical effects:

```text
top-level functions, non-Main boxes, instance boxes/methods, constructors,
sync/record/interface boxes, static init, fields/delegates, partial or
mismatched rows, and duplicate callable identities
```

## Q4/Q5 — explicit rejection gates

```text
Lambda/closure             -> UnsupportedClosureAccess
StaticConstTable           -> UnsupportedStaticDataAuthority
instance/constructor/New,
birth-slot or slot resolve  -> UnsupportedProcessGlobalSlot
```

`ROOT0-CLOSURE0`, `ROOT0-STATICDATA0`, and `ROOT0-SLOT0` are future widening
rows. S0 never mutates process-global `TYPE_IDS`, `NEXT_TYPE_ID`, or
`EXPLICIT_SLOTS`.

## Implementation split and file budget

Do not grow the existing ~600-line `raw_root_plan0.rs`. Keep new files below
800 lines:

```text
raw_runtime_inputs.rs             pure capture/parser
raw_root_eligibility.rs           products and consuming terminal
raw_root_eligibility_classifier.rs ScalarControl0 recursive classifier
raw_root_eligibility_p0.rs        positive/negative/retention fixtures
one S0 guard and this task card
```

`compiler/mod.rs` receives only module registrations. Existing PACKAGE0,
PLAN0, and binding paths remain behavior-neutral.

## Required fixtures

```text
raw_root_runtime_inputs_absent_seal_none_false
raw_root_runtime_inputs_valid_args_and_safepoint_are_retained_exactly
raw_root_runtime_inputs_malformed_rejects_before_token_mint
raw_root_eligibility_empty_script_is_eligible
raw_root_eligibility_plain_static_main_is_eligible
raw_root_eligibility_scalar_control_order_and_cardinality_are_exact
raw_root_eligibility_unknown_or_preprocessed_shape_rejects
raw_root_eligibility_nested_lambda_rejects
raw_root_eligibility_static_table_rejects
raw_root_eligibility_instance_constructor_or_new_rejects
raw_root_eligibility_partial_catalog_rejects
every_rejection_retains_whole_package_and_has_zero_physical_effects
```

Every rejection compares a before/after owner snapshot and proves:

```text
token/source/continuation/config/module/plan unchanged
session/shell/collector/ledger/tracker construction = 0
process-global slot delta = 0
retry/fallback/re-pairing = 0
```

## Guard and evidence

The S0 guard must require exactly one eligibility product/terminal and freeze:

```text
RawRootEligibilityV1 definition = 1
EligibleSourceBoundRawRootPackageV1 producer = 1
RejectedRawRootEligibilityV1 producer = 1
prepare_eligibility(self) definition = 1
discard(self) only rejection exit = 1
wildcard RuntimeStatement classifier = 0
new Raw lowerer ambient runtime reads = 0
process-global slot calls in S0 = 0
physical/production consumer = 0
all touched source/check files < 800 lines
```

The guard must scan an explicit S0 manifest and avoid treating comments or
historical legacy readers as new-path consumers.

Evidence commands:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_source_binding_p0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_root_plan0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_root_package --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_root_runtime_inputs --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_root_eligibility --lib -- --test-threads=1
```

## Landed implementation slice (2026-07-24)

The first buildable S0 slice is landed and pushed on `public-main`:

```text
a03ec16d6f  mir: add raw root eligibility boundary
02fbfba892  mir: keep raw eligibility source-derived
```

It contains the strict runtime-input snapshot, the recursive wildcard-free
ScalarControl0 classifier, and the consuming eligibility/rejection products.
The source plan performs the classification once; eligibility inspects the
sealed work dispositions and does not re-run the body classifier. No
session/shell/collector/ledger/tracker is opened by this slice.

Verified evidence for the landed slice:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib                         green
RUSTFLAGS='-Awarnings' cargo test -q raw_ --lib -- --test-threads=1 green (176)
bash tools/checks/current_state_pointer_guard.sh                  green
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_owner0_eligibility0_s0_guard.py green
```

The explicit negative/retention fixture matrix and manifest guard are now
landed in the same disconnected S0 boundary. Physical OWNER0 opening remains
zero; only the final S0 closeout/pointer update remains.

## Non-claims and next row

S0 does not claim child traversal, callable-Main descent, root-body lowering,
declaration/access installation, slot publication, closure interning,
static-table publication, Main/condition batch, drain, finalization,
postprocess, external commit, public ingress, JSON parity, or CUT0 activation.

Only after S0/G0 is green may `OWNER0-PHYSICAL0` consume
`EligibleSourceBoundRawRootPackageV1`. Cleanup census remains a separate
`CLEAN0-*` lane and is not mixed into this semantic slice.
