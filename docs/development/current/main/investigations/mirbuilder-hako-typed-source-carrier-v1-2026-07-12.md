# Hako Typed Source Carrier V1 — Staged Parser Migration Taskboard

Status: Active; P0 closed and P1 Return vertical slice selected.
Date: 2026-07-12
Decision: `A — typed_parser_core_with_sealed_source_arena_and_compat_projection`

## Target architecture

```text
source text
  -> one authoritative Hako parser
  -> explicit ParserNodeProductV1
  -> parser-private SourceCarrierBuilderV1
  -> sealed HakoSourceTreeV1
       +-> SourceBodyAnalysisSnapshotV1
       +-> SourceTreeToProgramV0Projector
```

The migration is vertical and branch-by-branch. It is not a wholesale parser
return-type rewrite.

## Authority

```text
source grammar:
  canonical grammar registry
  authoritative Hako parser branch logic
  authoritative Rust frontend parser

Hako source-node construction:
  ParserSourceNodeFactoryV1
  SourceCarrierBuilderV1

sealed parser-private source structure:
  HakoSourceTreeV1

ProgramV0 compatibility mapping:
  SourceTreeToProgramV0Projector

source snapshot vocabulary:
  SourceBodyAnalysisSnapshotSchemaV1

derived analysis:
  SourceScanObservationV1
  BoolPredicateScanSummaryV1
```

The parser branch owns source kind at the moment it recognizes syntax.
ProgramV0 never owns or recovers that kind.

## Non-authority

```text
ProgramV0 JSON/tag/fragment
old JSON-returning helper result
raw MapBox/token sequence/source scan
hidden ctx.last_node side channel
Rust AST handle or Rust-generated carrier replay
V0 snapshot
MIRBuilder/planner/route/backend/runtime
inferred type/resolved symbol/dispatch category
```

## Closed initial carrier vocabulary

```text
root:
  SourceBody

statements:
  Loop, If, Local, Assignment, Return, Break, Continue

expressions:
  Literal(Int, String, Bool, Null)
  Variable, Me, This
  UnaryOp, BinaryOp, MethodCall
```

The arena uses invocation-local `SourceNodeRefV1` indices. Optional values use
explicit presence enums, never null, zero, empty text, or other sentinels.

```text
ReturnValuePresenceV1 = Absent | Present(node)
InitializerPresenceV1 = Absent | Present(node)
ElsePresenceV1 = Absent | Present(list)
```

## ParserNodeProductV1

Every migrated parser helper returns an explicit product:

```text
Typed {
  node: SourceNodeRefV1,
  next_pos: i64
}

CompatOnly {
  branch_id: ParserBranchIdV1,
  source_kind: SourceKindTagV1,
  compat_fragment: String,
  next_pos: i64
}

ParseError {
  code,
  diagnostic
}
```

Rules:

```text
ProgramV0 entry:
  Typed -> compat projector
  CompatOnly -> temporary legacy fragment

SourceTreeV1 entry:
  Typed -> continue
  CompatOnly -> source.parser_branch_unmigrated + poison

migrated parent:
  all semantic children Typed -> build typed parent
  any CompatOnly child -> never publish a mixed typed/legacy tree
```

V1 never reads `compat_fragment`. Branch dispatch supplies `source_kind`; it
is never inferred from the fragment.

## Builder and lifetime contract

```text
state = Open | Poisoned | Sealed

Open:
  parser-private node/list construction only

Poisoned:
  parse error, CompatOnly in V1 mode, invalid ref, invariant failure
  no append/finish/publication

Sealed:
  exactly one complete root
  immutable read-only access
```

Bottom-up construction is mandatory:

```text
children sealed before parent
child_id < parent_id
one invocation -> one builder -> zero or one sealed tree
global registry = 0
raw pointer = 0
partial tree publication = 0
```

Lists may be mutable only inside the builder. No mutable list alias escapes;
node sealing freezes referenced lists; finish performs defensive
reconstruction and reachability validation.

## Failure domains

```text
SourceParseError:
  invalid syntax / grammar-profile violation

SourceCarrierUnsupported:
  valid branch outside typed migration subset

InternalCarrierContractViolation:
  invalid ref/family/list, cycle, double seal, mutation after seal

ProgramV0CompatProjectionViolation:
  migrated node lacks a declared compat mapping or schema drifts
```

