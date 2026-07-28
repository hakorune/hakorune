---
Status: SSOT
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT-POLICY-v1
Scope: Rust MirBuilderを稼働させたまま、責務単位で本番内部を交換する
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - src/mir/builder/README.md
  - docs/development/current/main/investigations/mirbuilder-structural-budget-d0-consultation-2026-07-28.md
---

# MirBuilder In-Place Replacement Policy

## Decision

最終architectureのauthorityは
`docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`である。
このpolicyは、その最終形を現在のproduction MirBuilderへin-placeで着地
させる移行規律だけを所有する。

MirBuilderは、独立した第二実装を完成させて最後に丸ごと交換する方式では
作り直さない。

現在の本番MirBuilderを動かしたまま、一つの責務を抽出し、同じ本番callerを
新ownerへ差し替え、その責務を持っていた旧branch／helper／callerを直ちに
削除する。

```text
existing production MirBuilder
  -> extract one responsibility
  -> switch its existing production edge
  -> delete the selected old edge and implementation
  -> prove post-cutover parity
  -> select the next responsibility
```

これは段階移行を否定する決定ではない。2026-07-18にLocal、Variable
Assignment、value Return、statement Ifで実際に成功したdescent方式を、
MirBuilder全体の移行規律として復活させる決定である。

各cellは、north-star上の責務／edge、named production caller、new owner、
同時に削除するold authorityを明示する。競合authorityを減らさず、単に
fixture、wrapper、guard、LOCだけを動かす作業はproduction replacement
cellとして数えない。

## Correction of the failed execution model

失敗したのは段階移行そのものではない。

```text
S0で新ownerを作る
-> I0で本番を差し替えない
-> production consumers = 0 のまま後続proofを積む
-> CUT0を遠い将来へ送る
```

という実行モデルである。

`I0`を含むrow名、fixture上の接続、candidate内部の接続、opt-in reference
laneだけの接続は、本番差し替えの証拠ではない。

## Production definition

このpolicyで`production`と数えるのは、対象cellが明示した既存のnon-test
callerから到達する経路である。

default compiler cellでは最低限、次から到達しなければならない。

```text
MirCompiler::compile_with_source
MirCompiler::compile_with_source_and_imports
  -> current default Legacy request
  -> MirBuilder::build_module / build_expression
```

AST JSON、runtime `env.mirbuilder.emit`、明示的なVM-reference backendなどを
対象にするcellは、caller familyを別に明記する。

次はdefault production callerとして数えない。

```text
#[cfg(test)] fixture
disconnected proof session
candidate-only port
production callerを持たないprepared owner
feature-gated reference laneだけのcaller
```

## Row vocabulary

### S0 — substrate

一つの既存production seamへ差し替えるための新ownerを作る。

`production caller = 0`を許すのはS0だけである。T0の機械的交換ではS0を
独立commitにせず、I0/R0と同じcommitにまとめることを優先する。

S0を分ける場合の上限は一つのlanded commitである。次のlanded commitは
必ず同じcellのI0/R0でなければならない。別cell、追加proof、追加相談を
挟んではならない。

I0/R0へ安全に進めないS0は、mainへ積み続けずrevertまたはstashして設計へ
戻す。

### I0 — production integration

`I0 closed`は次の全条件を意味する。

```text
named existing production caller uses the new owner = yes
new production edge count                          >= 1
selected old production edge count                  = 0
fallback / retry / route reselection                = 0
```

candidate内部、collector内部、prepared owner間だけの接続には`I0`を使わない。
そのような作業が必要なら`S0`、`CONNECT0`、`PROBE0`と呼ぶ。

### R0 — retirement

I0で不要になった旧branch／symbol／caller／inline orchestrationを物理的に
削除する。

T0 cellではI0とR0を同じcommitにする。Rustのbuildability上分ける必要が
ある場合だけ、I0直後の一commitをR0に予約し、他作業を挟まない。

旧helperが別の未交換branchにも必要なら、そのhelper自体を残してよい。
ただし、選択した旧edgeは0でなければならず、残るconsumerと後続packを
cell closeoutへ明記する。

### P0 — post-cutover parity

P0はI0後に、本番で選択済みの新経路をhistorical oracleまたは固定snapshotと
比較する。

