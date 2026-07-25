# Language feature status index

Status: Current navigation / evidence index
Decision: `LANGUAGE-DOCS-STATUS-SSOT-D0`
Scope: discoverability only; this page does not create grammar or runtime meaning.

## How to read this page

Grammar status and support status are separate axes.

```text
grammar:
  canonical | compatibility_only | reserved | rejected | topic_owned

availability/profile:
  live | guarded | transport | scaffold | metadata-only | pending | deferred |
  prohibited | historical | status_conflict
```

Authority order:

1. [grammar contract](grammar-contract.md) and
   [`grammar/language-v1-registry.toml`](../../../grammar/language-v1-registry.toml)
   own spelling and normalization.
2. Topic SSOTs own semantic meaning and boundary law.
3. [Stage profiles](stage-profiles.md) own Stage0/Stage1 support.
4. [Concurrency semantics](../concurrency/semantics.md) and
   [boundary model](../concurrency/boundary-model.md) own concurrency profiles.
5. Historical documents are evidence only.

Parser acceptance and examples are implementation evidence, not authority.
When rows disagree, keep `status_conflict` and open a design decision; do not
silently select the most permissive document.

## Current index

| Feature | Grammar status | Availability/profile | Authority / evidence | Current note |
| --- | --- | --- | --- | --- |
| `throw` | `reserved`/`rejected` | `prohibited` | [scope-exit](scope-exit-semantics.md), [option](option.md), EBNF | Parser rejects with a stable freeze tag; no activation row. |
| statement `try` | `status_conflict` | `status_conflict` | [grammar contract](grammar-contract.md), EBNF, registry | SSOT says reserved/rejected; EBNF describes gated compatibility acceptance. Resolve in [LANGUAGE-DOCS-TRY-CATCH-D1](../../development/current/main/investigations/language-docs-status-ssot-cleanup-task-2026-07-25.md). |
| postfix `catch` | `canonical` / `status_conflict` | `status_conflict` | [grammar contract](grammar-contract.md), [semantic kernel](semantic-kernel.md), [scope-exit](scope-exit-semantics.md) | Grammar calls it canonical while canonical Fault is non-catchable; handler boundary is unresolved. |
| postfix `cleanup` | `canonical` | `status_conflict` | [scope-exit](scope-exit-semantics.md), EBNF, registry | Semantic channel is named, but EBNF support is Stage-3 gated and standalone/local productions are incomplete. |
| `fini { ... }` / `local ... fini` | `status_conflict` | `status_conflict` | [scope-exit](scope-exit-semantics.md), [lifecycle](lifecycle.md), grammar contract, registry | Grammar labels it canonical; scope/lifecycle describe it as legacy DropScope compatibility. |
| `box.fini()` | `canonical` | `live` only where lifecycle owner permits | [lifecycle](lifecycle.md), [scope-exit](scope-exit-semantics.md) | Object finalization is distinct from scope cleanup; last-strong drop does not imply user `fini()`. |
| `co` / `nowait` / `await` | topic-owned | `guarded` concurrency profile | [concurrency semantics](../concurrency/semantics.md), stage profiles | Not a general Stage1/selfhost promise; no Language v1 registry row currently. |
| `Channel<T>` | topic-owned | `scaffold`/`deferred` by route | [concurrency semantics](../concurrency/semantics.md), stage profiles | Reference queue surface; broad Program/MIR/LLVM use remains gated. |
| `sync box` / `context` / `task_scope` | topic-owned | `scaffold`/`deferred` | [concurrency boundary](../concurrency/boundary-model.md), stage profiles | Do not infer parser-live or broad runtime support from reference docs. |
| `worker_scope` / `parallel` / raw `thread` / `lock<T>` / `worker_local` | reserved/deferred | `prohibited`/`deferred` | [stage profiles](stage-profiles.md), concurrency docs | Design/substrate surfaces; no language-core activation. |
| `move` / `share` / `view` | registry/profile-dependent | `pending`/`prohibited` until exact row | [ownership](ownership.md), stage profiles | Target semantics are not parser support; retain the existing reject-row boundary. |
| `Option<T>` / `Result<T,E>` constructors | `canonical` narrow rows | `live` narrow enum/prelude profile | [option](option.md), EBNF, stage profiles | `?`, `try`, and `throw` are not implied by enum support. |

## Known conflict set

The following are intentionally not resolved by this index:

```text
try profile and default acceptance
catch versus canonical Fault
cleanup/fini grammar naming and Stage-3 gates
concurrency topic rows versus Language v1 registry rows
```

Resolution owner: [LANGUAGE-DOCS-TRY-CATCH-D1](../../development/current/main/investigations/language-docs-status-ssot-cleanup-task-2026-07-25.md), followed by a coordinated
grammar/registry/EBNF/profile update. No parser, runtime, backend, or grammar
registry implementation change is authorized by this index row.

## Historical rule

[`LANGUAGE_REFERENCE_2025.md`](LANGUAGE_REFERENCE_2025.md) is a historical
snapshot. Its examples and status tables must not override this index or the
current topic SSOTs. When a historical example is useful, cite it as migration
evidence and link back here.
