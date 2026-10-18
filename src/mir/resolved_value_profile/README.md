# Resolved Value Profile

This directory owns immutable, pre-Builder executable representation proofs for
canonical resolved source.

## Boundary

Inputs are exact located source carriers plus the sealed resolved-semantic
product. Outputs may contain only:

- `FunctionOwnerIdV1`, `BindingRefV1`, and exact source sites;
- closed representation vocabulary owned by this directory;
- sealed parameter ABI rows whose source names are transport/diagnostic data,
  never lookup or binding identity authority;
- a sealed return ABI witness that refers to the existing exact terminal and
  never owns another return value or terminal analysis;
- a direct-call row co-sealed from the source-unit callable
  index, with exact target projection, ordered argument sites, result, effect,
  and one `DirectCall(site)` coverage subject;
- exact value/definition/join/terminal coverage.

This layer must not import or infer from `MirBuilder`, `ValueId`,
`BasicBlockId`, `MirType`, `StorageClass`, runtime values, spans, pointers, or
names. It decides no CFG layout and emits no MIR.

## Shared expression source view

`expression_source` borrows structural rows from one original callable source
ledger and its same-owner body shape. The result classifier and the live Home
scalar preflight share this selection; recursion keeps the same ledger loan.
It preserves the original row-family priority, including direct-call target
and observation membership as one family. Shape-only fields, receivers and
constructors remain unresolved source structure.

The view issues no result class, operator envelope, call target, Recipe, Home
or executable permission. Consumers retain their exact call/field/operator
checks and BlockExpr prelude restrictions. The source result product remains
a representation proof. Ordered expression obligations, String temporaries,
Fault/End, physical correspondence and final verification are later consumers;
this structural connection grants no String activation.

`expression_children` retains exact ordered child sites and roles from that
same source view. Binary operands keep LHS/RHS identity; logical RHS and
conditional branches retain their original condition-dependent reach. Receiver
relations are structural: original target/receiver issuers still decide value
evaluation and qualification. Unknown structure is Opaque, not an empty leaf;
missing original rows, wrong owners and child/ordinal drift are identity errors.
BlockExpr retains its original prelude statements and tail as opaque obligations.
The result operation vector records these source relations without promoting
representation evidence to executable ownership or effect permission. The live
Home scalar walker and result projection share the same binary-child proof.

`expression_root` borrows the original PrintValue or ReturnValue root from
that same owner's body-shape inventory, requiring its exact canonical Value
site, source membership and unique statement-to-value relation. Bare Return
and other statement shapes supply no value root. Roles do not classify the
value, grant Home neutrality, or authorize Print/Return execution.

`expression_plan` retains one immutable original root and its preorder nodes
using those same source/children loans. Local roots retain the exact original
initializer ledger row; statement roots retain the original Print/Return role.
Static/current-owner and qualified receiver leaves borrow the exact original
parent method row's receiver disposition, preserving shape-independent Local
profiles. They grant no value class or dispatch; future semantic consumers must
reborrow original operator/call facts rather than infer them from node tags.
Ordered child reach remains attached to each parent. Field/Call nodes are
unresolved lender needs; opaque scope/prelude/tail obligations are retained.
This structure alone grants neither value classes nor executable coverage.
The same plan's `field_path_v1` lends original field-node indices in root-to-leaf
order, checking each Receiver edge/site and the final lexical root's owner.
Calls and arithmetic cannot supply that root. Class, canonical declaration,
root liveness and value proofs remain separate. The existing direct-read
adapter requires exactly one edge; nested syntax grants no direct-read credit.
`ordered_postorder_v1` preserves declared child order (LHS before RHS,
arguments in source order) before each parent; it validates one complete tree
and refuses opaque/missing children. The class projection uses this shared
structural walk. Conditional reach remains on the SAME original child rows;
postorder grants no permission to evaluate a guarded child eagerly.
Assignment roots retain the original sealed target/RHS/form row, owner and both
canonical source sites; they use the same ordered node builder. Assignments do
not have a statement-to-Value relation. Compound RHS structure is retained as
Compound and grants no stored-value class without its evaluated-place/operator
contract; it is never silently treated as a Plain store.
Statement If conditions use a separate `IfCondition` root from the same plan
builder. Its factory checks the original function's sealed If bundle, exact
condition child relation, owner and source inventories. The resolver records
that relation while resolving the original statement. AND/OR retain their
conditional RHS reach; a missing or non-If site cannot become a condition root.
This is source structure only, without a Bool/Home/physical execution grant.


The existing Home owner holds the same source plan and stable qualified
operation loans in its one expression-plan collection. The current Local
Sub/positive-literal-Div cohort keeps all previous preflight restrictions and
publishes only after the complete original field/call batch succeeds. Physical
and independent final scalar consumers flatten only those operation loans,
never all structural nodes. There is no plan-to-loan ownership backedge.
Print/Return execution and String temporary/Fault/End remain later consumers.

## SSA-I0-PROFILE contract

