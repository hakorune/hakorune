# RAW public cutover PARITY0 App scalar-return design question

Decision: `RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-APP-D0`

Status: superseded as canonical before implementation.

`FUNCTION-EXIT-SEMANTICS-prime-r1` supersedes `BODY-RETURN-APP-prime-r1` as
the language authority. The Legacy-compatible `AppLastValueOrVoid` selection
below is retained only as historical parity evidence. It is not a canonical
or public production policy, and the paired S0 task is not executable.

```text
superseded_by =
  docs/reference/language/function-exit-and-entry-result.md

canonical replacement =
  FUNCTION-EXIT-F1-RETURN0-S0

compatibility-evidence replacement =
  RAW-BODY-RETURN-COMPAT-P0

sunset =
  RAW-BODY-RETURN-COMPAT-SUNSET-001
```

## Historical selected parity decision — not executable

```text
Q1 = A
Q2 = existing LinearScalar0, all admitted tail kinds, exact supported types
Q3 = N/A; AppFixedVoid/discarded_tail production authority retires
Q4 = one PreparedRawRootBodyCommitV1 inside drive_root_body
Q5 = preflight all failures; private commit infallible; discard-only rejection
Q6 = empty/scalar/helper/Print/Local/Assignment/Compound/reuse rows block promotion
Q7 = normal entry, JSON, executor, selfhost, fastmem, old retirement, CUT0 unchanged
```

## Evidence already fixed

Legacy and Raw currently have different owners:

```text
Legacy:
  build_module
  -> lower_root(ast) -> result ValueId
  -> finalize_module(result_value)
  -> Return(result_value)
  -> signature.return_type = Builder TypeContext(value)

Raw Script:
  LinearScalar0 -> RootBodyResultV1::Value(v)
  -> BODY exit plan resolves Builder type(v)
  -> Return(v), signature type(v), completion Value(v)

Raw App first slice:
  Main.main/0 is static, arity 0, no declared return/uses/contracts/attrs
  -> AppFixedVoid recipe policy
  -> BODY may lower a tail ValueId, but records it as discarded evidence
  -> synthetic Void constant + Return(Void)
  -> completion NoValue
```

The Raw route is not an accidental adapter: `RawRootExitPolicyV1::AppFixedVoid`
is sealed in the source-derived recipe contract, and the BODY finalizer emits
the signature, physical Return, completion disposition, and
`RawRootBodyExitWitnessV1` from one plan. ROOTBATCH0 only validates that witness
by borrow before collector/ledger mutation. Main identity remains `main` with
arity 0; return type is not a collector/ledger identity field.

## Why this is a design stop

Changing App scalar return policy changes more than one snapshot row:

```text
App body recipe exit policy
root skeleton provisional/fixed contract
physical main/0 ABI return type
completion Value(v) vs NoValue
exit witness disposition
public NarrowV1 admitted grammar
Legacy-vs-Raw parity claims
downstream verifier/backend expectations
```

The decision must not be hidden in postprocess return inference, a public
adapter repair, a module-symbol heuristic, or a fallback to `build_module`.

## Q1 — What is the public App compatibility authority?

The A/B comparison below is migration history. Candidate A is not the
accepted canonical function/Main semantics. Under
`FUNCTION-EXIT-SEMANTICS-prime-r1`, its observed result relation may be
recorded only by the test-only `LegacyObservationOracleV1` and parity
witnesses. It must not become a recipe policy, runtime mode, public ingress
option, or other executable compatibility profile.

Choose exactly one authority:

```text
A (Legacy-compatible scalar App; recommended for PARITY0):
  The public Legacy wrapper ABI is the compatibility authority. App gets a
  distinct AppLastValueOrVoid policy (do not infer it from symbols or silently
  reuse the Script variant).
  A non-empty scalar body returns the exact last ValueId and Builder type.
  Empty App remains Void/NoValue.

B (FixedVoid App):
  App Main keeps AppFixedVoid as an intentional new public ABI. Exact Legacy
  scalar parity is not claimed. For a clean NarrowV1 parity boundary, every
  non-empty App body whose Legacy wrapper would expose a value is rejected
  before PHYSICAL0; physical main/0 remains Void.
```

The authority must be a single Raw recipe/eligibility contract. Caller policy,
locator presence, module symbols, postprocess inference, and public result
adaptation are not selection authorities.

## Q2 — What exact route/recipe grammar is admitted?

The selected Q2 contract is fixed before code:

```text
route policy                            = distinct AppLastValueOrVoid
accepted statement/expression grammar   = existing LinearScalar0 only
last-value rule                         = Expr, Print, Local, Assignment, CompoundAssignment
empty body disposition                  = NoValue/Void
supported return types                  = Integer, Float, Bool, String, Void
Unknown/Box/Array/Future/WeakRef        = typed reject or new capability row
explicit return declaration             = prohibited or exact metadata contract
```

The answer must state whether `Print`, `Local`, assignment, and compound
assignment affect the returned value, and where a missing tail becomes
`NoValue`. No inferred “whatever the lowerer happens to return” rule is
allowed.

