# Phase 126: available_inputs SSOT wiring (dev-only)

## 目的

- Phase 125 で EnvLayout に inputs レーンを導入したが、available_inputs は空のまま（structure-only）
- Phase 126 では available_inputs を実際のソース（function params + CapturedEnv）から収集し、EnvLayout.inputs に流し込む
- reads-only 変数（外側スコープの読み取り専用入力）が実際に解決できることを Phase 125 fixture で確実に通す

## Scope

- 対象: if-only（loopなし）の Normalized（dev-only）
- 既定挙動は不変: `joinir_dev_enabled()` のときだけ available_inputs を収集・配線

## SSOT 方針

### available_inputs のソース（優先順位）

1. **function params**: MirBuilder の `scope_ctx.function_param_names` + `variable_ctx.variable_map`
2. **CapturedEnv**: pinned/captured の host ValueId（Pattern2 等で使用）
3. **禁止**: AST からの推測 capture（Phase 100 の CapturedEnv と混同しない）

### 契約

- **入力**: StepTreeContract.reads（何を読むか）
- **出力**: available_inputs: BTreeMap<String, ValueId>（どこから読むか）
- **不変条件**: reads にあるのに available_inputs に無い → `freeze_with_hint(phase126/unknown_read/...)`（strict mode）

## 実装戦略

### P1: AvailableInputsCollectorBox（新規箱）

- 場所: `src/mir/control_tree/normalized_shadow/available_inputs_collector.rs`
- API:
  ```rust
  pub struct AvailableInputsCollectorBox;

  impl AvailableInputsCollectorBox {
      pub fn collect(
          scope_ctx: &ScopeContext,
          variable_ctx: &VariableContext,
          captured_env: Option<&CapturedEnv>,
      ) -> BTreeMap<String, ValueId> {
          // 1. function params から収集
          // 2. CapturedEnv から収集（pinned/captured）
          // 3. BTreeMap で決定的順序保証
      }
  }
  ```

### P2: 配線（dev-only）

- 配線点: `src/mir/builder/calls/lowering.rs` の `lower_function_body()`
  - StepTree 生成後、`StepTreeNormalizedShadowLowererBox::try_lower_if_only()` 呼び出し前
- 手順:
  1. `AvailableInputsCollectorBox::collect()` を呼ぶ
  2. `try_lower_if_only()` のシグネチャを拡張（available_inputs を受け取る）
  3. `EnvLayout::from_contract(contract, &available_inputs)` で inputs を決定

### P3: Fixture 強化

- `apps/tests/phase125_if_only_return_readonly_input_min.hako` を実際の reads-only パターンに変更
  - 例: `local outer_x = 7; if flag==0 { /* outer_x を読まない */ } return outer_x`
  - `outer_x` が writes に入らない（代入しない）
  - `outer_x` が reads に入る（return で参照）
- smoke test: `tools/smokes/v2/profiles/integration/apps/phase125_if_only_return_input_vm.sh`
- 期待: `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` で exit code 7

## 受け入れ基準

- `cargo test --lib` が PASS
- Phase 121-125 の smokes が退行しない
- Phase 126 fixture/smoke で reads-only inputs から return が解決できる

## 関連

- Phase 121-125: StepTree→Normalized dev-only の段階投入
  - `docs/development/current/main/design/control-tree.md`
  - `docs/development/current/main/phases/phase-125/README.md`
- Phase 100: CapturedEnv (pinned/captured)
  - `docs/development/current/main/phases/phase-100/README.md`
