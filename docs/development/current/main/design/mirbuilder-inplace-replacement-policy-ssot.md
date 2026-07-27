---
Status: SSOT
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT-POLICY-v1
Scope: Rust MirBuilderを稼働させたまま、責務単位で本番内部を交換する
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - src/mir/builder/README.md
---

# MirBuilder In-Place Replacement Policy

## Decision

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

## Growth budget

目的は箱を増やすことではなく、旧責務を新ownerへ移して総量を収束させる
ことである。

```text
all modified/new source and check files < 800 lines
T0 cell detached_asset_delta            <= 0
five-cell rolling production Rust LOC   <= 0
new per-cell shell guards                = 0
```

T1で一時的にproduction LOCが増える場合、同じpack内のrepayment cellと削除
対象を先に予約する。名前だけのsunsetは認めない。

## Completion

`MIRBUILDER-INPLACE-REPLACEMENT0`は次がすべて成立したときだけ完了する。

```text
macro packs closed                              = 8 / 8
replacement ledger remaining                   = 0
accepted AST vocabulary classified              = 57 / 57
unclassified / wildcard production dispatch     = 0

selected old production owners / edges           = 0
detached production-capable replacement routes   = 0
fallback / retry / profile reselection            = 0

compile_with_source cleaned owner graph           = 1
compile_with_source_and_imports same owner graph   = 1
direct build_module caller families reconciled    = all

Legacy-named orchestration/facade consumers        = 0
proof-only assets classified and settled          = all
full accepted corpus/backend parity                = green
```

`Legacy`という語が互換data formatやdiagnostic名に残る場合は、production
orchestrationではないことをledgerへ明記する。