The selected route keeps the existing App metadata preflight: zero params,
zero param declarations, no declared return type, no uses, contracts, or attrs.
It infers the physical wrapper return type from the last lowered ValueId; it
does not invent a source return declaration. `TypedInteger` and any result type
outside Integer/Float/Bool/String/Void require an explicit capability decision
or typed rejection before promotion.

## Q3 — FixedVoid alternatives (rejected by the selected decision)

FixedVoid has two materially different forms:

```text
B1: admit LinearScalar0 tails, retain discarded_tail in AppVoid witness,
    return synthetic Void, completion NoValue. This is a deliberate
    non-parity capability and cannot close Legacy-vs-Raw App parity.

B2: reject scalar App before physical open with a typed eligibility/recipe
    error; no lowering, reservation, receipt, or BODY entry occurs.
```

Neither B1 nor B2 is selected. B1 would preserve a deliberate non-parity
success; B2 would narrow the already admitted Raw public capability. Both are
parked alternatives, not implementation targets for this task.

## Q4 — Which owner co-seals App facts?

The selected policy must preserve one owner chain:

```text
recipe policy
-> InstalledRawRootEnvironmentV1::drive_root_body
-> one prepared exit plan
-> signature + physical Return + completion + exit witness
-> function cleanup
-> CompletedRawRootBodyPhysicalV1
```

ROOTBATCH0 may only borrow-validate the witness and retain it in
`RawInvocationRootWitnessV1`. It must not infer App return type, rewrite the
draft signature, or create a second Return.

## Q5 — What is the fail-fast and retention law?

For both candidate policies, define typed causes for:

```text
route/policy mismatch
missing or Unknown ValueId type
unsupported return type
already-terminated block
completion/witness mismatch
signature/Return mismatch
foreign brand/family
```

Failure must retain the exact unpublished owner, recipe policy, lowered
function state, active tracker/physical carrier, and failing source site where
available. For B2, the source-bound owner is retained before physical open.
Retry, fallback, signature repair, snapshot normalization, and legacy re-entry
are forbidden.

## Q6 — What parity is actually claimed?

The acceptance matrix must distinguish these rows:

```text
empty Script                         = already green
Script scalar literal/binary         = already green
empty App FixedVoid                  = already green
non-empty App scalar                 = decision-dependent
App helper + scalar tail             = decision-dependent
App callable-Main Selected           = separate route evidence
reportable pre-transform verifier Err= evidence, not publication failure
```

If A is selected, parity must compare at least:

```text
main signature return type
physical Return operand
completion disposition
exit witness disposition/type
module snapshot/runtime-observable result
compiler reuse on the same MirCompiler
```

If B is selected, the parity claim must explicitly say that App scalar
Legacy-parity is not claimed and identify the typed rejection/discard boundary.
The former non-empty FixedVoid success fixture must become either a deliberate
non-parity capability fixture (B1) or a pre-physical rejection fixture (B2).

Required common rows include empty App, helpers plus empty main, Script
regressions, callable-Main `Omitted`, unchanged Main/condition ledger identity,
and witness mismatch before collector/ledger mutation. If A is selected, add
Integer/String/ordinary-binary App plus helper+scalar rows; Print/Local/
Assignment/Compound rows must be either green or explicitly outside the
admitted matrix.

## Q7 — What is the cutover and non-claim boundary?

This consultation must not silently activate normal entry. Whichever policy is
chosen, the row must leave these unchanged until a later cutover decision:

```text
compile_with_source legacy/Raw routing
JSON and Program(JSON v0)
executor, selfhost, fastmem
old Raw-chain retirement
public adapter repair
CUT0
```

The selected App policy must be recorded as a separate semantic row, with one
producer, one focused guard, one parity matrix, and an explicit sunset or
promotion condition. Structural guards must keep App policy producer = 1,
BODY exit prepare/commit = 1, old split finalizer = 0, symbol/module route
inference = 0, return type in ledger identity = 0, postprocess/adapter repair
= 0, and normal-entry/JSON/executor/CUT0 consumers = 0.

The accepted later function-exit decision reclassifies that historical
requirement as follows:

```text
historical parity behavior =
  LegacyAnyStatementValueOrUnit

evidence owner =
  test-only LegacyObservationOracleV1 + parity witness

executable compatibility policy/profile =
  0

canonical/public production consumers =
  0

promotion =
  forbidden under accepted F1; reopening F3 requires a new language decision

sunset_id =
  RAW-BODY-RETURN-COMPAT-SUNSET-001

owner of retirement decision =
  FUNCTION-EXIT-COMPAT-RETIRE0

target row/card =
  RAW-BODY-RETURN-COMPAT-RETIRE0-S0
```

## Required closeout output

```text
Decision: BODY-RETURN-APP-prime-r?
Q1 authority = A or B
Q2 exact grammar/ABI = ...
Q3 fixed-Void disposition (if B) = B1 or B2
Q4 sole co-seal owner = ...
Q5 typed failure + owner retention = ...
Q6 parity matrix = ...
Q7 cutover/non-claims = ...

first executable row = only after this consultation closes
```