Facade mapping:

```text
source.parse.* -> InvalidInput($.source, ...)
typed subset outside support -> Unsupported(SourcePathV1, kind, ...)
internal carrier failure -> internal freeze, not user InvalidInput
compat projection failure -> compatibility freeze, not reader outcome
```

No outcome may fall back to V0, old JSON parsing, false, None, NoMatch, or an
empty tree/snapshot.

## ProgramV0 compatibility boundary

Migrated source nodes have exactly one ProgramV0 mapping owner:

```text
SourceTreeToProgramV0Projector
```

Initial explicit lossy mappings:

```text
UnaryOp(Neg, rhs) -> Binary(Int(0), Subtract, rhs)
Return(Absent) -> Return(Int(0))

later:
  Assignment(variable, value) -> Local(name, value)
  Local(binding without initializer) -> Local(name, Int(0))
  Me / This / Variable -> Var
  Print -> existing console form
```

For every migrated branch, direct JSON construction is deleted in the same
commit. Unmigrated direct JSON branches remain exhaustively classified as
`CompatOnly` and are temporary.

## Task order

### P0 — construction substrate (closed)

- Add small parser-private modules for:
  - closed node/branch/presence enums;
  - `SourceNodeRefV1` and ordered list references;
  - immutable node records;
  - `ParserNodeProductV1`;
  - Open/Poisoned/Sealed builder lifecycle;
  - failure domains.
- Add no parser branch migration yet.
- Prove invalid refs, wrong child family, list mutation after node seal,
  incomplete root, unreachable node, cycle attempt, poison, and double finish.
- Keep every source file below 800 lines and compile each family separately.

P0 acceptance:

```text
parser behavior changed = 0
ProgramV0 output changed = 0
typed branch count = 0
partial publication = 0
raw Map/JSON carrier = 0
```

P0 landed structure:

```text
lang/src/compiler/parser/source_carrier_v1/
  source_vocabulary_v1.hako
  source_refs_v1.hako
  source_records_v1.hako
  parser_node_product_v1.hako
  source_carrier_outcome_v1.hako
  source_carrier_builder_v1.hako
  source_carrier_sealer_v1.hako
```

The builder owns mutation and Open/Poisoned/Sealed transitions. The separate
sealer owns bottom-up validation, complete node/list reachability, defensive
reconstruction, and one-shot immutable publication. This split was required
after the first combined reachability loop exceeded the accepted JoinIR loop
shape; no new accepted compiler shape was added.

Executable gate:

```text
bash tools/checks/hako_parser_source_carrier_p0_guard.sh
```

Closeout evidence:

```text
release compile build_module/semantic_refresh = green under 10 seconds
VM-reference lifecycle fixture = RC 0
common Hako compile-shape matrix = green
Language-v1 grammar contract substrate = green
parser branch connection = 0
ProgramV0 behavior change = 0
typed branch count = 0
partial publication = 0
all source files < 800 lines
```

### P1 — Return presence vertical slice (active)

Accepted fixtures:

```text
return
return 0
return -1
```

Migrate only:

```text
SourceBody
Literal(Int)
UnaryOp(Neg)
Return(Absent | Present)
```

Requirements:

1. integer parser returns Typed node;
2. unary minus returns `UnaryOp(Neg)` rather than Binary;
3. Return preserves explicit presence;
4. compat projector reproduces existing ProgramV0;
5. V1 proves all three source trees differ;
6. every other V1 branch is exact
   `source.parser_branch_unmigrated`;
7. migrated legacy JSON constructors are removed in the same commit.

### P2 — binding distinction

```text
Variable
Local ordered bindings
initializer presence
Assignment with Variable target
```

Preserve multi-binding order. Typed Local remains explicit Unsupported until
its type-syntax authority opens.

### P3 — receiver distinction

```text
Me
This
Variable
MethodCall(receiver, method, ordered arguments)
```

Do not infer dispatch category.

### P4 — expression algebra

- Migrate all four UnaryOperator rows and all eighteen BinaryOperator rows.
- Prove exact precedence/associativity parity and no folding/desugaring before
  the carrier.

