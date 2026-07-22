# CUT0-I0 ROOT0-CANON0 CANON-FIXTURE0 / CANON-BRIDGE0 設計相談

Status: **Closed — Candidate CB-prime selected; execution task next**

Related:

- `cut0-i0-root0-canon0-fixture0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-recursive0-execution-task-2026-07-22.md`

## Pro先生へ渡す要約

CUT0直前のCANON-FIXTURE0で、四canonical routeの
`source -> token -> package -> LOWER0 -> physical shell/collector -> receipt
-> completion`を同じ非Clone owner chainとして証明したい。しかし現HEADは
compiler側のSOURCE-BIND0/LOWER0とbuilder側のROOT0 completionを別体系で
実装している。compilerは`CanonicalInvocationTokenV1`／独自brandと
`LoweredCanonicalPlanV1`を所有し、builder completionは
`ModuleInvocationTokenV1`／独自brandをtest factoryから受け取り、planを別引数
で再受理する。production bridgeはゼロである。ここでfixtureだけを追加すると
test tokenまたはpost-hoc rewrapになり、唯一owner chainの証拠を偽造する。

## 棚卸しで確定した事実

```text
SourceBoundCanonicalPackageV1 -> ActiveModuleInvocationV1 bridge = 0
CanonicalLoweringCandidateV1 -> canonical completion terminal      = 0
compiler CanonicalInvocationToken/Brand -> builder token/brand     = 0
builder canonical completion production callers                    = 0
```

既存のfocused testsは緑だが、証明範囲は分離している。

```text
SOURCE-BIND0 package/lowering       6/6
callable batch collection           6/6
RECEIPT0                            3/3
RECURSIVE0                          3/3
```

これらをaggregate chainの証拠へ昇格してはいけない。

## Source authority / non-authority

| 境界 | authority | non-authority |
|---|---|---|
| source/package | compiler `ExactCanonicalPreflightPlanV1` と `SourceBoundCanonicalPackageV1` | builder test token、family flag、再取得header |
| identity | packageが保持するcompiler token/brand、将来の一つのissuer | builder `TestInvocationPreflightFactoryV1`、post-hoc brand wrapper |
| lowering | packageを一度だけconsumeするLOWER0 terminal | completion側のplan再受理、drop-only `Option::take` |
| physical owner | bridgeが一度だけ作るactive session/shell/collector | standalone `CanonicalModuleLoweringSessionV1`、別shell再構築 |
| receipt/completion | 同じphysical collectorが発行するreceiptとroute-specific completion | loose receipt、別collector receipt、module再観測 |
| synthetic key | canonical collector terminal（Raw/Mainは別許可） | generic collectorのcaller-provided key |

## Decision questions

### Q1 — bridgeのownerをどこに置くか

1. **Compiler-owned one-shot bridge (推奨候補)**

   `MirCompiler`が`SourceBoundCanonicalPackageV1`をby-valueで消費し、同じ
   compiler brandをactive physical ownerへ渡すprivate terminalを持つ。planは
   active sessionへmoveし、single/callableのdraftを同じcollectorへ集める。

2. **共通identityを`crate::mir`へ昇格**

   compiler tokenとbuilder tokenを一つのtoken/brandへ統合する。長期的には
   二重authorityを消せるが、既に閉じたSOURCE-BIND0/BRAND0の型境界を大きく
   巻き戻す。

3. **二層fixtureへ縮退**

   compiler package/loweringとbuilder completion/receiptを別fixtureで検証し、
   cross-layer chainのclaimを撤回する。変更量は最小だが、CANON-FIXTURE0の
   元の目的（唯一owner chainのaggregate proof）は達成しない。

### Q2 — token/brandのco-seal

Compiler tokenをbuilder tokenへ変換する場合、単なる`from_source`やordinalの
コピーを禁止する。必要なのは、

```text
one package-owned non-Clone identity
-> private bridge terminal
-> physical owner carries same opaque brand
-> foreign compiler package / physical owner fails before mutation
```

