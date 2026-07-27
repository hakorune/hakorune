---
Status: Active workstream
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

現在の本番MirBuilderを稼働させたまま、責務ownerを一つずつ交換する。

```text
new owner
-> existing production caller switch
-> selected old branch deletion
-> post-cutover parity
```

独立した第二MirBuilder、遠い一括CUT0、production consumer 0のroute建設は
行わない。

## Current front

```text
MIRBUILDER-NEXT-EDGE-DESIGN-STOP
```

第六cellのReturn source partitionはclosed。value-bearing generic ownerと
exact Void leafが本番selectorを分有し、旧Option/raw facadesは物理削除済み。
第七production edgeは未選択であり、fresh bounded censusなしにsource editへ
進まない。

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
| COMPILER-RESIDUE0 | old facades／selectors／proof routes | Legacy orchestration consumers 0 |

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
live_replacement_cells_closed      = 6
replacement_ledger_remaining       = 0 scheduled production rows
detached_assets_remaining          = 2 recorded rows
legacy_production_edges_remaining  = 0 selected edges
```

LOC and file counts are supporting evidence, not semantic authority. The
five-cell rolling production Rust LOC budget must still be non-positive.

## Active replacement cell

```text
none
```

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
