# BoundedBodyAnalysisSnapshotV0

Status: Active implementation task; `wire_vocabulary` accepted after design
consultation.
Date: 2026-07-12

## Decision

Create one shared Hako analysis capability before migrating another Fact
facade. It is not a full AST owner and does not move planner authority.

`BoundedBodyAnalysisSnapshotV0` is a lossy, removable wire observational
quotient over Program(JSON v0), not a reduced source AST:

```text
source node A ~ source node B
iff their accepted Program(JSON v0) observations are equal
```

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

Closed wire statement subset:

```text
Local, Expr, If, Loop, LoopRange, Return, Break, Continue
```

Closed wire expression subset:

```text
Int, Str, Bool, Null, Var, Binary, Compare, Logical, Call, Method, Field
```

Closed operators:

```text
Binary:  + - * / % & | ^ << >>
Compare: == != < > <= >=
Logical: && ||
```

V0 intentionally does not preserve source declaration/assignment, Print,
UnaryOp, Return(None), source spans, inferred types, resolved symbols, or MIR
identity. `Local.declared_type` is known-but-unobserved. Unknown tags are
InvalidInput; known current wire variants outside this subset are Unsupported.

Before implementation, `ProgramV0WireContractInventoryV0` must classify every
producer-emittable and consumer-decodable shape as Accepted,
KnownUnsupported, or SchemaMismatchStop. `Float`, `FastMemRegion`, tolerated
extra fields, and `Local.declared_type` are explicit seam checks.

## Schema constants

All limits are inclusive and schema-owned:

```text
max_depth = 64
max_node_count = 32768
max_children_per_body = 2048
max_arguments = 128
max_literal_bytes = 65536
max_atom_bytes = 1024
max_total_text_bytes = 4194304
```

The root body container is depth 0 and is not a node. Concrete Stmt/Expr nodes
count once in preorder; list containers and absent optional children do not
count. Decoded UTF-8 bytes are measured. Exceeding a limit is Unsupported.

Paths are traversal-generated, zero-based, and never contain user text:

```text
$.body[2].then[1].expr.args[0]
```

Traversal order is preorder and schema-fixed: condition/value children before
ordered bodies/arguments; binary-like nodes visit lhs then rhs; Method visits
recv then args; Field visits recv.

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

Also stop on an unresolved producer/consumer seam, a need for source
provenance, an unclassified new ProgramV0 variant, a strict parser that cannot
detect duplicate keys/trailing input, or any attempt to expose a partial
snapshot.

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

## Accepted consultation decision

Decision: `wire_vocabulary`.

The Rust oracle is verification-only: it projects canonical AST directly to
snapshot algebra without generating/parsing Program JSON or importing the
authoritative serializer. The Hako reader consumes only a strict structured
ProgramV0 body view. They share declarative schema vocabulary, limits, paths,
operators, and equality, but not source-to-wire branching implementation.

Loss equivalences are contractual V0 behavior:

```text
source Local        ~ source Assignment
source Print        ~ ordinary env.console.log Call
UnaryOp(-, Int(1))  ~ Int(-1)
Return(None)        ~ Return(Int(0))
uninitialized Local ~ Local initialized with Null
```

Source-aware distinctions, if later required by a real Fact consumer, belong
in a separate `SourceBodyAnalysisSnapshotV1` decision. Do not add optional
source provenance to V0.

## Task order

### S0 — ProgramV0WireContractInventoryV0 (active)

- inventory producer-emittable shapes and consumer-decodable shapes;
- classify every StmtV0/ExprV0 variant with no wildcard;
- classify fields as known-and-observed, known-but-unobserved, or
  forbidden-unknown;
- resolve or stop on `Float`, `FastMemRegion`, `Local.declared_type`, duplicate
  keys, trailing input, and tolerated extra fields;
- add one reusable inventory fixture/guard; no snapshot implementation yet.

Acceptance: the producer/consumer intersection and every mismatch are
machine-checkable, and a new unclassified variant makes the guard fail.

### S1 — SnapshotSchemaV0

- immutable kinds, child roles, scalar encodings, operator sets;
- limits, reason codes, structural PathV0, and exact snapshot equality;
- `Ready` / `Unsupported` / `InvalidInput` outcome types;
- no AST, ProgramJSON producer, MIR, planner, route, backend, or runtime import.

### S2 — Strict structured ProgramV0 body view

- strict full-input JSON parse with duplicate-key detection;
- version/kind/envelope and field-type validation;
- no raw scanner, substring tag detection, token offset, or fallback;
- excluded known variants remain distinguishable from malformed/unknown input.

### S3 — Hako ProgramV0 snapshot reader

- split schema/outcome/path/budget/model/builder/stmt/expr responsibilities;
- publish an immutable snapshot only after full traversal succeeds;
- no input mutation or partial publication;
- keep every `.hako` source below 800 lines.

### S4 — Rust AST wire-observation oracle

- test/parity-only independent implementation;
- direct AST-to-snapshot projection with no JSON generation/reparse;
- no serializer helper import or product caller;
- context-sensitive enum/brand/typed-array/record/dynamic-call shapes are
  Unsupported until structurally proven.

### S5 — Fixture packs

- every accepted kind, child role, and operator;
- loss-equivalence pairs;
- Unsupported at root and nested cond/body/rhs/args/recv positions;
- InvalidInput for missing/wrong/unknown/null/duplicate/out-of-range cases;
- every limit at limit-1, limit, and limit+1 including decoded multibyte text.

### S6 — Exact parity and isolation gates

- Rust oracle snapshot equals authoritative serializer output read by Hako;
- compare snapshot structure, atoms, and child order, not JSON text;
- dependency guards prohibit serializer/MIR/planner imports in the oracle and
  MIRBuilder/route/runtime/raw scanner use in the Hako reader;
- current corpus yields only Ready+parity or explicit Unsupported; no skip.

### S7 — One read-only consumer

- connect `LoopFeatureSummaryV0` as observation/parity only;
- preserve all three outcomes unchanged;
- no planner/route/backend/runtime connection.

### S8 — Follow-up boundary

- token-only facade retirement is a separate slice after snapshot parity;
- source-level distinctions require a new V1 decision;
- Program(JSON v0) provenance widening requires a separate transport decision.

## Implementation claims

May claim only bounded structural observation and exact parity for the accepted
wire subset. Must not claim full AST support, source-kind preservation,
semantic equivalence, complete ProgramV0 support, MIR/planner parity, route or
runtime authority, Program(JSON v0) permanence, or Source Selfhost completion.
