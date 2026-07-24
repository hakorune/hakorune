# Function Exit and Entry-Result Semantics

Status: SSOT
Decision: accepted
Decision token: `FUNCTION-EXIT-SEMANTICS-prime-r1`
Date: 2026-07-25
Scope: Canonical callable completion, Script results, selected source-entry
results, and the target process-exit projection.

Related:

- `docs/reference/language/semantic-contract-charter.md`
- `docs/reference/language/semantic-kernel.md`
- `docs/reference/language/grammar-contract.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/types.md`
- `docs/reference/language/failure-outcome-relations.md`
- `docs/reference/language/scope-exit-semantics.md`
- `docs/reference/language/block-expressions-and-map-literals.md`
- `docs/reference/language/repl.md`
- `docs/reference/architecture/llvm-harness.md`

## Purpose and Authority

This topic owns how an already-evaluated canonical Outcome crosses these
boundaries:

```text
expression / statement evaluation
  -> callable completion
  -> selected source-entry result
  -> process termination
  -> native OS status
```

It does not redefine the Outcome vocabulary. `Normal`, `Return`, `Break`,
`Continue`, and `Fault` remain owned by `semantic-kernel.md` and
`failure-outcome-relations.md`.

It also does not make parser, MIR, runtime, or backend behavior normative.
Existing code and parity fixtures are implementation evidence. Where they
disagree with this topic, the difference is an explicit migration gap.

The accepted target is:

```text
ordinary function/method fallthrough = ExplicitReturnOnly
source Main.main                     = ordinary method semantics
Script result                        = ScriptLastExpressionOrUnit
physical entry                       = source-result transport only
ny_main                              = process-exit projection ABI only
Legacy AnyStatement tail             = migration observation only
```

## Canonical Unit Provenance

`Unit` is the successful no-useful-value result already defined by
`failure-outcome-relations.md`. An implementation may retain one of these
origins for diagnostics, conformance, or migration evidence:

```text
EmptyBody
ImplicitFallthrough
ExpressionStatementDiscard
PrintStatement
LocalStatement
AssignmentStatement
CompoundAssignmentStatement
ExplicitVoid
ExplicitNull
BareReturn
```

These origins are not distinct language values. Programs cannot branch on,
compare, store, or otherwise observe a Unit origin. The exact source relation
between `null` and `void` remains owned by `types.md`; this topic does not
silently create a second null policy.

## Ordinary Function and Method Boundary

Canonical ordinary functions and methods use
`ExplicitReturnOnly`.

```text
explicit `return expr` -> Return(Value) or Return(Unit)
bare `return`          -> Return(Unit)
ordinary fallthrough   -> Normal(Unit)
```

The last internal value produced by a statement lowerer is never callable
return authority. In particular, a value used while lowering `print`, a fresh
local binding, or an assignment publication does not become a return value
because it was produced last.

At the callable boundary:

```text
Normal(Unit)  -> FunctionResult::Unit
Return(Unit)  -> FunctionResult::Unit
Return(Value) -> FunctionResult::Value
Fault         -> propagate the final Fault
```

An ordinary expression statement evaluates its expression exactly once and
then discards the value, producing `Normal(Unit)` at the statement boundary.
Consequently, `Normal(Value)` must not escape a sealed ordinary callable
statement body. `Break` and `Continue` must be consumed by their loop boundary
and cannot cross the callable boundary.

### Cleanup and ownership

Callable completion consumes the final Outcome only after the cleanup law in
`semantic-kernel.md` and `scope-exit-semantics.md` has run. A cleanup Fault may
therefore replace a pending `Return` or `Normal` outcome.

Returning an owned value is terminal owner forwarding. It does not imply a
clone, an additional owner, or a second `move`.

## `Main.main/0`

Source `Main.main/0` is an ordinary source method. Entry selection does not
give it an implicit-tail exception.

```hako
static box Main {
    main() {
        1
    }
}
```

The expression statement is evaluated and discarded; the method falls through
with Unit.

```hako
static box Main {
    main() {
        return 1
    }
}
```

The explicit return produces a source value result.

Long term, a selected source entry and a synthetic physical entry are separate
owners:

```text
source Main.main/0
  -> selected source-entry call
  -> synthetic physical entry transport
```

Inlining that call is permitted only as an optimization derived from the
sealed callable-exit contract. It cannot reclassify the source body's last
statement.

## Script Result

Script is an evaluation context, not an ordinary callable body. Its target
source contract is:

```text
prelude statements*
optional final source expression
```

Only a final node classified by the source grammar as an expression supplies a
Script value. A final statement supplies Unit.

| Final source form | Script result |
| --- | --- |
| source-classified expression | `Value` |
| `print` statement | `Unit` |
| `local` statement | `Unit` |
| assignment statement | `Unit` |
| compound-assignment statement | `Unit` |
| no statement/expression | `Unit` |
| explicit `void` expression | `Unit` |

