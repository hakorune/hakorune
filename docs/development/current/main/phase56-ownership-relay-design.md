# Phase 56: Ownership-Relay Design

## Overview
「読むのは自由、管理は直下 owned だけ」アーキテクチャの設計文書。

Phase 56 は **インターフェース設計のみ**。実装は Phase 57 以降。

## Core Definitions

### owned (所有)
- 変数を定義したスコープが唯一の owner
- Loop直下の `local x` → そのループが owned
- body-local（if/block内の local）→ 最も内側の enclosing loop が owned

**例**:
```rust
fn outer() {
    local a = 0    // owned by outer
    loop {
        local b = 0    // owned by this loop
        if cond {
            local c = 0    // owned by enclosing loop (not if!)
        }
    }
}
```

### carriers (管理対象)
- `carriers = writes ∩ owned`
- そのスコープが定義 AND 更新する変数のみ
- loop_step の引数として管理

**重要**: Carrier は「所有 AND 更新」のみ。読み取り専用の owned 変数は carrier ではない。

### captures (読み取り参照)
- `captures = reads \ owned` (かつ carriers ではない)
- 祖先スコープの変数への read-only アクセス
- CapturedEnv / ConditionEnv 経由

**例**:
```rust
local limit = 100
loop {
    local sum = 0
    if sum < limit {  // limit は capture (read-only)
        sum++         // sum は carrier (owned + written)
    }
}
```

### relay (中継)
- 内側スコープが祖先 owned を更新する場合
- 更新責務を owner へ昇格（relay up）
- 中間ループは引数として素通し

**例**:
```rust
loop outer {
    local total = 0    // owned by outer
    loop inner {
        total++        // relay to outer (inner doesn't own total)
    }
}
// outer の exit PHI で total を merge
```

## Invariants (不変条件)

1. **Ownership Uniqueness**: 各変数は唯一の owner を持つ
2. **Carrier Locality**: carriers = writes ∩ owned (借用なし)
3. **Relay Propagation**: writes \ owned → owner に昇格
4. **Capture Read-Only**: captures は read-only (PHI 不要)

## Shadowing Rules

```nyash
local x = 0          // outer owned
loop {
    local x = 1      // inner owned (shadows outer)
    // 外の x は capture 可能だが、inner の x が優先
    print(x)         // inner の x (1)
}
print(x)             // outer の x (0)
```

- Shadowing = 新しい ownership 発生
- 名前解決は最内スコープ優先
- 外の x は capture として参照可能だが、内の x が存在する限り内が優先

## Multi-Writer Merge

```nyash
loop outer {
    local total = 0
    if a { loop inner1 { total++ } }  // relay to outer
    if b { loop inner2 { total-- } }  // relay to outer
}
// outer の exit PHI で merge
```

- Relay は「更新意図の伝達」
- 実際の PHI merge は owner 側で実行
- 複数の内側ループが同じ変数を relay → owner の exit PHI で統合

## JoinIR Mapping

### Current System → New System

| Current | New |
|---------|-----|
| CarrierInfo.carriers | OwnershipPlan.owned_vars (where is_written=true) |
| promoted_loopbodylocals | (absorbed into owned analysis) |
| CapturedEnv | OwnershipPlan.captures |
| ConditionEnv | OwnershipPlan.condition_captures |
| (implicit) | OwnershipPlan.relay_writes |

### OwnershipPlan Structure

```rust
pub struct OwnershipPlan {
    pub scope_id: ScopeId,
    pub owned_vars: Vec<ScopeOwnedVar>,  // All owned vars (carriers = is_written subset)
    pub relay_writes: Vec<RelayVar>,
    pub captures: Vec<CapturedVar>,
    pub condition_captures: Vec<CapturedVar>,
}
```

**設計意図**:
- `owned_vars`: このスコープが所有する変数（更新されるものは carriers）
- `relay_writes`: 祖先の変数への書き込み（owner へ昇格）
- `captures`: 祖先の変数への読み取り専用参照
- `condition_captures`: captures のうち、条件式で使われるもの

## Implementation Phases

- **Phase 56**: Design + interface skeleton (this phase) ✅
- **Phase 57**: OwnershipAnalyzer implementation (dev-only)
- **Phase 58**: P2 plumbing (dev-only)
- **Phase 59**: P3 plumbing (dev-only)
- **Phase 60**: Single-hop relay threading for fixtures (dev-only)
- **Phase 61**: P3 側の接続点を決めて段階接続（dev-only）
  - まずは fixtures ルート（Program(JSON v0)）で、if-sum+break を別箱として構造的に接続する
  - 詳細: `docs/development/current/main/PHASE_61_SUMMARY.md`
  - MIR→JoinIR の本番ルート（`pattern3_with_if_phi.rs`）へ寄せるのは別フェーズで設計→接続

