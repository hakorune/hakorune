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

`topic_owned`, `scaffold`, and `status_conflict` are navigation-index values:
they describe ownership or unresolved availability, not additional parser
acceptance. A row may use `status_conflict` on either axis while the decision
stop is open.

Only bracketed values such as `[freeze:contract][parser/throw_reserved]` are
existing implementation tags. Other gate/reject values in the table are
`index:` labels for this documentation stop and must not be passed to a parser
or runtime as if they were active switches.
```

Authority order:

1. [semantic contract charter](semantic-contract-charter.md),
   [semantic kernel](semantic-kernel.md), [grammar contract](grammar-contract.md), and
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

Historical examples for every row are collected in
[LANGUAGE_REFERENCE_2025.md](LANGUAGE_REFERENCE_2025.md); individual rows do
not inherit permission from that snapshot.

## Current index

| Feature | Grammar status | Availability/profile | Authority / evidence | Gate/reject tag | Promotion/retirement | Current note |
| --- | --- | --- | --- | --- | --- | --- |
| `throw` | `reserved`/`rejected` | `prohibited` | [scope-exit](scope-exit-semantics.md), [option](option.md), EBNF | `[freeze:contract][parser/throw_reserved]` | New language decision required; no implicit promotion | Parser rejects, including handler bodies; no activation row. |
| statement `try` | `status_conflict` | `status_conflict` | [grammar contract](grammar-contract.md), EBNF, registry | `[freeze:contract][parser/try_reserved]` or named compat profile | [LANGUAGE-DOCS-TRY-CATCH-D1](../../development/current/main/investigations/language-docs-try-catch-d1-consultation-2026-07-25.md) | SSOT says reserved/rejected; EBNF describes gated compatibility acceptance. |
| postfix `catch` | `canonical` / `status_conflict` | `status_conflict` | [grammar contract](grammar-contract.md), [semantic kernel](semantic-kernel.md), [scope-exit](scope-exit-semantics.md) | `NYASH_CATCH_NEW=1` and `NYASH_PARSER_STAGE3=1`; block/method productions additionally use their named gate | D1 must choose canonical outcome or compat-only retirement | Grammar calls it canonical while canonical Fault is non-catchable; handler boundary is unresolved. |
| postfix `cleanup` | `canonical` | `status_conflict` | [scope-exit](scope-exit-semantics.md), EBNF, registry | `NYASH_PARSER_STAGE3=1`; exact handler gate remains production-specific and unresolved | D1 must list exact productions and gate owner | Semantic channel is named, but EBNF support is Stage-3 gated and standalone/local productions are incomplete. |
| standalone `cleanup { ... }` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), EBNF, registry | `index:cleanup-production-missing` | D1 decides canonical production or rejection | Scope SSOT names it; EBNF has no complete standalone row. |
| `local x = e cleanup { ... }` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), EBNF, registry | `index:local-cleanup-production-missing` | D1 decides canonical production or rejection | Local cleanup is described semantically but not fully represented in EBNF. |
| `fini { ... }` / `local ... fini` | `status_conflict` | `status_conflict` | [scope-exit](scope-exit-semantics.md), [lifecycle](lifecycle.md), grammar contract, registry | `index:fini-alias-status-unresolved` | D1 chooses canonical spelling or Compat2025 sunset | Grammar labels it canonical; scope/lifecycle describe it as legacy DropScope compatibility. |
| `box.fini()` | `canonical` | `guarded`/`status_conflict` | [lifecycle](lifecycle.md), [scope-exit](scope-exit-semantics.md) | `index:lifecycle-owner-check` | Lifecycle owner must name the supported profile | Semantic target is distinct from scope cleanup; production support is not claimed for every backend. |
| `co` / `nowait` / `await` | `topic_owned` | `guarded` concurrency profile | [concurrency semantics](../concurrency/semantics.md), stage profiles | `concurrency-profile-required` | Separate concurrency decision | Not a general Stage1/selfhost promise; no Language v1 registry row currently. |
| `Channel<T>` | `topic_owned` | `scaffold`/`deferred` by route | [concurrency semantics](../concurrency/semantics.md), stage profiles | `channel-route-gated` | Separate concurrency decision | Reference queue surface; broad Program/MIR/LLVM use remains gated. |
| `sync box` / `context` / `task_scope` | `topic_owned` | `scaffold`/`deferred` | [concurrency boundary](../concurrency/boundary-model.md), stage profiles | `sync-boundary-gated` | Separate concurrency decision | Do not infer parser-live or broad runtime support from reference docs. |
| `worker_scope` / `parallel` / raw `thread` / `lock<T>` / `worker_local` | `reserved`/`topic_owned` | `prohibited`/`deferred` | [stage profiles](stage-profiles.md), concurrency docs | `concurrency-substrate-only` | New language decision required | Design/substrate surfaces; no language-core activation. |
| `move` / `share` / `view` | `topic_owned` | `pending`/`prohibited` until exact row | [ownership](ownership.md), stage profiles | `OWN-GRAM-REJECT0` | Ownership grammar row required | Target semantics are not parser support; retain the existing reject-row boundary. |
| `Option<T>` / `Result<T,E>` constructors | `canonical` narrow rows | `live` narrow enum/prelude profile | [option](option.md), EBNF, stage profiles | `option-result-narrow` | Expand only by explicit capability row | `?`, `try`, and `throw` are not implied by enum support. |

## Known conflict set

The following are intentionally not resolved by this index:

```text
try profile and default acceptance
catch versus canonical Fault
cleanup/fini grammar naming and Stage-3 gates
concurrency topic rows versus Language v1 registry rows
EBNF exception/rethrow/finally pseudo-lowering and no-op backend wording
```

Resolution owner: [LANGUAGE-DOCS-TRY-CATCH-D1](../../development/current/main/investigations/language-docs-try-catch-d1-consultation-2026-07-25.md), followed by a coordinated
grammar/registry/EBNF/profile update. No parser, runtime, backend, or grammar
registry implementation change is authorized by this index row.

The EBNF exception/rethrow/finally lowering text is evidence of an older or
compatibility route, not permission to degrade an unsupported handler to a
no-op. Until D1 closes the route, unsupported handler/backend combinations
must reject before user-visible effects.

## Historical rule

[`LANGUAGE_REFERENCE_2025.md`](LANGUAGE_REFERENCE_2025.md) is a historical
snapshot. Its examples and status tables must not override this index or the
current topic SSOTs. When a historical example is useful, cite it as migration
evidence and link back here.
