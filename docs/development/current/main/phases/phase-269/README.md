# Phase 269: bool_predicate_scan route への Frag 適用（P0=test-only → P1=実装）

Status: ✅ 完了（P1）
Date: 2025-12-22

## サブフェーズ状況

- **P1（bool_predicate_scan EdgeCFG lowering）**: ✅（SSA の `i` PHI を含めて閉じた）
- **P1.1（call_method return type）**: ✅（署名SSOTで型注釈）
- **P1.2（static box の this/me → static call 正規化）**: ✅（runtime receiver を禁止して SSOT 化）

## 目的

**bool_predicate_scan route（legacy Pattern8 label）を EdgeCFG Fragment（Frag + emit_frag）で実装し、legacy numbered route label の列挙を “exit配線” に収束させる。**

- **P0**: test-only stub + 最小 fixture/smoke で “入口” を固定（DONE）
- **P1**: 実装（MIR CFG層で Frag を組み、emit_frag で terminator を SSOT 化）（DONE）

## 実装範囲（重要：スコープ境界）

### ✅ 触る（P1 スコープ）
- historical path token: `src/mir/builder/control_flow/joinir/patterns/pattern8_scan_bool_predicate.rs`
- current route family: `src/mir/join_ir/lowering/scan_bool_predicate_minimal.rs`
- `src/mir/builder/emission/loop_predicate_scan.rs`（薄い入口：Frag 構築 + emit_frag）
- fixture/smoke（`apps/tests/phase269_p0_pattern8_frag_min.hako`, `tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh`）

### ❌ 触らない（P1 スコープ外）
- cf_loop（JoinIR-only hard-freeze）
- merge/EdgeCFG plumbing（Phase 260-268 の SSOT は維持）
- scan_with_init / split_scan / accum_const_loop（Phase 269 は bool_predicate_scan に集中）

## 実装戦略（Phase 268 パターン踏襲）

### アーキテクチャ図
```
pattern8_scan_bool_predicate.rs (historical bool_predicate_scan entry)
  ↓ P1: emission 入口に委譲
emission/loop_predicate_scan.rs::emit_bool_predicate_scan_edgecfg()
  ↓ 内部で使用（break/continue 無しなので手配線）
Frag（branches+wires）+ emit_frag()
  ↓ 最終的に呼び出し
set_branch_with_edge_args() / set_jump_with_edge_args() (Phase 260 SSOT)
```

## P1 実装のポイント（重要）

### 1) Return を exits に置かない
- `emit_frag()` が emit するのは `branches` と `wires` のみ（`exits` は “上位に伝搬する未配線”）
- したがって early-exit の `return false` は `wires` に `ExitKind::Return` として入れる（`target=None` を許可）

### 2) `return true` は loop 後 AST に任せる
- Frag 経路は “loop の制御” と “early-exit” のみ担当し、関数全体の return は既存 AST lowering に任せる
- P1 の最小は「失敗で return false / それ以外は after に落ちる」

### 3) SSA: ループ変数 `i` の PHI が必須
- header の条件評価は毎周回評価されるが、SSA 的には `i` の “現在値” を header に合流させる必要がある
- 最小の形（header に挿入、先頭に置く）:
  - `i_current = phi [i_init, preheader_bb], [i_next, step_bb]`
  - header/body/step は `i_current` を参照
  - step で `i_next = i_current + 1` を作り、backedge の入力にする

### 完了確認（P1）

