---
Status: Design consultation stop
Date: 2026-07-20
Scope: define measured production-call and route evidence for FINALIZE0 census
Parent: docs/development/current/main/investigations/mirbuilder-finalize0-census-task-2026-07-20.md
---

# FINALIZE0-CENSUS0-P0 production census consultation

## Question

`FINALIZE0-CENSUS0-P0` で、schema v2 の
`production_invocation_count`、`route_reachability`、
`canonical_repair_reachable` を手書き値から機械証拠へ置き換えたい。

どの proof architecture を採択すべきか。

推奨は Candidate A-prime である。ただし、Rust source scanner 自体を
新しい semantic authority にせず、production CallExpr の静的 census と
selected route の runtime observation を別製品にする。

## Confirmed problem

SCHEMA0 は operation taxonomy と登録済み source anchor の双方向照合を
閉じた。しかし、次の三 field はまだ fixture の宣言値である。

```text
production_invocation_count
route_reachability
canonical_repair_reachable
```

現在の count は意味が統一されていない。

```text
inline finalizer operation:
  physical operation statement count = 1

TypePropagationPipeline child:
  direct production callers of facade = 3

semantic/contract child:
  child statement count = 1
  facade production callers = multiple
```

runtime invocation count は module の function 数、loop、外部 API caller に
依存するため、finite static inventory としては定義できない。

また、現 fixture には既に一つの concrete drift がある。

```text
build_resolved_trivial_function_module
  -> finalize_module

current module rows:
  CanonicalBindingSsa route missing
```

## Read-only source findings

default production source上の直接 CallExpr 候補は次の通り。

```text
TypePropagationPipeline::run = 3
annotate_missing_result_types_from_calls_and_await = 2
verify_typed_values_are_defined = 3
materialize_all_phi_inputs = 3
finalize_module = 5
finish_built_module = 2
refresh_module_semantic_metadata = 5
refresh_and_validate_for_boundary = 2 direct sites
refresh_owned_for_boundary = 4 boundary ingress sites
```

ただし、これは `rg` 結果を source context で手分類した preliminary count
であり、P0 completion proof ではない。

`finalize_function_draft` は特に filename heuristic が危険である。
`exprs_check.rs` と `indexing.rs` の call は普通の production風 path にあるが、
実際には inline `#[cfg(test)] mod tests` 内にある。逆に filenameだけで
`tests.rs` を除外する方法では、module graph inclusion を証明できない。

## Candidate A-prime — static CallExpr census plus route observation

### Static product

```text
Rust production module graph
  -> cfg-aware direct CallExpr inventory
  -> reachability-entry family
  -> measured static call-site count
```

count の意味を次に固定する。

```text
production_invocation_count =
  count(distinct production CallExpr source sites
        whose resolved target is the row's reachability entry symbol)
```

runtime execution multiplicity は claim しない。

必要なcheck-side製品:

```text
tools/checks/lib/rust_source_reverse_census.py
  Rust lexical/token boundary
  Cargo target/module inclusion
  cfg(test) / cfg(feature) domain
  direct CallExpr source identity

tools/checks/lib/mirbuilder_finalize0_production_census.py
  FINALIZE0 entry-family mapping
  child-operation reverse coverage
  route-root graph
  schema-v2 comparison

tools/checks/fixtures/mirbuilder_finalize0_route_roots_v1.json
  stable root symbols
  route labels
  allowed guard vocabulary
  helper allowlist with reason
```

scanner が unresolved alias、wildcard import、macro-generated call、indirect
function/trait dispatch に遭遇した場合、推測せず typed check failure とする。

### Route axes

現 `route_reachability` は次を一配列に混ぜている。

```text
authority route:
  LegacyBuilder
  CanonicalAPlus
  CanonicalBindingSsa
  JoinIrBridge

execution boundary:
  CompilerPostBuild
  BackendPreflight
  VmExecution
  MirJsonExport
  HostProvider
  NyLlvmc
```

A-prime では二軸へ分ける。

```text
reachable_authority_roots
reachable_boundary_roots
```

