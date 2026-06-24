---
Status: Landed
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter acceptance
---

# 296x-1641: MirBuilder Loop Value Enum Constructor Acceptance

## Decision

Accept known enum constructor values inside normalized loop bodies through the
generic CoreEffectPlan path.

This closes the first blocker exposed by the RegionObserver
`variable_map().iter()` read-fold probe:

```text
ty == MirType::Box("ArrayBox")
```

The fix is not RegionObserver-specific and does not special-case `MirType`,
`ArrayBox`, or slot classification.

## Implemented Shape

```text
ASTNode::FromCall(parent, method, args)
  -> resolve known enum variant from compilation context
  -> CoreEffectPlan::VariantMake
  -> MirInstruction::VariantMake
```

Limits:

```text
known enum constructors only
unit or single-payload variants only
compat payload box = denied
multi-payload variants = denied
unknown from-call = denied
```

## Validation

```text
cargo check -q
cargo build --release --bin hakorune
./target/release/hakorune --emit-mir-json /tmp/loop_value_enum_constructor.mir.json apps/tests/phase296x_loop_value_enum_constructor_min.hako
./target/release/hakorune --emit-exe /tmp/loop_value_enum_constructor_exe apps/tests/phase296x_loop_value_enum_constructor_min.hako
/tmp/loop_value_enum_constructor_exe
```

The fixture proves loop-body enum constructor value acceptance and EXE
generation. It does not claim full enum structural equality semantics for the
RegionObserver output; that remains gated by the ordered-map source-order
blocker below.

The RegionObserver probe then reaches MIR/EXE generation but stops at the next
semantic blocker:

```text
ORDERED-MAP-SOURCE-ORDERED-STRING-COMPARE-001
```

Reason: `OrderedMapBox` currently does not prove Rust `BTreeMap<String>` source
ordering for the `b, a, args` insertion case. The converter must not change the
expected order to insertion order just to pass the smoke.

## Next Blocker

```text
ORDERED-MAP-SOURCE-ORDERED-STRING-COMPARE-001
```

Required next step:

```text
Make OrderedMapBox String-key ordering match Rust BTreeMap<String> for selected
ASCII names, or deny SourceOrdered read-fold conversion until that ordering is
available.
```
