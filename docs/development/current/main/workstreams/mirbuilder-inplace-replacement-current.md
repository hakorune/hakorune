---
Status: Active workstream
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを次の一本へ収束させる。

```text
Resolve
-> Observe
-> Facts
-> Recipe
-> Verify
-> Lower
-> Seal
-> Collect
-> Atomic Publish
```

現在の本番MirBuilderを稼働させたまま、競合する責務ownerを一つずつ交換して
この形へ近づける。

```text
new owner
-> existing production caller switch
-> selected old branch deletion
-> post-cutover parity
```

独立した第二MirBuilder、遠い一括CUT0、production consumer 0のroute建設は
行わない。

cell数、pack数、LOC、structural observationは進捗と増殖検知の手段であり、
architecture goalやcompletion authorityではない。cellはnorth-star上の
authorityを一つ減らす場合だけ選択する。

## Current design stop

```text
GENERAL-FUNCTION-PLAN0-D0
```

最小structural observation、第七Binary cell、第八record-helper body descent
cellはclosed。helper driverはReturn-as-value ownerのまま、選択済み
MethodCall portをtagged child loanとして短期再借用し、旧direct edge二本と
dead same-family facade三本はゼロになった。

第九FieldAccess property getter cellもclosed。exact zero-argument adapterは
選択済みportをcatalog-child descentだけへ貸し、terminalはA1の
`lookup=None`を維持する。旧raw handler、Legacy adapter、value-only entry、
dead field facadeはゼロ。第十cellは未選択。

六workerのbounded censusは、局所のdead facade／proof cleanupより先に、
north-star最大の入口分岐を設計停止へ固定した。compiler-ingress D0は
Candidate Bを受理した。後続のsource/catalog D0は、最初の有限familyを
`Main0WithPlainInstanceBoxes0`、最初のproductを
`VerifiedNormalModuleSourceV1`へ固定してclosedした。

```text
CLI default / explicit mir
-> selected normal family
-> compile_with_source*
-> compile_legacy_request

explicit vm
-> compatibility authority

canonical exact fronts
-> bounded explicit families
-> normal/default production caller = 0

missing
-> current normal general Program
-> verified plan + candidate/publication owner
```

T2 prerequisite S0はclosedした。既存のowned input、source inventory、Main
relation、same-module callable catalogを再利用し、exact Program、
Main.main/0、source-orderのplain instance Box、catalogとの完全なkey対応を
一つのopaque productへsealした。production caller、Builder effect、
body/function plan、candidate/publicationはすべてゼロを維持した。

このownerとcurrent-normal `MirCompileResult` parityが閉じ、さらにpark済みの
Ownership/View readiness trainがproduct/default境界を閉じるまで、one total
typed ingressのCandidate A、Candidate A production edit、第十manifest rowを
禁止する。
REPL／JSON／VM keep／referenceは別authorityのまま保つ。

## First three replacements

```text
1. CALLABLE-DRAFT-PORT-CUTOVER0
   production callable body callerをport-aware descentへ交換
   old draft/body loweringを削除

2. CALLABLE-DRAFT-COLLECTOR-CUTOVER0
   direct per-function publicationをcollector + atomic insertionへ交換
   old direct publication branchを削除

3. MODULE-CANDIDATE-SESSION-CUTOVER0
   live Builder mutationをcandidate + success-only replacementへ交換
   old live-mutation entryを削除
```

この三つの間へStage-B、Ownership、cleanliness、新しいsource profileを
挟まない。

## Fixed macro packs

| Pack | Responsibility | Close condition |
| --- | --- | --- |
| REPLACEMENT-LEDGER0 | production ownerとdetached assetの有限台帳 | remaining 0 |
| DESCENT-SPINE0 | body／statement／expression／argument descent | direct recursion/facade 0 |
| FUNCTION-STATE0 | function state／facts／PHI／finalization state | duplicate writers 0 |
| CALL-OBJECT0 | calls／new／fields／index／collections／lambda | old call/object branches 0 |
| CONTROL0 | If／Loop／Match／QMark／cleanup／async | old control branches 0 |
| FUNCTION-LIFECYCLE0 | draft／collector／function finalize | direct draft publication 0 |
| MODULE-LIFECYCLE0 | declarations／catalog／module transaction | live partial module mutation 0 |
| COMPILER-RESIDUE0 | compiler ingress／old selectors／proof routes | normal/default canonical ingress 1; Legacy production callers 0 |

