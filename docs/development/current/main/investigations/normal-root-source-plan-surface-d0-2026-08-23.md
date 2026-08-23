# Normal root parser-backed source-plan surface D0

Status: design_stop — D0 shape selected; acceptance packet is still open
Date: 2026-08-23
Decision: NORMAL-ROOT-SOURCE-PLAN-SURFACE-D0
Owner: parser source relation -> pure normal source-plan policy boundary

## Six-line brief

Decision:
  Freeze one parser-owned, AST-free source-plan surface before reopening the
  atomic normal-root cutover. The existing classifier remains policy owner;
  its AST inventory is fixture-only after cutover.
Source authority + canonical issuer:
  Existing parser postpass/source anchors plus full static-Main member
  relation; the future sole issuer is `ParserNormalRootSourcePlanConsumerV1`
  under `src/parser/callable_parameter_source/normal_source_plan_surface/`.
Non-authority:
  `NormalSourceSurfaceInventoryV1` in production, AST/name/ordinal/pointer
  scans, `PreparedNormalSourcePlanInputV1::new`, Builder, MIR, and Raw/compat.
Fail-fast boundary:
  Missing/foreign/duplicate relation, incomplete member coverage, or a
  second source-plan loan rejects before policy selection and Builder effect.
Smallest next slice:
  Close one D0 acceptance packet for the full parser static-Main/member
  relation, transform handoff, single `SealedNormal*` family, and retained
  terminal; no semantic Rust is authorized yet.
Non-claims:
  No C0 production switch, normal/default consumer, Raw discard change,
  physical lowering, fallback, fixture expansion, or policy behavior change.

Census boundary: parser postpass completion -> parser-backed canonical
source-plan policy selection, including every Main/member, top-level callable,
executable, unsupported, retained, and AST-only fixture handoff; excludes
post-terminal lowering/publication after a sealed plan.

## Why C0 is stopped

The current canonical production path is:

```text
prepare_source_plan_request
  -> PreparedNormalSourcePlanInputV1::from_parser_callable_source
  -> NormalSourcePlanClassifierV1::seal
  -> NormalSourceSurfaceInventoryV1::collect
  -> AST/name/ordinal scan
```

`NormalSourceSurfaceInventoryV1::collect` is still the only production
classifier input. The parser authority currently provides body kind rows,
the exact `Main.main/0` admission, and ordinary module rows, but it does not
co-seal the complete static `Main` member relation needed for:

```text
App + Main helper -> CallableModule
App + top-level callable -> CallableModule
App + executable sibling -> MixedSourceFamilies
```

`ParserStaticBoxSourceSealV1` currently closes the bounded direct-method
cohort, and `issue_parser_main_app_entry_v1` requires exactly one direct
method. `ParserNormalRootPreservedV1` preserves the admitted App/Script
relation, but does not expose a complete App+helper source surface. Adding a
transport adapter or reusing `NormalTopLevelSiteV1.statement_index` would
therefore create a second authority or an ordinal/name re-pairing.

This is a real `NoSafeSlice`, not a missing caller or a test inconvenience.

## Authority map

| Boundary | Current/selected owner | Allowed responsibility | Forbidden responsibility |
| --- | --- | --- | --- |
| parser invocation and callable anchors | `src/parser/callable_parameter_source/product.rs`, `normal_root_source.rs`, `main_app_entry.rs`, `static_box_source.rs` | same-invocation source relation and coverage | policy selection, Builder/MIR |
| full source-plan surface | new `normal_source_plan_surface/{model,issuer}.rs` candidate | one opaque `ParserBackedNormalSourcePlanBoundV1` | AST reread, Recipe, physical IDs |
| pure source-plan policy | `src/mir/compiler/normal_source_plan/classifier.rs` | precedence and `Script/Main0/CallableModule/typed reject` | parser observation, AST scan on production path |
| AST-only inventory | `src/mir/compiler/normal_source_plan/inventory.rs` | fixture/test evidence only | canonical production authority |
| frontdoor transport | `src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs` | profile, receipt, route transport | source-plan reclassification |
| final output boundary | `src/mir/compiler/normal_source_plan/product.rs` | one plan product for an admitted surface | independent parser/AST products |

