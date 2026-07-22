# CUT0-I0 ROOT0-CANON0 SOURCE-BIND0 設計相談

Status: **回答済み — Candidate SB-prime-r1を採択。SOURCE-BIND0実行タスクへ移行**

Related:

- `cut0-i0-root0-canon0-design-question-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-design-question-2026-07-22.md`
- `cut0-i0-t-prime-r1-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## 結論を先に

追加の設計相談はまだある。現HEADのCANON0 scaffoldは箱の形を用意しただけで、
source authorityと物理receiptの唯一owner chainをまだ証明していない。

今回の棚卸しで、次の一行にまとめるべき論点が確定した。

```text
exact preflight plan
-> compiler-owned sole identity issuer
-> source-bound package (token + lowering plan + continuation)
-> active session/shell/collector
-> collector-issued receipt retained by completion
-> recursive capability install receipt co-seal
-> source-derived drain plan consumed exactly once by DRAIN0
```

この順序を飛ばしてfixtureやproduction ingressへ進むと、family-only token、
receipt drop、guardのhardcoded zero、drain時の再観測が再発する。

## 棚卸しで確認した事実

### 1. source bindingはfamily-only

`PreparedCanonicalSingleSourceV1::prepare(token, plan)`とcallable batchの
`prepare_*`はfamilyだけを照合する。同じfamilyの別preflight planを渡しても
拒否できない。これはfixture不足ではなく、exact source provenanceの欠落である。

### 2. production token producerがない

`ModuleInvocationTokenV1`の発行は現在test factoryに閉じている。global atomic、
raw ledger ordinal、route-local ordinalを追加して穴埋めするのは、ID0/BRAND0で
決めた唯一identity ownerに反する。

### 3. completionがreceiptを捨てる

canonical single / callable batch completionはreceiptを検査した後にdropする。
root witnessとcomplete productにexact collector-issued receiptが残らないため、
「source・shell・collector・receiptのco-seal」はまだ完了していない。

### 4. recursive receiptのidentityが薄い

recursive install receiptはshell portから発行されるが、receipt自身のbrandを
保持していない。source disposition、shell projection、completion productの
三者co-sealを明示しないと、別invocationからのreceipt混入を型で拒否できない。

### 5. guardは証明ではない

現CANON0 guardはfragment presenceと一部の行数を検査し、
`production_consumers=0`を文字列で出力しているだけである。focused testの登録・
実行、production callsite census、receipt field、source-bound constructor、
全touched fileの800行制限をまだ検証していない。

### 6. DRAIN0は別境界

CANON0はcomplete productとprivate drain planを発行するまでに限定する。
DRAIN0だけがcompleteをby-valueでconsumeし、source planからinventoryを投影する。
caller symbols、`require_main`、`Optional`、`current_module`再観測、shell/collector
再構築はDRAIN0でも禁止する。

## 設計質問

### Q1. source-bound packageの唯一constructorはどこか

**推奨: plan駆動private constructor**

```rust
SourceBoundCanonicalPackageV1::from_plan(
    compiler_identity_issuer,
    exact_preflight_plan,
)
```

plan variantからfamilyを導出し、constructor内部で一度だけtokenを発行する。
callerからtoken、header、catalog、familyを別々に受け取らない。

不採択:

- `prepare(token, plan)`をproductionに残す
- family-only tokenをcompletionで再検証する
- compiler planへBuilder identityを埋め込む

### Q2. invocation ID issuerのownerとlifecycleは何か

**推奨: `MirCompiler`に一つだけprivate non-Clone issuerを持たせる**

```text
MirCompiler
  owns InvocationIdentityIssuerV1
  -> source-bound package constructorだけがmint
  -> tokenをactive invocationへmove
```

global atomic、raw ledger専用ordinal、route別producer、test factoryのproduction
昇格は不採択。compiler再利用時のID増加、failure/drop時の未使用ID、thread safety、
overflow errorをissuer契約に含める。

さらにidentityのcollision domainを決める。同じordinalを持つ別`MirCompiler`を
並列に動かしてもbrandが同一になってはいけない。

```text
compiler instance/domain
  + local monotonic ordinal
  -> globally distinct opaque invocation brand
```

per-compiler `next = 1`だけの案は不採択。process-global atomicを直接routeへ漏らす
案も不採択だが、domain seedを一度だけ発行するprivate ownerを置く案は比較対象に
残す。

### Q3. plan split後のlowering authorityは何か

**推奨: packageがplanを一度だけmoveし、continuationはheader/catalog/policyだけを保持**

```text
package
  -> BrandedLoweringPlan (move, no clone)
  + NonCloneSourceContinuation (completion owner)