Pack数は8で固定する。新しい発見は既存packへ入れる。

## Historical live replacements to verify, not redo

次はdefault productionでの差し替え実績を持つ。

```text
Local
Variable-target Assignment
value-bearing Return
statement-position If
Binary
ShortCircuit
```

名称だけでclosed扱いにはしない。REPLACEMENT-LEDGER0がnew callerとold
selected branch 0を機械確認した時点でcreditする。

## Detached asset queue

```text
IntegrateNow:
  port-aware callable draft lowering
  generic draft collector after its first cutover
  candidate/config/success replacement after collector cutover

ReuseNeutral:
  source-neutral unified Call receipts
  verifier / module transaction kernels
  shared receiver policy and prepared route seams

FixtureOnly:
  parity snapshots that still protect a live replacement

Delete:
  ParserBox one-row Stage-B classifiers/activation/type publisher
  route-specific adapters with no production replacement target
```

The exact file-level classification is recorded as cells land. A Delete asset
must not be revived as a production route.

## Progress dashboard

Keep only these counters current:

```text
macro_packs_closed                 = 0 / 8
live_replacement_cells_closed      = 9
replacement_ledger_remaining       = 0 manifest rows
accepted_next_responsibility       = 0; GENERAL-FUNCTION-PLAN0-D0 design stop
detached_assets_remaining          = 2 recorded rows
legacy_production_edges_remaining  = 0 selected edges

structural_budget_status           = closed minimal observation
source_files_baseline              = 952
source_LOC_baseline                = 182452
test_files_baseline                = 139
test_LOC_baseline                  = 40826
```

Semantic evidence remains completion authority. The four structural values and
five-cell production Rust LOC are measurement history. Growth requires a
closeout explanation, not automatic rejection.

## Active replacement cell

```text
none
  current authority = GENERAL-FUNCTION-PLAN0-D0 design stop
  repository artifact lifecycle R1-R4-first / RETURN0 = closed
  tenth responsibility = not selected
```

## Closed S0 execution

```text
row                         = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-S0
parent                      = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
accepted family             = Main0WithPlainInstanceBoxes0
product                     = VerifiedNormalModuleSourceV1
ceremony                    = T2 prerequisite S0
production caller           = 0
replacement credit          = 0
tenth manifest row          = absent
Builder effect              = 0
fallback / retry            = 0
```

### Exact finite family

```text
root:
  Program exactly

entry:
  one static Main exactly
  one static Main.main/0 exactly
  Main helper methods = 0
  all other Main Box features = empty

module:
  one-or-more non-Main Box declarations
  source order and unique Box names retained
  is_static / is_record / is_sync / is_interface = false
  constructors / static_init = empty
  extends / implements / type_parameters = empty
  delegates / invariants / transitions / Box attrs = empty
  every method is a non-static FunctionDeclaration
  method map key = declaration name
  method contracts = empty
  method override = false

field and method source:
  fields / field_decls / visibility / init / weak metadata are retained
  body / return type / uses / method attrs are retained
  their semantic verification and lowering are later plan rows

top-level:
  executable statements = 0
  FunctionDeclaration = 0
  Enum / Brand / TypeAlias / Global / StaticConstTable = 0
  non-Main static Box / Using / Import / BuildGate = 0
```

The family name includes `Main0` intentionally. It does not claim
`Main.main(args)`, constructors, top-level functions, or all current-normal
user-box programs.

### One observation and one product

`inventory.rs` retains non-Main Box sites instead of collapsing them into the
generic unsupported bucket. The existing canonical classifier continues to
reject the first such site as the historical `Box` rejection. S0 does not add a
`SealedNormalSourcePlanV1` variant and does not connect canonical dispatch.

The disconnected S0 fixture consumes a fresh `PreparedNormalSourcePlanInputV1`,
runs the same `NormalSourceSurfaceInventoryV1::collect` exactly once, and calls
one module-source seal terminal. It is never invoked after canonical rejection.

```text
PreparedNormalSourcePlanInputV1
  AST + existing NormalSourceIdentityV1
-> NormalSourceSurfaceInventoryV1
-> VerifiedNormalModuleSourceV1
   Main site
   Main.main/0 site
   ordered NormalInstanceBoxSiteV1 rows
   existing VerifiedSameModuleCallableDeclarationCatalogV1
```