The future issuer and policy consumer are design names, not implementation
permission. Their constructors must remain private and their production
cardinality must be one each after the slice is accepted.

## Proposed bounded surface

The parser issuer must move one opaque aggregate. It must not return owner,
rows, and root relation as independently pairable values.

```text
ParserBackedNormalSourcePlanBoundV1 {
  source_owner: ParserNormalSourcePlanOwnerV1,
  surface: ParserBackedNormalSourcePlanSurfaceV1,
  _seal: ParserBackedNormalSourcePlanSealV1,
}

ParserBackedNormalSourcePlanSurfaceV1
  = CompleteEmpty(ExactEmptyWitness)
  | CompleteRows(NonEmptyCompleteRows)
```

`CompleteRows` must prove non-empty coverage. Each row is a single product,
not parallel arrays:

```text
Parser invocation witness
+ exact parser statement relation
+ root role: App | ProgramRuntime
+ exactly one observation:
   Executable
   TopLevelCallable(exact callable source relation)
   MainBox(exact box relation + all ordered member rows)
   Unsupported(parser-owned syntax kind)
```

Each Main member row carries its source relation and one closed observation:

```text
Function { source relation, callable identity, declaration syntax,
           arity/staticness evidence, RootMain | Helper role }
NonFunction { source member relation, parser-owned member kind }
```

Names, arity, and ordered positions are syntax/coverage evidence only. They
are never a cross-product key. The primary identity is the opaque parser
invocation witness plus the parser-issued source relation.

## Policy boundary

The selected policy shape is:

```text
ParserBackedNormalSourcePlanBoundV1
  -> NormalSourcePlanClassifierV1::seal_parser_bound
  -> existing precedence kernel
  -> SealedNormalSourcePlanV1 | typed rejection
```

`NormalSourceSurfaceInventoryV1::collect` remains available only for the
AST-only fixture/test route. The parser-backed consumer must not call it.
Pure Main validation helpers may be reused only after they accept parser-owned
relations; `locate_main_function`, raw `statement_index` lookup, and method
name discovery are not reusable authority on the parser-backed route.

The output boundary was the remaining design decision and is closed below;
implementation remains forbidden until the acceptance packet is complete:

1. Convert existing `SealedNormal*` sites to parser-bound source relations,
   with a scoped HRTB source loan for exact syntax; or
2. introduce a parser-backed output family while keeping AST-only products
   fixture-only, with one shared policy result authority.

The design must choose exactly one. A parser surface adapter that feeds the
current index/name-based `SealedNormal*` products is forbidden.

## D0 decision after premise audit

The bounded design is now fixed as a narrowed single-source route. This is a
design decision and task contract; it does not authorize semantic Rust while
`CURRENT_STATE.toml.work_mode` remains `design_stop`.

### D0-A — Full parser relation

`ParserNormalRootSourcePlanConsumerV1` is the sole composer of the full
parser-backed source-plan surface. It is called from the parser product while
the completed postpass, callable anchors, parser invocation witness, and
static-parent/member relations are still under one parser invocation.

It may reuse existing parser-owned relation issuers, including the narrow
`Main.main/0` admission, but it must not treat that narrow admission as the
whole source-plan classifier. The full product must retain, in one ordered
relation, every top-level statement and every static-Main member needed to
distinguish:

```text
Main.main/0
Main.main/0 + Main helper
Main.main/0 + top-level callable
Main.main/0 + executable sibling
Script executable rows
unsupported/non-Main rows
```

The narrow App entry remains a projection/consumer of parser evidence. It is
not expanded into a second policy authority. A missing member, duplicate
relation, foreign invocation, or unsupported parser row is a typed terminal of
the full issuer before any policy product is emitted.

### D0-B — One bound, one transform handoff

