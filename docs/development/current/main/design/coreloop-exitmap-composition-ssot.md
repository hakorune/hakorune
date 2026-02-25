---
Status: SSOT
Scope: CoreLoop（Loop skeleton）に対する ExitMap/Cleanup/ValueJoin の合成規約
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/edgecfg-fragments.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# CoreLoop ExitMap Composition (SSOT)

目的: Loop skeleton に ExitMap/Cleanup/ValueJoin を合成する規約を 1 枚に固定し、Normalizer/Composer/Emit の責務境界を揺らさない。

## 1. Vocabulary（語彙）

- **ExitKind**: `Return` / `Break(loop_id)` / `Continue(loop_id)`（将来 `Unwind`）
- **ExitMap presence**:
- `exit_kinds_present` は “存在集合” の SSOT（Facts/Canonical の投影）
  - `Frag.exits` は “出口エッジ” の SSOT（presence と実エッジを同じ語彙で扱う）
- **Cleanup presence**:
  - `cleanup_kinds_present` は ExitKind 語彙として扱う
  - cleanup の意味論は `exitkind-cleanup-effect-contract-ssot.md` に従う
- **ValueJoin**:
  - join 値は `Frag.block_params` と `EdgeArgs(values)` で表現する
  - `EdgeArgs.layout` は `post-phi-final-form-ssot.md` の SSOT に従う

## 2. Composition Rules（合成規約）

### 2.1 ExitMap presence の投影

- `exit_kinds_present` を `Frag.exits` に投影する（presence を SSOT 化）
- presence は “実エッジの有無” を保証しない（後段で edge を作る責務は Normalizer/Composer）
- emit/merge が CFG/AST を再解析して exit を推測するのは禁止

### 2.2 Exit edges（実エッジ）の責務

- `Continue(loop_id)` は header へ、`Break(loop_id)` は after へ配線する
- `Return`/`Unwind` は外側へ伝搬する（Loop 内で勝手に消費しない）
- ループ内で “exit edge を増やす” ことはせず、`Frag.exits` を入口 SSOT として扱う

### 2.3 Cleanup

- cleanup は ExitKind の語彙に属する（pattern 固有の挿入は禁止）
- 合成は `compose::cleanup` を唯一の入口にする
- cleanup の前後関係は `exitkind-cleanup-effect-contract-ssot.md` に従う

### 2.4 ValueJoin

- join 値が必要な場合、`Frag.block_params` を必ず使う（PHI の暗黙推論は禁止）
- `EdgeArgs.layout` は join 入力の順序SSOTで固定する
- `emit_frag()` が PHI 挿入の唯一の接続点（他の層で PHI を作らない）

### 2.5 責務境界（Fail-Fast を前提にする）

- Facts: presence まで（edge/PHI を作らない）
- Normalizer/Composer: `Frag` と `block_params` を作る
- Emit: `Frag` の情報をそのまま PHI/terminator に落とす（再推論禁止）

## 3. Fail-Fast / Verify（SSOT）

- **Ok(None)**:
  - Skeleton が Loop でない / 対象外（StraightLine 等）
- **Freeze(unstructured)**:
  - Loop skeleton が一意に決まらない（irreducible / multi-entry）
- **Freeze(unsupported)**:
  - `Unwind` 等、ExitKind 語彙はあるが現実装が扱えない
- **Freeze(contract)**:
  - `exit_kinds_present` と `Frag.exits` が矛盾する
  - `Frag.block_params` と `EdgeArgs.layout/values` が整合しない

検証の入口:
- `planfrag-freeze-taxonomy.md` のタグを使用（strict/dev で Fail-Fast）
- `edgecfg-fragments.md` の `verify_*` を通す（release は挙動不変）

## 4. References（入口）

- CorePlan Skeleton/Feature model: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`
- ExitKind/Cleanup contract: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- Post-PHI final form: `docs/development/current/main/design/post-phi-final-form-ssot.md`
- EdgeCFG/Frag SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
