---
Status: SSOT
Date: 2026-09-01
Decision: MIRBUILDER-INPLACE-REPLACEMENT-POLICY-v1
Scope: Rust MirBuilderを稼働させたまま、責務単位で本番内部を交換する
Related:
  - docs/development/RULES.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - src/mir/builder/README.md
  - docs/development/current/main/investigations/mirbuilder-structural-budget-d0-consultation-2026-07-28.md
---

# MirBuilder In-Place Replacement Policy

## Current Capsule

実装着手・停止・検証・commit・退役の運用正本は
[開発ルール](../../../RULES.md)だけである。このpolicyは本番交換の構造を
説明し、独立した停止条件やcommit数上限を追加しない。現在のユーザー指定の
相談停止・goal pauseを解除するものではない。

- **Current decision:** replace one live production responsibility in place;
  do not grow a second Builder or a chain of disconnected prerequisites.
- **Current implementation status:** caller/owner/old-edge accounting is the
  accepted migration discipline.
- **Next ordered task:** the active pointer selects the only executable cell;
  this policy does not select a row.
- **Production series:** one source family and responsibility connect the
  canonical issuer to its named final consumer; construction and retirement
  timing follow RULES section 4.
- **Retirement finish line:** the live caller uses the new owner, the selected
  old edge is physically gone, and fallback/retry/reselection are zero.

## Decision

最終architectureのauthorityは
`docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`である。
このpolicyは、その最終形を現在のproduction MirBuilderへin-placeで着地
させる移行規律だけを所有する。

MirBuilderは、独立した第二実装を完成させて最後に丸ごと交換する方式では
作り直さない。

一つの本番pipeline内で、一つのsource familyから最終consumerまでの
責務をまとまりとして置き換える。既存owner内部のまとまった書き直しも
許可する。同じ本番callerを新ownerへ接続し、選択した旧edgeと専有資産は
RULES第4・6節のcaller-zeroと必要検証が成立した同じseriesで物理削除する。
共有ownerや未選択callerを残す理由は既存cardに明示する。

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
  -> typed normal Program request
  -> candidate root/catalog lifecycle
  -> typed Program lowering and atomic publication
```

The post-C0 current decision explicitly classifies the repository-public
`compile_with_source*` surface as a retained whole-file AST Compatibility
adapter (`MIRCOMPILER-PUBLIC-AST-COMPAT-PARK-D0`). It is not the canonical
MIR/LLVM production ingress while the finite census has zero non-test
downstream callers. This is a scoped `ParkedSealed` classification, not API
removal or permission to synthesize parser/macro source facts. Any future
production cell must name a separate actual caller family and preserve the
adapter edge until its own exact sunset contract closes.

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

### Reachability census vocabulary

replacement／retirement cellのcensusは、次の4軸を混ぜずに記録する。

```text
structural_sites:
  code上に存在するproducer／branch／consumer site数

production_reachable_callers:
  named non-test ingressから全upstream gateを通過し、対象edgeへ到達できるcaller数

test_only_injections:
  cfg(test)、fixture、direct authority injection、disconnected proofからだけ開くsite数

public_contract_owners:
  language／CLI／profile／reference acceptanceがその挙動を要求するnamed owner数
```

各censusは`start -> end`、includes、excludesを同時に固定する。grep上のbranch、
classifier variant、shared consumer、test fixtureの存在だけで
`production_reachable_callers > 0`と数えてはならない。逆にpublic contract ownerは
現在のproduction reachが0でも削除許可へ読み替えない。

4軸は責務の分類に使う。建設・退役の許可判断はRULES第4節に従う。

```text
production_reachable_callers > 0:
  named caller、新owner、選択旧edgeを同じ本番seriesで置き換える

production_reachable_callers = 0 and structural_sites > 0:
  上流gateで止まる選択実sourceの責務と、利用callerのない資産を区別する
  前者の接続はseriesの出力、後者はcaller-zero reconciliationまたはRET0

