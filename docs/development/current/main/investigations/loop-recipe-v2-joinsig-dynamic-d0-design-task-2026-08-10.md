# LOOP-RECIPE-V2-JOINSIG-DYNAMIC-D0

Status: design stop; implementation 0
Date: 2026-08-10
Depends on: `LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0` closed

## Goal

Define the smallest profile-neutral V2 JoinSig authority needed by the
complete unchanged Dynamic Loop Recipe without letting physical layout,
profile code, Completion, or Fault become a second logical control owner.

## Premise

The current verified product already owns complete source/Recipe/Dynamic-call
relations and a borrowed V10/ch/I7 local relation. It does not own JoinSig,
continuation, Fault transfer, Home Flow, cleanup, Tail, or Completion
consumption.

## Questions for independent review

1. Which existing V1 JoinSig algebra can be reused unchanged for the V2 root
   predicate, nested If, inner Return, fallthrough, backedge, and After?
2. Does V2 need a new neutral transfer row, or only a V2 adapter into the
   existing JoinSig owner?
3. How is the inner Return authorized while the outer Return remains Callable
   Tail/Completion rather than a Loop exit?
4. Where must Dynamic `Fault` branch away so it stays outside Recipe values,
   Recipe exits, and ordinary JoinSig edge inference?
5. Which exact carrier payload crosses Header/Body/After, and how is the
   iteration-local V10 excluded from backedge payloads by construction?
6. What is the atomic co-seal boundary tying JoinSig to this exact verified
   Recipe without accepting caller-supplied Continuation?

## Required output

- owner/non-owner table;
- exact edge/port/payload golden for the unchanged Recipe;
- typed failure matrix;
- decision on reuse versus minimal V2 vocabulary;
- ordered D0/I0 task ladder;
- same-slice reference/README/test update list;
- explicit removal conditions for any adapter.

## Hard stops

```text
no Builder / MIR / CFG / PHI
no physical layout or block allocation
no Home install/cleanup
no runtime-tag lifetime inference
no Fault-as-Recipe-value or Fault-as-ordinary-Loop-exit
no Tail/Completion/return ABI absorption
no profile-specific JoinSig owner
no caller-supplied Continuation re-pairing
no retry/fallback/provider execution
no source rewrite or fixture narrowing
no implementation before the Decision is accepted
```

## Acceptance for D0

The Decision is complete only when one authority owns every logical transfer,
all payloads are derivable from the verified Recipe/JoinSig relation, Fault and
Callable Tail remain explicit external siblings, and a bounded I0 can be
implemented without introducing a second control-flow truth.
