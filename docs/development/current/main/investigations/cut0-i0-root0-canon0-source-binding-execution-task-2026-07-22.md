# CUT0-I0 ROOT0-CANON0 SOURCE-BIND0 実行タスク

Status: **Active — Candidate SB-prime-r1 selected; SOURCE-BIND0 implementation next**

Decision source:

- `cut0-i0-root0-canon0-source-binding-consultation-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

canonical four routesのexact preflight planを、`MirCompiler`が一度だけ発行する
invocation identityと結び付け、source-bound packageへ封印する。familyラベルだけの
既存scaffoldをproduction authorityへ昇格させない。

対象routeは次の4つだけである。

```text
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

Rawは閉じたROOT0-RAW0 chainを維持し、このrowで再包装しない。

## Selected design

```text
MirCompiler
  owns one private non-Clone InvocationIdentityIssuerV1
  -> bind_canonical_source(exact_plan)
  -> token is minted internally after source validation
  -> SourceBoundCanonicalPackageV1
```

brandの論理identityは、

```text
process-scoped compiler domain + compiler-local monotonic ordinal
```

とする。process-crossing uniquenessはclaimしない。domain seedのallocator方式、
overflow、thread-safetyの詳細はissuer内部に閉じ、route／ledger／collectorへ新しい
identity authorityを漏らさない。global invocation ordinalをrouteへ直接配る方式、
per-route producer、raw ledger ordinal、test factoryのproduction昇格は禁止する。

## Implementation slices

### 1. Compiler-owned identity issuer

- `MirCompiler`にprivate issuerとcompiler domainを追加する。
- production token producerを一箇所に固定する。
- source/header/catalog validationが完了するまでtokenをmintしない。
- validationまたはissuer failureは、exact planを保持するtyped rejected ownerで返す。
- mint後のpackage constructionはinfallibleにし、drop後のordinal再利用はしない。

### 2. Plan-driven source-bound package

new box候補:

```text
src/mir/builder/module_invocation_source_binding.rs
```

```rust
enum ExactCanonicalPreflightPlanV1<'src> {
    APlus(CanonicalCurrentAPlusPlanV1<'src>),
    BindingSsaTrivial(CanonicalTrivialBindingSsaPlanV1<'src>),
    BindingSsaAcyclic(VerifiedAcyclicCallableModulePlanV1<'src>),
    BindingSsaRecursive(VerifiedRecursiveCallableModulePlanV1<'src>),
}
```

`SourceBoundCanonicalPackageV1`はnon-Cloneとし、plan variantからfamily、exact
owner headerまたはcallable source/catalog continuation、route policyを導出する。
callerはtoken、family、header、catalog、route boolを渡せない。

SOURCE-BIND0ではpackageをsplitするpublic terminalを作らない。LOWER0だけが後続で
packageをby-value consumeし、planを実lowererへmoveする。`Option<Plan>`、
`Option::take().expect`、loose `(token, plan)` production APIは禁止する。

### 3. Rejected owner and continuation lifetime

fallible bindingはbare errorではなく、次のownerを返す。

```text
RejectedCanonicalSourceBindingV1
  owns exact rejected plan
  owns typed binding/issuer error
```

A+/trivial continuationはplanから内部sealしたexact owner headerをownedで保持する。
acyclic/recursive callable continuationは、選択されたpreflight planが参照していた
verified source/catalogをborrowする。lowering後のre-resolve、`current_module`再取得、
別catalog lookup、`Arc`/`Clone`によるauthority複製は禁止する。

### 4. Evidence guard (SOURCE-BIND0 slice only)

guardは文字列presenceや固定値を証拠にしない。SOURCE-BIND0で実測する対象は次の通り。

```text
production source-bound constructor count = 1
production token producer count = 1
legacy prepare(token, plan) production callsites = 0
caller-supplied family/header/catalog constructors = 0
TestInvocationPreflightFactory production callers = 0
SourceBoundCanonicalPackageV1 Clone/Arc = 0
Option<Plan>/take().expect seam = 0
focused fixture registration = 1
all touched source/check files < 800 lines
```

static censusとfocused test実行は別証拠として記録する。guard greenだけでsource
provenanceやlowering completionをclaimしない。

## Acceptance

```text
same MirCompiler -> monotonic, non-reused invocation brands
distinct compiler domains -> equal local ordinals are not equal brands
foreign exact plan/header/catalog pairing -> impossible or typed reject before mutation
source validation failure -> no token mint, rejected owner retains plan
issuer failure -> typed error, rejected owner retains plan
package construction -> infallible after mint
live Builder/session/shell/collector mutation before package success = 0
package Clone/Arc = 0
public package split = 0
production lowering/drain/finalizer/external commit = 0
```

focused fixtureは別ファイルへ置き、既存の大きいcollector／shell箱へfixtureを詰め込ま
ない。候補:

```text
src/mir/builder/tests/canonical_source_binding_p0.rs
tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
```

実装前にfixtureを追加してsemantic failure boundaryを固定するが、production ingress
へは接続しない。

## Explicit non-claims and parked rows

このrowはtransport/source provenanceを閉じるだけで、次をclaimしない。

```text
LOWER0       = packageを実lowererへmoveする唯一consumer
RECEIPT0     = collector + exact receipt inseparable product / root retention
RECURSIVE0   = branded install receipt / acyclic absence witness
CANON-FIXTURE0 = four-route aggregate parity and negative matrix
DRAIN0       = completeのone-shot source-derived inventory consumption
POST0/P0     = postprocess, finalizer, external commit
CUT0         = production capture, public ingress, atomic activation
```

child failure後のsibling継続、fallback、retry、`current_module`再観測、bare
`MirModule` handoffも、このrowでは追加しない。

## Row order

```text
SOURCE-BIND0
  -> LOWER0
  -> RECEIPT0
  -> RECURSIVE0 / GUARD0
  -> CANON-FIXTURE0
  -> DRAIN0
  -> P0
  -> atomic CUT0
```

Q3のone-time moveは設計lawとして本カードでlockするが、実lowering consumerは
LOWER0へ分離する。Q4/Q5も同様に、SOURCE-BIND0でfieldを先取りしてcompletionを
昇格させない。

## Required checks

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
git diff --check
RUSTFLAGS='-Awarnings' cargo check -q --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
```

focused fixture commandは実装時にカードへ確定し、成功結果をcloseoutへ記録する。
既存のCANON0 scaffold guardはSOURCE-BIND0 semantic proofの代用にしない。

## Stop line

このカードの受け入れ後も、production canonical capture、drain、finalizer、external
commitはゼロでなければならない。SOURCE-BIND0の目的外の修正を同じcommitへ混ぜない。
1 semantic row = 1 commitを守り、800行を越えそうな箱は先に分割する。
