# Function-exit F1 RETURN0 S0

Decision authority: `FUNCTION-EXIT-SEMANTICS-prime-r1`

Status: active executable task.

First executable row:

```text
FUNCTION-EXIT-F1-RETURN0-S0
```

Normative SSOT:

- `docs/reference/language/function-exit-and-entry-result.md`
- `docs/reference/language/semantic-kernel.md`
- `docs/reference/language/types.md`

Current implementation owners:

- `src/mir/resolved_control_flow/function_control.rs`
  - `VerifiedFunctionCompletionV1`
  - `verify_function_completion_v1`
- `src/mir/function/types.rs`
  - `ReturnExitContract`
- `src/mir/type_contracts/return_exit.rs`
  - the existing exact-numeric return-contract refresh/validation owner

## Objective

Make the accepted F1 rule executable for the exact completion topology already
admitted by `VerifiedFunctionCompletionV1`:

```text
ordinary function / method =
  explicit Return only

ordinary fallthrough =
  Unit

last statement-lowering ValueId =
  never function-return authority
```

This row strengthens the relation among the existing source completion owner,
the declared result contract, and the existing MIR `ReturnExitContract`
carrier. It must not create a second AST walk, completion analyzer,
annotation-contract producer, or physical Return owner.

The current admitted topology remains deliberately narrow:

```text
zero resolved Return exits
or
one resolved Return exit at the exact root-body terminal statement
```

Nested returns, multiple returns, branch coverage, cleanup-bearing all-path
coverage, and general CFG completion are later design/capability rows.

## Current evidence

`verify_function_completion_v1` already owns the sole pre-Builder completion
decision for the current family:

```text
zero exits
  -> VerifiedFunctionCompletionV1::ImplicitVoid

one exact root-terminal Return
  -> VerifiedFunctionCompletionV1::ExplicitReturn

nested / multiple / nonterminal Return
  -> typed pre-Builder rejection
```

`ReturnExitContract` already owns the currently active exact-numeric
annotation runtime-check carrier. It is refreshed from
`MirFunction.metadata.declared_return_type_name` and validated by the existing
type-contract owner.

The missing relation is semantic, not another lowering route:

```text
Verified source completion
+ exact declared result contract
+ F1 disposition
+ expected relation to the existing ReturnExitContract carrier
```

## Authority lock

### Sole topology and completion producer

`verify_function_completion_v1` remains the only producer of callable
completion truth for this slice.

The F1 contract is co-sealed by that function while it already owns:

```text
FunctionOwnerIdV1
resolved function region
root body site
resolved Return exits
exact root terminal statement
source function declaration
```

Forbidden:

```text
second AST traversal
second resolved-exit collection
Builder-side source reclassification
MIR terminator scan as source authority
last ValueId / signature / symbol inference
Raw recipe or physical-main route inference
```

### Existing return-contract carrier

`ReturnExitContract` remains the sole executable MIR carrier for the currently
supported exact-numeric annotation check. S0 does not clone or replace it.

The new semantic contract records the source-side expectation needed to relate
the verified completion to that existing carrier:

```text
unannotated
  -> no declared source result contract

declared void
  -> Unit result contract

declared non-void
  -> value result contract

currently supported exact numeric annotation
  -> existing ReturnExitContract carrier required later

other annotated result
  -> declared semantic contract retained;
     physical/backend capability remains a later decision
```

The relation is checked without manufacturing a second
`ReturnExitContract`. Full physical consumption belongs to
`FUNCTION-EXIT-F1-MATERIALIZE0-S0`.

## S0 product

Extend the existing verified completion product in place. The exact names may
follow local module style, but there must be one nested semantic seal and one
producer.

Conceptual shape:

```rust
pub(crate) struct SealedFunctionExitContractV1 {
    owner: FunctionOwnerIdV1,
    declared_result: DeclaredFunctionResultContractV1,
    disposition: SealedFunctionExitDispositionV1,
    coverage: FunctionExitCoverageV1,
    return_contract_relation: ReturnExitRelationV1,
    _seal: SealedFunctionExitContractSealV1,
}

pub(crate) enum DeclaredFunctionResultContractV1 {
    Unannotated,
    Void,
    Annotated(Box<str>),
}

pub(crate) enum SealedFunctionExitDispositionV1 {
    ExplicitValue {
        site: SourceStmtSiteV1,
    },
    ExplicitUnit {
        site: SourceStmtSiteV1,
        origin: FunctionUnitOriginV1,
    },
    ImplicitUnit {
        body: SourceBodySiteV1,
        body_end: u32,
        origin: FunctionUnitOriginV1,
    },
}

pub(crate) enum FunctionUnitOriginV1 {
    EmptyBody,
    ImplicitFallthrough,
    ExplicitVoid,
    ExplicitNull,
    BareReturn,
}

pub(crate) enum FunctionExitCoverageV1 {
    ExactZeroExitRootBody,
    ExactOneTerminalRootReturn,
}
```

