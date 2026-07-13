---
Status: accepted; B0-L1/B0-L2a closed; B0-L2b selected
Date: 2026-07-13
Scope: B0-L0 canonical Lower ingress and exact source-site carrier
Parent: mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
Decision: A-prime_staged_carrier_then_atomic_sa3b_then_blockexpr_lower
---

# B0-L0 Canonical Lower Ingress / Source-Site Carrier Consultation

## Accepted resolution

Decision A′ is accepted with this exact order:

```text
B0-L1  ingress inventory
B0-L2a typed source-unit ingress (activation zero)
B0-L2b immutable FunctionSourceView / LocatedNode navigator (activation zero)
B0-L2c closure-scoped function transaction (behavior preserving)
SA3-B   first closed owner-family atomic identity activation
B0-L3a straight-line BlockExpr resolved ScopeId/RegionId consumption
B0-L3b/B0-L4/B0-L5 located control flow, CorePlan, Lambda child transport
```

The exact source carrier is immutable: `FunctionSourceViewV1` produces
`LocatedBodyV1` / `LocatedStmtV1` / `LocatedExprV1`. A mutable Builder cursor,
AST pointer, Span, name, or encounter order is never identity authority.

Function families may activate in stages, but one source unit is
all-or-nothing. `CanonicalFunctionLoweringSessionV1` owns the closure-scoped
transaction and explicit fallible cleanup; Drop is only an assertion and
panic-safety backstop. Carrier infrastructure may land disconnected, but the
first production canonical owner must atomically install the sealed product,
adopt exact BindingIds, resolve use/assignment sites, forbid legacy allocation,
account source coverage, and close cleanup before function publication.

## 相談したいこと

Resolved Semantic Owner Forest V1 の BlockExpr lexical semantics は B0-F
まで閉じた。次の B0-L では canonical Rust Lower が sealed product の
`ScopeId` / `RegionId` / `BindingRefV1` を使う必要がある。

しかし現在、production には次の二つの入口がない。

```text
sealed semantic product / owner forest production install = 0
Lower SourceStmtSiteV1 / SourceExprSiteV1 carrier = 0
```

この状態で `exprs.rs` の BlockExpr arm に `LexicalScopeGuard` だけを足す
と、名前ベースの旧 Lower の挙動は改善するが、resolved identity cutover
にはならない。また product を install しながら BlockExpr 内 Local が
legacy allocator を使うと、同一宣言へ resolver と Lower が別々の
BindingId を発行する partial truth になる。

相談したい中心点は次の三つ。

1. canonical source と legacy/compat source を Lower 前のどの入口で型分離するか。
2. exact source-site cursor の owner をどこに置くか。
3. source-site carrier と SA3 BindingId authority cutoverを同じatomic sliceにするか、段階分割できるか。

## 現在閉じている契約

B0-F は次を fixture と guard で固定済み。

```text
every canonical BlockExpr owns one sealed ScopeId/RegionId pair
pair origin = exact Source(BlockExprPreludeRoot)
prelude resolves in source order; tail resolves exactly once
tail sees completed prelude declarations
inner declarations do not leak after the expression
same-name shadow restores the outer binding
outer binding rebind survives BlockExpr exit
condition BlockExpr scope ends before If branch / Loop body
non-local exits escaping BlockExpr are rejected recursively
nested-loop Break/Continue remain accepted
Lambda children observe exact prelude/tail declaration order
normalized owner-forest parity is green
Planner / RegionFlow / Lower connection = 0
```

ProgramV0 は source-scope authorityではなく、新しいv0 tagや互換carrierも
追加していない。

## Production Lower の現状

現行の直接Rust Lowerは次の形。

```text
MirCompiler::compile_with_source_internal
  -> MirBuilder::build_module(ASTNode)
  -> function body lowering
  -> build_statement / build_expression
  -> exprs.rs BlockExpr arm
       non-local-exit precheck
       prelude build_statement in current scope
       tail build_expression once
```

