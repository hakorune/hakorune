# CUT0-I0 RAW-SOURCE0 Consultation

Status: **Design stop — Candidate SOURCE-FIRST-prime-r1 selected; RAW-SOURCE0-PLAN0 next**
Date: 2026-07-23
Scope: Builder-side Raw source authority only. No production executor,
public-ingress wiring, runtime JSON change, or `MirBuilder::build_module`
retirement is allowed in this row.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-atomic-cutover-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`

## Decision inherited from SOURCE-FIRST-prime-r1

```text
current executor scope = disconnected
Raw authority          = new compiler-owned source-bound ingress
AST JSON               = future parity-gated Raw route
Program(JSON v0)       = separate design row / existing compatibility lane
Builder config         = one sealed outer snapshot
activation evidence   = measured zero direct Builder callers at CUT0
```

## Builder-side ownership chain

```text
LegacyModuleLoweringInputV1
  + sealed outer ingress options
  + BuilderInvocationConfigV1 snapshot
        |
        v
RawIngressRequestV1
        |
        v
source-only Raw preflight
        |
        v
SourceBoundRawPackageV1
        |
        v
existing Raw session / shell / collector / ledger / root chain
        |
        v
Raw source continuation retained through finalization evidence
```

The package is non-Clone and single-use. It is a Builder-side source authority
product, not a generic executor input and not a second identity kernel.

## Questions to lock in RAW-SOURCE0-PLAN0

### Q1 — source origin vocabulary

Which inputs may create a Raw package in this row?

```text
1. Owned legacy AST only:
   BareAst and ReplCompatibility are explicit origins. AST JSON is not wired
   until AST-JSON-PARITY0, and Program(JSON v0) stays outside this row.

2. All legacy-compatible inputs:
   includes AST JSON and Program(JSON v0); rejected because their lowering
   authorities and output contracts are not yet proven identical.
```

Recommended: **1**.

### Q2 — root projection

How is `VerifiedRawRootExpansionV1<'src>` retained without a self-reference?

```text
1. Source-only owned projection:
   preflight Script/App classification, Main source site, static-child sites,
   callable-Main disposition, and source locators into an owned product.

2. Borrowed expansion in the package:
   rejected because the package owns the AST and would borrow its own field.
```

Recommended: **1**. The projection contains source locators and policy facts,
not a copied final function inventory or predicted receipts.

### Q3 — identity issuance

Where may the Raw token be minted?

```text
1. One compiler-owned private `issue_raw` terminal after source preflight,
   compatibility selection, and config sealing.

2. Generic family-selected `issuer.issue(Raw)` or test factory reuse:
   rejected because callers could select a family without source authority.
```

Recommended: **1**. The existing compiler issuer remains the sole identity
authority; Raw gets a source-specific private terminal.

### Q4 — compatibility and config snapshot

What is sealed before Builder effects?

```text
root classification
duplicate-Main result
callable-Main Omitted/Required selection
source origin
imports disposition
source-file hint and module name
REPL/log settings and Core ID seed
```

Selected callable-Main failure aborts the whole Raw invocation. It never
downgrades to `NotSelected`, retries, or proceeds to inline root lowering.

### Q5 — evidence lifetime

What must survive the Raw owner chain?

```text
RawSourceContinuationV1
  -> RawCompleteInvocationV1
  -> RawFinalizationInputV1
  -> PostprocessEvidenceInputV1::Raw
  -> PostprocessEvidenceSealV1::Raw
  -> PreparedModuleExternalCommitV1
```

The continuation carries source origin, root policy, and compatibility
selection. It does not carry a predicted collector list, receipt clone, or
`MirModule` function map. Expected physical inventory remains owned by the
sealed Raw ledger and collector evidence.

## Proposed products

```rust
struct RawIngressRequestV1 {
    input: LegacyModuleLoweringInputV1,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
}

struct SourceBoundRawPackageV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawSourceContinuationV1,
}

struct OwnedRawSourceV1 {
    ast: ASTNode,
    origin: RawSourceOriginV1,
}

struct RawSourceContinuationV1 {
    origin: RawSourceOriginV1,
    root_projection: OwnedRawRootProjectionV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
    policy: ModuleInvocationPolicyV1,
}
```

These are design vocabulary only until PLAN0 closes. Constructors must be
compiler-private and source-driven; callers cannot provide a Raw family token,
root inventory, or callable-Main disposition independently.

## Executable row order after this consultation

```text
RAW-SOURCE0-PLAN0
  owned source projection and policy shape

RAW-SOURCE0-BIND0
  compiler token + source continuation + sealed Builder config

RAW-SOURCE0-LOWER0
  real Raw package into the existing collector/ledger/root chain

RAW-SOURCE0-P0/G0
  source-to-commit evidence with production consumers still zero

AST-JSON-PARITY0
  old bridge versus the Raw AST source route

PROGRAM-V0-SOURCE0-CONSULT0
  independent ProgramV0 authority and future merge policy
```

## Non-claims while stopped

```text
production outer executor = 0
production Raw token consumer = 0
public compile wrapper changes = 0
runtime JSON behavior changes = 0
MirBuilder::build_module retirement = 0
test-only Raw issuer promoted = 0
source continuation re-resolution = 0
retry/fallback = 0
```

## Acceptance for RAW-SOURCE0-CONSULT0 closeout

```text
Q1-Q5 decision text is present
Raw source owner is compiler-owned
borrowed self-referential expansion = 0
generic family-selected Raw issuer = 0
Program(JSON v0) remains outside this row
Builder config snapshot point is explicit
selected callable-Main failure law is explicit
source continuation lifetime reaches paired evidence
all new/check files below 800 lines
```

Until these decisions are locked, do not add a production Raw consumer or
modify runtime JSON semantics.
