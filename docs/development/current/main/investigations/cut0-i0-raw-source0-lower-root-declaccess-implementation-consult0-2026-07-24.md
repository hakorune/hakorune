# DECLACCESS0 implementation consultation

Status: **Design stop — implementation boundary requires a decision**  
Date: 2026-07-24  
Parent decision: **DECLACCESS-prime-r1**

## Why this stop exists

The decision lock requires the exact source manifest to be sealed before the
physical owner opens, and requires BODY0 to receive owned literal/source-site
payload rather than an AST lookup capability. The current disconnected
owners do not yet satisfy either condition:

```text
EligibleSourceBoundRawRootPackageV1::open_physical
  -> RawRootPhysicalStateV1::open
  -> ModuleBuilderInvocationSessionV1::open_for_token
  -> RawRootPhysicalCoreV1
```

The physical shell/session are therefore opened before a DECLACCESS manifest
exists. Separately, `RawRootEnvironmentPlanV1` stores only summary facts and
the current ScalarControl0 witness stores `Literal` without its value or
source site. Adding `declare_environment(self)` on top of these owners would
either re-scan the AST or weaken the locked source-authority claim.

## Questions to close

### Q1 — physical-open ordering

Choose one structural boundary:

1. **Refactor PHYSICAL0** so an eligible package first consumes into a
   source-manifest owner, and only a manifest-bearing owner may open the
   Builder session/shell/collector.
2. **Split manifest sealing from physical open** but explicitly amend the
   DECLACCESS decision to allow a pre-manifest physical shell/session, with a
   mutation-free guarantee and no environment installation before the
   manifest.

Option 1 preserves the current decision literally. Option 2 is smaller but
requires changing the claim and the acceptance guard.

### Q2 — exact ScalarControl0 payload

Choose the single source of the located body payload:

1. Extend PLAN0/classification once to own literal values, variable names,
   operators, recursive children, source path, and span; DECLACCESS consumes
   that product without reclassification.
2. Add a manifest-only source projection that performs the classification
   after eligibility but before physical installation, and record that this
   is the one permitted classifier invocation.

The current summary-only plan cannot satisfy the exact manifest requirement
without one of these choices.

### Q3 — Builder/shell co-install seam

Choose where the prepared aggregate is consumed:

1. Add a Builder sibling terminal that privately owns the session and
   `RawRootPhysicalStateV1`, validates both destination lanes, and returns a
   named declared owner.
2. Move the co-install terminal into compiler code and widen the current
   `builder_mut()`/physical parts visibility.

Option 1 preserves the no-tuple/no-raw-parts guard and is the recommended
choice.

## Evidence already green

The behavior-neutral PLAN0 accessors landed in `c3f286fdfe`; cargo check and
the existing `raw_root` test family remain green. No production consumer was
added. This consultation is not a failure of those checks; it records that
the next implementation slice needs a source/physical ownership decision.

## Non-claims while stopped

```text
exact manifest producer       = 0
declare_environment consumer  = 0
Builder/shell installation    = 0
BODY0 consumer                = 0
production consumer           = 0
```

Do not add a fallback, AST re-read, `current_module` lookup, compiler-side
physical tuple, or partial commit while this consultation is open.