問題点:

- `build_module` 以降は裸の `ASTNode` で、canonical/compat provenanceを失う。
- recursive Lower APIは `SourceStmtSiteV1` / `SourceExprSiteV1` を持たない。
- BlockExpr armはlexical scopeをpush/popしない。
- `ResolvedBindingLoweringStateV1::install` はtest-onlyでproduction callerがない。
- `declare_resolved_local_in_current_scope` もproduction callerがない。
- `finish_claims()` はfunction全宣言のall-or-nothing accountingを要求する。
- owner forestのLambda child productをLowerへ渡すproduction transportもない。
- function loweringの一部は途中の `?` でcontext/region cleanupを飛ばし得る。

既存 `LexicalScopeGuard` はerror-safeなname/ValueId/BindingId restorationには
使えるが、resolved `ScopeId` / `RegionId` を受け取らないためidentity consumer
ではない。

## Source authority

採用したいauthority境界:

```text
semantic authority:
  VerifiedResolvedFunctionV1
  VerifiedSemanticOwnerForestV1

identity lookup authority:
  exact SourceStmtSiteV1 / SourceExprSiteV1

BlockExpr identity:
  exact source site -> sealed ScopeId + RegionId pair
```

非authorityとして固定したいもの:

```text
AST pointer identity
Span
variable name
Lower traversal/encounter order
raw ScopeId ordinal construction
producer path or producer kind
ProgramV0 reconstruction
name-keyed Planner state
```

## 選択肢

### A. B0-LをSA3-Bのtyped production ingress後へ並べ替える（推奨）

先に次を設計・導入する。

```text
ResolvedFunctionLoweringInputV1 {
  borrowed syntax,
  sealed function product or owner forest,
}

LegacyFunctionLoweringInputV1 {
  explicitly non-authoritative legacy input,
}
```

routeはLower開始前に一度だけ選択する。canonical route選択後のresolver/
Lower failureはcontract errorであり、legacy routeへretryしない。

その後、exact source cursorと全宣言のatomic BindingId cutoverを閉じ、B0-L
でBlockExpr resolved scope lifecycleを接続する。

利点:

- source authorityとroute ownerが明確。
- partial BindingId truthを構造的に禁止できる。
- BlockExprだけを特例接続しない。

欠点:

- B0-L単体では進まず、SA3-Bとのtask order変更が必要。

### B. B0-L内でtyped ingressとsource cursorまで一括導入する

B0-Lを拡張し、function ingress、owner forest install、source cursor、全宣言
claim、BlockExpr scope cutoverを一つのatomic seriesとして実装する。

利点:

- BlockExpr cutoverまで連続して閉じられる。

欠点:

- 実質的にSA3-Bを内包する。
- 1 blocker = 1 semantic sliceとしては広すぎる。
- function family、CorePlan、Lambda childまで同時に影響しやすい。

### C. BlockExprへlegacy LexicalScopeGuardだけを先行追加する（却下案）

名前可視性だけなら改善するが、resolved ScopeIdを使わない。B0-L完了を
claimできず、後のcanonical cutoverと二重実装になるため採用しない。

## 推奨するtask order

選択肢Aを推奨する。

### B0-L1: canonical function ingress inventory

次を全件分類する。

```text
free/static function
constructor
instance/static method
inline Main / callable Main
script main
REPL
Lambda child owner
CorePlan-produced function body
AST JSON / ProgramV0 compatibility route
```

各rowで、route selection site、AST clone/by-value seam、function syntaxが
params/bodyへ分解される地点、resolverを起動できる最終地点を記録する。

### B0-L2: typed route and source cursor contract

- canonical/legacy inputを型で分離する。
- route selectionをLowerより前に一度だけ行う。
- borrowed structural cursorのowner/APIを決める。
- `Body(index)` から全nested child segmentをexactに運ぶ。
- pointer/name/Span/encounter-order lookupをguardで禁止する。
- success/errorの両方でfunction contextと全stackをrestoreするownerを決める。