## Module Boundary

`src/mir/join_ir/ownership/` - 責務は「解析のみ」

**This module does**:
- ✅ Collect reads/writes from AST/ProgramJSON
- ✅ Determine variable ownership (owned/relay/capture)
- ✅ Produce OwnershipPlan for downstream lowering

**This module does NOT**:
- ❌ Generate MIR instructions
- ❌ Modify JoinIR structures
- ❌ Perform lowering transformations

Lowering/MIR生成は既存モジュールが担当。

## Example Ownership Plans

### Example 1: Simple Loop

```nyash
local sum = 0
loop {
    sum++
}
```

**OwnershipPlan (loop scope)**:
- `owned_vars`: [`sum` (is_written=true)]
- `relay_writes`: []
- `captures`: []

### Example 2: Nested Loop with Relay

```nyash
local total = 0
loop outer {
    loop inner {
        total++
    }
}
```

**OwnershipPlan (inner loop)**:
- `owned_vars`: []
- `relay_writes`: [`total` → relay to outer]
- `captures`: []

**OwnershipPlan (outer loop)**:
- `owned_vars`: [`total` (is_written=true, via relay)]
- `relay_writes`: []
- `captures`: []

### Example 3: Capture + Carrier

```nyash
local limit = 100
loop {
    local sum = 0
    if sum < limit {
        sum++
    }
}
```

**OwnershipPlan (loop scope)**:
- `owned_vars`: [`sum` (is_written=true)]
- `relay_writes`: []
- `captures`: [`limit` (read-only)]
- `condition_captures`: [`limit`]

## References
- **Phase 53-54**: Structural axis expansion
- **Phase 43/245B**: Normalized JoinIR completion
- **ChatGPT discussion**: 「読むのは自由、管理は直下だけ」設計
- **JoinIR Architecture**: [joinir-architecture-overview.md](joinir-architecture-overview.md)

## Phase 57: Algorithm Implementation

### Analysis Steps

1. **Scope Tree Construction**
   - Function/Loop/Block/If each get a ScopeId
   - Parent-child relationships tracked

2. **Variable Collection (per scope)**
   - `defined`: Variables declared with `local` in this scope
   - `reads`: All variable reads (including nested)
   - `writes`: All variable writes (including nested)
   - `condition_reads`: Variables read in loop/if conditions

3. **Ownership Assignment**
   - Body-local rule: `local` in if/block → enclosing Loop/Function owns it
   - `owned_vars` = variables defined in Loop/Function scopes

4. **Plan Generation**
   - `carriers` = owned_vars where is_written=true
   - `relay_writes` = writes - owned (find owner in ancestors)
   - `captures` = reads - owned - writes (read-only)
   - `condition_captures` = captures ∩ condition_reads

### Implementation Details

**Input JSON Compatibility**:
- テスト用の簡易スキーマ: top-level `functions` + stmt/expr の `kind` を解釈
- Program(JSON v0): top-level `defs` + stmt/expr の `type` を解釈
- `Local` は JSON v0 で「新規束縛」と「rebind/update」が混在し得るため、
  解析では「scope chain で既に定義済みなら write / 未定義なら define」として扱う（dev-only 前提）。
  - Note: `docs/private` は submodule のため、fixture JSON を参照する場合は submodule 側で追跡されていることを前提とする。

**Body-Local Ownership Rule**:
```rust
// Example: local in if/block → enclosing loop owns it
loop {
    if cond {
        local temp = 0  // owned by LOOP, not if!
    }
}
```

**Relay Path Construction**:
- Walk up ancestor chain to find owner
- Collect only Loop scopes in relay_path (skip If/Block)
- Inner loop → Outer loop → Function (relay chain)

**Invariant Verification** (debug builds):
- No variable in multiple categories (owned/relay/capture)
- All relay_writes have valid owners
- condition_captures ⊆ captures

## Status

- ✅ Phase 56: Design + interface skeleton completed
- ✅ Phase 57: Analyzer implemented (dev-only)
- ✅ Phase 58-59: plan_to_lowering helpers (P2/P3) with Fail-Fast relay
- ✅ Phase 60 (dev-only): single-hop relay threading for P2 fixtures
  - `plan_to_p2_inputs_with_relay` promotes relay_writes to carriers (relay_path.len()<=1 only)
  - Frontend Break(P2) lowering uses ownership-with-relay; legacy path preserved for comparison
  - P3 stays analysis-only; real threading is Phase 61+
