# SSA-I1-T Trivial Binding SSA Cutover Evidence

Status: Closed — one admitted trivial whole owner uses production Binding SSA

Date: 2026-07-15

Decision: whole-unit atomic route selection before Builder effects. The
selected Binding-SSA route never enters legacy RC insertion and never retries
the temporary A+ route after a lowering failure.

Parent taskboard:
`mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`

## Production boundary

Preflight produces exactly one sealed variant:

```text
CanonicalFirstFamilyPlanV1
  TrivialBindingSsa
  CurrentCanonicalAPlus
```

The compiler matches that variant once before opening the candidate module
session. `TrivialBindingSsa` owns one dedicated lowerer under:

```text
src/mir/builder/resolved_lowering/trivial_ssa/
  README.md
  mod.rs
  identity.rs
  operation.rs
  lowerer.rs
```

Non-admitted current canonical owners remain whole-unit A+ owners. There is no
function/body/site-level authority mixing and no error fallback.

## Authority split

```text
pre-Builder profile:
  exact trivial representation and source coverage

carrier-free If control:
  exact control topology and source coverage

BindingSsaBuilderV1:
  sole BindingRef -> ValueId reaching-definition and PHI authority

CanonicalCfgSessionV1:
  terminator-derived predecessor truth and block sealing

PhiTxn:
  provisional PHI lifecycle and rollback

Lower:
  exact located materialization only
```

The trivial production box contains no flat `variable_map`, RegionFlow effect
or join rows, `may_rebind`, branch-map diff, legacy If session, legacy operator
route, `ReleaseStrong`, `CopyOwned`, or `DestroyOwned` authority.

## Runtime and publication law

Declarations and assignments define the current block directly. Reads ask the
function-owned Binding SSA for the reaching value; merge PHIs are created only
when a later read requires them. Then state is never restored into else state.
BlockExpr scope close retires lexical identity without deleting historical SSA
definitions.

Every touched block is sealed from exact C1 witnesses. Before a candidate
function is published, the lowerer finishes semantic/profile/If-control/
identity/completion coverage, CFG, Binding SSA, and PHI transactions. The
function draft then passes `MirVerifier::verify_function`. Before module commit,
the fully transformed canonical module passes
`canonical_post_transform_verify`.

The selected finish schedule is closed:

```text
TrivialBindingSsa      -> legacy insert_rc_instructions = Skip
CurrentCanonicalAPlus -> legacy insert_rc_instructions = Run
Legacy                -> legacy insert_rc_instructions = Run
```

## Guard

`tools/checks/lib/resolved_binding_ssa_i1_t.py` fixes:

```text
BindingSsaBuilderV1 external production caller files = 1
MirBindingSsaAdapterV1 external production caller files = 1
CanonicalCfgSessionV1 external production caller files = 1
profile analyzer production caller files = 1
legacy RC on selected trivial route = 0
Ownership SSA/opcode production activation = 0
accepted grammar delta = 0
all source/check files below 800 lines
```

The helper is wired into the existing private Binding-SSA authority contract;
no second public guard is introduced.

## Validation evidence

```text
cargo fmt --check
git diff --check
cargo check -q
  -> green (pre-existing warnings only)

cargo test -q mir::builder::resolved_lowering --lib
  -> 75/75 green
  -> exact Float BinOp result stays MirType::Float
  -> selected trivial route emits ReleaseStrong 0

cargo test -q mir::compiler::capability_tests --lib
  -> 5/5 green

cargo test -q finish_schedule --lib
  -> 2/2 green

cargo test -q mir::resolved_value_profile --lib
  -> 10/10 green

cargo test -q mir::resolved_control_flow::if_control_tests --lib
  -> 11/11 green

cargo test -q --features vm-reference \
  mir::builder::resolved_lowering::if_tests --lib
  -> 6/6 green

python3 tools/checks/lib/resolved_binding_ssa_i1_t.py .
  -> green

bash tools/checks/current_state_pointer_guard.sh
  -> green

cargo build --release --bin hakorune
  -> green (pre-existing warnings only)

bash tools/checks/dev_gate.sh quick
  -> PASS 66/66

largest new production/check source
  -> 605 / 164 lines
```

## May claim

```text
one admitted trivial whole owner uses one production Binding SSA
fallthrough statement If uses carrier-free control plus demand-driven SSA PHIs
BlockExpr and nested fallthrough If stay inside the same value authority
the selected route skips legacy RC insertion
coverage, CFG, SSA, PHI, and function verification close before publication
```

## Must not claim

```text
parameter/receiver/Outbox/BorrowedText/Null compatibility
ownership-managed Box values
production Ownership SSA or CopyOwned/DestroyOwned
all current A+ owners cut over
Loop production activation
legacy A+ or ReleaseStrong retirement
default source, ProgramV0, REPL, Lambda, or capture support
```

## Next decision stop

SSA-I1-COMPAT is next, but its first compatibility row must be selected before
implementation. Parameter/receiver ABI, Outbox/Void disposition, BorrowedText,
and Null are separate semantic rows and must not be bundled into one patch.
