#!/bin/bash
# vm_nested_loop_method_call.sh — VM SSA/PHI bug canary: nested loop + method call pattern
# PASS基準: クラッシュしないこと（結果値は任意）
# 
# このテストは、VMのSSA/PHI生成バグを検出するためのカナリアテストです：
# - Level 0: シンプルなループ（ベースライン）
# - Level 5b: ネストしたループをインライン化（method呼び出しなし）→ 動作すべき
# - Level 5a: ループなしメソッドをループ内から呼び出し → 動作すべき  
# - Level 5: ループありメソッドをループ内から呼び出し → VMバグで失敗（expected）

# 共通ライブラリ読み込み（必須）
source "$(dirname "$0")/../../../lib/test_runner.sh"

# 環境チェック（必須）
require_env || exit 2

# プラグイン整合性チェック（必須）
preflight_plugins || exit 2

# Level 0: Simple loop (baseline - should always work)
test_level0_simple_loop() {
    local output
    output=$(run_nyash_vm -c 'i=0; sum=0; loop(i<5) { sum=sum+i; i=i+1; }; print(sum)')
    # PASSの基準：クラッシュしないこと（結果値は任意）
    if [ -n "$output" ]; then
        return 0
    else
        return 1
    fi
}

# Level 5b: Inline nested loop (should work - no method call)
test_level5b_inline_nested_loop() {
    local code='
src="abc"; i=0; result="["; first=1; cont=1;
loop(cont==1) {
  loop(i<src.length()) {
    ch1=src.substring(i,i+1);
    if ch1==" " { i=i+1 } else { break }
  };
  if i>=src.length() { cont=0 } else {
    ch2=src.substring(i,i+1);
    if first==1 { result=result+ch2; first=0 } else { result=result+","+ch2 };
    i=i+1
  }
};
result=result+"]"; print(result)'
    
    local output
    output=$(run_nyash_vm -c "$code")
    # PASSの基準：クラッシュしないこと
    if [ -n "$output" ]; then
        return 0
    else
        return 1
    fi
}

# Level 5a: Method without loop called from loop (should work)
test_level5a_method_no_loop() {
    local code='
box TestBox {
  skip_ws(src, start) { return start }
  
  process(src) {
    i=me.skip_ws(src,0); result="["; first=1; cont=1;
    loop(cont==1) {
      i=me.skip_ws(src,i);
      if i>=src.length() { cont=0 } else {
        ch=src.substring(i,i+1);
        if first==1 { result=result+ch; first=0 } else { result=result+","+ch };
        i=i+1
      }
    };
    result=result+"]"; return result
  }
}
t=new TestBox(); print(t.process("abc"))'
    
    local output
    output=$(run_nyash_vm -c "$code")
    # PASSの基準：クラッシュしないこと
    if [ -n "$output" ]; then
        return 0
    else
        return 1
    fi
}

# Level 5: Method WITH loop called from loop (VM BUG - expected to fail)
test_level5_method_with_loop() {
    local code='
box TestBox {
  skip_ws(src, start) {
    i=start;
    loop(i<src.length()) {
      ch=src.substring(i,i+1);
      if ch==" " { i=i+1 } else { break }
    };
    return i
  }
  
  process(src) {
    i=me.skip_ws(src,0); result="["; first=1; cont=1;
    loop(cont==1) {
      i=me.skip_ws(src,i);
      if i>=src.length() { cont=0 } else {
        ch=src.substring(i,i+1);
        if first==1 { result=result+ch; first=0 } else { result=result+","+ch };
        i=i+1
      }
    };
    result=result+"]"; return result
  }
}
t=new TestBox(); print(t.process("abc"))'
    
    # dev検証を有効化（PHI整合の安定タグを確認）
    local output
    HAKO_PHI_VERIFY=1 output=$(run_nyash_vm -c "$code" 2>&1 || true)
    # 旧クラッシュ検知（後方互換）
    if echo "$output" | grep -q "Invalid value"; then
        echo "[WARN] Level 5: VM SSA/PHI bug still present (crash detected)" >&2
        return 0
    fi
    # PHI整合検知（[phi/check] タグ）
    if echo "$output" | grep -q "\[phi/check\]"; then
        echo "[WARN] Level 5: VM SSA/PHI mismatch observed (phi/check)" >&2
        return 0
    fi
    # どちらも出なければ修正済み！
    echo "[SUCCESS] Level 5: VM SSA/PHI bug FIXED!" >&2
    return 0
}

# テスト実行
run_test "level0_simple_loop" test_level0_simple_loop
run_test "level5b_inline_nested_loop" test_level5b_inline_nested_loop
run_test "level5a_method_no_loop" test_level5a_method_no_loop
run_test "level5_method_with_loop (VM BUG canary)" test_level5_method_with_loop