```

completionがplanを捨てるだけのAPIは不採択。実lowering consumerが未接続なら、
CANON0は「plan transport proof」に留め、lowering完了をclaimしない。

### Q4. exact collector-issued receiptをどこが保持するか

**推奨: route-specific root witnessがby-valueで一つだけ保持する**

```text
CanonicalSingleRootWitness
  owns InvocationBranded<CollectedDraftAdmissionReceiptV1>

CallableBatchRootWitness
  owns InvocationBranded<CallableCollectorBatchReceiptV1>
```

receiptの再branding、clone、completion後の再取得、header/catalog再観測は不可。
DRAIN0がcompleteをconsumeする際に、このreceiptをinventory proofとしてmoveする。

### Q5. recursive capabilityのco-sealをどう表現するか

**推奨: source disposition + shell install receipt + brandを同一completionで照合**

```text
Recursive source continuation
  -> shell installs exactly once
  -> RecursiveCapabilityInstallReceipt { brand, family, one-shot seal }
  -> recursive completion witness
```

Acyclicはabsence witnessを持ち、recursive markerをinstallしていないことを証明。
Copy/Clone marker値だけを保持する案、DRAIN時にshell metadataをauthorityとして
再判定する案は不採択。

### Q6. guardが何を証明すれば十分か

**推奨: static census + focused test gate + explicit file manifest**

必須:

```text
production constructor/caller count = 0 (test-onlyを除外した実census)
focused CANON0 fixture registration = 1
focused cargo test = green
source-bound API shape = exact
receipt retention field = exact
all touched source/check files < 800
```

guardのpassをsource provenanceの証明と扱わない。guardは「証拠が実行されたこと」
だけを示し、semantic co-sealはfixtureのtyped assertionsが担う。

### Q7. fixtureの実装順序を固定するか

**推奨: 次の順序を一つのtask chainとして固定する**

```text
SOURCE-BIND0:
  plan-driven package/token co-mint
  same-family foreign pairing impossible/rejected

CANON-FIXTURE0:
  single A+/trivial success and negative rows
  callable acyclic batch and late-collision zero delta
  recursive install exactly once / acyclic absence
  exact receipt retention

DRAIN0:
  consume complete by-value once
  source-derived inventory only
```

この順序以前にpublic ingressを接続しない。

### Q8. CANON0とDRAIN0の停止線はどこか

CANON0の完了条件は、source-bound package、active physical owner、receipt-retaining
route completion、private drain planの発行まで。DRAIN0はそれをconsumeする別row。

CANON0ではまだ次をclaimしない。

```text
production canonical lowering
public ingress activation
actual module drain
finalizer correctness
external commit
full selfhost / Stage-1 progress
```

## 追加Q9〜Q12: 実装前に固定すべき契約

### Q9. packageの型形状とrejected owner

**推奨: five-familyを内部enumにしたnon-Clone package**。

token、exact plan、source continuation、route policyを一つのpackage ownerが持ち、
route-specific private constructorだけがvariantを作る。`family + bool`、string
flag、`Arc`/`Clone`、looseな`(token, plan)`引数は不採択。fallible terminalは
bare errorではなくrejected ownerを返し、部分moveや`Option::take().expect`による
panic seamを残さない。

### Q10. planを本当にloweringへmoveする場所

**推奨: active invocationのprivate phase terminalを唯一のplan consumerにする**。

```text
source-bound package
-> plan-consuming lowering terminal
-> unpublished draft(s)
-> collector preflight
```

現状の`complete()`がplanを`take`してdropするだけの形は、plan transportの型が
あるだけでloweringを証明しない。実consumerが未接続の間は、CANON0をtransport
scaffoldとして明示し、canonical lowering完了をclaimしない。

### Q11. continuationのsource lifetime

**推奨: exact preflight source/catalogを借用するnon-Clone continuation**。

split後に`current_module`やcatalogを再取得しない。`Arc`/cloneでauthorityを複製する
案、lowering後に再resolveする案は不採択。入力sourceのlifetimeがcontinuation消費
まで成立することをpackage APIで表現する。

### Q12. route shapeをどの型で閉じるか

**推奨: typed route enum**。

```text
Raw
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

route policyはこのenumから導出し、callerの`require_main`、`Optional`、symbol
inventory、string flagを受け取らない。RawのMain stateをcanonicalへ流用しない。

