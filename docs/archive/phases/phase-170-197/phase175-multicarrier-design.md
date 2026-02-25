# Phase 175: P5 Multiple Carrier Support Design

**Date**: 2025-12-08
**Purpose**: Extend P5 pipeline to support multiple carriers for complex loops like `_parse_string`

---

## 1. Target: _parse_string Carrier Analysis

### 1.1 Loop Structure (lines 150-178)

```hako
_parse_string(s, pos) {
    if s.substring(pos, pos+1) != '"' { return null }

    local p = pos + 1      // Carrier 1: position index
    local str = ""         // Carrier 2: result buffer

    loop(p < s.length()) {
        local ch = s.substring(p, p+1)  // LoopBodyLocal (promotion candidate)

        if ch == '"' {
            // End of string
            local result = new MapBox()
            result.set("value", me._unescape_string(str))  // Uses str carrier
            result.set("pos", p + 1)                        // Uses p carrier
            result.set("type", "string")
            return result
        }

        if ch == "\\" {
            // Escape sequence (Phase 176 scope)
            local has_next = 0
            if p + 1 < s.length() { has_next = 1 }
            if has_next == 0 { return null }

            str = str + ch
            p = p + 1
            str = str + s.substring(p, p+1)
            p = p + 1
            continue  // ⚠️ Phase 176 scope
        }

        str = str + ch  // Update carrier 2
        p = p + 1       // Update carrier 1
    }

    return null
}
```

### 1.2 Carrier Candidates Table

| Variable | Type | Update Pattern | Exit Usage | Carrier Status |
|---------|------|----------------|------------|----------------|
| `p` | IntegerBox | `p = p + 1` | Position in `result.set("pos", p + 1)` | ✅ **Required Carrier** |
| `str` | StringBox | `str = str + ch` | String buffer in `result.set("value", me._unescape_string(str))` | ✅ **Required Carrier** |
| `ch` | StringBox | `local ch = s.substring(p, p+1)` | Loop body comparison | ❌ **LoopBodyLocal** (promotion target) |
| `has_next` | IntegerBox | `local has_next = 0` | Escape processing guard | ❌ **Loop body only** (Phase 176) |

### 1.3 Carrier Classification

**Required Carriers (Exit-dependent)**:
1. **`p`**: Position index - final value used in `result.set("pos", p + 1)`
2. **`str`**: Result buffer - final value used in `result.set("value", me._unescape_string(str))`

**Promoted Carriers (P5 mechanism)**:
3. **`is_ch_match`**: Bool carrier promoted from `ch` (Trim pattern detection)
   - Pattern: `ch = s.substring(p, p+1)` → `ch == "\""` equality chain
   - Promotion: `LoopBodyCarrierPromoter` converts to bool carrier

**Loop-Internal Only (No carrier needed)**:
- `ch`: LoopBodyLocal, promotion target → becomes `is_ch_match` carrier
- `has_next`: Escape sequence guard (Phase 176 scope)

---

## 2. Phase 175 Minimal PoC Scope

**Goal**: Prove multi-carrier support with 2 carriers (`p` + `str`), excluding escape handling

### 2.1 Minimal PoC Structure

```hako
_parse_string_min2() {
    me.s = "hello world\""
    me.pos = 0
    me.len = me.s.length()
    me.result = ""

    // 2-carrier version: p + result updated together
    loop(me.pos < me.len) {
        local ch = me.s.substring(me.pos, me.pos+1)
        if ch == "\"" {
            break
        } else {
            me.result = me.result + ch  // Carrier 2 update
            me.pos = me.pos + 1         // Carrier 1 update
        }
    }

    // Exit: both pos and result are used
    print("Parsed string: ")
    print(me.result)
    print(", final pos: ")
    print(me.pos)
}
```

**Carrier Count**: 2 (`pos`, `result`) + 1 promoted (`is_ch_match`) = **3 total**

**Excluded from Phase 175**:
- ❌ Escape sequence handling (`\\"`, `continue` path)
- ❌ Complex nested conditionals
- ✅ Focus: Simple char accumulation + position increment

---

## 3. P5 Multi-Carrier Architecture

### 3.1 Existing Boxes Already Support Multi-Carrier ✅

#### 3.1.1 LoopUpdateAnalyzer (Multi-carrier ready ✅)

**File**: `src/mir/join_ir/lowering/loop_update_analyzer.rs`
**API**: `identify_updated_carriers(body, all_carriers) -> CarrierInfo`
**Multi-carrier support**: ✅ Loops over `all_carriers.carriers`

**Code**:
```rust
pub fn identify_updated_carriers(
    body: &[ASTNode],
    all_carriers: &CarrierInfo,
) -> Result<CarrierInfo, String> {
    let mut updated = CarrierInfo::new();
    for carrier in &all_carriers.carriers {  // ✅ Multi-carrier loop
        if is_updated_in_body(body, &carrier.name) {
            updated.add_carrier(carrier.clone());
        }
    }
    Ok(updated)
}
```

#### 3.1.2 LoopBodyCarrierPromoter (Adds carriers ✅)

**File**: `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`
**API**: `try_promote(request) -> PromotionResult`
**Multi-carrier support**: ✅ Generates **additional carriers** from promotion

