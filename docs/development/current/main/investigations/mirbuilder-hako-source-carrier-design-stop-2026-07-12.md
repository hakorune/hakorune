# Hako Source AST Carrier — Design Stop

Status: Closed; staged typed-parser-core Decision A accepted.
Date: 2026-07-12
Blocker: `HAKO-SOURCE-CARRIER-DESIGN-STOP-001`

## Accepted decision

```text
A — typed_parser_core_with_sealed_source_arena_and_compat_projection
```

The implementation taskboard is:

```text
docs/development/current/main/investigations/
mirbuilder-hako-typed-source-carrier-v1-2026-07-12.md
```

Parser helpers return explicit `ParserNodeProductV1` values. Migrated branches
construct parser-private typed arena nodes and emit ProgramV0 only through the
compat projector. Unmigrated branches are isolated as `CompatOnly`; they can
serve only the ProgramV0 entry and poison V1 publication.

## Decision requested

Choose the authoritative Hako parser architecture needed before
`SourceBodyAnalysisSnapshotV1` S1-S4 can proceed.

```text
A. typed_parser_core_with_program_v0_projection
   Change the existing Hako parser core to construct private typed source
   nodes first, then project ProgramV0 from those nodes at the compat boundary.

B. independent_analysis_source_parser
   Add a second bounded Hako source parser used only by SourceSnapshotV1.

C. rust_ast_opaque_bridge
   Share the Rust canonical AST through an opaque structured accessor and let
   Hako project V1 from it.

D. stop_source_snapshot_v1
   Park V1 and return to a different workstream.
```

Preliminary recommendation: A, but only after defining a bounded migration
that does not require a monolithic parser rewrite. B duplicates grammar and
parser policy. C cannot prove direct dual-frontend parity and contradicts the
accepted V1 mode. D is valid if the parser restructuring cost is not currently
justified.

## Closed S0 evidence

Generated inventory:

```text
tools/checks/fixtures/source_ast_vocabulary_inventory_v1.json
```

Guard:

```text
bash tools/checks/source_ast_vocabulary_inventory_v1_guard.sh
```

Counts:

```text
ASTNode = 57
UnaryOperator = 4
BinaryOperator = 18
LiteralValue = 7
```

Rust canonical AST can represent the accepted V1 source distinctions. The
current Hako parser cannot.

## Current Hako parser architecture

```text
source text
  -> ParserBox / ParserProgramBox
  -> statement and expression parser functions
  -> JSON fragment strings
  -> Program(JSON v0) string
```

There is no ProgramV0-independent private typed source node carrier between
parsing and serialization.

Concrete losses before any V1 projector can observe the result:

```text
source Unary Minus       -> Binary(Int(0), Subtract, rhs)
source Assignment        -> ProgramV0 Local
source local without init-> Local initialized with Int(0)
source Return(None)      -> Return(Int(0))
source Me / This / name  -> Var(name)
```

MethodCall receiver and argument order survive, but only inside the JSON
fragment string. That is not an accepted private typed source carrier.

Primary anchors:

```text
lang/src/compiler/parser/program/parser_program_box.hako
lang/src/compiler/parser/stmt/parser_stmt_box/core.hako
lang/src/compiler/parser/stmt/parser_stmt_box/local_stmt.hako
lang/src/compiler/parser/expr/parser_expr_precedence_box.hako
lang/src/compiler/parser/expr/parser_expr_box.hako
```

## Required authority decision

The consultation must fix:

1. the owner of Hako source node construction;
2. whether ProgramV0 becomes a projection from typed nodes or remains the
   parser's direct output;
3. the smallest migration unit that preserves current parser behavior while
   adding exact source kinds;
4. how recursive nodes are represented without raw MapBox or JSON strings;
5. optional-child and ordered-list representation;
6. parser failure versus carrier-contract failure;
7. whether the typed carrier is parser-private or reusable;
8. how current ProgramV0 callers remain compatible during migration;
9. independent Rust/Hako parser parity evidence;
10. compile-shape and file-size boundaries for Hako modules.

## Constraints inherited from V1

Any accepted design must preserve:

```text
ProgramV0 schema widening = 0
ProgramV0 source-kind inference = 0
raw source/token scanner = 0
Rust AST handle as direct HHako input = 0
Rust snapshot/carrier replay as parity authority = 0
planner/route/backend/runtime connection = 0
Unsupported fallback = 0
```

The private carrier must directly retain at least:

```text
UnaryOp exact operator and operand
Local statement and ordered binding initializer presence
Assignment statement and Variable target
Return value absence/presence
Me / This / Variable node distinction
MethodCall receiver, method, and ordered arguments
If and Loop ordered bodies
```

## Option assessment

### A — typed parser core with ProgramV0 projection

Potential shape:

```text
source text
  -> parser-owned SourceNodeV1 factories
  -> private immutable typed tree
       +-> SourceSnapshotV1 projector
       +-> ProgramV0 compat serializer
```

Benefits:

```text
one Hako grammar/parser authority
source distinctions preserved before lossy projection
ProgramV0 becomes visibly removable compat output
direct dual-frontend parity remains possible
```

Risks requiring a bounded plan:

```text
large parser return-type migration
JSON-string assumptions across recursive helpers
compile-time growth from recursive Hako types/factories
temporary dual-output truth
```

### B — independent analysis source parser

This can be bounded to the first Fact subset, but it duplicates lexical,
precedence, statement, optional-child, and grammar-profile policy. It must not
be selected merely because it is smaller to implement.

### C — Rust AST opaque bridge

This could share syntax substrate similarly to the strict JSON arena, but it
would prove only independent Hako traversal after Rust parsing. It does not
satisfy the accepted `direct_dual_frontend_ast_projection_v1` claim unless the
decision explicitly changes that claim.

### D — park V1

No implementation follows. V0 remains bounded and lossy; bool-predicate and
string-is-integer Fact migration remain blocked.

## Required response format

```text
Decision: A | B | C | D

Canonical authority
Non-authority
Carrier node vocabulary and ownership
Parser-to-carrier construction boundary
ProgramV0 compatibility projection boundary
Mutation / sealing / lifetime rules
Failure and outcome domains
Rust/Hako independence claim
Shared declarative schema
Required fixtures and gates
Smallest implementation slice
Migration order
Retirement path
Implementation may claim
Implementation must not claim
Stop conditions
```

For A, explicitly answer whether parser helpers return typed nodes immediately
or use a temporary builder/observation interface, and how mixed migrated and
unmigrated parser branches fail fast without reconstructing source kinds from
ProgramV0.

## Stop conditions

Do not implement while this decision is open. Stop any proposal that:

1. recovers source kind from existing ProgramV0 JSON;
2. adds provenance fields to ProgramV0;
3. makes raw MapBox/JSON/token text the typed carrier;
4. uses the Rust AST as HHako direct parity input without changing the parity
   claim through an explicit decision;
5. duplicates the full parser without a grammar-drift prevention contract;
6. normalizes Local/Assignment, Unary syntax, or Return presence before V1;
7. publishes partially built source trees;
8. silently falls back from typed parsing to the old JSON parser;
9. connects snapshot or derived summaries to planner;
10. creates a parser/source file at or above 800 lines.

## Parked parallel task

The zero-new-syntax low-level fast-path inventory and task order are parked at:

```text
docs/development/current/main/investigations/
low-level-fast-path-v0-task-2026-07-12.md
```

It does not change this card's active blocker or authorize implementation in
the direct-memory/backend lane.
