# SourceBodyAnalysisSnapshotV1 — Direct Dual-Frontend Taskboard

Status: S0 closed; stopped at Hako source-carrier design boundary.
Date: 2026-07-12
Decision: `B — source_snapshot_v1`
Mode: `direct_dual_frontend_ast_projection_v1`

## Objective

Build a bounded, immutable source-syntax observation family directly from the
Rust and Hako frontend ASTs. Preserve the source distinctions required by the
bool-predicate Fact family without widening or reinterpreting ProgramV0.

```text
Rust source -> authoritative Rust frontend AST -> Rust V1 projector
same source -> independent Hako parser carrier -> Hako V1 projector

assert exact SourceBodyAnalysisSnapshotV1 parity
```

The first semantic consumer is `BoolPredicateScanSummaryV1`. Planner, route,
backend, runtime, MIR mutation, and Fact authority remain unchanged.

## Authority

```text
canonical Rust source syntax and child roles:
  hakorune_frontend_ast::ASTNode
  canonical grammar registry
  authoritative Rust frontend parser

Hako source observation:
  independent Hako parser private typed carrier

V1 structural vocabulary:
  SourceBodyAnalysisSnapshotSchemaV1

derived loop observation:
  SourceScanObservationV1

individual Fact meaning:
  existing BoolPredicateScanFact owner
  existing StringIsIntegerFact owner
```

`crate::ast` is a compatibility facade, not a second syntax authority.
Snapshot V1 observes canonical syntax; it does not define language meaning.

## Non-authority

```text
Program(JSON v0)
BoundedBodyAnalysisSnapshotV0
ProgramV0 reader / serializer / strict JSON arena
Rust AST -> ProgramV0 mapping
V0 Method/Local/Int observations
raw source scanner or token-offset sidecar
MIRBuilder / planner / route / backend / runtime
CondProfile or ScanConditionObservation as source truth
inferred type / resolved symbol / dispatch category
```

V1 and V0 are parallel families:

```text
V0 = removable ProgramV0 wire observational quotient
V1 = bounded canonical source AST observation
```

Neither family may fall back to the other.

## Initial accepted vocabulary

The closed first subset is only the syntax required to express the first
bool-predicate target and the later string-is-integer target.

```text
root:
  Body(statements[])

statements:
  Loop(condition, body[])
  Local(bindings[])
  Assignment(target=Variable, value)
  If(condition, then[], optional else[])
  Return(Absent | Present(value))
  Break
  Continue

expressions:
  Literal(Int | String | Bool | Null)
  Variable(name)
  Me
  This
  UnaryOp(exact operator, operand)
  BinaryOp(exact operator, lhs, rhs)
  MethodCall(receiver, method, arguments[])
```

All eighteen canonical `BinaryOperator` variants are accepted as exact source
operators. `UnaryOperator` has four current variants (`Minus`, `Not`,
`BitNot`, `Weak`); S0 must classify every one as Accepted or
KnownUnsupported, with `Not` necessarily Accepted. No wildcard is allowed.

Required distinctions:

```text
Local != Assignment != CompoundAssignment
UnaryOp(Minus, Int(1)) != Literal(Int(-1))
Return(Absent) != Return(Int(0)) != Return(Bool(false))
Me != This != Variable
MethodCall != FunctionCall
```

MethodCall preserves only receiver syntax, method text, and argument order.
It never classifies ordinary/static/brand/record/typed-array dispatch.

Typed Local is initially Unsupported with
`source.type_syntax_deferred`. Local binding order and initializer presence
must be retained. Assignment initially accepts only a Variable target.

## Deferred vocabulary

```text
Print
CompoundAssignment
GroupedAssignmentExpr
FunctionCall
FieldAccess / Index
field or index assignment target
LoopRange
lambda / match / ternary
record / enum / brand forms
type annotation meaning
resolved symbol or method route
source span in exact equality
comments, token positions, and parentheses trivia
```

