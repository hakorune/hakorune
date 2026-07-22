# CUT0-I0 ROOT0-RAW0 未決定質問

Status: **回答済み — Candidate A selected**
Date: 2026-07-22

## 質問

`ROOT0-RAW0` の実装範囲を、次のどちらで固定するか決めてください。

### Candidate A（推奨）

receipt provenanceとroot witnessを、同じRAW0の不可分な設計として扱う。

```text
collector-bound branded receipt
-> raw root-batch preflight
-> retained CompletedRootBodyV1
-> exact condition receipt + callable-main disposition
-> one Raw completion witness
```

### Candidate B

receipt provenanceだけを先に別rowへ分割する。

```text
ROOT0-RAW0-RECEIPT
  = collector-bound branded receipt only

ROOT0-RAW0
  = root witness retention later
```

## 決定

**Candidate Aを採択**する。receipt provenanceはRAW0の前提箱として
同じnon-Clone ownership chainへ入れ、root witnessと別rowには分割しない。

## 根拠と制約

- 現行ROOT0 briefは `CompletedRootBodyV1`、required condition receipt、
  callable-Main dispositionの保持までRAW0に要求している。
- receipt provenanceだけでRAW0を閉じると、既存SSOTより狭い意味論になる。
- どちらを選んでも、production capture、drain、finalizer、CUT0 wiringは
  この質問の回答後まで未接続のままにする。
- receipt実装WIPは、採択済みAの実装時に必要な部分だけを取り込む。

関連:

- [ROOT0 design-stop brief](cut0-i0-root0-design-stop-2026-07-22.md#root0-raw0-d0--scope-boundary-design-stop)
- [T-prime-r1 execution task](cut0-i0-t-prime-r1-execution-task-2026-07-22.md#cut0-i0-root0-raw0-d0--design-stop-receipt-seam-versus-root-witness)
