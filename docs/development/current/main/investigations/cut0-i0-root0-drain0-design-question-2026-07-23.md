# CUT0-I0 ROOT0-DRAIN0 設計相談

Status: **Closed — Candidate D-prime-r1 selected; execution task next**

Related:

- `cut0-i0-root0-canon0-fixture0-execution-task-2026-07-23.md`
- `cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md`
- `cut0-i0-root0-drain0-execution-task-2026-07-23.md`
- `cut0-i0-root0-design-stop-2026-07-22.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Pro先生へ渡す要約

CANON-FIXTURE0-S0/P0/C0/G0は、compiler-owned bridgeの4 canonical routeを
`source -> token -> package -> physical owner -> receipt -> completion`まで
実証して閉じた。次のDRAIN0では、`CanonicalPhysicalCompleteInvocationV1`を
by-valueで一度だけconsumeし、保持しているheader/catalogとcollector-issued
receiptから具体的なexpected inventoryをsource-drivenに投影したい。しかし現状
のcompiler completionにはdrain terminalがなく、既存Builder drainはcaller指定
symbols・`require_main`・`ConditionFnPolicyV1::Optional`を受け取り、旧
`DrainedModuleCandidateV1`は無条件に`main`を要求する。さらにcompiler側の
`CanonicalRoutePolicyV1`とbuilder側の`RouteOwnedInvocationInventoryV2`が分離し、
shell/collectorのunpack可視性もbuilder内に閉じている。旧drainへ接続すると、
source authorityとphysical ownerを再構築するため、ここで設計を止める。

## 現在の証拠

```text
CANON-FIXTURE0 four-route aggregate = 4 passed
canonical completion product         = Single / Callable only
completion retains token/session/physical/receipt/capability
compiler completion drain consumer   = 0
old InvocationDrainExpectation      = caller-authored inventory
old DrainedModuleCandidate           = unconditional MissingMain
old canonical_root_completion        = Builder-only scaffold; do not edit
old drain production consumers       = 0
```

## Authority / non-authority

| boundary | authority | non-authority |
|---|---|---|
| exact inventory | retained canonical owner header or verified callable catalog | caller `Vec<String>`, `require_main`, `Optional` |
| physical state | the same completion-owned shell/collector/receipt product | `ModuleLoweringInvocationDrainOwnerV1::new(shell, collector)` |
| route policy | one shared policy SSOT selected before lowering | re-observing `current_module` or module map |
| root names | canonical keys/header/catalog; no synthetic roots | bare `main`/`condition_fn` assumptions |
| receipt provenance | collector-issued receipt moved into drained witness | loose receipt, clone, rebrand, reacquisition |

## Decision questions

### Q1 — drain terminal owner

Where is the one-shot physical drain terminal owned?

1. **Compiler completion private terminal (recommended candidate)**

   `CanonicalPhysicalCompleteInvocationV1::prepare_drain(self)` derives the
   source inventory, then calls one narrow builder physical-drain terminal.

2. **Builder-owned source-aware terminal**

   Move source continuation/inventory authority into builder and let builder
   consume the compiler completion. This risks duplicating compiler source
   authority.

3. **Reuse old `ModuleLoweringInvocationDrainOwnerV1`**

   Rejected candidate: it accepts caller-authored inventory and reconstructs
   shell/collector ownership.

### Q2 — inventory SSOT and granularity

Should DRAIN0 project exact identity rows or only symbols?

Recommended candidate:

```text
single:
  exact canonical owner key + symbol + arity + publication policy

callable:
  exact catalog key + symbol + arity + cardinality + policy, deterministic order
```

`RouteOwnedInvocationInventoryV2` currently owns policy/root semantics but not
all concrete rows. Decide whether to extend that neutral SSOT or introduce a
canonical inventory product without duplicating policy.

### Q3 — physical unpack and receipt fate

The completion currently owns session plus opaque collected physical products.
Choose the one terminal that unpacks shell/collector/receipt exactly once.
The drained result must retain a compact non-Clone receipt/source witness for
the later finalizer; receipt assertion followed by drop is forbidden.

### Q4 — drained product shape

Should DRAIN0 issue a new route-specific product?

Recommended candidate:

```text
CanonicalDrainedInvocationV1::Single
CanonicalDrainedInvocationV1::Callable
```

Each variant retains the original token/session, drained module or draft
collection, exact source continuation, receipt witness, and recursive/acyclic
capability witness. Adapting old `DrainedModuleCandidateV1` is rejected because
it imposes synthetic `main` and old condition policy.

### Q5 — failure and one-shot law

Recommended law:

```text
inventory/header/catalog/receipt mismatch
published shell
missing/surplus row
foreign brand
  -> rejected complete owner before shell mutation
  -> live Builder unchanged
  -> retry/fallback/second drain = 0
```

Repeated drain must be structurally impossible by consuming the complete
product. Raw remains on its already closed RAW0 chain and is not silently
merged into canonical DRAIN0.

### Q6 — lifetime and source re-observation

Single routes may retain an owned header; callable routes may borrow the exact
verified catalog/source carried by the package. No re-resolution, catalog
reacquisition, `current_module` read, or module-map reconstruction is allowed.

## Candidate D-prime-r1 decision lock

```text
exact source-bound complete
-> compiler private source-derived inventory preflight
-> one builder physical unpack/drain terminal
-> CanonicalDrainedInvocationV1 route enum
-> later finalizer (separate row)
```

Q1–Q6 are accepted as D-prime-r1. The detailed implementation order and
acceptance evidence are now owned by
`cut0-i0-root0-drain0-execution-task-2026-07-23.md`.

The old drain owner remains disconnected. Production drain, finalization,
external commit, fallback, retry, and Raw convergence remain forbidden.

## Required answer shape

```text
Q1 terminal owner
Q2 inventory SSOT/granularity
Q3 physical unpack + receipt witness
Q4 drained product shape
Q5 failure/one-shot law
Q6 lifetime/re-observation law
```

The next executable card is
`cut0-i0-root0-drain0-execution-task-2026-07-23.md`.
