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

cell数、pack数、LOC、structural ratchetは進捗と増殖検知の手段であり、
architecture goalやcompletion authorityではない。cellはnorth-star上の
authorityを一つ減らす場合だけ選択する。

## Current front

```text
MIRBUILDER-NEXT-EDGE-DESIGN-STOP
```

最小structural ratchet、第七Binary cell、第八record-helper body descent
cellはclosed。helper driverはReturn-as-value ownerのまま、選択済み
MethodCall portをtagged child loanとして短期再借用し、旧direct edge二本と
dead same-family facade三本はゼロになった。

第九FieldAccess property getter cellもclosed。exact zero-argument adapterは
選択済みportをcatalog-child descentだけへ貸し、terminalはA1の
`lookup=None`を維持する。旧raw handler、Legacy adapter、value-only entry、
dead field facadeはゼロ。第十cellは未選択。

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
accepted_next_responsibility       = 0; design stop
detached_assets_remaining          = 2 recorded rows
legacy_production_edges_remaining  = 0 selected edges

structural_budget_status           = closed minimal ratchet
source_files_ceiling               = 952
source_LOC_ceiling                 = 182452
test_files_ceiling                 = 139
test_LOC_ceiling                   = 40826
```

Semantic evidence remains completion authority. The four ratchet values are
result metrics that fail on growth; five-cell production Rust LOC also remains
non-positive.

## Active replacement cell

```text
none
  current = MIRBUILDER-NEXT-EDGE-DESIGN-STOP
  tenth responsibility = not selected
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
default compiler ingress remain parked; none is selected by this closeout.

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