I0前のfixtureやdisconnected proofへP0という名前を付けない。それらは
`PROBE0`または`VP0`である。

### G0 — macro guard

G0はcellごとに増やさない。macro packの共有guardだけが、少なくとも次を
固定する。

```text
new production caller >= 1
selected old caller    = 0
fallback / retry       = 0
detached route delta  <= 0
```

### CUT0 — authority cutover

CUT0は、production authorityの切替と旧authorityの物理削除を同じcommitで
行う場合だけ使う。

```text
production consumers = 0
```

のCUT0は禁止する。それはproof consolidationであり、`CONNECT0`または
`RET0`として扱う。

## Replacement cell contract

各cellは実装前に、active task mapまたは共有ledgerへ次を一行で固定する。

```text
cell_id
responsibility
production_caller_before
new_owner
old_edge_or_symbol_to_delete
focused_parity_gate
residual_consumers
```

closeout時に次を記録する。

```text
production_callers_before / after
old_callers_before / after
deleted_old_symbols_or_branches
fallback_count
retry_count
production_rust_loc_delta
detached_asset_delta
```

cellは`after >= 1`かつ`old_after = 0`になるまでclosedではない。

## Progress admission and circuit breaker

この節はMirBuilderのproduction replacement実装rowに適用する。D0、
障害修正、post-cutover parity、docs／asset closeoutはnon-replacementと
明記し、replacement creditを持たせない。

replacementの実装単位は次で固定する。

```text
one named production responsibility
+ one existing non-test caller
+ one selected new owner
+ one selected old-edge delete set
+ one parity / failure / reuse gate
```

既存のExpr／Stmt／Body Recipe部品の合成だけで表せる入力に、新しい
whole-function body-shape variantを作らない。signature、entry、completion、
call topology、publicationのように、本当に関数全体の義務を所有するplanは
この禁止に含めない。

新しいtype、file、test、proof、wrapperは、selected production責務、
verified contract、parity／failure／reuse evidence、または責務上必要な
size splitへ直接必要な場合だけ追加する。各assetはdurable ownerまたは
retirement conditionを持つ。disconnected route、one-row shell guard、
wrapper自体は進捗に数えない。

### Production compatibility-owner sunset law

production compatibility ownerは、一つのselected pipelineの内部で、
selected residual inputへのBuilder effectを開始する前に、migrated ownerと
total／pairwise-disjointにexactly once selectされるbounded branchとしてだけ
許可する。第二のpublic ingress、独立したcaller family、独立した
candidate／publication、どちらかのrejection後に他方を試すretry／fallback
terminalを所有してはならない。

compatibility ownerを作るcommitは、
`current-docs-update-policy-ssot.md`のsunset recordに加えて、次を同時に
登録しなければならない。

```text
compatibility_owner_id
selected_pipeline and production callers
exact residual responsibility / AST-node surface
caller_count_at_creation
sunset_id
sunset_row
retire_when
retirement owner and shared-gate evidence
```

`delete later`、未指定のfuture row、最終cleanupだけを削除条件にしては
ならない。登録後のresidual responsibility／AST-node surfaceとproduction
ingress authorityは単調非増加である。縮小または削除だけを許し、wideningが
必要なら実装を止めてD0へ戻る。caller countは観測値であり、それ自体を
authorityとしない。責務不変のhelper split等でcaller countが増える場合も、
exact before／after mappingとauthority delta = 0をD0で承認する。
ownerを分割する場合は同じcommitで旧ownerを縮小または削除し、登録済み
surfaceの総和を増やしてはならない。detached S0でownerを作る場合も、直後の
same-cell I0/R0またはrevert／stashという既定順序を緩和しない。

最終compatibility retirement rowは、作成時から登録済みのsunset ledgerを
zeroにする工程である。未登録debtを発見した場合はcloseせず、ledgerを訂正して
D0へ戻る。debtを隠したままcompletionまたは新featureへ進んではならない。

T0はatomic I0/R0を原則とする。detached S0を分けた場合は最大一commitで、
次のforward semantic commitは同じcellのI0/R0だけである。進めなければ
revert／stashしてD0へ戻る。承認済みRefactor Series Modeだけは2〜5個の
buildable commitを許すが、全commitが一目的で、series終端がnamed
production cutoverとselected old-edge deletionを閉じなければならない。

