# Card Hygiene Rule (phase-296x)

## card を作るのは実タスクだけ
- ✅ card 化: Implement / Write / Fix / Passes(実装作業)
- ❌ card 化しない: Select / Record / Inventory / Probe / Choose / Document(process)
  - これらは **commit message** で記録する。card にしない。

## 理由
process step 毎に card を作ると pile が無限増殖する(1686→1426 archive の原因)。
実タスクだけ card 化すれば、card 数 ≈ 実タスク数 に収まる。

## 例外
設計 stop(人間判断待ち)は card 化してよい(blocked 状態を追うため)。

## closeout lifecycle

Card closeout must make one explicit lifecycle decision in the same commit:

```text
active/current reference remains:
  keep live with owner and retire_when

closed and no tracked live reference:
  move to the phase archive manifest/bucket

closed but a live guard/doc still names the old path:
  move only after resolver adoption, or keep a short forwarding stub
```

Closeout must also review card-owned guards:

```text
durable reusable contract guard:
  keep and point at code/fixture/SSOT

one-row historical assertion:
  convert to a shared manifest row or retire

unknown callers or proof ownership:
  retain with an explicit retire_when; never delete from card status alone
```

Do not keep both a full live card and a full archive copy. One card remains one
file; archive movement, not ledger concatenation, owns history.