**Behavior**:
```rust
let promoted = LoopBodyCarrierPromoter::try_promote(&request)?;
carrier_info.merge_from(&promoted.to_carrier_info());  // Add promoted carrier
// Result: carrier_info.carriers = [pos, result, is_ch_match]
```

#### 3.1.3 CarrierInfo (Multi-carrier container ✅)

**File**: `src/mir/join_ir/lowering/carrier_info.rs`
**API**: `carriers: Vec<CarrierData>`, `merge_from(&other)`
**Multi-carrier support**: ✅ `Vec` holds arbitrary number of carriers

**Phase 175-3 Usage**:
```rust
let mut carrier_info = CarrierInfo::new();
carrier_info.add_carrier(CarrierData {  // Carrier 1
    name: "pos".to_string(),
    update_expr: UpdateExpr::Simple { ... },
});
carrier_info.add_carrier(CarrierData {  // Carrier 2
    name: "result".to_string(),
    update_expr: UpdateExpr::Simple { ... },
});
// carrier_info.carriers.len() == 2 ✅
```

#### 3.1.4 ExitMeta / ExitBinding (Multi-carrier ready ✅)

**File**: `src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs`
**API**: `carrier_exits: Vec<(String, ValueId)>`
**Multi-carrier support**: ✅ `Vec` holds all carrier exits

**ExitMetaCollector Behavior**:
```rust
for carrier in &carrier_info.carriers {  // ✅ Multi-carrier loop
    exit_bindings.push((carrier.name.clone(), exit_value_id));
}
// exit_bindings = [("pos", ValueId(10)), ("result", ValueId(20)), ("is_ch_match", ValueId(30))]
```

#### 3.1.5 ExitLineReconnector (Multi-carrier ready ✅)

**File**: `src/mir/builder/control_flow/joinir/merge/mod.rs`
**API**: `reconnect_exit_bindings(exit_bindings, loop_header_phi_info, variable_map)`
**Multi-carrier support**: ✅ Loops over all `exit_bindings`

**Behavior**:
```rust
for (carrier_name, _) in exit_bindings {  // ✅ Multi-carrier loop
    if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi_dst(carrier_name) {
        variable_map.insert(carrier_name.clone(), phi_dst);
    }
}
// variable_map: {"pos" -> ValueId(100), "result" -> ValueId(200), "is_ch_match" -> ValueId(300)}
```

---

## 4. Conclusion: Architecture Supports Multi-Carrier ✅ (Pattern2 limitation found)

### 4.1 Phase 175-3 Test Results

**Test**: `local_tests/test_jsonparser_parse_string_min2.hako`

**MIR Analysis**:
```mir
bb3:
    %9 = copy %8  // result = "" (initialization)

bb5:  // Loop header
    %25 = phi [%4, bb3], [%21, bb10]  // Only pos carrier!
    // ❌ Missing: result carrier PHI

bb10:  // Loop update
    %21 = %25 Add %20  // pos = pos + 1
    // ❌ Missing: result = result + ch update

bb12:  // Exit block
    %29 = copy %9  // Still uses original %9 (empty string)!
```

**Root Cause**: Pattern2's Trim optimization only emits `pos` carrier, ignoring `result` updates in loop body.

**Architecture Validation** ✅:
- ✅ `CarrierInfo` detected 3 carriers (`pos`, `result`, `is_ch_match`)
- ✅ `variable_map` contains all carriers at pattern2_start
- ✅ Existing boxes (ExitMeta, ExitLineReconnector) support multi-carrier
- ❌ Pattern2 lowerer only emits loop update for `pos`, not `result`

**Conclusion**:
- **Architecture is sound** - all boxes support multi-carrier
- **Pattern2 implementation gap** - Trim optimization doesn't emit body updates for non-position carriers
- **Phase 176 scope** - Extend Pattern2 to emit all carrier updates, not just position

### 4.2 Next Steps

- **Phase 175-3**: Run PoC test (`test_jsonparser_parse_string_min2.hako`)
- **Phase 176**: Add escape sequence handling (`continue` path, Phase 176 scope)
- **Phase 177**: Full `_parse_string` with all edge cases

---

## 5. Phase 176 で解決済み ✅ (2025-12-08)

**実装内容**:
- Pattern2 lowerer を全キャリア対応に拡張
- ヘッダ PHI / ループ更新 / ExitLine で複数キャリアを正しく処理
- CarrierUpdateLowerer ヘルパで UpdateExpr → MIR 変換を統一

**修正されたバグ**:
1. Trim pattern で loop_var_name が上書きされていた問題（pattern2_with_break.rs）
2. InstructionRewriter が loop_var を exit_bindings から除外していなかった問題

**テスト結果**:
- ✅ 2キャリア E2E テスト全てパス（pos + result）
- ✅ 回帰テストなし
- ✅ Trim pattern も正常動作

**次のステップ**: Phase 177 で JsonParser の複雑ループへ拡張

---

## 6. References

- **Phase 170**: LoopUpdateSummary design
- **Phase 171**: LoopBodyCarrierPromoter implementation
- **Phase 174**: P5 minimal PoC (quote detection only)
- **Phase 176**: Pattern2 multi-carrier implementation ([phase176-completion-report.md](phase176-completion-report.md))
- **Pattern Space**: [docs/development/current/main/loop_pattern_space.md](loop_pattern_space.md)
Status: Historical