`BareReturn` is target vocabulary only in this row. Existing direct AST
fixtures may exercise the already representable `Return { value: None }`
shape, but S0 does not change parser, grammar registry, EBNF admission, or
public producer behavior. Canonical grammar activation remains
`FUNCTION-EXIT-BARE-RETURN-GRAM0`.

`VerifiedFunctionCompletionV1` must own or expose this seal without creating a
parallel completed-function product:

```text
VerifiedFunctionCompletionV1
  -> completion topology
  -> cleanup obligations
  -> one SealedFunctionExitContractV1
```

## Exact semantic matrix

### No explicit Return

```text
empty root body
  -> ImplicitUnit(origin = EmptyBody)

non-empty body without Return
  -> ImplicitUnit(origin = ImplicitFallthrough)

last Expr / Print / Local / Assignment / CompoundAssignment
  -> still ImplicitUnit
  -> no value-return promotion
```

This is the key F1 correction. The final source statement and any ValueId its
lowerer may later produce are not return authority.

### One root-terminal Return

```text
return <non-void expression>
  -> ExplicitValue

return void/null
  -> ExplicitUnit(origin = ExplicitVoid / ExplicitNull)

bare return AST shape
  -> ExplicitUnit(origin = BareReturn)
  -> no parser/grammar activation claim
```

Classification of `return void` and `return null` must observe the exact source
form; both are Unit at this boundary, but their provenance remains distinct.
It must not treat every `Some(expr)` as a value-return merely because the AST
has a payload.

### Declared result relation

S0 may reject only relations that are definite from the sealed source shape:

```text
declared non-void + empty/fallthrough Unit
  -> MissingReturnValueOnPath

declared non-void + explicit void/bare Unit
  -> MissingReturnValueOnPath

declared void + exact non-void literal Return
  -> ReturnContractMismatch

declared void + non-literal Return expression
  -> retain ExplicitValue and defer exact value/type checking

unannotated + explicit value or Unit
  -> accepted source contract
```

For a general return expression whose runtime value is not statically known,
S0 retains the declared contract and defers exact value/type checking to the
existing return-exit owner and later materialization/capability rows. It must
not invent source-level type inference.

## Typed failure and retention

Extend `FunctionCompletionVerificationErrorV1` only where the semantic
relation needs a new exact cause. Reuse existing topology errors:

```text
UnsupportedExitCardinality
UnsupportedExitSite
NonTerminalReturn
WrongSourceRegion
WrongExitOrigin
WrongTransferKind
WrongFunctionTarget
```

New relation errors may include:

```text
DeclaredResultSourceMismatch
MissingReturnValueOnPath
ReturnContractMismatch
ReturnExitRelationDrift
UnsupportedCompletionCoverage
```

The error retains the current owner boundary by returning before any Builder,
MIR, or runtime mutation. Do not flatten the nested cause into an unstructured
string at the producer.

No rejection path may:

```text
retry
fallback
re-run source resolution
enter Legacy build_module
repair a signature
install a physical Return
change Main / Script / process-entry policy
```

## Implementation boundary

Preferred source delta:

```text
src/mir/resolved_control_flow/function_control.rs
  extend the existing completion product and sole producer

src/mir/resolved_control_flow/function_control_tests.rs
  focused F1 / declaration / topology matrix

src/mir/type_contracts/return_exit.rs
  only a narrow borrowed relation helper if required

src/mir/type_contracts/return_exit/tests.rs
  prove the existing carrier remains sole and exact

src/mir/resolved_control_flow/mod.rs
  only if a small sibling module is necessary
```

If `function_control.rs` would become unclear, a small sibling
`function_exit_contract.rs` is allowed only as vocabulary/validation owned by
`function_control.rs`. It must have no public constructor, no independent AST
entry, and no second producer.

Do not add this work to `if_control.rs`; nested/all-path coverage is outside
S0 and that file is already near the per-file boundary.

## Implementation order

```text
RETURN0-CONTRACT0
  add the nested F1 semantic vocabulary
  retain exact declared annotation text
  keep ReturnExitContract as the existing MIR carrier

RETURN0-COSEAL0
  issue the semantic seal inside verify_function_completion_v1
  distinguish empty/fallthrough/explicit void/explicit value
  preserve existing exact 0-or-1 topology rejection

RETURN0-RELATION0
  seal the source expectation for the existing ReturnExitContract
  add no second contract producer or Builder consumer

RETURN0-P0
  focused positive/negative fixtures
  source-owner and annotation-relation checks

RETURN0-G0
  extend the reusable resolved-region-flow authority guard
  add no one-row shell wrapper
```

## Promotion-blocking fixture matrix

Positive:

```text
unannotated empty body
  -> ImplicitUnit / EmptyBody

unannotated final expression statement
  -> ImplicitUnit / ImplicitFallthrough
  -> never ExplicitValue

unannotated final Print / Local / Assignment / CompoundAssignment
  -> ImplicitUnit

unannotated terminal return Integer
  -> ExplicitValue

unannotated terminal return void
  -> ExplicitUnit / ExplicitVoid

unannotated terminal return null
  -> ExplicitUnit / ExplicitNull

declared void + empty/fallthrough
  -> Unit contract

declared void + return void
  -> Unit contract

declared void + return null
  -> Unit contract

declared void + non-literal return expression
  -> ExplicitValue; exact relation deferred

declared exact numeric + terminal value return
  -> value disposition
  -> existing exact-numeric ReturnExitContract relation required
```

