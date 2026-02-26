# Phase 124: Reads Facts SSOT + Return(Variable) 解禁 (dev-only)

## 目的

StepTreeFacts に `reads` フィールドを追加し、Normalized側が「変数return/cond」を再解析なしで扱えるようにする。

## 背景

現在、Normalized builder は writes のみを env として持ち、`Return(Variable(name))` が来たときに name が env に無いと処理できない。Phase 123 では `Return(Unknown)` を導入したが、これは "値が決まらない" 場合であり、「変数名は分かっているが env に無い」ケースとは異なる。

Phase 124 では、StepTreeFacts に reads を追加し、変数参照を明示的に記録することで、Normalized側が「この変数は読まれている」という情報を持てるようにする。

## 方針

### reads の収集範囲

- **Variable(name)**: 式中の変数参照は reads に追加
- **Assign の RHS**: 代入の右辺に出てくる Variable も reads に追加
- **条件式**: cond_ast / cond_sig 内の Variable も reads に追加（式の意味解析は不要、構文木から拾う）

### reads の扱い

- reads は「識別できた local 名だけ」でOK
- 不明な読み込みは capability/unknown-read に落とす（Phase 125以降で対応）
- reads は BTreeSet<String> で決定的にソート
- StepTreeContract の signature に含める（basis string に影響）

### Return(Variable) の実装

- Normalized builder で Return(Variable(name)) を処理
- name が env にあるなら env から値を取り `Ret(Some(v))`
- env に無いが reads にあるなら Phase 124 では Fail-Fast にする
  - **dev-only**: `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` でのみ動作
  - env に無い return var は strict モードで `freeze_with_hint` を呼び出してエラー

## 実装計画

### P1: StepTreeFacts に reads を追加

- `src/mir/control_tree/step_tree_facts.rs`
  - `reads: BTreeSet<String>` 追加
  - API: `add_read(name)` / `merge_reads()`

- `src/mir/control_tree/step_tree.rs`（または facts builder）
  - Variable(name) を見たら `add_read(name)`
  - Assign は writes に入れるが、RHS 内の Variable は reads として拾う
  - 条件式（cond_ast / cond_sig）も "Variable が出たら reads" に入れる

### P2: StepTreeContractBox に reads を反映

- `src/mir/control_tree/step_tree_contract_box.rs`
  - `StepTreeContract` に reads を追加（順序は決定的）
  - `signature_basis_string()` に reads を含める（BTreeSet の順で安定）

### P3: Normalized builder で Return(Variable) を実装

- `src/mir/control_tree/normalized_shadow/builder.rs`
  - Return(Variable(name)):
    - name が env にあるなら env から値を取り `Ret(Some(v))`
    - env に無いが reads にあるなら Phase 124 では Fail-Fast にする
      - **dev-only**: `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` でのみ動作
      - env に無い return var は strict モードで `freeze_with_hint` を呼び出してエラー
  - unit test: `x=7; return x` が生成できる

### P4: integration smoke

- fixture: `apps/tests/phase124_if_only_return_var_min.hako`（expected 7）
  - `local x = 7; return x`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase124_if_only_return_var_vm.sh`
  - `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` で PASS 固定

### P5: docs 完了

- `docs/development/current/main/10-Now.md` / INDEX / backlog 更新

## 受け入れ基準

- [ ] StepTreeFacts に reads が追加される
- [ ] StepTreeContract の signature に reads が含まれる
- [ ] Normalized builder で Return(Variable) が処理できる（dev-only）
- [ ] integration smoke が PASS する
- [ ] 既存テストが全て PASS する（signature 変更によるテスト更新は許容）

## 制約事項

- **dev-only**: `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` でのみ動作
- **既定挙動は不変**: 環境変数無しでは従来通り（Phase 123 まで）
- **Fail-Fast 原則**: env に無い return var は strict モードでエラー

## 次のステップ（Phase 125以降）

- unknown-read capability の導入
- reads に基づく環境拡張（env に無い変数を reads から補完）
- loop との統合（loop 内の reads を継続的に追跡）

## 参照

- Phase 123: Return(Unknown) 導入
- Phase 121: Shadow If Only 基本実装
- Phase 118: Loop Nested If Merge
