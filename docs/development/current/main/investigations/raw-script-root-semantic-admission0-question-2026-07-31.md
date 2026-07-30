# RAW-SCRIPT-ROOT-SEMANTIC-ADMISSION0-D0 — ChatGPT Pro 設計相談

## 相談の目的

`RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-D0` と
`RAW-SCRIPT-ROOT-SEMANTIC-SURFACE0-D0` は、既存 semantic machinery が
Function/Lambda root 専用であり、selected Script の受理面をそのまま載せる
安全な I0/R0 がないため `NoSafeSlice` で閉じています。

次に必要なのは実装案ではなく、Script の semantic admission と opaque
boundary を一つの SSOT として確定する設計判断です。

```text
owned Program source
  -> Script source-kind/root profile
  -> one resolver traversal
  -> one forest + one Program projection
  -> CatalogInstall前の immutable port loan
```

この相談ではコード変更を始めません。

## 現在確認できている事実

```text
AST variants                         = 57
shadow CurrentResolved vocabulary    = 34
FunctionSyntaxView root              = Function / Lambda only
VerifiedSourceProjection::seal root = FunctionDeclaration only
function-root verifier               = FunctionBody / LambdaBodyRoot only
```

selected Script の runtime admission は shadow resolver より広く、QMark、
Match、TryCatch、Throw、This 系、各種 call/object surface などを含みます。
一方、`Break` / `Continue` のように、resolver が扱えても Script root では
既存 unsupported 診断を維持し、Loop body では semantic exit として扱うべき
nodeもあります。したがって disposition は AST kind だけではなく、
`source kind + enclosing context + AST kind` で決まります。

未対応 node を単に飛ばす opaque 化は、forest / projection / fact coverage を
欠損させます。resolver に直結すると、grammar を狭めるか、診断を
`RootLower` より前へ移す危険があります。

## 決めてほしいこと

### 1. Context-sensitive admission matrix

次の disposition と、各境界が保持する exact source site / lexical effect /
failure owner を定義してください。

```text
ScriptRootProfile
ResolveLexical
OpaqueDiagnostic
OpaqueRuntimeCompletion
OpaqueCallableBoundary
OpaqueNestedOwner (Lambda inventory-only)
```

最低限、次を区別してください。

```text
Script root の Break / Continue / Return
Loop body の Break / Continue
Script内の BoxDeclaration / FunctionDeclaration
Script内の Lambda
StaticConstTable / Using / Import
QMark / Match / TryCatch / Throw / This 系
```

kind-only table ではなく、文脈依存の表として成立するか判定してください。

### 2. Opaque boundary の完全性

opaque nodeを semantic traversal が通過する場合、次をどう保証しますか。

```text
hidden declaration / capture が Script factsへ漏れない
forest / projection が partial にならない
source path が exact のまま保持される
opaque nodeの成功・失敗を既存 ownerへ一度だけ渡す
```

typed boundary を作れない場合は、opaque skip を採用せず `NoSafeSlice` と
してください。

### 3. Source-kind と唯一 issuer

実装可能になった場合の最小契約を確認してください。

```text
FunctionSyntaxViewV1 は Function/Lambda のまま
ScriptSyntaxViewV1 または private source-owner core を別に持つ
Program rootをsynthetic FunctionDeclarationに変換しない
CatalogSeal
  -> Script semantic issuer (exactly once)
  -> forest + projection co-seal
  -> CatalogInstall
```

Script product は少なくとも次を一体所有する想定です。

```text
owned Program
Script source-kind/root profile
FunctionOwnerIdV1
VerifiedSemanticOwnerForestV1
VerifiedSourceProjectionV1
```

既存 Function/Lambda product の public contract を汚さずに成立するか判定
してください。

### 4. 診断順序と再利用

semantic admission を追加しても、次を変えない契約にしてください。

```text
RootExpansion
< PrepareModule
< CatalogSeal
< ScriptSemanticSeal (必要なら)
< CatalogInstall
< RootLower
< FinalizeModule
```

duplicate catalog、unresolved lexical name、unsupported node の複合入力で、
どの error が先に出るべきかを明記してください。失敗後は candidate/session
を破棄し、同じ compiler の fresh request だけを許可します。

```text
fallback = 0
retry = 0
partial forest reuse = 0
```

### 5. 最小の実行row

上記がすべて閉じる場合だけ、次を具体化してください。

```text
Change / Contract / Done / Stop
named production caller
atomic old-edge deletion
success/rejection terminal
focused fixture and existing guard update
source/check file count and <800-line evidence
```

1回の resolver traversal と完全な facts coverage を証明できない最初の
context/surface で `NoSafeSlice` としてください。I0、production caller、
新guardは先行して発行しません。

## 明示的な禁止事項

```text
Program -> synthetic FunctionDeclaration
FunctionSourceViewV1への直接Program分岐追加
AST clone / reparse / source reread
partial forest / partial projection / best-effort facts
resolverの二重走査・別observer traversal
Lambda bodyの同時publication / capture redesign
opaque boundaryを無型のskipで済ませる
grammar / diagnostic precedence の暗黙変更
semantic rejection後のraw fallback / retry
```

## 回答形式

```text
Decision: Accept / Reject / NoSafeSlice
Context-sensitive disposition matrix:
Typed opaque boundary:
Script source-kind / root contract:
Unique issuer and stage order:
Diagnostic precedence:
Named production caller:
Atomic old-edge deletion:
Focused evidence:
Hard stops:
Next executable row or prerequisite D0:
```

実装を始めてよいのは、既存 Script grammar と診断順序を狭めず、
一回の traversal で forest / projection の完全性を保てる最小 slice が
明示された場合だけです。