- bool_predicate_scan Frag lower が header に PHI を挿入し、`i_current` を `Compare/substring/step` の参照に使用する
- integration smoke:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh` PASS
  - `tools/smokes/v2/profiles/integration/apps/archive/phase259_p0_is_integer_vm.sh` PASS（回帰なし）

## P1.2（DONE）: static box の `this/me` を static call に正規化（runtime receiver 禁止）

### 目的（SSOT）

static box 内の `this.method(...)` / `me.method(...)` を **runtime receiver（NewBox / 文字列 receiver）にしない**。
compile-time に `current_static_box.method/arity` の canonical key を構築し、static call へ正規化する。

### SSOT / 禁止（再掲）

- SSOT:
  - `comp_ctx.current_static_box`（box 名の唯一の出どころ）
  - `BoxName.method/arity`（canonical key: call_method 署名注釈と共用）
- 禁止:
  - `emit_string("StringUtils")` などの文字列レシーバによる by-name 的回避
  - static box の this/me を `NewBox` で runtime object 化（退行の原因）

### 実装（責務分離）

- `src/mir/builder/calls/build.rs`
  - MethodCall の共通入口で `This/Me` receiver を最優先で検出し、static call に正規化する
  - box 名は `comp_ctx.current_static_box` のみから取り出す（ハードコード禁止）
- `src/mir/builder/stmts.rs`
  - static/instance の文脈エラーを Fail-Fast で明確化（誤誘導のメッセージ整理）
- same historical bool_predicate_scan lane as P1 scope above (`pattern8_scan_bool_predicate.rs`)
  - **現状の安全策**: static box 文脈の loop は bool_predicate_scan route 対象外にし、汎用 lowering（loop_simple_while など）へ戻す
  - 目的: receiver 正規化を “1箇所” に収束させ、bool_predicate_scan route が runtime receiver を作る経路を封じる
  - 撤去条件: bool_predicate_scan route が「正規化後の MethodCall（static call key）」前提で安全に動くことを fixture/smoke で確認できたら、この除外を削除する

### 検証（fixture/smoke）

- `apps/tests/phase269_p1_2_this_method_in_loop_min.hako`
- `tools/smokes/v2/profiles/integration/apps/archive/phase269_p1_2_this_method_in_loop_vm.sh`
- 受け入れ条件:
  - MIR dump に `const "StringUtils"` が receiver として出ない
  - `call_method StringUtils.is_digit/1`（または同等の static call）になる

## テスト手順（固定）

1. `cargo build --release`
2. `cargo test -p nyash-rust --lib --release`
3. `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase259_p0_is_integer_vm.sh`
4. `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh`
5. `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase269_p1_2_this_method_in_loop_vm.sh`
6. `./tools/smokes/v2/run.sh --profile quick`（45/46 を維持）

## P0（historical）

P0 の test-only 記録は履歴として残す（入口の固定に寄与）。

### 検出基準（Phase 259 P0 固定形式）
1. Loop condition: `i < s.length()`
2. Loop body has if statement:
   - Condition: `not this.method(...)` (UnaryOp::Not + MethodCall)
   - Then branch: `return false` (early exit)
3. Loop body has step: `i = i + 1`
4. Post-loop: `return true`

### 既存実装の処理フロー
```rust
// 1. Extract pattern parts
let parts = extract_bool_predicate_scan_parts(condition, body)?;

// 2. Get host ValueIds
let s_host = variable_map[haystack];
let i_host = variable_map[loop_var];
let me_host = build_me_expression()?;

// 3. Create JoinModule
let join_module = lower_scan_bool_predicate_minimal(...);

// 4. Build boundary
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(join_inputs, host_inputs)
    .with_loop_invariants(loop_invariants)
    .with_exit_bindings(exit_bindings)
    .with_carrier_info(carrier_info)
    .with_loop_var_name(Some(loop_var))
    .with_expr_result(Some(join_exit_value))
    .build();

// 5. Execute JoinIRConversionPipeline
let result = JoinIRConversionPipeline::execute(self, join_module, Some(&boundary), ...)?;
```

## P0 Frag 版 lowerer 設計

### 目的
- JoinModule から MIR terminator を生成する部分を Frag 化
- 既存の JoinModule 生成・boundary 構築は維持
- emit_frag() による terminator 生成を導入

### 実装方針

```rust
#[cfg(test)]
pub(crate) fn lower_pattern8_frag(
    builder: &mut MirBuilder,
    join_module: JoinModule,
    boundary: &JoinInlineBoundary,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // 1. JoinModule から必要な情報を取得
    //    - loop_step function (loop body)
    //    - k_exit function (return true)

    // 2. Loop body Frag 構築
    //    - loop_step の処理を Frag で表現
    //    - early return (return false) は Return exit として扱う

    // 3. compose::loop_() で合成
    //    - loop_id, header, after, body_frag

    // 4. emit_frag() で MIR terminator に変換

    // 5. result 返却（expr_result）
    Ok(boundary.expr_result)
}
```

### Unit Test 設計

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern8_frag_lowering() {
        // 1. Create minimal JoinModule
        // 2. Create boundary
        // 3. Call lower_pattern8_frag()
        // 4. Verify MIR terminator generation
        //    - Branch terminator exists
        //    - Jump terminator exists
        //    - Return terminator exists (early exit)
    }
}
```

