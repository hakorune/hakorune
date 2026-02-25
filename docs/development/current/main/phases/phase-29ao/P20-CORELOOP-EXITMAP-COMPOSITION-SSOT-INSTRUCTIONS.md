---
Status: Ready
Scope: docs-first（ExitMap 合成の SSOT 固定。仕様不変）
Related:
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29ao/README.md
---

# Phase 29ao P20: CoreLoop ExitMap composition（docs-first SSOT）

Date: 2025-12-30  
Status: Ready for execution  
Scope: docs-only。CoreLoop（Loop skeleton）に対する ExitMap/Cleanup/ValueJoin の「合成規約」を SSOT として 1 枚に固定する。

## 目的

- “Pattern2/4/5 を別patternとして増やす” ではなく、`LoopSkeleton + ExitMap + Cleanup + ValueJoin` の合成で表現できることを設計として固定する。
- Normalizer/Composer が「どこまで合成し、どこから Freeze すべきか」を揺れない形で明文化する。
- 既存の `exitkind-cleanup-effect-contract-ssot.md` / `post-phi-final-form-ssot.md` と衝突しない、より具体の “CoreLoop 版の規約” を定義する。

## 非目的

- Rust 実装変更（P20は docs-only）
- 新しい Env 変数やログの追加
- unwind/coroutine 等の新機能追加（設計として “unsupported/unstructured” に分類するだけは可）

## 追加する SSOT（新規）

- 新規ファイル: `docs/development/current/main/design/coreloop-exitmap-composition-ssot.md`

含める内容（最低限）:

1. **語彙（Vocabulary）**
   - `ExitKind`（Return/Break/Continue + 将来Unwind）
   - `Frag.exits`（presence と “実際の出口エッジ” の違い）
   - `EdgeArgs`（layout と values の意味）
   - `Frag.block_params` と “join 受け口” の扱い（post-phi の最終表現）

2. **合成規約（Composition Rules）**
   - Loop skeleton からどの出口を “exit edges” として表すか（Return/Break/Continue）
   - Cleanup の扱い（どの ExitKind に対して、どの位置で走るか）
   - ValueJoin（join 値が必要なときの表現）と block_params の使用条件
   - “presence だけある” 状態（facts から投影された kinds_present）と、実際の join を作る責務の境界

3. **Fail-Fast 規約（Freeze/Verify）**
   - `Ok(None)` に落としてよいケース / `Freeze(contract|unsupported|unstructured)` にすべきケース
   - strict/dev の検証責務（どこで verify するか、どのタグで観測するか）

4. **既存SSOTとのリンク**
   - `exitkind-cleanup-effect-contract-ssot.md`（“越えてはいけない境界”）
   - `post-phi-final-form-ssot.md`（join 入力の最終表現）
   - `edgecfg-fragments.md`（Frag の SSOT）

## 参照導線（docs）

- `docs/development/current/main/design/planfrag-ssot-registry.md` の References に上記 SSOT を追加する（入口導線の強化）。

## 検証

- docs-only のため smoke は不要（ただし link/typo の確認は行う）

## コミット

- `git add -A`
- `git commit -m "docs(phase29ao): add coreloop exitmap composition ssot (p20)"`