production_reachable_callers = 0 and test_only_injections > 0:
  fixtureはproduction acceptanceにしない。削除、移行、またはtyped retirementへ閉じる

public_contract_owners > 0:
  production reachにかかわらず、明示Decisionとsuccessorなしにretireしない

未分類または境界未確定:
  source authority・意味・失敗境界を確認し、RULES第4・5節の判断を適用する
```

### Caller-first vertical admission gate

Before adding another precursor row, classify the intended semantic Promote
vertical as one responsibility with the tuple below. Stop/Delete instead use
the [construction/retirement proof timing](../../../RULES.md#4-建設と旧経路退役の証明タイミング).
A finite caller set may share one owner; it is not competing authority.

```text
exact semantic family
finite named set of live non-test callers
canonical source issuer and selected new owner
lossless selected result/effect/ABI/backend path
finite old-edge delete set
```

`lossless selected path` means a settled contract for consuming the canonical
product without name/registry repair, fallback, or semantic reconstruction.
Implementing/connecting that named consumer and proving execution are series
outputs; do not require the replacement to be implemented before starting.
`RejectBeforeEffect` proves only a retirement row, not a successful vertical.

一つのseriesは複数commitに分けてもよいが、同じsource familyの
producerから最終consumerまでを接続する一責務を持つ。建設の入口、設計へ
戻る条件、旧edge削除の証拠はRULES第4・5節に従う。未実装consumerや
caller-zeroを着手前のgreen条件にはしない。準備・fixtureの局所greenだけでは
製品到達、本番交換、退役を主張しない。

同じ責務のcaller・consumerを発見したら、そのmappingを既存cardで確認する。
選択source authority・意味・失敗境界を広げる必要がある場合は設計へ戻る。
WIPは未完条件とともに保持し、commit数だけでrevert・stash・parkしない。
別責務へ移る場合のpointerと引継ぎはRULES第2節に従う。

## Row vocabulary

### S0 — substrate

一つの既存production seamへ差し替えるための新ownerを作る。

本番接続前のprepared ownerは本番交換の証拠ではない。S0を分ける場合も
同じseriesのnamed caller、最終consumer、未完の接続と退役条件を保持する。
準備を独立した製品進捗として積まず、必要な検証が閉じた接続・専有資産退役へ
進む。commit数や追加発見だけで停止条件を作らない。

## Design before substrate

S0や新しい`Verified*` / `Prepared*`を作る前に、対象責務の
production意味を次の順で閉じる。

```text
all production consumers
  -> all accepted / rejected states and counterexamples
  -> source authority + sole issuer
  -> preservation / scoped loan
  -> atomic production cutover
  -> old authority retirement
```

subset用のadmission productの`Outside`、`Deferred`、或いはbooleanを、
total execution roleの既定値として読み替えない。consumer censusは直接caller
だけでなく、そのproductから派生するrole、catalog、work plan、
dispatch、physical entryまで閉じる。一つでも未分類のstateまたはconsumerが
残る場合は選択責務の意味と失敗境界を確認し、RULES第4・5節の
建設条件と設計停止判断を適用する。未実装や未実行のconsumer自体を
建設前の完成条件にはしない。

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

I0とR0は同じbounded seriesで閉じる。必要なbuildability・検証のため
commitを分けてよい。物理削除はRULES第4節の条件が成立した専有集合から行う。

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

CUT0はproduction authorityの切替と選択旧authorityの物理削除を
閉じた成果を表す。実行時期とcommit分割はRULES第3・4節に従う。

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
structural_sites_before
production_reachable_callers_before
test_only_injections_before
public_contract_owners_before
production_caller_before
new_owner
old_edge_or_symbol_to_delete
focused_parity_gate
residual_consumers
```

closeout時に次を記録する。