Negative:

```text
declared non-void + empty body
declared non-void + ordinary fallthrough
declared non-void + return void
declared void + exact non-void literal return
nonterminal root return
nested return
multiple returns
wrong owner/region/transfer relation
```

Non-activation:

```text
bare-return parser/grammar producer delta = 0
nested/multiple/all-path success producer = 0
Builder physical Return consumer = 0
Script-tail classifier = 0
Main/physical-entry special semantics = 0
```

Existing resolved completion, resolved value-profile return, and Builder
completion tests remain green without changing their physical behavior.

## Gates

Run the focused owner and consumer tests first:

```bash
cargo test -q mir::resolved_control_flow::function_control_tests
cargo test -q mir::resolved_value_profile::return_tests
cargo test -q mir::builder::resolved_lowering::completion_tests
cargo test -q mir::type_contracts::return_exit::tests
cargo check -q
```

Run the reusable authority guard:

```bash
bash tools/checks/resolved_region_flow_authority_guard.sh
```

If new structural assertions are required, extend that reusable guard or its
existing library implementation. Do not add a dedicated
`function-exit-f1-return0-s0-guard.sh` wrapper.

## Guard contract

```text
verify_function_completion_v1 production definition          = 1
VerifiedFunctionCompletionV1 production producer              = 1
SealedFunctionExitContractV1 production producer              = 1
sole semantic-seal producer                                   = verify_function_completion_v1

independent completion analyzer                               = 0
second AST/source walk for function exit                      = 0
MIR terminator scan as source authority                       = 0
last lowered ValueId as return authority                      = 0

ReturnExitContract production authority                       = existing owner only
second ReturnExitContract producer                            = 0
Builder behavior branch on new semantic seal                  = 0
physical Return/signature materialization in S0               = 0

exact 0/1 terminal-return success slice                       = 1
nested/multiple/all-path success producer                     = 0
bare-return parser/registry activation                        = 0

Script-tail result classifier                                 = 0
Main-specific implicit-tail policy                            = 0
physical-entry/process-exit projection                        = 0
normal/public ingress consumer                                = 0
JSON/executor/selfhost/fastmem/CUT0 consumer                  = 0

new one-row shell guard                                       = 0
all modified/new source and check files                       < 800 lines
```

## Proof ceremony

```text
ceremony_tier =
  T1 bounded extension of an existing production authority

sunset_id =
  not_applicable_durable_semantic_contract

proof_inventory_before =
  one VerifiedFunctionCompletionV1 producer
  + one existing ReturnExitContract carrier/refresh owner

new_proofs =
  one nested F1 relation seal emitted by the existing completion producer
  + zero independent analyzers or contract authorities

retired_or_merged_proofs =
  none in S0; the relation is co-sealed rather than layered beside the owner

net_proof_delta =
  zero authority owners; one durable relation field

sunset_budget =
  zero temporary/disconnected proof owners

sunset_row =
  not applicable; this is accepted durable language semantics

retire_when =
  not applicable; replacement requires a new accepted language decision

budget_repayment_evidence =
  sole-producer census
  + zero second AST walk
  + zero second ReturnExitContract producer
  + reusable resolved-region-flow authority guard green
```

## Closeout law

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

Closeout requires:

```text
F1 exact 0/1 source contract implemented
focused fixture matrix green
existing completion consumers green
reusable authority guard green
no Builder/runtime/public behavior delta
current pointers advanced to the next explicitly selected row
```

## Follow-up boundaries

The only future physical consumer of this S0 contract is:

```text
FUNCTION-EXIT-F1-MATERIALIZE0-S0
```

That row may prepare one physical signature/Return/completion commit for the
already sealed exact slice. It may not broaden source coverage implicitly.

General nested, multiple-return, cleanup-bearing, and all-path coverage must
first pass a separate design stop:

```text
FUNCTION-EXIT-F1-ALLPATH-D0
```

Bare-return grammar reconciliation remains:

```text
FUNCTION-EXIT-BARE-RETURN-GRAM0
```

Script result classification, source-entry transport, and process-exit
projection remain their separately ordered rows.

## Non-claims

```text
nested/multiple/all-path function completion
cleanup-bearing return coverage
physical Return/signature materialization
dynamic or heterogeneous result ABI
expression-bodied function syntax
bare-return parser/grammar activation
ScriptLastExpressionOrUnit activation
source Main / synthetic physical-main split
process-exit projection or ny_main ABI change
normal-entry cutover
Raw App compatibility execution
JSON / Program(JSON v0)
executor / selfhost / fastmem
old Raw-chain retirement
public adapter repair
CUT0
```
