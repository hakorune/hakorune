# RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-D0 — ChatGPT Pro 設計相談

## 相談したい判断

現在、MirBuilder in-place replacement の次の設計停止は
`RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-D0` です。

目的は、selected normal/default の `ScriptRoot` に対して、Builder lowering
前に既存の semantic owner authority を一度だけ用意できるかを判断することです。
まだ実装は開始していません。

## 現在のproduction graph

```text
NormalDefaultPublishedPipelineV1
  -> PreparedNormalDefaultProgramRootV1
  -> VerifiedRawRootExpansionV1::Script
  -> lower_program_root_after_catalog_install_v1
  -> RawInvocationChildPortV1
  -> RawInvocationRootLineageV1::ScriptRoot
  -> raw body / expression lowering
```

現在 `ScriptRoot` が保持するものは、owned Program AST、source path、raw
variable map、既存のBuilder/session stateです。

現在の経路にはありません。

```text
FunctionOwnerIdV1
VerifiedSemanticOwnerForestV1
VerifiedSourceProjectionV1
OwnerParentEdgeV1
exact Script root owner
parent ScopeId
```

`FunctionSourceViewV1`、`VerifiedSemanticOwnerForestV1`、
`VerifiedSourceProjectionV1` は別の compiler source-plan / resolved-callable
経路には存在しますが、selected normal/default raw production callerへは
接続されていません。

また、現在の `VerifiedSourceProjectionV1` は function root を前提にしており、
Program/Script bodyをそのまま owner root としてsealする契約は未確定です。

## ここまでの設計判断

Lambda source-lineageについて、次を確認済みです。

```text
Lambda parent site transport = closed
Lambda source-lineage co-seal = NoSafeSlice
generic semantic-owner carrier = NoSafeSlice
first missing producer        = ScriptRoot
```

Lambda固有の `FunctionOwnerIdV1` をBuilder内で発行したり、raw root key・name・
statement indexから owner を推測したりする案は禁止しています。

## ChatGPT Proに判断してほしいこと

### 1. ScriptRootの意味論

Program-owned Script bodyを、既存 `FunctionSemanticResolverSessionV1` の
semantic owner forestへ変換する正しい最小単位は何でしょうか。

候補は次です。

```text
A. Script body専用の新しい root owner / source projection を定義する
B. 既存 FunctionSourceViewV1 を Script root に拡張する
C. Scriptを synthetic FunctionDeclaration に包んで既存 resolverへ渡す
D. 現行 raw routeを維持し、ScriptRootは当面 retain-fenced にする
```

特に、CがAST rewrite／偽のsource identityになるか、A/Bが既存 function
owner semanticsを汚染するかを判定してください。

### 2. owner forestの発行場所

正しいSSOT issuerはどこでしょうか。

```text
Program root admission前
Program root expansion後
catalog seal後
raw invocation port生成時
Lambda encounter時
```

要求は、同じProgram/source rowを複数のphysical demandが通っても、
`FunctionOwnerIdV1` と forest が一度だけ発行され、同じsource ownerから
borrowされることです。

### 3. Source projectionの形

Script bodyに対して、次を一つのimmutable productとして持つべきでしょうか。

```text
owned Program/source identity
exact Script root owner
VerifiedSemanticOwnerForestV1
VerifiedSourceProjectionV1
root SourcePath / body site
```

それとも、Program source ownerとfunction semantic ownerを別productにし、
短命 loan だけで結ぶべきでしょうか。自己参照にならない lifetime/ownership
境界も示してください。

### 4. 受理範囲とfailure

Script semantic ownerを導入した場合、現在のScript grammar／diagnostic parityを
どう保つべきでしょうか。

```text
resolver failure stage
raw lowering failure stage
source identity retention
live Builder unchanged
same compiler fresh request reuse
```

Builder effect前にsemantic resolutionを行うことで、現在のfailure orderingが
変わる場合の扱いも判断してください。fallback/retryは許可しません。

### 5. 次の実行row

以下の設計が閉じた場合、最小のI0/R0をどう切るべきでしょうか。

```text
Program-owned Script source
  -> exactly one semantic root owner
  -> one owner forest including nested topology
  -> exact Script projection
  -> one borrow into RawInvocationChildPortV1
  -> existing raw lowering
```

production caller、同時に削除するold edge、success/failure terminal、focused
parity fixtureを具体化してください。

## 明示的な禁止事項

```text
FunctionOwnerIdV1の新規発行をLambda/Builder内で行う
ownerをname/symbol/RawSourceLocator/siteから推測する
resolverをLambda encounterやphysical demandごとに再実行する
Program/ASTをclone・reparse・synthetic FunctionDeclaration化する
source ownerとforestを独立に渡して後からpairingする
第二source/owner registryを作る
ClosureBodyIdをsource identityとして扱う
Lambda capture/receiver/variable_map semanticsを変更する
CallObject / NestedBoxAdmission / Ownership / Viewを同じrowへ混ぜる
```

## 求める回答形式

```text
Decision: Accept / Reject / NoSafeSlice
Recommended owner/product:
Exact source-of-truth issuer:
Production caller:
Atomic old-edge deletion:
Failure/reuse contract:
Focused tests and guard:
Hard stops:
Next executable row or prerequisite D0:
```

「ScriptRootを実装したことにする」ための薄いwrapperではなく、
Facts/Recipe/Verifyへ進める本物のauthorityかどうかを判定してください。
