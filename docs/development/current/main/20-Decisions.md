# Self Current Task — Decisions (main)

Status: Public Stub
Private Canonical Path: `docs/private/development/current/main/20-Decisions.md`

## Purpose

- Public repo には最小の方針サマリだけを置く。
- 実運用の詳細 decision log は private canonical で管理する。

## Public Summary

- Selfhost / de-rust mainline priority を維持する。
- `stage0 / stage1 / stage2-mainline / stage2+` は execution-lanes-and-axis-separation-ssot.md の build/distribution vocabulary として読む。
- `K0 / K1 / K2` は kernel-replacement-axis-ssot.md の replacement reading として読む。
- except for OS/kernel/substrate boundaries and explicit compat/bootstrap keeps, implementation should move to `.hako` rather than grow new Rust meaning owners.
- selfhost mirbuilder migration 中は Rust builder に新しい source-aware lowering / shape intelligence を増やさず、canonical MIR / MIR-to-MIR / backend optimization を継続しながら `.hako` builder authority を先に進める。ただし `CURRENT_STATE.toml` が明示的に選択した MirBuilder in-place replacement の既存production authority整理は、この一般則の限定例外としてそちらを優先する。
- backend lane vocabulary (`llvmlite`, `ny-llvmc`, `native`) は stage2-aot-fast-lane-crossing-inventory.md と llvm-harness.md を正本にする。
- current active lane / blocker / latest-card pointer は `CURRENT_STATE.toml` を正本にする。`CURRENT_TASK.md` と thin mirrors は必要時だけそこへ誘導する。
- Language v1 semantic coherence now precedes selfhost migration. The public
  seven-law contract is `docs/reference/language/semantic-contract-charter.md`;
  current topic activation follows `CURRENT_STATE.toml`.
- Function exit and entry-result semantics are accepted as
  `FUNCTION-EXIT-SEMANTICS-prime-r1`: ordinary functions and `Main.main` use
  ExplicitReturnOnly, Script evaluation has its own final-expression result,
  and process termination is a separate projection. The normative owner is
  `docs/reference/language/function-exit-and-entry-result.md`.
- Normal source planning is accepted as `NORMAL-SOURCE-PLAN0-prime-r1`: one
  source-owned classifier seals Script, Main0, or CallableModule exactly once;
  profile admission is separate, the existing narrow VM-reference profile is
  frozen, and canonical-core activation uses a separately named profile.
  The durable task order is
  `docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md`.
- Language failure/exit C′ is accepted as
  `LANGUAGE-RESULT-EXIT-C-PRIME0-D0`: recoverable failure is `Result<T,E>`,
  unchanged propagation is typed Result-only postfix `?`, `Option ?`, source
  `try`/`throw`/`catch`, and `RecoverableFailure` are rejected, terminal
  `Fault` remains non-catchable, and lexical cleanup has one standalone
  `cleanup {}` spelling. C′ lifecycle makes Box-member `fini {}` a
  non-callable last-Home hook; `close()`/`shutdown()` remain ordinary methods.
  Its lifecycle Decision row is
  `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0`.
  Both targets have production activation 0 and do not change the current
  lane. After implementation and backend parity, the mandatory Result/cleanup
  and Home/lifecycle DOC0 receipts must update EBNF, registry, both parsers,
  reference pages, examples, and migration guides from landed behavior.
  Durable owners are
  `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`
  and
  `docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md`.
- AI-verifiable development North Star is accepted as
  `AI-VERIFIABLE-DEVELOPMENT-NORTH-STAR0-D0`. Hakorune optimizes for the
  fewest and cheapest verified iterations to correct high-performance code,
  not grammar size or a blanket claim of replacing Rust. Canonical source,
  one directional Facts/Recipe/Verify authority, stable machine-readable
  diagnostics, read-only semantic queries, capability-backed repairs, exact-
  front latency, corpus, and tooling are one product goal. This policy opens
  no current lane and claims no query/repair API before implementation-backed
  closeout. The durable owner is
  `docs/development/current/main/design/ai-verifiable-development-north-star-ssot.md`.
- minimal MirBuilder execution path frontier review resolves the design stop to `MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001`; the older MirModule gap is already closed on current `public-main`.
- the current selfhost roadmap now narrows the remaining work to family-by-family `HakoAdopted` decisions, Python SemanticProjector freeze, and consultation-gated ABI / syntax boundaries.
- `stage2-mainline` への entry task pack は `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md` を正本にする。

## Migration Rule

- private 側で decision を更新した場合、public 側には必要最小限の summary のみ反映する。
- machine guard が依存する文書（`CURRENT_TASK.md` など）へは、必要な同期のみ行う。