各 edge は source-verified CallExpr と guard witness を持つ。

### Canonical repair evidence

次を同一視しない。

```text
canonical_repair_code_reachable:
  canonical rootからrepair child codeへ静的に到達可能

canonical_repair_delta_observed:
  selected canonical fixtureで実際にMIR/fact deltaが発生
```

P0a は前者を閉じる。後者は test-only observer を用いる P0b で閉じる。
production log、environment toggle、persistent compiler counterは追加しない。

### Facade child reverse census

各 facade body の direct CallExpr は必ず次のどちらかに属する。

```text
inventory child operation
reason付き nonsemantic/helper allowlist
```

未分類 child が一つでもあれば P0 を閉じず、schema補正へ戻る。

既に追加棚卸しが必要な候補:

```text
optimizer child schedules
contract validation and carrier-summary children
semantic all-functions/layout/fixpoint children
callsite canonicalization rewrite children
extern route/result-fact children
```

## Candidate B — explicit anchor manifest only

全 CallExpr を手書きfixtureへ列挙し、substring ordinalだけを検査する。

利点:

```text
small implementation
fast guard
```

不採択理由:

```text
cfg(test) classification is manual
new unregistered callee spelling can escape reverse census
route edges remain assertions
alias/macro/indirect call ambiguity is hidden
```

SCHEMA0 の登録anchor照合を繰り返すだけで、P0の不足を閉じない。

## Candidate C — dynamic observation only

selected fixturesでfacade/child countersを観測し、static source censusを持たない。

利点:

```text
actual selected route and actual repair delta are observable
```

不採択理由:

```text
unexecuted production callsites are invisible
fixture coverage becomes source inventory authority
repository-wide reverse coverage cannot be claimed
```

dynamic observation は A-prime の第二証拠としてのみ使う。

## Recommended task order

```text
FINALIZE0-CENSUS0-P0a-S0
  generic cfg/module-aware source scanner
  FINALIZE0 policy = 0

FINALIZE0-CENSUS0-P0a-P0
  entry-family direct CallExpr counts
  facade child reverse census
  authority/boundary root separation

FINALIZE0-CENSUS0-P0b-S0
  test-only child-id / changed-bit observation
  production state/log/env delta = 0

FINALIZE0-CENSUS0-P0b-P0
  Legacy / A+ / BindingSSA selected fixtures
  potential reachability versus actual delta matrix

FINALIZE0-CENSUS0-P0-G0
  no manual count/route/canonical-repair booleans
  unresolved symbol uses = 0
  unclassified facade children = 0

then:
  FINALIZE0-VERIFY-SPLIT0
```

Every new or modified source/check file remains below 800 lines.

## Decisions requested

1. Is `production_invocation_count` fixed to distinct production direct
   CallExpr sites, never runtime multiplicity?
2. Is Candidate A-prime selected, with static census and runtime delta
   observation as separate proof products?
3. Are authority roots and execution-boundary roots split into separate axes?
4. Must every facade direct child be inventory-owned or reason-allowlisted?
5. May P0a close static potential reachability before P0b closes actual
   repair-delta observation?

## Implementation may claim after full P0

```text
production direct-call counts are source-derived
test-only CallExpr sites do not inflate production counts
every registered facade child has one semantic or helper disposition
authority and boundary route roots are independently measured
canonical code reachability and observed repair delta are not conflated
```

## Implementation must not claim

```text
runtime invocation multiplicity
all external public API callers are known
static reachability proves repair mutation
filename or substring alone proves cfg domain
fixtures are a substitute for reverse source census
macro/trait/indirect calls were resolved when they were not
```

## Stop conditions

1. cfg(test) must be classified by path/filename heuristic alone.
2. unresolved alias, wildcard, macro, trait, or function-pointer call is guessed.
3. potential static reachability is reported as actual repair delta.
4. facade child operations remain unclassified.
5. one route array continues to mix authority and boundary roots.
6. runtime count is represented as a finite static scalar.
7. P0 requires compiler behavior, production logging, or persistent counters.
8. any source/check file reaches 800 lines.
