# CUT0-I0 ROOT0-CANON0 SOURCE-BIND0 未決定質問

Status: **設計相談停止 — 同一familyのforeign planを拒否するsource authorityが未確定**

## 棚卸し結果

ROOT0-CANON0のC-prime実装は、次を実装済みです。

- one-time source package splitの型
- BRAND0の実session / shell / collectorを保持するactive owner
- collector-issued callable batch receipt
- canonical single / callable batch completion product
- recursive install receipt
- source-derived drain plan

ただし、現APIは`ModuleInvocationTokenV1`をfamily-only witnessとして受け取り、
`Prepared*SourceV1::prepare(token, plan)`でfamilyだけを照合しています。
そのため、同じfamilyに属する別preflight planをtokenへ渡しても拒否できません。

これは単なるfixture不足ではなく、次のsource-authority境界が未決定であることを
示します。

```text
token family
  != exact preflight plan identity
```

## 未決定質問

### Q1. exact planとtokenをどこでco-mintするか

1. **Source-bound package constructor（推奨）**

   preflight planをby-valueで受け取るprivate constructorがtokenを一度だけ
   発行し、同じplanからcontinuation/headerをsealする。callerはtokenを別途
   渡せない。

2. Plan binding stampをcompiler planへ埋め込む

   planとtokenが同じopaque stampを保持し、prepare時にstampを照合する。
   ただしcompiler-layerへBuilder invocation identityを持ち込むため、責務が
   増える。

3. family-only tokenを維持し、completion co-sealで再検証する

   exact source provenanceを証明できず、C-primeのforeign same-family reject
   lawを満たさないため、恒久案としては不採択。

### Q2. production token producer

Source-bound package constructorを採択する場合、test-onlyの
`TestInvocationPreflightFactoryV1`をproduction token producerへ昇格する必要が
あります。独立したledger ordinalやroute-local ID producerを増やさず、ROOT0の
唯一のinvocation ID authorityに統合する方法を決めます。

## 推奨回答

Q1は1、Q2は「ROOT0の唯一のtoken producerをsource-bound package ingressに置く」
で閉じるのが自然です。これなら、

```text
exact plan
-> private package constructor
-> token + plan + continuation
```

を一度だけ作り、foreign same-family planの組み合わせ自体をproduction APIから
消せます。

## 次の最小semantic row

```text
ROOT0-CANON0-SOURCE-BIND0
  -> source-bound package constructor
  -> one production token producer
  -> exact plan/continuation identity
  -> same-family foreign plan rejection fixture
  -> no public canonical ingress
```

この相談が閉じるまで、CANON0のfixture昇格・DRAIN0・production wiringは停止します。

## 非claim

- CANON0のactive owner型があることは、production canonical loweringが接続済みで
  あることを意味しない。
- `Branded*LoweringPlanV1`の存在は、実lowering consumerがあることを意味しない。
- guardの文字列検査は、source provenanceの証明ではない。
