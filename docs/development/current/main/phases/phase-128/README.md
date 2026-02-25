# Phase 128: if-only partial assign keep/merge in Normalized (dev-only)

## 目的

- Phase 113 の片側代入パターン（else保持）を、StepTree→Normalized 側でも表現できるようにする
- PHI を使わず、env 更新＋join 継続で合流を機械化（Normalized の責務）

## Scope

- 対象: if-only（loop なし）
- 対応代入: `x = <int literal>` のみ（Phase 128）
- 既定挙動は不変: `joinir_dev_enabled()` のときだけ生成・検証

## 契約（SSOT）

### Assign 対応

- `StmtKind::Assign { target: Some(x) }` を受理
- RHS が int literal のときだけ実装（Phase 128）
- RHS がそれ以外は strict で `freeze_with_hint`（hint: "Phase128 supports int literal only"）
- env 更新: `env_map[x]` に対応する ValueId に `Const(Integer)` を生成

### If then/else env merge

- if cond:
  - then: `env_then = env with x updated`（代入実行）
  - else: `env_else = env unchanged`（元の値を保持）
  - `join_k(env_phi)` に tail-call（env_phi は join 引数）
- **PHI 禁止**: env 更新＋join 継続で表現

### Strict Fail-Fast

- RHS が int literal 以外: `freeze_with_hint`（hint必須・1行）
- unknown-read（Phase 127 で導入予定）: 別 Phase で対応

## 設計

### env 更新の表現

- then: `env_then[x] = ValueId(new_const)`
- else: `env_else[x] = env[x]`（元の ValueId を保持）
- join: `join_k(env_phi)`（env_phi は then/else の env を merge した結果）

### PHI 禁止の理由

- Normalized の責務: env 更新＋join 継続で合流を表現
- PHI は MIR 生成時に必要に応じて生成される

## 実装計画

### P0: docs-only

- このドキュメント作成

### P1: StepTreeFacts writes 確認

- `StmtKind::Assign { target: Option<String> }` で writes が取れている前提
- 足りなければ最小修正のみ

### P2: Normalized builder に Assign(int literal) 追加

- `src/mir/control_tree/normalized_shadow/builder.rs`
- `StmtKind::Assign { target=Some(x) }` を受理
- RHS が int literal のときのみ実装
- RHS がそれ以外は strict で `freeze_with_hint`

### P3: If の then で env 更新 → join 継続

- then: `env_then = env with x updated`
- else: `env_else = env unchanged`
- `join_k(env_phi)` に tail-call

### P4: fixture + smoke（VM）

- `apps/tests/phase128_if_only_partial_assign_normalized_min.hako`
  - `x=1; if flag==1 { x=2 }; return x`
  - 期待: flag=0→1, flag=1→2
- `tools/smokes/v2/profiles/integration/apps/phase128_if_only_partial_assign_normalized_vm.sh`
  - `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1`
  - output_validator で `1\n2` を固定

### P5: 回帰

- `cargo test --lib`
- Phase 121-127 代表 smokes
- Phase 118（loop 側退行防止）

### P6: docs DONE

- `docs/development/current/main/10-Now.md` 更新
- INDEX / Backlog 更新

## 受け入れ基準

- `cargo test --lib` が PASS
- Phase 121-127 の smokes が退行しない
- Phase 118（loop 側）が退行しない
- 新規 fixture が VM で期待通りに動作

## 関連

- Phase 113: if-only partial assign parity
  - `docs/development/current/main/10-Now.md`（2025-12-18 完了記録）
- Phase 121: StepTree→Normalized Shadow Lowering
  - `docs/development/current/main/phases/phase-121/README.md`
- Phase 127: unknown-read strict Fail-Fast（planned）
  - `docs/development/current/main/phases/phase-127/README.md`