である。process domainとlocal ordinalのauthorityを二つ残す案は不可。

### Q3 — lowering outputのhandoff

現LOWER0は`LoweredCanonicalPlanV1`としてdraftまたはcallable draft setを返し、
builder completionはplanを受けて別にactive ownerを開く。次のどちらを採用するか。

1. bridgeがpackageをconsumeする前にactive ownerを開き、plan-consuming loweringが
   そのsession/collectorを直接borrowする。
2. LOWER0が返すdraft setをbridgeが同じactive ownerへone-shot admissionする。

2は既存LOWER0を保ちやすいが、draftの元plan／physical owner provenanceを
   by-valueで保持するreceipt productが必要になる。1はowner chainが自然だが、
   `CanonicalModuleLoweringSessionV1`をactive physical ownerへ統合する設計になる。

### Q4 — canonical synthetic identityの責務

`Main`と`SyntheticConditionFn`はRaw rootでは正当だが、canonical completionでは
禁止する。現active collectorはcanonical keyを内部生成するため、completionの
`SyntheticRoot` error branchは通常到達不能で、generic collectorはloose keyを
受け入れてしまう。次のどれをcanonical terminalのlawにするか。

1. canonical collector APIをtyped key専用に分割し、Main/Syntheticを型上作れなくする
2. canonical terminalでkeyを検査し、collector delta=0のtyped rejectionを返す
3. Raw/canonical共通collectorを維持し、fixtureだけで禁止を証明する（非推奨）

物理symbolが`condition_fn/N`でも、canonical key/header/catalog由来なら許可する。

### Q5 — CANON-FIXTURE0の範囲

1. bridge決定・実装後にのみ四route aggregateを実装する（推奨）
2. 今回は二層fixtureへ縮退し、cross-layerをnon-claimとして明記する
3. test-only adapterで暫定的にchainをつなぐ（禁止）

## 推奨回答（暫定）

```text
Q1 = 1: compiler-owned one-shot bridge
Q2 = one package identity; no post-hoc rebrand
Q3 = bridgeがactive physical ownerを先に開き、plan-consuming loweringを同じsessionへmove
Q4 = 1: canonical terminalをtyped key専用にする
Q5 = 1: bridge決定前はCANON-FIXTURE0を実装しない
```

この回答はCANON-BRIDGE0の設計選択であり、まだ実装許可ではない。bridgeの
具体的なowner型、compiler/builder identityの統合方法、single/callable draftの
receipt形、failure時のrejected owner、旧standalone sessionの撤去条件を決めて
から実装へ進む。

## Explicit non-claims

```text
existing four focused fixture families = aggregate owner-chain proofではない
builder test token = production identityではない
ordinal copy = provenance co-sealではない
completion scaffoldのpresence = production consumerではない
synthetic-root branchの存在 = reachable canonical rejectionではない
```

相談が閉じるまで、canonical production ingress、capture、drain、finalizer、
external commit、fallback、retryはすべて0のままにする。

## Decision lock

Candidate CB-prime is selected:

```text
Q1 bridge owner       = Compiler-owned one-shot bridge
Q2 identity           = one shared identity kernel; token conversion forbidden
Q3 lowering handoff  = open the physical owner first and lower in that session
Q4 synthetic identity = canonical typed admission facade
Q5 fixture scope     = aggregate fixture only after the bridge rows close
```

The compiler remains the sole identity issuer. The shared kernel contains the
process-scoped compiler domain and compiler-local invocation ordinal, while
the non-Clone token remains owned through completion and drain. No ordinal
copy, compiler-token-to-builder-token conversion, post-hoc rebrand, or test
factory token is permitted on the new path.

The implementation order is fixed:

```text
CANON-BRIDGE0-IDKERNEL
  -> CANON-BRIDGE0-OWNER0
  -> CANON-BRIDGE0-COLLECT0
  -> CANON-FIXTURE0
  -> DRAIN0
```

The active execution task is
`cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md`. Until its first
row closes, canonical production ingress, drain, finalizer, external commit,
fallback, and retry remain zero.
