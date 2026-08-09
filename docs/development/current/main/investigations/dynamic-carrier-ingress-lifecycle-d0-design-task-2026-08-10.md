# DYNAMIC-CARRIER-INGRESS-LIFECYCLE-D0

Status: current design stop
Date: 2026-08-10

## Question

Seal the exact initial lifecycle disposition of the root Dynamic carrier:

```text
plain parameter pos
  -> PreludeInitializerPos
  -> local i
  -> Recipe V1
  -> carrier C0 / L0 / B0 / Dynamic
```

The Decision must choose the canonical resolver/callable authority proving
whether initial B0 is:

```text
BorrowedIngressNoEnd
or
OwnedCarrier(EndExactlyOnceUnlessForwarded)
```

The language target says a plain parameter has Handle demand and only `take`
may consume a Home. The implementation must determine how that verified
callable parameter demand projects into the separate opaque Dynamic carrier
lifecycle without duplicating Home ABI or inferring from Recipe `Dynamic`.

## Required authority census

- exact normal-callable declaration and parameter ordinal;
- absence/presence of accepted transfer syntax;
- resolver-issued parameter demand or an explicitly identified missing issuer;
- exact `pos` BindingRef and prelude initializer source relation;
- exact V1 input and C0/L0/B0 carrier relation;
- same owner/frame/source authority;
- distinction between borrowed ingress and later owned V17 replacements.

If the repository has no canonical normal-callable parameter-demand issuer,
the result is `NoSafeSlice`; do not create a profile-local empty Home receipt.

## Acceptance

- one non-Clone ingress product, issued from complete source authority;
- arbitrary key/site/demand constructors absent;
- borrowed ingress produces no carrier End when displaced;
- owned ingress, if ever admitted by a later explicit contract, carries one
  exact lifecycle obligation;
- foreign owner/parameter/site/input/carrier and duplicate rows reject;
- runtime tags, selector/provider names, `MirType`, ValueId, Recipe class,
  `ReleaseStrong`, and body-name inference are forbidden;
- no rebind, cleanup execution, CFG/MIR/PHI, Completion, retry, or fallback.

## Next row

Only an accepted ingress Decision may open
`DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0`. Rebind I0 remains closed until that
product exists and can be consumed whole.