This classification is parser-neutral source authority. An AST implementation
name, Builder return value, `ValueId`, module symbol, or route name cannot
decide it. The REPL expression-versus-statement rule is supporting evidence,
not the Script contract owner.

The current canonical program grammar does not yet expose a dedicated
`script_tail` production. Parser/registry admission and the source projection
that seals this classification require a later activation row. Until then,
`ScriptLastExpressionOrUnit` is accepted target semantics, not permission for a
Builder to infer a tail from its last value.

## Statement Completion Table

| Source form | Ordinary function / `Main.main` | Script final position |
| --- | --- | --- |
| expression statement | evaluate, discard, `Normal(Unit)` | source-classified final expression may yield `Normal(Value)` |
| `print(expr)` | `Normal(Unit)` | `Normal(Unit)` |
| `local ...` | `Normal(Unit)` | `Normal(Unit)` |
| assignment | `Normal(Unit)` | `Normal(Unit)` |
| compound assignment | `Normal(Unit)` | `Normal(Unit)` |
| empty body | `Normal(Unit)` | `Normal(Unit)` |
| `void` | `Normal(Unit)` | `Normal(Unit)` |
| `return expr` | `Return(Value)` or `Return(Unit)` | rejected at Script root unless a later Script-control row accepts it |
| bare `return` | `Return(Unit)` | rejected at Script root unless a later Script-control row accepts it |

Parenthesized assignment is an expression only in the exact grammar profile
that admits the grouped-assignment expression. Bare assignment remains a
statement.

The target meaning of bare `return` is fixed above. Its canonical grammar
status must be reconciled between `EBNF.md`, `statements.md`, the grammar
registry, and both parsers before a new route relies on it. Current parser
acceptance alone is not grammar authority.

## Return Annotation and Physical Signature

An explicit return annotation `: T` is a source semantic contract owned by
`types.md`. It is not a `MirType` hint.

Omitting the annotation means an unannotated result contract:

```text
omitted annotation != declared Void contract
omitted annotation != source-level static type-inference contract
```

An unannotated function may use an explicit value return. An annotated
non-Void function must not reach a normal Unit fallthrough, bare return,
`return void`/`return null`, or a mismatching value. An explicit `: void`
function admits Unit fallthrough, bare return, and `return void`/`return null`,
but rejects a definite non-Unit return value.

Physical signature planning may use a fresh verifier-backed proof derived from
the sealed declaration and explicit return sites. `MirType`, `value_types`,
last-lowered `ValueId`, route metadata, or homogeneous-looking Builder facts
alone are not semantic proof.

If a source result is valid but a backend lacks the required dynamic or
heterogeneous result carrier, that backend rejects before effects. Backend
capability cannot narrow the source-language contract and cannot trigger a
fallback.

### Existing implementation owners

Current narrow implementation evidence already includes:

```text
VerifiedFunctionCompletionV1
  = ExplicitReturn | ImplicitVoid

ReturnExitContract
  = active exact-numeric annotation enforcement on its supported route
```

A later implementation row must reuse or aggregate these owners. It must not
create a second callable-completion authority or a second annotation-contract
authority. The current verified completion slice is narrower than the full
target: broad nested, multiple-return, cleanup-bearing, and all-path coverage
remain separate capability rows.

## Source Entry and Physical Entry

Entry selection produces a typed source-entry result:

```text
SourceEntryResult = Unit | Value | Fault
```

This notation is a boundary relation, not a replacement for the semantic
kernel's Outcome type.

The synthetic physical MIR entry has one role:

```text
selected source entry
  -> transport SourceEntryResult
```

It owns no independent source-language return rule and no OS exit-code policy.
Symbol `"main"`, arity, module name, or physical inventory cannot select source
semantics.

The repository does not yet implement this source-call/physical-thunk split on
every route. The rule above is accepted target architecture; physical-entry
activation remains zero in this decision.

## Target Process-Exit Projection

Source result and process termination are separate boundaries. Evaluation APIs,
the REPL, interpreters, embeddings, and test harnesses may preserve values that
are not valid process exit statuses.

The accepted target portable process profile is:

| Source-entry result | Target process projection |
| --- | --- |
| Unit / Void | exit status `0` |
| Integer in `0..=255` | exact status |
| Integer outside `0..=255` | `ExitCodeOutOfRange` Fault |
| Bool | `UnsupportedProcessResult(Bool)` Fault |
| Float | `UnsupportedProcessResult(Float)` Fault |
| String | `UnsupportedProcessResult(String)` Fault |
| Box / Array / Future / other value | `UnsupportedProcessResult(type)` Fault |
| final program Fault | diagnostic plus reserved process status `70` |

