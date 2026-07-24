# RAW public cutover PARITY0 design question

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-prime-r1`

ワーカーによる read-only 棚卸しで、Legacy-vs-Raw 全体比較の正規化 authority と
fixture 範囲が未固定であることを確認した。PARITY0 は production cutover ではなく、
test-only の bounded parity proof として実装する。

## Q1 — normalized snapshot の authority

**A を採用する。** 正規化 snapshot は test-only sibling の一箇所だけで生成する。
production lowerer、`MirPrinter`、`module_to_mir_json`、parser、backend serializer は
parity authority にしない。

```text
raw_public_cutover_parity_snapshot.rs
  function symbol lexical order
  entry block + fixed edge order
  definition order ValueId normalization
  allowlisted MIR instruction/value/op snapshot
  unknown shape = typed fail-fast
```

snapshot は invocation-local ID、HashMap iteration、debug formatting に依存しない。

## Q2 — success matrix

**A を採用する。** NarrowV1 の admitted grammar に限定し、次を固定する。

```text
Script empty
7 literal variants
3 admitted unary operators
16 ordinary binary operators (And/Or は除外)
Expr / Print / Local / Assignment / CompoundAssignment
App Main empty/scalar
App exact-empty StaticHelper0
optimize on/off
source-file hint
```

CompoundAssignment は ordinary operator 表の受理範囲をそのまま使用し、算術だけへ
暗黙に狭めない。metadata 比較は `source_file`、Main declaration facts、symbol、
arity、return、effects、specialized-lane absence の明示フィールドに限定する。

## Q3 — failure and reuse

**A を採用する。** 失敗行は stable raw-public stage/code、live Builder 不変、結果なし、
fallback なしを一組で証明する。REPL、invalid root、Script declaration、非 Main App、
If/Loop/LoopRange/Return/Break/Continue/ScopeBox、And/Or、unsupported unary、typed
local/cardinality drift、invalid assignment、App metadata/arity drift、undefined
variable、helper outside StaticHelper0、dirty publication target を対象にする。

再利用は別 test-only sibling で固定する。

```text
Raw -> Raw
Raw failure -> Raw
Raw -> Legacy
Legacy -> Raw
```

POST0 fault injection、production fallback、JSON/executor/selfhost/fastmem は対象外。

## Q4 — implementation shape and stop line

**A を採用する。** production source delta は原則 0 とし、test-only の次の4箱だけを
許可する。

```text
raw_public_cutover_parity_snapshot.rs
raw_public_cutover_parity_success_p0.rs
raw_public_cutover_parity_failure_p0.rs
raw_public_cutover_reuse_p0.rs
```

`compiler/mod.rs` は test module の登録だけに留め、各 source/check file は 800 行未満。
snapshot mismatch が出た場合は PARITY0 内で production policy を修正せず、専用の
repair/design row へ停止する。

## Q5 — retirement ordering

**A を採用する。** PARITY0 の次は旧 Raw chain retirement の proof-migration row とする。
旧 Raw chain は non-test caller が既に 0 であり、legacy `compile_with_source` の通常入口
とは別の disconnected surface なので、retirement と normal-entry cutover は混ぜない。

```text
PARITY0
  -> OLD-RAW-RETIRE0-R0a proof migration
  -> OLD-RAW-RETIRE0-R0b source/variant deletion
  -> measured normal-entry cutover decision
```

## Q6 — guard contract

guard は implementation profile として、次を別々に測定する。

```text
normal-entry Raw consumer = 0
normalizer producer = 1
Legacy/Raw pair helper = 1
literal=7 / unary=3 / binary=16
reuse direction=4
MirPrinter/module_to_mir_json/parser authority=0
fallback/catch_unwind/env fault hook=0
JSON/runtime/executor delta=0
all modified source/check files < 800
```

`#[cfg(test)]` を含む test-only caller は production census から除外する。guard は
active/closed の文言に過度に依存せず、closeout 後も再実行できる構造にする。

## Non-claims

```text
normal compile_with_source cutover
public ingress change
old Raw source deletion
eligibility/grammar widening
JSON, executor, selfhost, fastmem, CUT0
production serializer or normalizer
```
