# Pattern2 (Loop with Break) - Phase 286 P3 で再開

**Date**: 2025-12-26
**Status**: **IN PROGRESS** (Phase 286 P3)

## Summary

Pattern2 requires complex value reconnection at the exit point:
- break経路ではcarrier更新が実行されない
- **after_bb に PHI 必要**（header経路 vs break経路の値選択）
- compose::loop_との統合、ExitKind::Break配線が必要

## 実装方針（Phase 286 P3）

**after_bb PHI が本質**:
```
carrier_out = PHI(header: carrier_current, break_then: carrier_break)
```

- break 前に update あり → carrier_break = 計算結果
- break 前に update なし → carrier_break = carrier_current（そのまま）

**PoC サブセット厳守**: 取れない形は `Ok(None)` で legacy へ（Fail-Fast 回帰防止）

## Reference

- [joinir-architecture-overview.md](../../joinir-architecture-overview.md) - Pattern2の制約記載
- [Phase 286 README](./README.md) - Phase全体の進捗