## 座長からの追加質問: Q13〜Q14

Claude側のレビューで、Q2のcollision domainと、Q1〜Q12の重さ自体が追加の
decision boundaryとして指摘された。これは相談文へ入れるべき論点である。

### Q13. collision domainは本当にCUT0のcorrectness条件か

尖った一問はこれである。

> 並列`MirCompiler`のbrand衝突は、self-host compilerで実際に起きるシナリオか。
> 実在するなら、却下したglobal atomicへ戻らず、globally-distinctなdomain seedを
> どこから調達するのか。プロセス起動時UUID、PID、constructor callerのseed、または
> 別の一意domain ownerのどれを採るのか。

現時点のコードから確実に言えるのは、production token producer自体が未接続で、
test factoryは各インスタンスの`next=1`を持つことだけである。したがって、
「並列compilerを今すぐサポートする」ことと「単一compilerを型で閉じる」ことを
混同してはいけない。

候補:

1. **単一compiler domainを明示し、並列compilerはSOURCE-BIND0のnon-claimにする**
2. process-scoped domain seed + compiler-local monotonic ordinal
3. constructor callerがdomain seedを注入
4. global atomic invocation ID

4はhidden/global authorityになるため現方針では不採択寄りだが、2/3を選ぶなら
seed ownerとlifetimeを型・fixtureで証明する必要がある。Q2はこの選択なしにlock
してはいけない。

### Q14. 12契約のうち、どれがload-bearingか

CUT0直前に全契約を一つのrowへ詰め込むと、BoxCountとceremonyが膨らむ。次の
tier分けをPro先生へ確認する。

```text
SOURCE-BIND0 decision lock (今すぐ必要):
  Q1 exact plan-driven package constructor
  Q2 issuer owner + collision domain / parallel non-claim
  Q9 non-Clone package shape + rejected owner
  Q11 continuation lifetime

next semantic rows:
  RECEIPT0  = Q4 exact receipt retention/provenance
  LOWER0    = Q10 real plan-consuming lowering terminal
  RECURSIVE0= Q5 branded recursive install receipt
  GUARD0    = Q6 evidence-grade census + focused gate
  DRAIN0    = Q8 source-derived one-shot consumption

route vocabulary:
  Q12 is a boundary contract, but Raw ledger rewiring remains a separate
  non-claim until its own row is selected.
```

特にQ5 recursive receiptとQ2 collision domainは、A+/trivial/acyclicの最初の
vertical sliceだけなら後続へ送れる可能性がある。ただし、それを送るなら
「recursive routeはCUT0前に未対応」「parallel compilerは非対応」という明示的
non-claimを残し、将来の型変更を隠さない。

## 回答を求める最小セット

一度に全部を実装する相談ではない。まず次を決める。

```text
SOURCE-BIND0 decision lock:
  Q1 source-bound package constructor
  Q2 sole issuer + collision domain
  Q9 package shape / rejected owner
  Q11 continuation lifetime

follow-up implementation contracts:
  Q4/Q5 exact receipt retention/provenance
  Q6 guard evidence
  Q10 real plan consumer
  Q12 route shape
```

SOURCE-BIND0を閉じた後も、receipt retentionとreal plan consumerを同じcommitへ
混ぜない。BoxCount/BoxShapeを分離し、`RECEIPT0`、`LOWER0`、`CANON-FIXTURE0`、
`DRAIN0`を順番にtask化する。

## acceptance / non-claimの強制線

SOURCE-BIND0のacceptance:

```text
prepare(token, plan) production callsites = 0
sole token producer = 1
same-family foreign plan pairing = structurally impossible or typed reject
package split = exactly once
live Builder mutation before active owner success = 0
all touched source/check files < 800 lines
```

後続rowのacceptance:

```text
completion root owns exactly one non-Clone branded receipt
receipt brand == collector brand == invocation brand
recursive install receipt is branded and retained
real plan consumer count is explicit
guard derives production consumer count instead of printing a literal
```

未claim:

```text
guard green alone != semantic provenance proof
completion product exists != production lowering connected
drain plan exists != drain executed
disconnected parity != CUT0 activation
```

## 採択候補

### Candidate SB-prime（推奨）

Q1=plan-driven private package、Q2=MirCompiler sole issuer、Q3=one-time move split、
Q4=completion root witness receipt retention、Q5=branded install receipt co-seal、
Q6=real census + focused gate、Q7=SOURCE-BIND0→fixture→DRAIN0、Q8=production zero。

