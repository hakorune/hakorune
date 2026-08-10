# Callable parameter demand

This module owns one source-bound semantic relation:

```text
borrowed VerifiedResolvedCallableSemanticBatchV1
  -> complete VerifiedCallableParameterDemandCatalogV1
```

The first cohort maps every parser-sealed `Ordinary` parameter to
`HomeDemandV1::Handle`. Zero-parameter declarations remain present as empty
declaration rows, so downstream consumers cannot confuse an empty parameter
list with a missing declaration.

The final catalog borrows the sole semantic batch and owns only its projected
demand rows. It does not retain, clone, issue, or expose a resolver forest.
There is no `resolved_forest()`, `into_parts`, row selector, name lookup, or
caller-supplied `BindingRef`.

## Boundary

The semantic batch already owns declaration coverage, root mode, owner, and
source projection. This module verifies parameter ordinal, exact batch-owned
binding, binding kind, source origin, diagnostic name, transfer syntax, and
duplicate-free `BindingRef` identity before issuing any demand.

It does not own or infer:

- `Take`, `Home`, `SharedHome`, or result Home relations;
- receiver ABI or instance callable contracts;
- Dynamic carrier classification or lifecycle;
- Recipe, JoinSig, CFG, SSA, PHI, MIR, or physical ABI;
- production route selection, retry, or fallback.

Any future transfer syntax must be admitted by its source-backed semantic
issuer. It must not be added here as another spelling-to-demand match arm.