Status `70` is a process-profile projection; it is not the semantic value of a
Fault. Parse errors, compile errors, CLI usage errors, and failures before
program execution remain owned by their respective CLI/runtime boundaries.

The `0..=255` range defines the accepted portable process profile. A native OS
adapter may have a wider host status representation, but it must not silently
change the portable result. Platform-specific policy requires a separately
named profile.

This target policy is not active in this decision.

```text
process_exit_projection_activation = 0
native_os_adapter_activation = 0
```

### `ny_main` target ABI

The target relation is:

```text
source entry
  -> physical entry: SourceEntryResult
  -> process projection: normalized status
  -> ny_main() -> i64: normalized status only
  -> native main: checked OS adaptation
```

`ny_main` must not permanently multiplex a raw source Integer, generic object
handle, and normalized status in the same untagged `i64`.

Current routes still include Bool/Float result conversion, modulo or host
truncation, non-numeric-to-zero behavior, and positive-handle normalization.
`llvm-harness.md` documents the current compatibility ABI. These are migration
facts, not alternate canonical semantics.

The current process compatibility has a separate retirement identity:

```text
profile_name =
  LegacyRunnerExitProjectionV1

sunset_id =
  ENTRY-EXIT-CODE-COMPAT-SUNSET-001

owner of the retirement decision =
  ENTRY-RESULT-PROJECTION0-D0

sunset row =
  ENTRY-EXIT-CODE-COMPAT-RETIRE0-S0

retire_when =
  current VM/MIR-interpreter/LLVM/native projection inventory fixed
  + one shared ProcessExitProjectionV1 selected and implemented
  + required backend parity green
  + normalized-status-only ny_main green
  + modulo/Bool/non-numeric-zero/handle-heuristic compatibility callers zero
```

Its exact caller inventory and deletion evidence are fixed by the named D0
before the target process projection is activated.

## Legacy App Tail Observation

`AppLastValueOrVoid` and `LegacyAnyStatementValueOrUnit` are not canonical
function semantics.

The historical behavior may be observed under this private migration
vocabulary:

```text
observation_name    = RawLegacyAppAnyStatementTailParityV1
policy_label        = LegacyAnyStatementValueOrUnit
sunset_id           = RAW-BODY-RETURN-COMPAT-SUNSET-001
```

This is a parity oracle, not a source-language execution profile. It may retain
the exact historical observation in disconnected tests, but it has:

```text
canonical MIR admission = 0
BODY production consumer = 0
public ingress consumer = 0
runtime/backend consumer = 0
fallback authority = 0
```

Attempted semantic entry fails before physical effects. This preserves the
compatibility law in `semantic-contract-charter.md` and the
`compatibility_transport` boundary in `grammar-contract.md`: the same source
spelling cannot silently acquire a different callable result.

The observation has no promotion path under the accepted
`ExplicitReturnOnly` decision. Reopening it as canonical semantics requires a
new language decision.

```text
retirement owner = FUNCTION-EXIT-COMPAT-RETIRE0
sunset row       = RAW-BODY-RETURN-COMPAT-RETIRE0-S0

retire_when =
  Main / Script / physical-entry target accepted
  + process-projection parity green on required backends
  + normal-entry profile explicitly selected
  + LegacyAnyStatement canonical consumer zero
  + LegacyAnyStatement public production consumer zero
  + AppLastValueOrVoid symbol/caller zero
```

## Activation Boundary

This decision accepts target semantics and reclassifies historical parity. It
does not activate a new parser, Builder, runtime, backend, or public route.

```text
normative function-exit decision accepted       = 1
normative Script-result decision accepted       = 1
normative source/physical-entry split accepted  = 1

repo-wide F1 production activation              = 0
Script source-tail projection activation        = 0
physical source-entry thunk activation          = 0
process-exit projection activation              = 0
normalized-status-only ny_main activation       = 0
Legacy AnyStatement production consumer         = 0

normal-entry cutover                             = 0
JSON / Program(JSON v0) behavior change         = 0
executor / selfhost / fastmem activation        = 0
old Raw-chain retirement                        = 0
CUT0 activation                                 = 0
```

The first implementation work must inventory and relate the existing
`VerifiedFunctionCompletionV1` and `ReturnExitContract` owners before adding
new types. Broader control-flow coverage, Script tail classification, physical
entry transport, and process projection remain separate rows.

## Acceptance Record

```text
function_exit_semantics_owner_count = 1
ordinary_function_fallthrough = ExplicitReturnOnly
Main_main_uses_ordinary_function_semantics = 1
Script_result = ScriptLastExpressionOrUnit
physical_entry_owns_source_semantics = 0
ny_main_owns_source_semantics = 0
Legacy_AnyStatement_is_canonical = 0
Outcome_redefined_by_this_topic = 0
process_policy_activated = 0
```
