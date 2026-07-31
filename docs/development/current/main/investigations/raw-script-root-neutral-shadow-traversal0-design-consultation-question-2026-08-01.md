# RAW-SCRIPT-ROOT-NEUTRAL-SHADOW-TRAVERSAL0-D0 — design consultation

```text
Exception:
  external architecture consultation at a shared-resolver boundary

ParentCurrentCard:
  docs/development/current/main/workstreams/
    mirbuilder-inplace-replacement-current.md
```

## Question

Hakoruneのfinal MirBuilder pipelineへ向けて、Function/LambdaとScriptが消費する
semantic traversalを一つのprivate root-neutral coreへ統合したいです。

次の条件を同時に満たすowner/product/APIと、最初のatomic production rowを設計して
ください。

```text
Program work-plan runtime demand window
  + original ProgramBody ordinal
  + typed transferred/transparent/diagnostic boundaries
-> exactly one root-neutral shadow traversal
-> Complete or Deferred selected once
-> Script root profile canonicalization
-> one shared semantic forest
-> one shared Program projection
-> existing Lower exactly once
```

最終cutoverは、現在のScript専用manual traversalを単に包むのではなく、同じcommit
で削除しなければなりません。

```text
normal_script_lexical_binding.rs::admit_expression_v1 = 0
manual visible-name map / Local / Variable fact construction = 0
second resolver = 0
fallback / retry = 0
```

## Why this stop exists

現在のScript Complete routeはproductionで動いていますが、次を独自に行います。

```text
admit_runtime_script_lexical_v1
  -> visible-name map
  -> Local declaration ordering
  -> Variable binding lookup
  -> recursive Unary / Binary / Await / Check source paths
```

一方、`resolved_semantics/shadow/**`には既に次があります。

```text
bindings / scopes / regions / exits
Local / Outbox / Assignment / If / Loop / Return
calls / object expressions / FastMem
exact source paths / Lambda owner topology / upvars
```

したがって、If/Call/Lambda等を現在のScript helperへ追加すると、第二resolverを完成
させる方向になります。これは以前止めたwhole-function variant列挙と同型の問題
です。

Census51では`UsingStatement`を既存zero-child boundaryへ追加する案も出ましたが、
今回は採用していません。StaticConstには既存metadata ownerと専用runtime terminalが
ありますが、Usingは現在generic statement-surfaceの`emit_void`であり、新しいno-op
authorityを正規化するだけになるためです。

## Existing production authorities

既にproductionへ入っているもの:

```text
ScriptSyntaxViewV1
SemanticOwnerRootProfileV1::Script
ProgramBodyRoot / ProgramBody(original ordinal)
PreparedProgramRootWorkPlanV1
one selected normal lifecycle
shared VerifiedSemanticOwnerForestV1
shared VerifiedSourceProjectionV1
VerifiedScriptSemanticSourceV1
Complete / Deferred whole-request selection
StaticConst typed transfer receipt
selected-unsupported typed diagnostic receipt
RawStructuredChildScopePortV1
existing If / FastMem / expression lowering owners
```

変更してはいけないもの:

```text
FunctionSyntaxViewV1 public contract
FunctionSourceViewV1 public contract
Function/Lambda production behavior
raw/reference routes
existing diagnostic text and order
lowering/runtime semantics
publication/result policy
```

## Exact questions

1. `ProgramBody(original ordinal)`が連続でないruntime demand windowを、既存shadow
   traversalへ渡すneutral input productはどの形がよいですか。
2. Function/Lambda public viewをProgram対応へ広げず、どのprivate coreを共有すべき
   ですか。
3. StaticConst、既存unsupported診断、top-level callable、Usingをそれぞれ
   `Resolved / Transparent / Transferred / Diagnostic` のどのtyped boundaryとして
   coverageへ含めるべきですか。Usingはproduction runtime demandとして残すべきか、
   work-plan以前のtransparent boundaryにすべきかも判定してください。
4. shadow resolverが見つけたundefined name等を、現在のRootLower first-error順序を
   変えずにComplete/Deferredへ写す契約は何ですか。
5. owner ID、forest、projectionを一回だけ発行するexact stageは、現在どおり
   `CatalogSeal -> semantic seal -> CatalogInstall`でよいですか。
6. `admit_expression_v1`とmanual Local/Variable factsをatomicに削除できる最初の
   production sliceは何ですか。Literal-only等の狭い完成プログラムvariantではなく、
   compositional責務として答えてください。
7. 最初のrowが同じcommitで削除するexact old edge、必要なfocused fixtures、
   failure/reuse contract、hard stopsを示してください。
8. 現在のComplete fixture IDsを一件もDeferredへ退行させず、既存10件すべてを
   mechanically anchored ratchetへする最小方法は何ですか。

## Candidate directions

```text
A. private root-neutral shadow traversal core
   Function/Lambda wrapperとScript wrapperが同じcoreをconsume

B. Script専用resolverを完成させる
   reject: second semantic authority

C. Programをsynthetic FunctionDeclarationへ変換
   reject: fabricated source identity

D. Usingなど小さいAST familyを先に列挙し続ける
   reject unless it deletes an authority on the shared-resolver path
```

Candidate Aを第一候補としていますが、既存型の実際の制約に基づいて補正して
ください。

## Required answer shape

```text
Decision: Accept / NoSafeSlice
selected owner and products
unique issuer and production caller
Complete / Deferred / rejection terminals
diagnostic precedence
atomic old-edge deletion
first executable row
focused evidence
800-line-safe file split if needed
hard stops
```

## Hard constraints

```text
Program -> synthetic FunctionDeclaration = 0
Function public views widened to Program = 0
second resolver / second forest / second projection = 0
partial forest = 0
AST-family-by-family whole-route enumeration = 0
Complete seal failure -> Deferred downgrade = 0
semantic rejection -> raw retry = 0
runtime compact index -> source identity = 0
source clone / reparse = 0
production caller 0 owner = 0
new per-row guard = 0
any source/check file >= 800 = 0
```