An encountered deferred canonical node is explicit Unsupported, never a
desugared accepted node.

## Outcome boundary

```text
ParseOutcome:
  parser success or source.parse.* failure

SourceSnapshotOutcomeV1:
  Ready(snapshot)
  Unsupported(SourcePathV1, kind, source.snapshot.* reason)
  InvalidInput(SourcePathV1, source.snapshot.* reason)

derived consumer:
  Ready(Matched(summary))
  Ready(NotMatched)
  Unsupported(...)
  InvalidInput(...)
```

Valid canonical syntax outside the subset and all schema-limit excesses are
Unsupported. A malformed private Hako carrier is
`InternalCarrierContractViolation`; a poisoned builder is
`InternalSnapshotContractViolation`. Neither is user InvalidInput.

## Schema limits

V1 owns fresh constants; do not alias V0 constants.

```text
max_depth = 64
max_node_count = 32768
max_children_per_body = 2048
max_arguments = 128
max_local_bindings = 128
max_atom_bytes = 1024
max_literal_bytes = 65536
max_total_text_bytes = 4194304
```

All comparisons are inclusive. S0 records current-corpus measurements before
claiming the initial values are sufficient.

## Task order

### S0 — SourceAstVocabularyInventoryV1 (closed)

- Inventory every `hakorune_frontend_ast::ASTNode`, `UnaryOperator`, and
  `BinaryOperator` variant as `Accepted`, `KnownUnsupported`, or
  `ContextRequired`.
- Inventory exact Rust field names and source child ordering.
- Inventory the actual Hako parser output for every proposed accepted kind.
- Determine whether the Hako parser already preserves UnaryOp, Local versus
  Assignment, Return absence, Me/This, and MethodCall receiver/argument order
  in a private typed source carrier.
- Record current corpus depth/node/body/argument/binding/text maxima.

Acceptance:

```text
all Rust variants classified = 1
wildcard classification = 0
all proposed Hako carrier fields evidenced = 1
ProgramV0 dependency = 0
raw scan dependency = 0
```

If the Hako parser only materializes ProgramV0 or loses any required source
distinction, stop at `HAKO-SOURCE-CARRIER-DESIGN-STOP-001`. Do not begin the
snapshot schema or add token/JSON sidecars.

S0 result:

```text
ASTNode variants = 57 classified
UnaryOperator variants = 4 classified
BinaryOperator variants = 18 classified
LiteralValue variants = 7 classified
private Hako typed source carrier = 0
required Hako source distinctions preserved = 0
decision = HAKO-SOURCE-CARRIER-DESIGN-STOP-001
```

Executable inventory:

```text
tools/checks/fixtures/source_ast_vocabulary_inventory_v1.json
bash tools/checks/source_ast_vocabulary_inventory_v1_guard.sh
```

The continuation consultation is:

```text
docs/development/current/main/investigations/
mirbuilder-hako-source-carrier-design-stop-2026-07-12.md
```

Decision A is accepted. The active staged parser migration is:

```text
docs/development/current/main/investigations/
mirbuilder-hako-typed-source-carrier-v1-2026-07-12.md
```

Snapshot schema/projector work resumes only after the required typed carrier
vertical slices close.

### S1 — SourceBodyAnalysisSnapshotSchemaV1

- Add source kinds, exact operators, child roles, Local binding presence,
  Return presence, SourcePathV1, limits, outcomes, flat immutable model, and
  exact equality.
- Share only declarative vocabulary and expected fixture values.
- Keep traversal and projection implementations independent.

### S2 — Rust direct AST projector

- Project canonical `ASTNode` directly to V1.
- Generate no JSON or ProgramV0 and call no serializer helper.
- Enforce preorder ordering, budgets, complete publication, and explicit
  Unsupported outcomes.

### S3 — Hako private typed source carrier

- Close parser-owned typed source nodes for the accepted subset.
- Preserve exact kinds, operators, optional-child presence, and ordered
  children.