## 最小 fixture 設計

### `phase269_p0_pattern8_frag_min.hako`

Phase 259 の `is_integer_min.hako` の縮小版:

```nyash
static box StringUtils {
    is_digit(ch) {
        return ch == "0" or ch == "1"
    }

    is_integer(s) {
        if s.length() == 0 {
            return false
        }

        local i = 0
        loop(i < s.length()) {
            if not this.is_digit(s.substring(i, i + 1)) {
                return false
            }
            i = i + 1
        }
        return true
    }
}

static box Main {
    main() {
        return StringUtils.is_integer("01") ? 7 : 1
    }
}
```

### Smoke Test 設計

`tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh`:

```bash
#!/bin/bash
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"
set +e
$HAKORUNE_BIN apps/tests/phase269_p0_pattern8_frag_min.hako > /tmp/phase269_out.txt 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 7 ]; then
    echo "[PASS] phase269_p0_pattern8_frag_vm"
    exit 0
else
    echo "[FAIL] phase269_p0_pattern8_frag_vm: expected exit 7, got $EXIT_CODE"
    cat /tmp/phase269_out.txt
    exit 1
fi
```

## 重要な設計判断

### なぜ test-only か

1. **非破壊的**: 既存 bool_predicate_scan 実装を壊さない
2. **段階的**: Frag 版の動作を先に確認してから統合
3. **デバッグ容易**: 問題切り分けが簡単（Frag 版 vs 既存実装）
4. **拡張性**: scan_with_init / split_scan にも同じ構成を適用可能

### なぜ最小 fixture か

1. **高速検証**: 小さいコードで問題を早期発見
2. **デバッグ容易**: MIR dump が読みやすい
3. **回帰テスト**: quick smoke に含めやすい

### P0 での制約

- VM のみ（LLVM は P1 以降）
- bool_predicate_scan のみ（scan_with_init / split_scan は P1 以降）
- test-only（既存実装置換は P1 以降）

## 次フェーズへの橋渡し

**Phase 269 P1+**: 既存 bool_predicate_scan route を Frag 版に置換
- `cf_loop_pattern8_bool_predicate_impl()` から bool_predicate_scan Frag lower を呼び出す
- JoinIRConversionPipeline を廃止（Frag 版に一本化）
- scan_with_init / split_scan にも適用

**Phase 270+**: legacy numbered route label 分岐削減
- legacy numbered route label による分岐を削減
- compose API による統一的なループ処理

## 関連ドキュメント

- **Phase 268**: `docs/development/current/main/phases/phase-268/README.md`
- **設計図**: `docs/development/current/main/design/edgecfg-fragments.md`
- **JoinIR アーキテクチャ**: `docs/development/current/main/joinir-architecture-overview.md`
- **現在のタスク**: `docs/development/current/main/10-Now.md`

## 受け入れ基準（P0）

- ✅ `cargo build --release` 成功
- ✅ `cargo test --lib --release` で全テスト PASS
- ✅ bool_predicate_scan Frag 版 lowerer の unit test PASS
- ✅ `apps/tests/phase269_p0_pattern8_frag_min.hako` 作成
- ✅ `tools/smokes/v2/profiles/integration/apps/archive/phase269_p0_pattern8_frag_vm.sh` 作成
- ✅ smoke test PASS（exit code 7）
- ✅ `tools/smokes/v2/run.sh --profile quick` で 45/46 PASS 維持
- ✅ MIR dump で Branch/Jump/Return terminator 正常生成確認
- ✅ ドキュメント更新完了:
  - ✅ `phases/phase-269/README.md` 新規作成（このファイル）
  - ✅ `10-Now.md` 追記
  - ✅ `30-Backlog.md` 更新

## まとめ

**Phase 269 P0 の核心**:

- ✅ bool_predicate_scan route を Frag 化（test-only、既存と並走）
- ✅ compose::loop_() + emit_frag() を使用
- ✅ 最小 fixture + smoke test で動作確認
- ✅ quick smoke 45/46 を悪化させない

**次のステップ**: P1 で既存 bool_predicate_scan route を Frag 版に置換 → scan_with_init / split_scan にも適用