Do not add another identity issuer, callable catalog, classifier, trait,
raw-source accessor, `into_ast`, lowering method, Builder, or publication
terminal. Imports/config/admission are outside the current input and are not an
S0 claim.

### Exact correspondence

The callable catalog does not cover constructors, fields, empty boxes, or
top-level functions. S0 must therefore validate the Box surface from the owned
Program first, seal the existing catalog with `seal_program`, and compare:

```text
expected keys from source:
  StaticBoxMethod(Main, main, 0)
  every InstanceBoxMethod(owner, method, arity)

actual keys:
  catalog.keys in canonical order

required:
  sorted expected tuple set = exact actual tuple set
  each row's params / param_decls / return type / body / uses / attrs
    corresponds to the source declaration
  missing = 0
  extra = 0
```

No public canonical-key constructor is required; compare borrowed key
namespace/owner/name/arity and use `declaration_for` for the exact row.

### Bounded implementation

```text
production source:
  normal_source_plan/inventory.rs
    retain ordered non-Main Box sites
  normal_source_plan/product.rs
    add NormalInstanceBoxSiteV1 and opaque product fields if needed
  normal_source_plan/main_source.rs
    reuse one exact Main relation validator
  normal_source_plan/module_source.rs
    new named owner: family seal, correspondence, rejection
  normal_source_plan/classifier.rs
    preserve exact historical non-Main Box rejection
  normal_source_plan/mod.rs
    private module and bounded product exports

tests:
  normal_source_plan/tests.rs
    success, source-order, exact catalog, forbidden Box surface,
    mixed top-level rejection, canonical-rejection preservation,
    rejection identity/discard

guard:
  normal_source_plan0_guard.py
    only one child invocation/watched-symbol update
  normal_source_plan0_callable_guard.py
    detailed module-source assertions
```

The main shared guard is already near 800 lines; detailed assertions belong in
the existing callable child guard. No new test/check file or per-row guard is
allowed. Every touched source/check file must remain below 800 lines.

Focused gate:

```text
RUSTFLAGS='-Awarnings' cargo test -q normal_source_plan --lib
python3 tools/checks/lib/normal_source_plan0_guard.py
cargo check -q
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

### Acceptance

```text
VerifiedNormalModuleSourceV1 definitions       = 1
module-source seal terminal                    = 1
test callers                                   >= 1
production callers                             = 0

Program / Main.main/0 / Box order              = exact
source/catalog key-set correspondence          = exact
unsupported source rejection before Builder   = green
existing canonical non-Main Box rejection      = unchanged

new identity / callable catalog / classifier   = 0
raw AST escape / source reread / parse          = 0
Builder / lowering / function plan             = 0
candidate / DraftSeal / publication             = 0
fallback / retry / route reselection             = 0
replacement row / credit                        = 0
new test/check file                              = 0
all modified source/check files                 < 800
```

### Closeout

```text
focused normal_source_plan tests                = 67 / 67
VerifiedNormalModuleSourceV1 production callers = 0
existing canonical classifier behavior          = unchanged
Builder / lowering / publication delta           = 0
fallback / retry / route reselection             = 0
replacement row / credit                         = 0

new source files                                 = 1
new test/check files                             = 0
production Rust LOC delta                        = +626
test Rust LOC delta                              = +201
check Python LOC delta                           = +103
largest touched source/check file                = 776
```

The positive LOC delta is the measured cost of one new T2 source/catalog
authority, not replacement-cell credit. The source/check files stay below 800,
the product is disconnected, and the next slice must consume it rather than
create another module-source owner.

### Hard stop

```text
existing canonical dispatch or frontdoor must change
the product requires recovery from canonical rejection
the input must be cloned or re-parsed
Main relation requires a second validator
catalog completeness requires widening catalog authority
constructor / top-level function / Main(args) support is required
body, field, or function-plan semantics must be decided
Builder, candidate, DraftSeal, collector, or publication must be opened
compatibility wrapper, Legacy fallback, retry, or route reselection is needed
any touched source/check file reaches 800 lines
```

## Post-Binary boundary

`DESCENT-SPINE0-CLOSE-AUDIT` is closed. Its
`RECORD-HELPER-BODY-DESCENT0-D0` decision selected Candidate A as the T1
`RECORD-HELPER-BODY-DESCENT0-I0-R0` execution. Proof consolidation and dead
raw-body facade retirement remain candidate cleanups; neither is current
execution authority.

## Closed property-getter replacement

```text
production caller:
  raw_expression_dispatch::ASTNode::FieldAccess
  -> fields::build_field_access_with_port_v1