```text
production_callers_before / after
structural_sites_before / after
production_reachable_callers_before / after
test_only_injections_before / after
public_contract_owners_before / after
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

R3のJoinModule/reference D0は、caller-zero assetまたはfenced reference
familyについて一つのRET0／REOWN／RETAIN-FENCEDを選ぶnon-replacement
disposition selectorである。production replacementの「D0の次は同scope
I0/R0」規律をR3へ拡張しない。代わりに、選択時にexact surface、consumer、
owner、sunset、retire_whenを固定し、closeout後はfresh censusへ戻る。

### Premise-reset breaker

同じ責務での再停止の扱いはRULES第5節を適用する。既存cardで
semantic unit、全classifier arms、transferred subtrees、型要件、counterexampleを
確認し、一つのDecisionに統合する。新しい全repo census、proof台帳、guardを
監査の着手条件にしない。検証と計算資源はRULES第6節に従う。

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
total／pairwise-disjointにexactly once選ぶbounded branchとしてだけ許可する。
第二のpublic ingress、独立したcandidate／publication、双方向の
retry／fallbackを所有してはならない。

作成commitでは`current-docs-update-policy-ssot.md`のsunset recordへ、次を
exactに登録する。

```text
selected_pipeline and production callers
exact residual responsibility / AST-node surface
caller-count baseline (observation only)
sunset row and retire_when
```

登録後のresidual surfaceとproduction ingress authorityは拡大禁止である。
caller countは観測値なので、責務不変の増減はcloseoutでexact mappingを示せば
よい。surfaceまたはauthorityが広がる場合だけ実装を止めてD0へ戻る。

最終retirement rowは登録済みsunset ledgerをzeroにする。未登録debtを
発見したらcloseせず、ledgerを訂正してD0へ戻る。

### R4 fence / residual registry law

R4 final conformanceの判定対象は、active workstream card内の一つの
`R4 fence / residual registry` にだけ登録する。`fenced`、`separate`、
`reference` という本文中の総称はregistry rowを代替しない。

active fenceまたはproduction compatibility residualの各rowは、少なくとも
次を持つ。

```text
stable ID
exact owner / asset surface
normal/default reachability
explicit activation or selected caller
retire / reown / retain-fenced disposition
release row and retire_when
```

既存のIDを持たないfamilyは`unregistered`としてregistryへ載せるが、
`RETAIN-FENCED`とは数えない。unregistered rowは、次のR3 disposition D0
またはR4 final conformanceで上の全fieldを満たすnamed rowへ解決するまで
R4 Completeを妨げる。

fenceまたはcompatibility residualを作成・拡大・再分類するcommitは、同じ
commitでregistry rowを追加または更新する。closeoutのcensusはregistered /
unregistered / closedの件数を報告し、`LegacyChildDraftAdmissionV1` のような
source occurrence countはowner-disposition数と混同しない観測値として別記する。

seriesの本番成果は、実sourceの到達、named callerの切替、重複責務の
解消、選択旧edgeの物理削除で記録する。準備commit数やtest数は成果の代わりに
しない。新しい受理形の追加はfeature additionとして区別し、存在しない旧edgeの
退役creditを付けない。未完seriesの次の判断はRULES第2・4・5節に従う。

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
commit grouping = one responsibility; RULES sections 3--4
focused fixture + shared pack guard = 1
```

### T1 — responsibility interface change

既存責務の境界を変えるが、言語意味・authority issuer・failure policyを
変えない場合だけ短いdesign noteを使う。

### T2 — new semantics or authority

新しい言語意味、source authority、identity issuer、ABI、backend policy、
failure ownerを扱う場合の建設・設計停止判断はRULES第4・5節に従う。

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
同じ本番series内のS0（準備だけでは製品進捗にしない）
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

source-level Home ownership、新文法、backend widening、およびその他の
未実装featureは、下記Completionがproduction graphでgreenになった後だけ
再開する。再開時はread-only readiness inventoryから始め、
`CURRENT_STATE.toml`が一つのfeature rowを明示選択するまでparked tokenを
実装しない。

`CondBlockView`のようなanalysis-only observation viewは、source-language
Home handle/result relationではない。既存挙動のproduction edge交換に必要
なら、このlane内で使ってよい。

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