`VerifiedTrivialCanonicalOwnerV1` is a pre-Builder whole-owner proof. It
admits exact `InlineI64`, `InlineBool`, `InlineF64`, and local-flow
`ExplicitVoidValue`, and local-flow `NullSentinel` values and proves their
propagation through locals, reads, rebinds, binary expressions, BlockExpr
results, fallthrough If merge profiles, and terminal disposition.
`ExplicitVoidValue` preserves exact source `void` as a value and remains
distinct from `return;` and implicit completion. `NullSentinel` preserves exact
source Null identity. Both reuse the existing MIR/runtime no-value
representation and are not ownership-managed values. Null itself is not yet a
return ABI.
Merge-profile rows prove representation homogeneity only; they never decide
whether a PHI is needed or placed. Function-owned Binding SSA retains that
authority.

Exact parameter rows are ABI sidecars. Their declaration `Definition` row is
the sole exact-once coverage subject, and parameter names never replace
`BindingRefV1` identity. The first row accepts only exact source `i64`; it
allocates no `ValueId` and has no production Builder connection until P0a-I1.

The first return witness accepts only exact source `i64` co-sealed with the
existing final explicit `InlineI64` terminal, completion, and coverage row.
R0a-I1 connects this witness through the route-local callable-ABI facade. Only
the co-sealed exact `i64` row reaches production; the terminal remains return-
value authority and resolved Lower does not reread the raw annotation.

The D0-B2 If mapper is a disconnected elaboration owned by this directory. It
consumes one sealed `VerifiedTrivialIfRecipeFactsV1` plus the function-origin
receipt, creates recipe-local keys, and immediately invokes the portable
IfRecipe verifier. It may not rescan source, infer missing entry values, select
routes, return `Option`, or emit Builder/SSA effects. A missing pre-If entry
witness is a typed rejection; JoinSig and canonical PHI remain later owners.

### D0-B2 rejection partition

The mapper's rejection enum deliberately has two layers:

```text
ordinary-input rows:
  MissingFacts       // producer declined before portable mapping
  OwnerMismatch      // source receipt belongs to another function
  EntryClassMismatch // sealed entry and branch representations differ

sealed-facts defenses:
  MissingEntryWitness, EntryBindingMismatch, MissingAssignment,
  EntryDefinition*, UnsupportedRepresentation, MissingExpression,
  CrossRegionDependency, ExpressionCycle, Binding*Mismatch,
  SourcePathMismatch, ContinuationMismatch, Recipe(...)
```

The first group is covered by real same-pass fixtures. The second group is an
invariant firewall for a malformed or future producer; it is not ordinary
negative-input coverage. Do not synthesize a malformed non-`Clone` facts
product just to execute those arms. If a future producer makes one reachable,
promote that variant into the first group with a fixture and an SSOT update.
Branch source paths preserve the actual item index; the current accepted
fixture demonstrates both index zero and a non-zero item without changing the
recipe's semantic normalization.

Profile rejection is data, not fallback. A later compiler route may select the
existing canonical A+ path from a sealed rejection before Builder effects, but
it must never retry A+ after a trivial-profile lowering failure or mix the two
authorities inside one source unit.

P0c-S0b introduced the row disconnected. The final callable Program authority
uses the same generic row for every finite one-or-more exact call set, including
singleton self calls, sibling edges, nested calls, and recursive SCCs. The
profile assumes neither catalog cardinality nor target-equals-caller identity.
Its call result is represented
only by `DirectCall(site)`, never by a duplicate generic `Value(site)` row. The
consumption API returns the whole row so Lower cannot pair target and ABI from
separate authorities. The trivial direct-call lowerer reads that already sealed
target projection before descending into argument expressions. Coverage
consumption remains in the existing postorder (nested argument calls before
their enclosing call), and the claimed row's target is checked against the
pre-effect projection before the canonical Call is emitted. No target is
resolved or reconstructed after argument effects.

The finite analyzer records nested calls in execution postorder (argument calls
before their enclosing call) and uses checked cardinality. Its internal
`TrivialCanonicalAnalysisModeV1` is an exhaustive four-quadrant vocabulary:
ordinary/main role crossed with closed/finite-direct-call policy. The mode maps
to the two existing policy dimensions; it adds no source or semantic shape.
There is no exact-one analyzer or retry path. Non-VM execution remains rejected
before effects, and callers must use the single mode entry rather than
reconstructing policy pairs.

SSA-I1-T consumes an admitted profile exactly once in the dedicated trivial
Binding-SSA lowerer. A non-admitted profile selects the whole-unit A+ route
before Builder effects; a lowering failure never retries another route.

## Nested If D0 sidecar

The analyzer may seal a separate `VerifiedNestedTrivialIfRecipeFactsV1`
sidecar when the source contains exactly one outer and one inner explicit-`else`
If over one shared `i64` binding. The sidecar is not the old
`VerifiedTrivialIfRecipeFactsV1`: the old one-If fixed shell remains immutable
and continues to reject `ifs.len() != 1`.

