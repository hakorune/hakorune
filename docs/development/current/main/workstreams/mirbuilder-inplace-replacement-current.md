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

## Active executable front

```text
row          = GENERAL-FUNCTION-PLAN0-INSTANCE-CUMULATIVE0-S0
parent       = GENERAL-FUNCTION-PLAN0-INSTANCE-SCALAR-BINDING0-D0
decision     = corrected Candidate C-prime
ceremony     = T1
input        = VerifiedNormalModuleSourceV1
intermediate = VerifiedNormalInstanceFunctionPlanSetV1
checkpoint   = VerifiedNormalModuleFunctionPlanSetV1
variants     = IntegerLiteralReturn exactly one
grammar delta / production caller / replacement credit = 0
```

Candidate Aのcumulative sum-typeを最終形に採用しつつ、BoxShape migrationと
新grammarを分離する。C0は既存integer-literal-return familyだけをenumの
第一variantへ包むgrammar-neutral migrationである。C1だけが、C0 close後に
exact `i64` parameter-return familyを追加できる。

```text
VerifiedNormalModuleFunctionPlanSetV1
├─ instance: VerifiedNormalInstanceFunctionPlanSetV1
│  ├─ source: VerifiedNormalModuleSourceV1
│  │  └─ Program / identity / Main sites / Box sites / callable catalog
│  └─ plans: BTreeMap<CanonicalKey, VerifiedNormalInstanceFunctionPlanV1>
│     └─ IntegerLiteralReturn(existing verified plan)
└─ main: existing VerifiedNormalMain0BridgePlanV1
```

sourceとcatalogのownerはcumulative setの一箇所だけである。Main bridgeは同set
からexact Main loanを借り、既存receiptを変更せず外側aggregateへ保存する。
旧setからのruntime conversion、source分解、alias、compatibility facadeは作らない。

### C0 atomic change

```text
new:
  instance_function_plan.rs
    cumulative enum / source-owning set / all-method iteration
    common function Facts / stage-error-rejection owner
    exact ordered key coverage

retain in instance_integer_return_plan.rs:
  integer Recipe / verified plan / receiver-only fact validation
  exact signature/body/completion pairing

replace atomically:
  producer and seal API
  Main bridge instance field/input and rejection owner
  exports, tests, guard expectations

delete to zero:
  VerifiedNormalInstanceIntegerReturnPlanSetV1
  seal_instance_integer_return_plans
  old concrete-set exports/usages
  compatibility aliases/converters
```

C0は一variantだけなので、投機的なmulti-family classifierを作らない。既存planの
sole producerが`IntegerLiteralReturn`を一度だけ直接構築する。各methodは
既存順序のまま、

```text
signature
-> resolver / projection
-> facts
-> Recipe
-> completion / pairing
-> IntegerLiteralReturn variant construction
```

を一度だけ通る。rejection stageとerror vocabularyを変えず、variant failure後の
retry/reselectionを作らない。total source-family classifierは第二variantが存在する
C1で初めて導入する。

### C0 proof and size boundary

既存fixtureをrename/置換して、literal payload、canonical order、rejection stage、
Main receipt parityを維持する。catalog coverageは件数一致ではなく、
`plans.keys().eq(keys.iter())`相当のexact ordered equalityに強化する。

```text
new test/check file          = 0
parent normal-source guard   = 776 lines; unchanged
callable child guard         = 760 lines before C0
child guard update           = old concrete assertionsの置換・圧縮
child guard target           <= 795 lines
all source/check files       < 800 lines
tests.rs target              < 780 lines
```

guardへの単純追記は禁止する。新file pathの読み込み、cumulative symbols、旧symbol
zero、production consumer zeroを、旧concrete blockとのnet置換で収める。

### C0 acceptance and hard stop

```text
module source / identity / catalog owners       = exactly one
all InstanceBoxMethod keys                      = exactly once, ordered equal
public cumulative variants                      = IntegerLiteralReturn only
existing literal grammar and rejection order    = unchanged
outer aggregate name / Main0 receipt             = unchanged
old concrete set / seal API / alias / converter = 0
partial product / fallback / retry               = 0
Builder / MIR / publication / Ownership          = 0
production caller / tenth replacement row        = 0
```

source clone/reparse/reseal、old/new set共存、Main receipt再seal、検証順変更、
multi-family classifier先行、guard 800行到達のどれかが必要ならD0へ戻る。

Gate:

```bash
cargo check -q
cargo test -q normal_source_plan --lib
bash tools/checks/run_row_guard.sh --only normal-source-plan0
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
git diff --check
```

## Closed M2b execution