The parser emits exactly one private,
non-`Clone` `ParserBackedNormalSourcePlanBoundV1`. The aggregate owns the
invocation witness, complete surface, and source-loan relation; callers never
receive parallel row arrays or an independently pairable root role.

The bound is carried through the source-backed final-transform product as a
required field. The transform validates that the same parser relation and
coverage survive; it does not recreate the bound from the transformed AST.
Compatibility and incomplete/invalid parser dispositions terminate before a
normal source-plan consumer. They cannot drop the bound and retry Raw.

The policy boundary is one pure consumer:

```text
ParserBackedNormalSourcePlanBoundV1
  -> NormalSourcePlanClassifierV1::seal_parser_bound
  -> one existing SealedNormalSourcePlanV1 authority
```

`NormalSourceSurfaceInventoryV1::collect` remains an AST-only fixture route
and is not called by the parser-bound consumer.

### D0-C — Output boundary: choose the single existing product family

Choose strategy **1**: evolve the existing `SealedNormal*` output family to
retain opaque parser-bound source relations. Do not add a parallel production
`ParserBackedSealedNormal*` family.

The resulting shape is:

```text
parser bound
  -> one SealedNormalSourcePlanV1 family
     (Script / Main0 / CallableModule / typed rejection)
  -> scoped HRTB source loan for exact syntax only
```

The existing AST-only constructor is fixture/test-only after cutover. It may
feed test evidence into the same policy result kernel, but it cannot be a
production source-plan issuer. The production `SealedNormal*` site products
must carry parser-issued source relations; statement indices, names, arity,
and ordinals remain coverage/diagnostic evidence and never become pairing
keys. Lowering may borrow exact syntax through the source owner, but it may
not scan the AST to decide Script/App/CallableModule again.

This choice is preferred over a second output family because it preserves one
policy result authority, one downstream lifecycle contract, and one eventual
root cutover. A second family would require duplicate terminal handling and
would make the current raw/compatibility distinction a new production fork.

### D0-D — Execution manifest after design acceptance

When the design stop is explicitly closed, implementation is split into these
bounded cells; no cell may silently include the next one:

1. Parser surface: add the full member/statement relation under the parser
   sole issuer; add focused positive/negative evidence for invocation,
   coverage, Main/member, empty-vs-missing, and unsupported rows.
2. Transform transport: make the bound required across the existing
   source-backed final-transform handoff; prove no AST reconstruction and no
   second loan.
3. Pure policy: add `seal_parser_bound`, preserve existing precedence, and
   make AST inventory production-caller-zero. Emit the existing single
   `SealedNormal*` family with parser relations.
4. Retained terminal: close `into_retained_source` as an explicit
   test-only terminal with named field movement/discard; do not invent a
   production consumer.
5. Root consumer: only after cells 1–4 pass, replace the lifecycle's two raw
   root classifications with one move-only admitted root input before the
   first Builder effect. The work plan receives a typed admitted root, never a
   classifier `bool`.

The first four cells are still design-frozen in this card. Their production
implementation requires a later `fast` mode decision and a fresh focused
execution card. No implementation is authorized by this manifest alone.

### D0 acceptance / stop rule

The design may advance only when one packet proves all of the following:

```text
one parser surface issuer = 1
one parser-bound policy consumer = 1
one production SealedNormal* result family
full Main/member and top-level coverage is co-sealed
transform preserves the same invocation relation
AST inventory production caller = 0
AST-only constructor production caller = 0
no second loan, fallback, retry, or raw reclassification
retained source has an explicit named terminal
```

If any item requires a second source authority, a parallel production output
family, name/ordinal re-pairing, or a compatibility bridge, return to
`NoSafeSlice` and park that finding with its owner and reopen trigger.

## Finite states

| State | Effect | Next |
| --- | --- | --- |
| `Unbound` | none | sole parser issuer |
| `CompleteEmpty` | none | pure policy -> Script |
| `CompleteRows` | none | pure policy -> terminal |
| `SourceAuthorityUnavailable` | none | typed terminal |
| `Incomplete` | none | typed terminal |
| `IntegrityInvalid` | none | typed terminal |
| `BoundConsumed` | none | no second loan/issue |