### Candidate SB-compat（不採択）

family-only tokenを残し、completion co-sealと文字列guardで不足を補う。foreign
same-family planを構造的に拒否できず、C-primeのauthority lawを満たさない。

### Candidate SB-global（不採択）

global atomicを新設してIDを発行する。issuer authorityがMirCompilerから漏れ、
raw ledger/route-local IDの再増殖とcompiler reuseのlifecycle ambiguityを招く。

## 回答依頼

次の四点を「採択 / 修正 / 不採択」で回答してほしい。

1. Q1/Q2: plan-driven package + MirCompiler sole issuerでよいか。
2. Q3/Q4: planを一度だけmoveし、exact branded receiptをcompletion root witnessが
   by-value保持するか。
3. Q5/Q6: recursive install receiptをbrand付きにし、guardを実census + focused
   test gateへ強化するか。
4. Q7/Q8: SOURCE-BIND0→CANON-FIXTURE0→DRAIN0の順序と、production consumer zeroを
  維持するか。

5. Q13: collision domainを今のcorrectnessに含めるか、単一compiler domainを
   non-claimとしてYAGNIにするか。
6. Q14: SOURCE-BIND0に必要なload-bearing契約をQ1/Q2/Q9/Q11へ絞り、receipt/
   lowering/recursive/guard/drainを後続rowへ分離するか。

## 実装停止線

回答が閉じるまで、`prepare(token, plan)`のproduction化、receipt retentionの実装、
fixture追加、DRAIN0、public ingress、finalizer、external commitを行わない。

## Decision closeout — SB-prime-r1

今回の回答で、次を採択する。

| 論点 | 決定 |
| --- | --- |
| Q1 plan-driven package | 採択。compiler layerのprivate constructorがexact canonical planからsource-bound packageを作る。callerはtoken/family/header/catalogを渡さない。 |
| Q2 sole issuer / collision | 採択。`MirCompiler`が唯一のproduction issuerを所有し、process-scoped compiler domain + compiler-local monotonic ordinalを論理brandとする。process-crossing uniquenessはclaimしない。 |
| Q3 one-time move | 方針を採択、実splitはLOWER0。SOURCE-BIND0ではpackageを不可分に保ち、LOWER0だけがby-valueでplanをconsumeする。 |
| Q4 receipt retention | 採択、実装はRECEIPT0。completion root witnessがcollector-issued receiptをby-valueで保持する。 |
| Q5 recursive receipt | 採択、実装はRECURSIVE0。recursive install receiptとacyclic absence witnessをbrand付きにする。 |
| Q6 evidence guard | 採択。実census、focused test gate、explicit manifestを分離して各rowで固定する。 |
| Q7/Q8 order and stop line | 修正採択。`SOURCE-BIND0 → LOWER0 → RECEIPT0 → RECURSIVE0/GUARD0 → CANON-FIXTURE0 → DRAIN0`。production ingress/capture/drain/finalizer/commitはP0までゼロ。 |
| Q9 package shape | 採択。SOURCE-BIND0はA+、BindingSsaTrivial、BindingSsaAcyclic、BindingSsaRecursiveのcanonical 4 variantだけ。Rawは閉じたRAW0 chainを維持する。 |
| Q11 continuation lifetime | 採択。singleはplanからsealしたexact headerをownedで保持し、callable batchはexact verified source/catalogを借用する。再resolve・再取得・`Arc`/`Clone`は禁止。 |
| Q13 collision domain | 採択。parallel/multi-compilerでもlocal ordinal衝突を同一brandと誤認しないためdomainを含める。domain allocatorの実方式はissuer内部に閉じ、routeへ漏らさない。 |
| Q14 load-bearing tier | 採択。SOURCE-BIND0はQ1/Q2/Q3方針/Q9/Q11/Q12/Q8 non-claimに限定し、receipt/lowering/recursive/fixture/guard実装/drainを後続rowへ分離する。 |

### SOURCE-BIND0の実行境界

```text
exact canonical preflight plan
  -> MirCompiler-owned token issuer
  -> non-Clone SourceBoundCanonicalPackageV1
  -> rejected owner on validation/issuer failure
  -> route-specific continuation
```

SOURCE-BIND0では、`prepare(token, plan)`をproductionへ残さない。packageのpublic
split terminal、`Option<Plan>`、`take().expect`、caller-authored family/header/catalog、
canonical production lowering、receipt retention、recursive install、DRAIN0、public
ingress、external commitも扱わない。

実装タスクは次のカードへ移した。

`cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