```text
row       = GENERAL-FUNCTION-PLAN0-MAIN0-BRIDGE0-S0
decision  = corrected C-prime
input     = VerifiedNormalInstanceIntegerReturnPlanSetV1
output    = VerifiedNormalModuleFunctionPlanSetV1
Main      = VerifiedNormalMain0BridgePlanV1
rejection = RejectedNormalMain0BridgeV1
```

one sibling-only exact Main loanが既存Main locator/resolver/source projection/
Main0 preflight kernelを一度ずつ通る。borrowed lowering inputはscope内で破棄し、
owned forest/control/completion/profileだけを、分解していないM2a ownerの横へ
保存する。Program clone、crate-wide AST escape、自己参照、第二resolver、
Main grammar差分、physical relation、partial aggregateはゼロ。

```text
closeout:
  focused normal-source tests = 76 / 76
  production Rust delta       = +249
  test Rust delta             = +184
  check Python delta          = +104
  new source files            = 1
  new test/check files        = 0
  production callers          = 0
  largest source/check file   = 776
```

## Closed M2a execution

```text
GENERAL-FUNCTION-PLAN0-INSTANCE-INTEGER-RETURN0-S0
```

```text
family       = InstanceMethodIntegerLiteralReturn0
input        = VerifiedNormalModuleSourceV1
coverage     = every InstanceBoxMethod exactly once
grammar      = no parameters; [Return(Some(Integer literal))] exactly
product      = VerifiedNormalInstanceIntegerReturnPlanSetV1
Main claim   = 0
```

exact source loan、既存receiver policy/resolver/source projection/completion
verifierを再利用し、lexical `me`を一件だけsealした。部分plan、raw AST保持、
Legacy復帰、field/Call/New、physical receiver/Ownership/ABI claimはゼロ。

```text
production caller / Builder / MIR       = 0
fallback / retry / reselection          = 0

closeout:
  focused normal-source tests = 73 / 73
  production Rust delta       = +583
  test Rust delta             = +232
  check Python delta          = +91
  new source files            = 1
  new test/check files        = 0
  largest source/check file   = 776
```

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
accepted_next_responsibility       = 1; INSTANCE-CUMULATIVE0-S0 prerequisite
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
  current authority = GENERAL-FUNCTION-PLAN0-INSTANCE-CUMULATIVE0-S0
  M2a / M2b disconnected prerequisites = closed; production caller = 0
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

### Contract and closeout

```text
root / entry                                    = Program / exact Main.main/0
module                                          = one-or-more plain instance Boxes
product                                         = VerifiedNormalModuleSourceV1
source/catalog callable correspondence          = exact
focused normal_source_plan tests                = 67 / 67
VerifiedNormalModuleSourceV1 production callers = 0
existing canonical non-Main Box rejection      = unchanged
Builder / lowering / publication delta           = 0
fallback / retry / route reselection             = 0
replacement row / credit                         = 0

new source files                                 = 1
new test/check files                             = 0
production Rust LOC delta                        = +626
test Rust LOC delta                              = +201
check Python LOC delta                           = +103
largest touched source/check file                = 776
all modified source/check files                 < 800
```

Fields and method bodies remain retained source, not yet verified plans.
Constructor, top-level function, Main(args), Builder/publication, recovery,
fallback/retry, and replacement credit remain outside this product.

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
D0 / module-source S0 status = closed
current executable row       = GENERAL-FUNCTION-PLAN0-INSTANCE-CUMULATIVE0-S0
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

M2a GENERAL-FUNCTION-PLAN0-INSTANCE-INTEGER-RETURN0-S0
    closed: exact all-instance-method integer-literal Return plan

M2b GENERAL-FUNCTION-PLAN0-MAIN0-BRIDGE0-D0
    closed: corrected C-prime exact Main0 reuse bridge selected

M2b GENERAL-FUNCTION-PLAN0-MAIN0-BRIDGE0-S0
    closed: retained M2a owner plus owned Main0 receipts

M2c GENERAL-FUNCTION-PLAN0-INSTANCE-SCALAR-BINDING0-D0
    closed: corrected Candidate C-prime selected

M2c GENERAL-FUNCTION-PLAN0-INSTANCE-CUMULATIVE0-S0
    selected: grammar-neutral cumulative one-variant owner migration

M2c GENERAL-FUNCTION-PLAN0-INSTANCE-I64-PARAMETER-RETURN0-S0
    parked until C0 green: first exact scalar-binding variant

M2c+ GENERAL-FUNCTION-PLAN0 finite family slices
    local/reassign/Binary bindings, field schema/read/write,
    default construction, Main-to-instance call

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