### P5 — control structure

```text
If(condition, then, optional else)
Loop(condition, body)
Break
Continue
```

Preserve schema child order and optional presence.

### P6 — SourceSnapshotV1 dual-frontend parity

```text
same source
  -> Rust parser/AST/projector
  -> Hako parser/sealed tree/projector
  -> exact snapshot equality
```

Compare parse outcome, kinds, atoms, operators, optional presence, ordered
children, paths, limits, and final immutable snapshot.

### P7 — SourceScanObservationV1

Derive loop condition/step observations from V1. Do not store CondProfile or
ScanConditionObservation as source atoms.

### P8 — BoolPredicateScanSummaryV1

Compare the Hako read-only summary with the existing Rust Fact oracle.

```text
planner_connection = 0
route_connection = 0
```

A separate consultation is required before Fact authority cutover.

## Required gates

1. Source distinctions:
   Unary Neg vs Binary subtraction; Local absence vs zero; Local vs
   Assignment; Return absent/zero/false; Me/This/Variable.
2. Explicit ProgramV0 loss equivalence for mappings intended to collapse.
3. Migrated branch has zero direct JSON construction.
4. CompatOnly isolation at top-level and nested positions.
5. V1 dependency prohibition on compat fragments, JSON scanning, and
   ProgramV0 tags.
6. Complete builder poison/seal/reachability/cycle lifecycle.
7. ProgramV0 corpus regression: byte-exact where text is contractual,
   otherwise strict structured equality.
8. Direct Rust/Hako source snapshot parity.
9. Compile-shape and source-file size under fixed release boundary.
10. Exhaustive parser branch inventory:
    `TypedProjected | CompatOnly | RejectedByCanonicalGrammar`, no wildcard.
11. Carrier dependency isolation from MIRBuilder/planner/route/backend/runtime.
12. Planner/route nonconnection through P8.

## Implementation may claim

After P1 only:

```text
Return presence and unary-minus syntax are preserved for the accepted slice
ProgramV0 output remains compatible for those fixtures
migrated ProgramV0 output is projected from typed nodes
V1 does not consume legacy fragments
partial source tree publication = 0
```

After P6 only:

```text
accepted V1 subset has independent Rust/Hako parse-and-snapshot parity
Local/Assignment, Me/This/Variable, Unary syntax, and Return forms differ
```

## Implementation must not claim

```text
full parser/AST migration or all 57 variants
ProgramV0 fully projector-owned while CompatOnly remains
public reusable Hako AST API
source semantic equivalence/type inference/symbol resolution
MethodCall dispatch classification
planner/route/backend/runtime authority moved
Source Selfhost complete
CompatOnly cleanup is Fact migration
```

## Stop conditions

Stop if implementation:

1. recovers source kind from ProgramV0;
2. adds ProgramV0 provenance fields;
3. exposes raw Map/JSON/token text as a node;
4. uses hidden parser side channels instead of explicit products;
5. constructs typed node and JSON independently for a migrated branch;
6. accepts or inspects CompatOnly in V1;
7. seals a mixed typed/legacy tree;
8. retries typed failure through the old parser;
9. normalizes Local/Assignment, Unary, Return, Me/This before carrier creation;
10. lets compat mapping flow back into parser core;
11. stores type/symbol/MIR/route meaning in the carrier;
12. maps internal failure to user InvalidInput;
13. adds wildcard parser-branch classification;
14. grows CompatOnly without a recorded reason;
15. uses environment-selected parser mode or silent fallback;
16. exceeds the fixed compile boundary or 800-line source limit;
17. connects Fact before snapshot parity or planner before Fact parity.

## Retirement path

```text
Phase 1: TypedProjected + CompatOnly coexist
Phase 2: migrate branches and reduce CompatOnly to zero
Phase 3: remove CompatOnly, legacy fragment field, direct JSON helpers
Phase 4: ProgramV0 produced only from sealed HakoSourceTreeV1
Phase 5: after ProgramV0 callers reach zero, remove compat projector/transport
```

The parser-private source carrier remains as the source-preserving internal
representation. Promotion to a public AST API requires a separate Decision.
