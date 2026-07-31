# RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0 — ChatGPT Pro相談文

## 相談したいこと

HakoruneのMirBuilder in-place replacementで、Script（トップレベル
`Program`）をsemantic pipelineへ接続する設計を決めたいです。

現在の停止点は次です。

```text
RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0
```

直前の`RAW-SCRIPT-SEMANTIC-SOURCE0-I0-R0`は、worker監査により
NoSafeSliceで閉じました。source-kind brandingとProgram work-planの
CatalogSeal前 hoistまでは実装済みですが、Script Completeを消費する
semantic ownerがまだありません。

## 現在確認できている事実

### 既存semantic stack

```text
FunctionOwnerIdV1
  = compilation/slotのbrandで、名前やAST形状は持たない

VerifiedSemanticOwnerForestV1
  = owner mapとLambda topologyを所有するが、payloadは
    VerifiedResolvedFunctionV1

ResolvedFunctionLoweringRootsV1
  = Function scope/regionと
    FunctionBody | LambdaBodyRootを検証する

VerifiedSourceProjectionV1::seal
  = FunctionDeclaration rootを要求する

FunctionSyntaxViewV1
  = FunctionDeclaration / Lambda専用
```

`SemanticOwnerSourceKindV1::{DeclaredFunction, Script, Lambda}`は既に
既存Function/Lambda productとnormalized identityへ追加しました。しかし、
このfieldだけではProgram rootを表現できません。

### Production lifecycle

```text
RootExpansion
-> PrepareModule
-> CatalogSeal
-> PreparedProgramRootWorkPlanV1::prepare(SelectedNormal)
-> CatalogInstall
-> RootLower
-> Finalize
```

Work planは元の`ProgramBody(original ordinal)`を保持します。runtime列は
単純なbodyではなく、次のdispositionが混在します。

```text
ImmediateOnly
ImmediateAndRuntime
DeferredAndRuntime
RuntimeOnly
```

従って「Script body = Program全体」も「宣言を全部runtimeから除外」も
正しくありません。Script semantic unitは、work planが作るoriginal-ordinal
付きruntime demand windowと、別ownerへtransferされるboundaryの組です。

### Raw route

selected Scriptは現在、次の既存raw ownerを通ります。

```text
RawInvocationSourceTransportV1::script_root(())
-> RawInvocationChildPortV1
-> existing raw statement/expression lowering
```

この`script_root(())`は、Script semantic Complete productが消費できるまで
削除できません。raw/reference route全体から削除してはいけません。

### Lambda capture

既存raw Lambda captureはname-based `variable_map`です。

```text
forest.upvars()
  = BTreeSet由来のcanonical setであり、ABI slot順ではない

raw Lambda observer
  = name-basedで、nested Lambda/capture forwardingを拒否する
```

`a, z, a`のfirst-demand順と`forest.upvars()`順が一致する保証はありません。
将来はresolver traversal中に、BindingRef単位でdedupeしたordered capture
receiptを発行する必要があります。

## 今回の推奨初期closure

最初のCompleteは次の範囲に限定する案です。

```text
Script runtime demand window
  + zero-or-more Literal-only expression items
  + transferred callable boundaries
```

当面Deferredにするもの：

```text
Variable
Local（ValueId/ABI/materializationを主張する場合）
Me
Lambda
Assignment
If / Loop / QMark / Match / Tryless postfix catch/cleanup
Call / MethodCall / FieldAccess / New / Array
Box runtime demand
```

`Variable`/`Local`をsemantic factsとして先にsealする場合も、ValueId、ABI、
runtime materializationを同じrowで主張してはいけません。

実在fixture候補は、

```text
tools/checks/fixtures/raw_vm_reference_conformance/integer_0.hako
```

です。ただしこれはRaw VM reference用fixtureなので、Complete evidenceでは
Raw VM CLI成功を使わず、ファイルを一度読み、ASTをselected
`NormalCompileRequestV1`へ渡すtyped source fixtureとして扱います。

## Candidate

### Candidate A′ — private generic semantic-owner core

```text
private SemanticOwnerSyntaxCoreV1
private generic root/profile contract
VerifiedResolvedFunctionV1 = Function wrapper
VerifiedResolvedScriptV1   = Script wrapper
public FunctionSyntaxViewV1はFunction/Lambda-onlyのまま
```

forest/projection/lowering rootの共有部分をneutral coreへ抽出し、
`DeclaredFunction / Script / Lambda`をroot profileとして明示する案です。

### Candidate B — Script-specific product

```text
VerifiedScriptSemanticSourceV1
VerifiedScriptOwnerForestV1
VerifiedScriptProjectionV1
```

Script側へ専用productを作り、resolverの共通部分だけprivate helperへ抽出する
案です。Function forestとの二重authority化が懸念です。

### 明示的に不採用

```text
Program -> synthetic FunctionDeclaration
FunctionSyntaxViewV1へProgram分岐を追加
source_kind fieldだけ追加して既存Function root verifierを通す
partial forest/projectionを作る
Complete失敗後にDeferredへ降格
semantic resolver失敗後にrawへretry/fallback
```

## ChatGPT Proへ質問したいこと

1. Candidate A′とBのどちらが、既存Function/Lambda APIを汚さず、
   forest・projection・lowering rootのauthorityを一本化できますか。

2. Script rootに対して、FunctionOwnerIdV1をexecution owner brandとして
   再利用しつつ、source shapeを`Script`として型で区別する最小契約は
   どのような形がよいですか。

3. `ResolvedFunctionLoweringRootsV1`のFunctionBody/LambdaBodyRoot契約を、
   Scriptの`ProgramBodyRoot`へ安全に一般化するには、private generic core、
   root profile enum、Script専用root productのどれが適切ですか。

4. Script Completeの初期closureをLiteral-onlyへ限定するのは妥当ですか。
   Variable/Localをsemantic factsだけとして先にsealする場合、partial truthを
   作らないための境界をどう定義すべきですか。

5. Lambda captureは、resolver traversal中に発行する
   `BindingRef + first-demand source site + access facts`のordered receiptを
   ABI/materialization authorityにする方針でよいですか。
   `forest.upvars()`をcanonical setとして保持しつつ、別receiptを持つ設計に
   問題はありませんか。

6. `RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0`後の最小実装rowは、
   どのnamed production callerを選び、どのold edgeを同じcommitで削除すべき
   ですか。

7. `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001`について、
   Complete適格率を固定fixture＋typed `.hako` fixtureで単調増加させ、
   Deferred理由を増やさないratchetとして管理する契約は十分ですか。

## 回答に求める形式

```text
Decision:
Ceremony:
Selected owner/product:
Exact first Complete closure:
Named production caller:
Atomic old-edge deletion:
Deferred owner and sunset:
Required fixtures:
Hard stops:
Next executable row:
```

設計相談中はコード変更をしません。第二resolver、synthetic function、
partial forest、fallback/retry、広域parallel censusは許可しません。