predeclared Refactor Series以外で、selected old edgeを減らさないforward
implementation commitが二つ連続したら、三つ目へ進まずdesignへ戻る。
D0、障害修正、revert、post-cutover parity、docs／asset closeoutはこの
連続数へ入れず、進捗creditにも使わない。split S0は一つ目になり得るため、
その次は必ずold edgeを減らすsame-cell I0/R0である。

```text
split S0 landed:
  next = same-cell I0/R0 or revert/stash

bounded series ended:
  selected old edge = 0
  fallback / retry / reselection = 0

otherwise:
  stop forward work
  classify disconnected assets
  return to design
```

800行境界は責務分割のhygiene triggerである。task selector、LOC非増加、
progress credit、completion conditionではない。超える前に同じselected
cell内のcohesive responsibilityで分割し、production switchをguard／testの
headroom作りだけのために遅らせない。

### Source-partition cell law

cellが宣言したcaller familyの一つのproduction selectorが、すでにliveな
複数semantic ownerへtotalかつpairwise-disjointに分岐する場合、一つのcellが
そのsource partitionをresponsibilityとして列挙してよい。

成立条件はすべて必須である。

```text
named production selector in declared caller family = exactly 1
source partition                                 = total and pairwise-disjoint
each branch enters an already-live owner          = yes
parity / failure / reuse gate                     = independent per owner
obsolete predecessor authority                    = one atomic delete set
shared semantic-owner claim                       = 0
later duplicate credit of listed owners           = 0
```

cellはsource partitionをcreditするのであって、列挙したownerが同じ意味論を
持つとは主張しない。repository内の別layerにあるobservation／resolved matcher
まで「唯一のselector」と数えてはならない。

## Ceremony

### T0 — existing behavior transport

既存のdescent／port／owner patternを本番へ移すだけならT0である。

```text
consultation card = 0
new per-cell shell guard = 0
default commits = 1 atomic I0/R0
focused fixture + shared pack guard = 1
```

### T1 — responsibility interface change

既存責務の境界を変えるが、言語意味・authority issuer・failure policyを
変えない場合だけ短いdesign noteを使う。

### T2 — new semantics or authority

新しい言語意味、source authority、identity issuer、ABI、backend policy、
failure ownerを導入する場合だけdesign stopを開く。

既存挙動を別ownerへ運ぶたびにT2相談を開いてはならない。

## No second builder / no detached expansion

禁止する。

```text
独立builder_v2を完成させて最後に全交換
Raw / Normal / Canonical / Legacyごとの第二production pipeline
production consumer 0のrouteを複数cell先まで建設
一つのfixture専用source classifierをdefault replacementとして育てる
```

許可する。

```text
同じproduction pipeline内部の責務owner交換
source-neutralなreceipt / verifier / collector / plannerの再利用
本番cutover前の最大一commitのS0
reference laneをreference laneとして維持
```

## Existing asset disposition

2026-07-22以降に追加されたdisconnected資産は、次の四つへ分類する。

```text
IntegrateNow
  exact production seamへ直ちに接続し、旧責務を削除できる

ReuseNeutral
  source-neutralなreceipt / verifier / collector / plannerとして再利用

FixtureOnly
  historical oracleまたはtest fixtureだけに残す

Delete
  route-specificで本番内部交換へ寄与しない
```

分類されない資産を新しいproduction routeへ育ててはならない。

Preloop Stage-Bのone-row source selection、special activation、専用type
publisherは現在proof-onlyであり、本番差し替えauthorityを持たない。汎用
部品だけを回収し、残りはretirement対象とする。

Raw／Canonical VM-reference laneは実consumerを持つreference laneである。
default MirBuilder内部交換とは別に維持し、default cutoverとして数えない。

## Post-replacement feature boundary

`MIRBUILDER-INPLACE-REPLACEMENT0`は既存production責務を収束させる
BoxShape laneであり、新しいsource-language semanticsを追加するBoxCount
laneではない。