このsliceは挙動不変にする。

### SA3-B: atomic declaration identity cutover

- receiver、parameters、locals、outboxをexact declaration siteでclaimする。
- install後のlegacy BindingId allocationを禁止する。
- exact variable-use/assignment-target siteからBindingRef/ValueIdを引く。
- Lambda child productをowner forestから選択する。
- publication前に `finish_claims()` を必須にする。

### B0-L3: resolved BlockExpr Lower cutover

```text
lookup exact BlockExpr ScopeId/RegionId pair
enter resolved lexical scope
lower prelude in source order
lower tail exactly once
retain tail ValueId
leave scope on success/error
publish verified outer rebind effects
```

## 最小インターフェース案

名称は相談後に確定するが、責務は次のように分けたい。

```text
builder/resolved_lowering/
  input.rs        typed canonical/legacy route input
  source_path.rs  borrowed exact source cursor
  scope.rs        sealed ScopeId/RegionId enter/leave guard
  block_expr.rs   BlockExpr-specific orchestration only
  README.md       authority, forbidden lookup, cleanup law
```

BlockExpr boxはpolicyを再判定しない。sealed productとexact cursorからpairを
取得し、既存Lowerを順番どおり呼ぶだけにする。

## Fail-Fast境界

次はsilent fallbackせずtyped/freeze contract errorにする。

```text
canonical route without sealed product
missing exact source site
site exists but is not a BlockExpr pair
foreign owner ScopeId/RegionId
scope/region pair mismatch
declaration claimed twice
installed product followed by legacy BindingId allocation
unclaimed/unpublished declaration at function finish
Lambda child product missing at exact definition site
scope/context stack mismatch after success or error
```

新しいdebug環境変数や無条件ログは追加しない。

## 実装fixture候補

設計決定後のfocused gateでは少なくとも次を固定する。

```text
typed route selected before Lower; retry count = 0
empty BlockExpr enters/leaves exact pair
tail lowered exactly once and ValueId remains usable after pop
inner local nonleak
same-name shadow restoration
outer resolved rebind survives pop
initializer-before-declaration
nested BlockExpr exact pair nesting
wrong/missing/foreign site fail-fast
prelude/tail injected error still balances every stack/context
all source declarations receive an explicit coverage disposition
all materialized declarations are published exactly once
Planner / RegionFlow / Recipe connection remains 0
ProgramV0 compatibility behavior remains unchanged
```

If/Loop/CorePlan and Lambda-child runtime fixtures are explicitly later than
the first straight-line B0-L3a capability.

B0-Fの `block_expr_tests.rs` は641行、authority guardは794行なので、Lower
fixturesはbuilder側の新規専用test moduleへ置く。guardは追記前に既存部分を
圧縮またはmanifest runnerへ分離し、800行を超えないようにする。

## まだclaimしてはいけない範囲

B0-L0相談と入口inventoryだけでは次をclaimしない。

```text
canonical resolver production install
resolved BindingId authority cutover
resolved ScopeId consumer activation
BlockExpr runtime lexical cutover
Planner / RegionFlow integration
CorePlan exact source-site support
Hako typed-source parity
ProgramV0 retirement
name-keyed Planner state retirement
Lambda runtime capture layout
```

## Accepted answers

```text
Decision 1 = A-prime staged carrier, atomic SA3-B, then B0-L3

Decision 2 = immutable FunctionSourceViewV1 + LocatedBody/Stmt/Expr

Decision 3 = family-staged implementation; source-unit all-or-nothing

Decision 4 = CanonicalFunctionLoweringSessionV1 closure transaction
```

`finish_claims()` is split conceptually into identity adoption and source
coverage. Unreachable declarations are accounted as `SkippedAfterTerminator`,
not forced into ValueId materialization.