`map_nested_trivial_if_recipe_v1` consumes only this same-pass sidecar and the
function-origin receipt. It emits a portable depth-one artifact, verifies its
source claims and join rows, and elaborates a logical nested JoinSig in tests.
It does not rescan AST, select routes, return `Option` as retry, touch Builder,
or create a physical CFG/PHI owner. Production physicalization is deliberately
outside this D0 profile and is gated by the nested execution task's D1/D2
rows.

## D7 source-result product

`source_result` owns the first source-catalog result-class product for an
exact catalog callable. The product retains the stable declaration-catalog
brand alongside the callable key. `SourceResultClassV1` is semantic (`I64` or
`String`) and does not project to `MirType`.

The issuer consumes `ResolvedFunctionLoweringInputV1` — the co-sealed
owner/forest/body-shape bundle — plus a `VerifiedSourceCallTargetCatalogV1`
branded by the same declaration catalog. The input must have been resolved
from the exact declaration node the catalog row sealed: name, parameters,
return type, uses, attributes, and body are compared field-by-field before
the ledger is consumed (a same-parameter-shape foreign declaration cannot
compose). The declaration body supplies only
the statement scaffold (`Local` / `Assignment` / `Return`); every expression
classification goes through `CallableSemanticSourceLedgerView` rows:
conditional, operator, literal, lexical-reference, method-call, and
direct-call rows are never re-derived from raw AST. Expression-If consumers
(`Value`, `Rhs`, `Initializer`) come from the sealed
`ResolvedConditionalExpressionSourceV1` relation, and transparent `BlockExpr`
wrappers are stepped through by the co-sealed `BodyExpressionShapeV1` rows,
not by syntax. A `BlockExpr` is transparent only when its prelude is empty:
the sealed statement inventory proves absence by checking for
`BlockExprPrelude` children of the wrapper site, and a non-empty prelude
rejects with `NonEmptyBlockExprPrelude` rather than silently dropping
statement effects. It also records the exact `String/null` inequality fact
used by a later conditional-value consumer. Pure binary class transfer lives
in `expression_class.rs`; the row-driven, site-bound classifier lives in
`source_result_classify.rs`; `source_result.rs` keeps the
product vocabulary, the sealed-input identity checks, and issuance.

Every observed call publishes one explicit `SourceCallDispositionV1`:
`Static` only where the branded route catalog proves the same-module
callable, `Dynamic` only where a source-bound dynamic member route exists,
and `Absent` for every other observed call — including resolver-proven
direct calls that carry no same-module catalog key. `Dynamic`/`Absent`
results stay unproven wherever a `SourceResultClassV1` is required.

The issuer fails before Builder effects on foreign callable keys, foreign or
unbranded route catalogs, resolved inputs whose declaration node differs from
the catalog row, missing ledger/body-shape inventories, consumer-site
drift, missing branch rows, non-empty `BlockExpr` preludes, unsupported or
mixed result classes, unproven call tails, and unknown expressions. Core-method placement/effect rows, recursive
body closure, JoinSig, physical PHI construction, and production route
selection remain owned by later slices. No result class is inferred from MIR
or from a callable name.

`expression_projection.rs` projects the SAME original plan using the shared
binary-class transfer and caller-qualified leaf requirements. Every ordered
child remains an obligation, including receiver identity and call arguments;
an unresolved child cannot disappear under a known parent class. Identity is
not an Integer operand. This projection grants no current-I64 result producer,
checked Normal/range, Home ownership, executable signature, or physical MIR.

The same pure fold also exposes per-node classes to checked statement frames.
An explicit transferred-node set preserves call-owned input structure as Identity;
its consumer must separately retain and verify the exact whole-input source loan.
Every node still passes the original Known-child/postorder checks; transfer does
not hide opaque or missing children. The original result-class API uses an empty
transfer set and retains its existing laws. Node classes alone do not authorize
a checked operation or ownership: the statement owner must supply producer facts,
operator envelopes and the complete Normal/Fault/End frame correspondence.

Call-argument expression plans use the same build_plan owner. Their root retains
the canonical outer method Arc, argument ordinal and nearest registered statement
ancestor from the SAME source-site inventory. The adapter checks owner, original
Arc, call/argument membership and canonical Argument(ordinal) relation before
building only that actual subtree. Statement ancestry is passive topology; it
proves no current locals, Home state, Loop After, argument class or execution.
No resolver-wide Rc registry or second expression classifier is introduced.

If and Loop conditions share the source inventory/child builder while retaining
distinct purposes. A Loop condition carries its exact resolver region bundle and
execution frame, preserving the original conditional child reach. The retained
callable Loop SourcePlan and its selected product share one original source
component, which survives affine planner consumption. Home and the physical
input borrow its same condition Rc. The physical boundary matches the complete
original condition Local-read census against ConditionRead receipts, including
receiver and opaque-child reads; missing, duplicate or extra receipts reject.
This is source correspondence only: the existing If Bool
qualifier cannot use it as an If bundle, Header proof or Home continuation.
