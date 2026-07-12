# BoundedBodyAnalysisSnapshotV0

Status: Active design consultation stop after the LocalContractWrite Fact
bridge.
Date: 2026-07-12

## Decision

Create one shared Hako analysis capability before migrating another Fact
facade. It is not a full AST owner and does not move planner authority.

```text
validated Program(JSON v0) body
-> BoundedBodyAnalysisSnapshotV0
-> read-only Fact facade
```

The frontend AST remains language semantic authority. Program(JSON v0) is the
validated transport boundary, not a second language SSOT.

## V0 schema boundary

Required envelope:

```text
schema_version
source_program_version
body
node_count
max_depth_observed
```

Closed statement subset:

```text
Local, Assignment, CompoundAssignment, Print, If, Loop, LoopRange,
Return, Break, Continue
```

Closed expression subset:

```text
Literal, Variable, UnaryOp, BinaryOp, GroupedAssignmentExpr, MethodCall,
FunctionCall, FieldAccess, Index
```

V0 excludes Lambda, Try/Catch/Throw, Task/Await, Match, ContextScope, FastMem,
declarations, general BlockExpr, QMark, and unknown future nodes.

## Three-way outcome

```text
Ready(snapshot)
Unsupported(path, node_kind, reason)
InvalidInput(path, reason)
```

Unsupported shape must never be collapsed into “no Fact”. Limits for depth,
node count, children, arguments, and literal bytes are fixed by the schema;
partial snapshots and consumer-specific limits are forbidden.

## Analysis-only API

Consumers may inspect node kind, child role/index, literal, name, operator,
body items, and diagnostic source path. They may not mutate nodes, resolve
symbols, infer types, emit MIR, allocate IDs, select routes, build Plans, or
execute runtime behavior.

## Acceptance

1. Rust canonical-AST traversal and Hako Program(JSON v0) traversal are
   independent implementations.
2. Snapshot parity is green before Fact parity is attempted.
3. Negative corpus covers quoted JSON-looking strings, escaped delimiters,
   nested If/Loop, missing/wrong fields, unknown nodes, depth/node limits, and
   unsupported Try/Lambda/Match.
4. No raw substring, token offset, or `indexOf` result reaches a Fact consumer.
5. No fallback from Unsupported to the token-only semantic facade.

## Non-claims

```text
full_ast_support = 0
fact_authority_moved = 0
planner_input = 0
route_selection_authority = 0
backend_lowering_authority = 0
mir_mutation = 0
id_allocation = 0
source_selfhost_claim = 0
```

## Stop boundary

Stop if the snapshot requires language-semantic inference, ValueId/BlockId
allocation, or a second raw-token semantic path.

## Design-stop brief

### Source authority

- Canonical source syntax and child roles: frontend `ASTNode`.
- Program transport shape: `ProgramV0` / `StmtV0` / `ExprV0` and the existing
  Program(JSON v0) producer.
- Hako may consume only a strict structured JSON parse result. Existing raw
  scanner offsets, substring matches, and `indexOf` results are not authority.

### Non-authority

- `parse_json_v0_to_module*` is a lowering entry and cannot own analysis; it
  allocates MIR/IDs and refreshes semantic metadata.
- `ProgramJsonV0ScannerBox`, phase-state scanners, and statement handlers are
  token/recipe consumers, not a general structured body owner.
- `env.console.log` must not be reverse-inferred as source `Print`, and wire
  `Local` must not be reverse-inferred as source `Local` versus `Assignment`.

### Mismatch that blocks implementation

Program(JSON v0) is not lossless for the card's canonical-AST vocabulary:

```text
Literal -> Int/Str/Bool/Null
BinaryOp -> Binary/Compare/Logical
Assignment and Local -> Local
Print -> Expr(Call env.console.log)
UnaryOp(-literal) -> folded numeric literal
Return(None) -> Return(Int(0))
CompoundAssignment / GroupedAssignmentExpr / Index -> no complete wire owner
```

Therefore source-kind parity cannot be claimed from the current transport
without either changing the snapshot vocabulary or widening Program(JSON v0).

### Fail-fast boundary

- Unknown/closed-out structured nodes return `Unsupported(path, kind, reason)`.
- Invalid JSON/envelope/field types and trailing input return
  `InvalidInput(path, reason)`.
- No empty snapshot/NoFact fallback is permitted.
- No MIR IDs, symbol resolution, type inference, route selection, or runtime
  behavior may be introduced by either option.

### Candidate slices

1. `wire_vocabulary`:
   define V0 over exact `StmtV0` / `ExprV0` transport kinds and explicitly
   normalize the Rust AST oracle to that lossy wire view. This does not widen
   Program(JSON v0), but it must not claim source-kind preservation.
2. `source_provenance_discriminator`:
   first add lossless source-kind provenance to Program(JSON v0), then retain
   the card's current canonical-AST vocabulary. This changes the transport
   contract and requires its own compatibility/consumer inventory.

### Recommended next slice

Choose `wire_vocabulary` for V0. It keeps the snapshot analysis-only, avoids a
Program(JSON v0) schema widening, and makes the Hako side a strict structured
transport reader. Revise the closed subset and parity claim to exact wire
kinds before implementation. Keep source-provenance parity as a separate
future transport decision.

Before implementation, the accepted option must also fix schema-owned values
for maximum depth, node count, body children, arguments, literal bytes, path
grammar, depth/node counting, and null-child treatment.

### Explicit non-claims at this stop

```text
implementation_started = 0
program_json_schema_widened = 0
source_kind_parity = 0
fact_authority_moved = 0
planner_input = 0
raw_token_fallback = 0
mir_or_id_allocation = 0
source_selfhost_claim = 0
```
