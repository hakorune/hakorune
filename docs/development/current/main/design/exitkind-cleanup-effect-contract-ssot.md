---
Status: SSOT
Scope: ExitKind / cleanup / effect の契約（JoinIR/PlanFrag/MIR で意味論を壊さない）
Related:
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- docs/development/current/main/phases/phase-29aa/README.md
---

# ExitKind / Cleanup / Effect Contract (SSOT)

目的: return/break/continue（将来 unwind を含む）に付随する cleanup（RC release 等）が、最適化や再順序で壊れないように、
**ExitKind と effect の契約**として固定する。

この SSOT は「新しいcleanup機能を追加する」ためではなく、既存の意味論を守るための境界を明文化する。

## 1. 用語

- ExitKind: 関数/ループからの脱出種別（Return/Break/Continue/将来 Unwind）
- Cleanup: Exit を跨いで走る必要がある処理（例: ReleaseStrong の注入、defer/finally 等）
- Effect: 命令の副作用分類（Pure/Mut/Io/Control）

## 2. SSOT: Cleanup は ExitKind の語彙に属する

cleanup は “パターン個別の都合” で散らさない。
必ず ExitKind の語彙として扱い、PlanFrag/CorePlan/merge で責務境界を固定する。

参照:
- Phase 29aa（RC insertion の安全拡張）: `docs/development/current/main/phases/phase-29aa/README.md`
- Plan/Frag SSOT: `docs/development/current/main/design/joinir-plan-frag-ssot.md`

## 3. Invariants（最小）

### I1: Cleanup 命令は `Mut`（非Pure）として扱う

例:
- `ReleaseStrong` / `RetainStrong` は `Mut`（少なくとも DCE/CSE の対象外になる）

理由:
- DCE/CSE が cleanup を消したりまとめたりすると意味論が壊れる。

### I2: Cleanup は Exit の直前/直後の “境界点” にのみ置く

“途中のブロック” に cleanup をばら撒くと、再順序/最適化の影響が広がる。
cleanup の挿入点は SSOT として固定する（例: Return terminator の直前）。

現状の例（RC insertion 系）:
- Return ブロックの BeforeTerminator のみ（Jump/Branch では入れない、などの安全条件）

### I3: Control（Exit/terminator）を跨いで cleanup を移動しない

- `Control` を跨ぐ移動は禁止（exit の前後関係が崩れる）
- `Io` を跨ぐ移動も禁止（外部観測が変わる）

### I4: Join（post-phi）と cleanup は “順序SSOT” を共有する

join 入力（carrier layout）と cleanup の対象（carrier/locals）は同じ順序SSOTを参照し、実装ごとに順序が揺れない。

参照:
- `docs/development/current/main/design/post-phi-final-form-ssot.md`

## 4. Freeze / Fail-Fast の方針（最小）

- cleanup が “安全条件外” に計画された場合は strict/dev で Fail-Fast（Freeze または debug_assert）
- release 既定では挙動を変えない（安全側に倒す）

## 5. Next（将来の拡張点）

- `ExitKind::Unwind` の設計（cleanup/defer/finally を ExitKind 経由で統一）
- effect と “panic/throw” の扱い（Pure/Io/Control 境界）

