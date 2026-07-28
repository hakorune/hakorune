---
Status: Open external consultation packet; not implementation authority
Date: 2026-07-29
CurrentStop: NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0
Baseline: 67485b3bb9
Exception: User-requested independently shareable question packet. Distill the answer into the rolling card, then delete this packet.
ParentCurrentCard: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0 — 設計相談

## 依頼

selected normal/defaultの非Program rootについて、現在の受理範囲を狭めずに、
選択済みinvocation portへ移せるAST-node責務を決めてください。

実装はまだ行いません。回答では、全非Program rootを一度だけ分類する
disjoint partition、各branchのowner、同一commitで消える旧edgeを確定してください。

## 現在地

最新のproduction graphは次です。

```text
four selected normal callers
-> NormalCompileRequestV1
-> NormalDefaultPublishedPipelineV1
-> ModuleBuilderInvocationSessionV1
-> complete_normal_default_root_catalog_lifecycle(ast)
-> Program expansion when Program
-> prepare_module
-> one root clone
-> callable catalog seal/install
-> lower_root_after_callable_catalog_install_v1
-> RawInvocationChildPortV1
-> lower_root_after_callable_catalog_install_with_callable_port_v1
```

`NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0`で、次は削除済みです。

```text
ExistingGeneralModuleCompatibilityV1 = 0
selected normal -> MirBuilder::build_module = 0
normal compiler-side session.builder_mut()  = 0
```

明示compatibilityの`build_module` callerは二本残っています。

```text
legacy_candidate_session.rs
runtime/mirbuilder_emit.rs
```

これらは今回の対象外です。

## 現在の切断点

Program rootは既に選択済み`RawInvocationChildPortV1`を使いますが、
non-Program rootだけは末尾でportを捨てます。

```rust
match ast {
    ASTNode::Program { .. } => {
        // existing Program root orchestration
    }
    other => self.build_expression(other),
}
```

`build_expression`は、

```text
drive_raw_legacy_expression_v1
-> fresh RawLegacyChildLoweringPortV1
```

へ戻ります。

選択済みportを保持する既存capabilityはあります。

```rust
drive_legacy_expression_v1(
    builder,
    selected_port,
    expression,
)
```

`RootCallableCapturePortV1`は`RawAstChildLoweringPortV1`を継承しているため、
capability不足ではありません。

```text
capability missing = false
total parity proof = missing
```

## blanket cutoverを禁止する理由

`RawLegacyChildLoweringPortV1`と`RawInvocationChildPortV1`は、全AST nodeで
同じ意味ではありません。

確認済みの差分：

```text
static Main Box root:
  Legacy port     -> build_static_main_box
  invocation port-> nested Mainとしてfail-fast

ordinary Box methods:
  Legacy port     -> existing direct behavior
  invocation port-> collector-backed pending draft publication

Loop with reachable BoxDeclaration:
  Legacy port     -> cf_loop
  invocation port-> pure-plan/function-session bridge不足でfail-fast
```

したがって、

```rust
other => drive_legacy_expression_v1(self, callables, other)
```

への一括置換は、現行non-Program compatibilityを狭める可能性があります。

## 設計候補

### Candidate A — source-only disjoint partition

非Program ASTをBuilder effect前に一度だけ分類します。

```text
PortParity(node)
  -> selected invocation port

RootCompatibility(node)
  -> existing raw Legacy expression owner
```

新経路の失敗後にcompatibilityへ戻るのではありません。分類は一回、
実行も一回です。

回答では、全non-Program AST kindを次へ分類してください。

```text
selected-port parity
explicit root compatibility
separate design stop
normal file ingress outside contract
```

### Candidate B — root-aware invocation adapter

Main／Box／Loopを含むroot固有差分を一つのroot-aware portへ吸収し、
non-Program root全体を一度に選択portへ移します。

採用する場合は、collector/publication/Loop failure authorityを同時変更せず、
既存挙動を保存できる根拠が必要です。単なるLegacy facadeの改名は不可です。

### Candidate C — bounded safe sliceなし

port-parity branchを正確に分離できない場合は実装を開かず、このedgeを
typed compatibility residualとして残し、別のlive production edgeを選びます。

## 回答で確定してほしいもの

```text
1. Candidateとceremony
2. exact non-Program AST-node census / partition table
3. partition issuerとfailure owner
4. selected branchのproduction graph
5. compatibility branchの限定surfaceと削除条件
6. same-commit atomic old-edge delete set
7. parity / failure / reuse fixtures
8. 既存shared guardへの最小assertion
9. next executable row、またはNoSafeSlice
```

特に、次を曖昧にしないでください。

```text
self.build_expression(other)全体を削除するのか
選択したAST branchの旧edgeだけを削除するのか
Main / Box / Loopをどのownerが保持するのか
compatibility ownerを作るならどのrowで削除するのか
```

## Preserve

```text
Program root behavior
non-Program current accepted behavior
root/catalog lifecycle failure order
one root-level AST clone
callable catalog authority
collector atomic publication
normal result/verification policy
external candidate commit
explicit compatibility callers
```

## Forbid

```text
try selected port -> failure -> Legacy retry
family reselection after descent
source clone / reparse / AST reconstruction
whole-program or whole-function accepted variants
NarrowV1 / Stage-B authority reuse
new source-language grammar
Ownership / View activation
new per-row guard
facade-only rename with old authority retained
```

## 構造境界

```text
module_lifecycle.rs = 799 lines
```

同ファイルへの単純追記は禁止です。実装を選ぶ場合は、sibling ownerへの抽出か、
同ファイル内のnet-zero置換で責務を閉じてください。

全source/check fileは800行未満を維持します。LOC/file countは観測値であり、
受理形追加のpermission gateにはしません。

## 期待する最終回答

最終回答は、長い背景説明より次の実行可能性を優先してください。

```text
Decision
AST partition
owner graph
atomic delete set
focused evidence
hard stops
next executable row
```

安全な有限sliceがなければ、無理に実装taskを作らず`NoSafeSlice`と判定してください。