- Publish no raw Map, token list, Rust AST handle, or ProgramV0 carrier.

### S4 — Hako direct projector and exact parity

- Build V1 from the Hako private carrier.
- Compare exact outcome, paths, nodes, atoms, children, presence, counts,
  depth, and text budgets against the Rust projection for the same source.

### S5 — SourceScanObservationV1

- Derive condition and step observations from V1.
- Own `VarLessLength`, `AssignAddConst`, loop variable, exact step, and the
  analysis-only CondProfile projection.
- Do not store these as snapshot atoms.

### S6 — BoolPredicateScanSummaryV1

- Add one read-only Hako matcher over V1-derived observations.
- Distinguish Matched, NotMatched, Unsupported, and InvalidInput.
- Compare against the existing Rust Fact extractor on positive and negative
  fixtures.
- Keep `planner_connection = 0`.

### S7 — closeout and next consultation

- Close direct snapshot and derived Fact parity.
- Consult separately before moving planner Fact authority.
- Keep string-is-integer as the second consumer with its own parity slice.
- Keep skeleton cleanup in a separate commit with
  `fact_migration_claim = 0`.

## Required fixture families

1. Exact source distinctions, including every inequality listed above.
2. Every accepted kind, child role, binary operator, and accepted unary
   operator.
3. Bool-predicate positive cases for `me`, `this`, and named receivers.
4. Bool-predicate negative cases changing length name, step, target,
   substring bounds, arity, UnaryOp, Return value, body count, and else.
5. Limit `limit-1`, `limit`, `limit+1` cases, including ASCII, `猫`, `😸`,
   combining text, and embedded NUL.
6. Direct dual-frontend exact snapshot parity.
7. Existing Rust Fact versus Hako derived-summary parity.
8. Dependency, outcome-isolation, no-fallback, and file-size gates.

## Implementation may claim

```text
the declared source subset is observed exactly
Local and Assignment remain distinct
UnaryOp syntax is preserved without folding
Return absence, zero, and false remain distinct
MethodCall receiver and argument syntax is preserved
Rust and Hako frontends independently produce exact V1 snapshots
SourceScanObservationV1 is independently derived
BoolPredicateScanSummaryV1 matches the Rust oracle on declared fixtures
ProgramV0 schema widened = 0
V0 source-kind recovery = 0
planner connection = 0
```

## Implementation must not claim

```text
full AST support or source semantic equivalence
type inference or symbol resolution
MethodCall dispatch classification
planner Fact authority moved
route/backend/runtime authority moved
MIR parity or complete Source Selfhost
V0 immediately replaced
string-is-integer migrated before its own parity gate
skeleton cleanup proves semantic Fact migration
```

## Stop conditions

Stop if implementation attempts to:

1. widen or route V1 through ProgramV0;
2. use a Rust AST handle, Rust snapshot, or Rust carrier as HHako direct input;
3. compensate for a lossy Hako parser carrier with raw/token scanning;
4. infer UnaryOp, Assignment, Return absence, or dispatch category;
5. silently desugar CompoundAssignment;
6. place CondProfile, inferred types, symbols, MIR IDs, or spans in snapshot
   equality;
7. convert Unsupported to NotMatched/false/None/NoMatch or V0 fallback;
8. begin Fact parity before exact snapshot parity;
9. connect planner before Fact parity and a separate cutover decision;
10. widen the first subset toward a Full AST project;
11. use VM fallback for an unsupported product backend;
12. let a file reach 800 lines instead of splitting schema, path, budget,
    stmt, expr, derived observation, and Fact matcher owners.

## Retirement relationship

V0 remains for explicit ProgramV0 consumers. V1 is required by
source-sensitive consumers. A consumer names its required family explicitly.
ProgramV0-specific adapters retire only after their callers reach zero; V1
completion alone is not a V0 deletion condition.