source-level Ownership/View、新文法、backend widening、およびその他の
未実装featureは、下記Completionがproduction graphでgreenになった後だけ
再開する。再開時はread-only readiness inventoryから始め、
`CURRENT_STATE.toml`が一つのfeature rowを明示選択するまでparked tokenを
実装しない。

`CondBlockView`のようなanalysis-only observation viewはsource-language
Viewではない。既存挙動のproduction edge交換に必要なら、このlane内で
使ってよい。

## Finite task packs

完了分母は次の8 packに固定する。

```text
0 REPLACEMENT-LEDGER0
1 DESCENT-SPINE0
2 FUNCTION-STATE0
3 CALL-OBJECT0
4 CONTROL0
5 FUNCTION-LIFECYCLE0
6 MODULE-LIFECYCLE0
7 COMPILER-RESIDUE0
```

新しい発見は必ずこのどれかへ入れる。pack追加は、新しい言語／backend
scopeを明示的に受理するT2 decisionなしには禁止する。

## Structural observation and growth review

目的は箱を増やすことではなく、旧責務を新ownerへ移してauthorityを収束
させることである。

```text
all modified/new source and check files < 800 lines
T0 cell detached_asset_delta            <= 0
new per-cell shell guards                = 0

production Rust files / LOC              = record before / after / delta
measured source files / LOC              = record against baseline
measured test files / LOC                = record against baseline
```

個別cellのLOC符号、five-cell rolling LOC、source/test総量は実装許可を決める
hard gateではない。T1/T2の責務分割やtyped owner追加でLOCが増えることを
許容する。closeoutには増えた責務、削除したauthority、files/LOC差分、その
差分が必要な理由を記録する。

five-cell rolling値はhistorical trendとして残してよいが、窓から古い削減が
外れることを理由にcellを選んだり、必要な設計を縮退させたりしない。
削減だけを目的とする無関係なcleanupを混ぜて帳尻を合わせることも禁止する。

Structural sizeはsemantic authorityではなく増殖検知用の結果指標である。
次の二rootを固定して、`*test*.rs`とそれ以外を機械的に分ける。

```text
src/mir/builder
crates/hakorune_mir_builder
```

2026-07-28の観測baselineは次である。

```text
source files = 952
source LOC   = 182452
test files   = 139
test LOC     = 40826
```

legacy名の`mirbuilder-structural-ratchet.tsv`は一行のbaseline dataとして
保持する。shared guardは現在値とbaseline差分を表示するが、増加だけでは
失敗しない。pack closeまたは明示的な構造レビューでbaselineを実測値へ更新
する。新しいchecker、path manifest、意味分類台帳は作らない。外部
MirBuilder責務のrootが増える場合は、この固定root listを明示更新する。

## Completion

`MIRBUILDER-INPLACE-REPLACEMENT0`は次がすべて成立したときだけ完了する。

以下は最終completion条件であり、最初のproduction replacement cellを
始める前提条件ではない。移行中の一つのselected production pipelineは、
source-onlyに一度だけ選ぶ明示的compatibility ownerを含んでよいが、
rejection後のretry／fallbackはしてはならない。

```text
final pipeline north-star production conformance = green
Facts -> Recipe -> Verify -> Lower                = one-way authority
Lower / DraftSeal route redecision                = 0
CompletedFunctionDraft-only collection            = yes
partial module publication                        = 0

macro packs closed                              = 8 / 8
replacement ledger remaining                   = 0
accepted AST vocabulary classified              = 57 / 57
unclassified / wildcard production dispatch     = 0

selected old production owners / edges           = 0
detached production-capable replacement routes   = 0
fallback / retry / profile reselection            = 0

normal/default runner typed canonical ingress      = 1
normal/default route-selection authority           = 1
compile_with_source* Legacy production callers     = 0
family-specific canonical competing prod fronts    = 0
canonical rejection -> Legacy retry/fallback       = 0
direct build_module caller families reconciled    = all

Legacy-named orchestration/facade consumers        = 0
proof-only assets classified and settled          = all
full accepted corpus/backend parity                = green
```

四つのbaseline値とLOC trendは上記semantic completionを置き換えない。
作り替えのstructural impactを可視化し、増加理由をレビュー可能にするだけで
ある。

`Legacy`という語が互換data formatやdiagnostic名に残る場合は、production
orchestrationではないことをledgerへ明記する。
