# Phase ドキュメント

このフォルダは、実装フェーズ（Phase 131, Phase 33 等）ごとの詳細記録を保管します。

## Current

- **Current (ACTIVE)**: Phase 264x numeric loop / SIMD reduction recognition proof seam
- **Sibling (ACTIVE GUARDRAIL)**: Phase 137x main kilo reopen selection

## Recent Landed

- **Phase 263x（LANDED）**: numeric loop / SIMD induction proof seam
- **Phase 262x（LANDED）**: numeric loop / SIMD policy owner seam
- **Phase 261x（LANDED）**: escape / barrier -> LLVM attrs first seam
- **Phase 260x（LANDED）**: memory-effect owner seam and stats surface
- **Phase 259x（LANDED）**: SimplifyCFG closeout judgment and handoff
- **Phase 258x（LANDED）**: SimplifyCFG single-input-PHI constant propagation
- **Phase 257x（LANDED）**: SimplifyCFG edge-arg trampoline threading
- **Phase 256x（LANDED）**: SimplifyCFG trivial-PHI-target jump threading
- **Phase 249x（LANDED）**: sibling failure runtime wiring
- **Phase 248x（LANDED）**: sibling-failure policy pin
- **Phase 247x（LANDED）**: detached / root-scope policy pin
- **Phase 246x（LANDED）**: await cancelled first cut
- **Phase 245x（LANDED）**: await failure first cut
- **Phase 244x（LANDED）**: VM await contract pin
- **Phase 242x（LANDED）**: task-scope structured-concurrency vocabulary alignment
- **Phase 227x（LANDED）**: semantic simplification owner seam

## Parked Corridors

- **Phase 96x（PARKED）**: vm_hako LLVM acceptance cutover

### Deeper History

- older landed phases remain in their `phase-*` folders
- use the phase folder directly when you need historical detail; this index is not a full landed ledger
- `phase-29cc` remains the long-range Rust -> `.hako` migration orchestration track

## Phase フォルダ構成（推奨）

```
phases/phase-142x/
└── README.md

phases/phase-141x/
├── README.md
├── 141x-90-string-semantic-boundary-review-ssot.md
└── 141x-91-task-board.md
```

## 参照方法

1. **現在の Phase を知りたい** → [../10-Now.md](../10-Now.md)
2. **該当 Phase を詳しく知りたい** → フォルダを開く
3. **設計背景を知りたい** → [../design/](../design/README.md)
4. **調査ログを見たい** → [../investigations/](../investigations/README.md)

---

**最終更新**: 2026-04-13
