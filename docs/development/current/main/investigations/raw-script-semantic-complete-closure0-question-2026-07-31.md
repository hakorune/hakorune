# RAW-SCRIPT-SEMANTIC-COMPLETE-CLOSURE0-D0 — 設計相談

## 相談の目的

`RAW-SCRIPT-PROGRAM-ITEM-ADMISSION-SSOT0-I0-R0` は closed しました。
selected Script の Program item disposition は一つの source-only policy に
統合されましたが、semantic owner / forest / projection はまだ発行していません。

次に必要なのは、Script semantic ownerを一括実装することではありません。
既存の production authority を壊さずに切れる、最小の compositional closure と
その前提rowを決めることです。

```text
owned Program
  -> one source admission SSOT
  -> one selected pre-RootLower authority
  -> RootLower exactly once
```

この相談ではコード実装を始めません。

## 現在の事実

```text
Program source spine:
  ProgramBodyRoot + original Program ordinal = landed

Program-item admission:
  NormalScriptProgramItemAdmissionV1 = one source-only SSOT
  former runtime -> non-Box classifier chain = deleted

semantic machinery:
  FunctionSyntaxViewV1 / FunctionSourceViewV1 = Function/Lambda rooted
  VerifiedSourceProjectionV1::seal = FunctionDeclaration rooted
  selected Script runtime surface = broader than current shadow resolver

production semantic Script owner caller = 0
partial forest/projection             = 0
second resolver                       = 0
fallback/retry                        = 0
```

`RAW-SCRIPT-ROOT-EXACT-PROGRAM-SOURCE0-I0-R0` と
`RAW-SCRIPT-PROGRAM-ITEM-ADMISSION-SSOT0-I0-R0` は、Program identity と
source-only admissionを整えただけです。ScriptをFunctionとして扱えること、または
全57 AST kindをsemantic resolverへ載せられることは証明していません。

## Hakoruneの構文補正（必須）

Hakoruneにはsource `try` 文とsource `throw` 文はありません。
正規の対象は、保護対象のscope / expression / member bodyの後ろに付く
postfix構文です。

```text
protected region
  catch (...) { ... }
  cleanup { ... }
```

```text
source try        = rejected
source throw      = rejected
postfix catch     = canonical target, RecoverableFailure laneは別D0
postfix cleanup   = canonical target, cleanup laneは別owner
ASTNode::TryCatch = internal/legacy normalized carrier only
```

`ASTNode::TryCatch` は postfix catch/cleanup のnormalized carrierだけでなく、
fini markerやgenerated property wrapperなどにも使われます。AST kindだけから
canonical source meaningを推測してはいけません。canonical language rowで
multiple catch、first-catch-only、catch binder、cleanup環境変数を新たに確定
しないでください。

## 決めてほしいこと

### 1. 次の最小row

次の候補を比較してください。

```text
A. RAW-SCRIPT-ROOT-PROFILE-TRANSPORT0-I0-R0
   ProgramBodyRoot / Script source-kindを既存transportへ型付きで渡すだけ。
   semantic owner / forest / projection = 0。

B. NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-I0-R0
   CatalogSealをroot lifecycleの一箇所へ移し、CatalogInstallより前の順序を
   物理的に固定する。Script resolverはまだ接続しない。

C. RAW-SEMANTIC-OWNER-SOURCE-PROFILE0-S0
   Function/Lambda public productを汚さず、既存resolver内部のsource-profile
   共通部だけを挙動不変で抽出する。S0なら直後のI0が必須。

D. RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-I0-R0
   Script root owner / forest / projectionをproductionへ一括接続する。
```

`D`を採用する場合は、全57 kind、context-sensitive exit、Lambda、opaque
boundary、診断順序を一つのI0で閉じる本当の根拠を示してください。示せない場合は
`NoSafeSlice`とし、A/B/Cのどこまでを実行できるかを切り分けてください。

### 2. Semantic terminal

`DeferredExistingDiagnostic`という名前だけに依存しないでください。既存
RootLowerが成功する内部carrierもあります。必要なら、次のように「既存の
RootLower authorityへ一度だけ渡す」typed terminalを検討してください。

```rust
enum ScriptAdmissionTerminalV1 {
    SemanticEligible(/* complete owner/forest/projection */),
    ExistingRootLowerAuthority(/* no forest/projection */),
}
```

これはsemantic resolverを試して失敗した後のfallbackではありません。RootLower
開始前に一度だけ終端を選び、選択したauthorityを一度だけ実行する契約です。
ただし、CompleteからExistingRootLowerAuthorityへの降格、semantic rejection後の
raw再実行、partial forestの保持は不可です。

### 3. Context and opaque boundary

semantic closureを主張する場合、最低限次を明示してください。

```text
Script root / Loop body の Break・Continue
Return target
Lambda definition site / parent scope
BoxDeclaration / FunctionDeclaration callable boundary
EnumMatchの実際のchild demand
postfix protected-region carrierの扱い
cleanup / RecoverableFailureの未確定境界
```

opaqueをbool skipで表現せず、次のどれかをtyped ownerとして明示してください。

```text
ResolvedInCurrentOwner
TransferredCallableBoundary
NestedOwnerDefinition
ExistingRootLowerAuthority
```

forest/projectionの完全性を証明できないnodeを、見なかったことにして
`SemanticEligible`へ入れてはいけません。

### 4. Diagnostic / reuse contract

次の順序を保てるか明記してください。

```text
RootExpansion
< PrepareModule
< CatalogSeal
< CatalogInstall
< RootLower
< FinalizeModule
```

duplicate catalog、unsupported carrier、unresolved lexical nameが混在するsourceで
diagnostic stage/text/orderを前倒ししないこと。失敗時はcandidate/sessionを破棄し、
同じcompilerのfresh requestだけを許可します。

```text
partial forest reuse = 0
fallback             = 0
retry                = 0
```

## 回答形式

```text
Decision: Accept / Accept-corrected / NoSafeSlice
Chosen next row:
Ceremony:
Exact input/output product:
Source-kind / root contract:
Semantic terminal and failure owner:
Named production caller:
Atomic old-edge deletion:
Postfix catch/cleanup and ASTNode::TryCatch boundary:
Focused evidence:
Hard stops:
Next executable row or prerequisite D0:
```

## 明示的な禁止事項

```text
Program -> synthetic FunctionDeclaration
FunctionSyntaxViewV1 / FunctionSourceViewV1へ直接Program分岐を追加
AST clone / reparse / source reread
partial forest / partial projection / best-effort facts
Script専用の第二resolver / 第二registry / 別observer traversal
ASTNode::TryCatchをsource tryまたはcanonical RecoverableFailureへ昇格
multiple catch / first-catch-only / catch binderをこのrowで言語仕様化
cleanup環境変数をsemantic SSOTへ昇格
Lambda capture / ClosureBodyId / publicationを同じrowで変更
semantic rejection後のraw fallback / retry
production caller 0のproof routeを増設
new per-row guard
```

実装を開いてよいのは、既存Scriptのgrammar・diagnostic parityを狭めず、
一回のsemantic traversalと完全なsource ownershipを証明できる最小rowが返った
場合だけです。