live authority break:
  property_reads::try_lower_property_read
  -> method_call_handlers::handle_standard_method_call
  -> LegacyMethodCallArgumentsV1

accepted owner:
  exact zero-argument property adapter
  -> borrow the selected FieldAccess port
  -> existing catalog-helper child capability

A1 terminal:
  raw lookup=None
  selected port terminal/header authority = 0

closeout:
  old property symbols              = 0
  source/test files                 = 952 / 139
  source/test LOC                   = 182452 / 40809
  production Rust LOC               = +22
  five-cell rolling production LOC  = -218
```

MethodCall AST/input reconstruction, receiver re-descent, fallback, retry,
route reselection, located activation, and header-authority delta are zero.
Dead raw-body facade retirement, proof consolidation, non-Program root, and
default compiler ingress were all unselected by the property closeout. The
fresh census below now selects only the compiler-ingress D0.

## Compiler-ingress return target

```text
pack                         = COMPILER-RESIDUE0
parent decision              = CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
accepted candidate           = B
ceremony                     = T2 bounded enabling design
future aggregate product     = VerifiedNormalGeneralProgramPlanV1
first missing authority      = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0
D0 status                    = closed
selected executable row      = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-S0
normal/default Legacy caller = present
canonical default caller     = 0
prerequisite caller          = 0
production/source edit       = 0
manifest row delta           = 0
fallback / retry             = forbidden
```

Authority:

```text
docs/development/current/main/investigations/
canonical-default-compiler-ingress0-d0-consultation-2026-07-28.md
```

The completed census corrected the first missing authority. The existing
canonical families are Script, Main0, and Callable; Callable itself seals
Acyclic or Recursive topology. The earliest source-backed residual fails on a
non-Main user `BoxDeclaration`, before body lowering. The return target must
therefore co-seal exact Program/declaration/entry/callable source facts before
adding body-plan vocabularies.

Candidate A remains parked. A typed `GeneralProgram -> Legacy` residual, a
catch-all source family, and a second monolithic MirBuilder are forbidden.
Local cleanup and proof consolidation remain parked because they do not switch
the default production authority.

## Scheduled docs interlude and return

The accepted cross-workstream order is:

```text
M0  bounded compiler-ingress census close
    return target fixed:
    NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0

D1  repository artifact lifecycle R1
    restore archive substrate gates

D2  repository artifact lifecycle R2
    make live / global archive / transitional archive resolution explicit

D3  repository artifact lifecycle R3
    move the exact two-file phase-296x pilot

D4  repository artifact lifecycle R4-first
    move one reference-closed bounded nested-archive batch

D5  DOCS-MEANING-RECOVERY-RETURN0
    strict lifecycle + pointer + reference gates
    recount
    return CURRENT_STATE to this MirBuilder workstream

M1a NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
    closed: Main0WithPlainInstanceBoxes0 selected

M1b NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-S0
    closed: disconnected source/catalog product

M2  GENERAL-FUNCTION-PLAN0 family slices
    current design stop: select one finite semantic vocabulary

M3  aggregate VerifiedNormalGeneralProgramPlanV1

M4  reuse DraftSeal / Collector / atomic publication

M5  current-normal MirCompileResult parity

M6  Candidate A technical readiness audit
    total partition / parity / failure / reuseを確認
    この時点ではproduction callerを切り替えない

O1  OWNERSHIP-SPARSE-RESUME-D0
    current compiler productsに対するprerequisite再確認

O2  Ownership/View Pack A
    inactive syntax rejection + exact evidence

O3  Ownership/View Pack B
    passive move/share/view grammar + resolved intent + Loan Flow

O4  Ownership/View Packs C-E
    Unique Box -> ScopedAlias -> callable ABI -> first Anchored View

O5  OWNERSHIP-SPARSE-PRODUCT-READINESS-D0
    supported backendまたはpre-effect rejectionを固定

M7  Candidate A final re-evaluation
    M1-M6 + O1-O5のreceiptを一度だけ照合

M8  Candidate A atomic cutover, only if M7 is green
    selected normal/default Legacy edgeをゼロへする