No state may be represented by empty/default/optional rows. A rejected bound
retains its owner for the named terminal; it cannot retry Raw, compatibility,
or an AST classifier.

## Exact NoSafeSlice conditions

Remain in `design_stop` if any condition holds:

1. Static Main member coverage is not co-sealed for one parser invocation.
2. App+helper and App+top-level-callable relations cannot be issued without
   AST/name/ordinal re-observation.
3. Existing `SealedNormal*` output still requires independent AST discovery
   after parser-bound input is issued and no output-boundary decision exists.
4. Parser-backed policy must call `NormalSourceSurfaceInventoryV1::collect`.
5. `PreparedNormalSourcePlanInputV1::new` or
   `from_parser_callable_source` remains a production source-plan issuer.
6. A second source-plan loan, fallback/retry, raw pointer, or independently
   transported relation is needed.
7. Retained source still silently drops the sibling with `..` and has no
   explicit terminal owner.

## Bounded task sequence

### D0-A — Full parser static-Main/member relation

Audit and design the parser-owned member cursor/coverage product for one
static Main parent, including Main0, Main helper, non-function members,
foreign invocation, duplicate, missing, and unsupported rows. Do not change
`main_app_entry` to guess the result; its narrow App admission remains a
consumer of the fuller relation.

### D0-B — Bound surface issuer

Freeze the sole issuer location, private aggregate shape, `CompleteEmpty` /
`CompleteRows` proof, same-invocation relation rules, transform transport,
retained transport, and exact typed errors.

### D0-C — Output boundary decision

Choose one of the two `SealedNormal*` output strategies above. Record how
Script/Main0/CallableModule products retain parser relation without exposing
AST or re-pairable sites. No implementation starts before this decision.

### D0-D — C0 re-entry manifest

After D0-A/B/C, bind the six original C0 rows to exact owner files, positive
and negative test files, and one reusable guard:

```text
tools/checks/normal_root_execution_reference_route_guard.sh
```

The guard must prove: production parser surface issuer=1; policy consumer=1;
AST inventory production caller=0; `PreparedNormalSourcePlanInputV1::new` and
`from_parser_callable_source` production caller=0; canonical preterminal
discard/`into_parts`/`into_ast` edges=0; second loan=0; fallback/retry=0;
explicit retained test terminal; and all touched source/test files < 760.

## Required evidence for D0-A/B/C

Positive:

- empty Program -> `CompleteEmpty` -> Script;
- executable-only Script;
- exact static `Main.main/0` -> Main0;
- top-level helper + Main -> CallableModule;
- Main helper + Main -> CallableModule;
- executable sibling + App -> Mixed;
- ordinary/non-Main Box -> existing typed Unsupported;
- one parser invocation witness reaches the policy terminal.

Negative:

- missing/duplicate/foreign statement or member row;
- parser witness mismatch;
- static Main member coverage mismatch;
- main arity/staticness mismatch;
- function-only -> MissingSourceEntry;
- empty coverage confused with missing coverage;
- AST-only input entering production;
- second source-plan loan;
- name/ordinal/pointer-only pairing.

## Retained terminal

`ParsedProgramWithCallableParameterSourceV1::into_retained_source` currently
has no production consumer; its caller is `retained_tests.rs`. C0 must close
row 5 as an explicit `RetainedTestTerminal`, with all fields moved or
discarded by named ownership and no silent `..` sibling drop. A production
semantic consumer must not be invented solely to make the census green.

## Non-claims / parking

This card does not open physical lowering, Recipe/Join, Raw execution,
compatibility expansion, source-plan performance fusion, Builder barrel
cleanup, or production branch cutover. Those remain `ParkedSealed` until a
named trigger reopens them. The C0 atomic cutover remains closed until this
surface, output boundary, and guard manifest are accepted together.
