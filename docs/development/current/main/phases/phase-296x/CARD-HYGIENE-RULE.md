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