```

R5 stale-phase cohorts and R6 design/investigation retirement are later
bounded lifecycle batches. They do not hold the compiler lane after the first
R4 batch has passed RETURN0 and may run only at a later explicitly selected
clean-worktree milestone. They are not an implicit prerequisite for M1-M8.

During D1 through D5:

```text
compiler/runtime/backend edit = 0
tenth replacement row         = absent
new consultation file         = 0
```

This is the single cross-workstream execution queue. The detailed
Ownership/View row order remains owned by:

```text
docs/development/current/main/investigations/
  ownership-view-missing-grammar-inventory-2026-07-28.md

docs/development/current/main/investigations/
  hakorune-sparse-ownership-surface-task-2026-07-15.md
```

The language-v1 semantic backlog after M8 and the later R5/R6 documentation
batches remain parked queues. Neither may preempt an earlier row merely
because its task card already exists.

## Landed replacement cells

```text
CALLABLE-DRAFT-PORT-CUTOVER0
  new production callers   = 2
  selected old callers     = 0
  fallback / retry         = 0
  deleted old symbols      = 4
  deleted protocol shells  = 1
  production Rust LOC      = -202

CALLABLE-DRAFT-COLLECTOR-CUTOVER0
  ordinary collector callers = 1
  direct publication callers = 0
  partial publication         = 0
  fallback / retry            = 0
  production Rust LOC         = +153
  two-cell rolling Rust LOC   = -49

MODULE-CANDIDATE-SESSION-CUTOVER0
  default production callers  = 1
  live pre-success mutation   = 0
  failed-candidate mutation   = 0
  selected old callers        = 0
  fallback / retry            = 0
  production Rust LOC         = +44
  three-cell rolling Rust LOC = -5

LOCAL-STATEMENT-DESCENT-CUTOVER0
  raw/default production caller = 1
  detached located caller       = 1
  detached root activation      = 0
  selected old symbols          = 0
  fallback / retry              = 0
  production Rust LOC           = -52
  four-cell cumulative Rust LOC = -57

VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0
  raw/default production callers = 2
  detached located caller        = 1
  detached root activation       = 0
  selected old symbols           = 0
  fallback / retry               = 0
  production Rust LOC            = -77
  five-cell rolling Rust LOC     = -134

RETURN-SOURCE-PARTITION-CUTOVER0
  raw/default value caller       = 1
  raw/default exact Void caller  = 1
  detached located value caller  = 1
  detached root activation       = 0
  selected old symbols           = 0
  fallback / retry               = 0
  production Rust LOC            = -141
  five-cell rolling Rust LOC     = -73

BINARY-SOURCE-PARTITION-CUTOVER0
  raw/default partition caller   = 1
  ordinary generic callers       = 2
  ShortCircuit generic callers   = 2
  selected old symbols           = 0
  fallback / retry               = 0
  production Rust LOC            = -68
  five-cell rolling Rust LOC     = -294

RECORD-HELPER-BODY-DESCENT0-I0-R0
  catalog-child tag issuers        = 2
  selected old direct edges        = 0
  dead same-family facades         = 0
  fallback / retry / reselection   = 0
  source/test files                = 952 / 139
  source/test LOC                  = 182430 / 40820
  production Rust LOC              = +46
  five-cell rolling Rust LOC       = -292

FIELD-PROPERTY-GETTER-DESCENT0-I0-R0
  raw/default FieldAccess caller    = 1
  exact zero-argument adapter       = 1
  selected catalog-child loan       = 1
  A1 raw lookup-none terminal       = 1
  selected old symbols              = 0
  fallback / retry / reselection    = 0
  source/test files                 = 952 / 139
  source/test LOC                   = 182452 / 40809
  production Rust LOC               = +22
  five-cell rolling Rust LOC        = -218
```

## Parked

```text
Preloop Stage-B special production activation
Ownership grammar/runtime activation
.hako selfhost MirBuilder/parser migration
cleanliness A-E
new language semantics
default Raw/Canonical route cutover
```

They may resume only after the active replacement task map explicitly selects
them as a real production replacement cell or this workstream closes.

## Gate order

For a T0 cell:

```text
1. exact production caller census
2. one atomic I0/R0 implementation
3. focused production-path test
4. cargo check / relevant existing tests
5. shared pack guard
6. update counters and commit
```

Do not add a per-cell shell guard.
