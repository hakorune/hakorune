# LOOP-CALLER-ZERO-PARITY-G0-I1-D0

Status: `Design stop after I0 closeout`
Date: `2026-08-08`
Parent: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-design-2026-08-08.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Sole design question

Define the smallest fresh-session canary that consumes the closed
`PreparedGenericG0LoopIngressV1` without creating a G0 physicalizer, a second
CFG/SSA/PHI owner, or a production route.

```text
I0 prepared ingress
  -> exact fresh CanonicalFunctionLoweringSessionV1
  -> G0 parameter-entry receipt
  -> common topology/operation services
  -> distinct G0 post-loop read + I64 tail
  -> existing completion/DraftSeal terminal
  -> discard or fresh-session repeat
```

## Accepted boundary

1. The I0 composite remains the only semantic input. I1 may borrow its exact
   resolver input, fifteen-row prepared program, G0 tail capability, and
   target; it may not re-resolve, inspect AST, or derive capabilities by name.
2. Session opening is a fresh unpublished transaction. Owner, function origin,
   frame, and caller fingerprint must match before any operation instruction or
   physical publication effect.
3. G0 entry materialization is a profile wrapper responsibility: the two exact
   parameter `BindingRef` entries from I0 are installed through canonical
   identity/Prelude services. The common physicalizer receives only the ready
   entry receipt and common prepared program.
4. Common topology and operation services may be reused only where their
   existing contracts admit the G0 Recipe rows. Unsupported nested/derived
   rows reject as typed `NoSafeSlice` before their first leaf effect; no G0
   shape relabel, partial success, or fallback is allowed.
5. The post-loop read and return use
   `VerifiedGenericG0TailCapabilityV1`. It is not a Callable Tail and does not
   move `L0.After/b1` into a generic return slot by name.
6. Successful completion must pass the existing canonical session finish and
   DraftSeal owners. A late failure discards the entire unpublished function;
   the same semantic request may be retried only by opening a fresh session in
   the test harness.

## Required design receipts before implementation

```text
G0 entry receipt: exact owner + parameter BindingRef + I64 ABI
topology receipt: root/child logical role -> canonical physical block
operation admission: all fifteen rows admitted or typed NoSafeSlice
tail receipt: post-loop source read + result ABI + owner/frame
finish receipt: Completion/DraftSeal owner and discard terminal
```

The common physicalizer must not receive profile identity, G0 Tail,
Completion, Return ABI, DraftSeal, module collector, or legacy route handles.
No new semantic owner is introduced; the existing session, identity, PHI,
completion, and DraftSeal owners remain sole authorities.

## Explicit non-claims

This design does not activate Generic G0, select a production caller, remove
the old scheduler/retry/fallback, prove backend parity, or claim M8/M9/19-row
coverage. It also does not require a single-operation extraction API. I1
implementation may open only after this card's receipts and reject seams are
accepted and the implementation/reference closeout is planned together.
