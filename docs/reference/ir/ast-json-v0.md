# AST JSON v0 (Macro Expansion)

Status: Draft. This document specifies a minimal JSON schema for representing Nyash AST to enable macro expansion by external processes (e.g., PyVM-based MacroBox).

Decision note (current):
- Rune declaration metadata carriage is declaration-local on AST JSON v0 and direct MIR; this includes base Rune v0 families plus Rune v1 optimization families (`Hint` / `Contract` / `IntrinsicCandidate`) after normalization. Program(JSON v0) is a retire target and is not widened for Rune metadata.
- SSOT:
  - `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
  - `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`

Top-level
- Object with `kind` discriminator.
- Nested nodes referenced inline; no IDs.
- Span is omitted in v0 (unknown). Future versions may include `span` with file/line/col.

Kinds (subset for Phase 2+)
- Program: { kind: "Program", statements: [Node] }
- Loop: { kind: "Loop", condition: Node, body: [Node] }
- Print: { kind: "Print", expression: Node }
- Return: { kind: "Return", value: Node|null }
- Break: { kind: "Break" }
- Continue: { kind: "Continue" }
- Assignment: { kind: "Assignment", target: Node, value: Node }
- If: { kind: "If", condition: Node, then: [Node], else: [Node]|null }
- FunctionDeclaration: { kind: "FunctionDeclaration", name: string, params: [string], body: [Node], static: bool, override: bool, attrs?: { runes: [RuneAttr] } }
- Variable: { kind: "Variable", name: string }
- Literal: { kind: "Literal", value: LiteralValue }
- BinaryOp: { kind: "BinaryOp", op: string, left: Node, right: Node }
- UnaryOp: { kind: "UnaryOp", op: string, operand: Node }
- MethodCall: { kind: "MethodCall", object: Node, method: string, arguments: [Node] }
- FunctionCall: { kind: "FunctionCall", name: string, arguments: [Node] }
- Array: { kind: "Array", elements: [Node] }
- Map: { kind: "Map", entries: [{k: string, v: Node}] }
- Local: { kind: "Local", variables: [string], inits: [Node|null] }

LiteralValue
- { type: "string", value: string }
- { type: "int", value: integer }
- { type: "float", value: number }
- { type: "bool", value: boolean }
- { type: "null" }
- { type: "void" }

Unary operators
- "-" for Minus, "not" for Not

Binary operators
- "+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "==", "!=", "<", ">", "<=", ">=", "&&", "||"

Notes
- The schema is intentionally minimal; it covers nodes needed for Phase 2 samples.
- Future: add `span`, typed annotations as needed.
- Current: declaration-bearing nodes may carry `attrs.runes` in AST JSON v0.
- Type checks (is/as) mapping
  - AST JSON v0 does not introduce a dedicated TypeOp node. Instead, write MethodCall with
    method "is" or "as" and a single string literal type argument:
    {"kind":"MethodCall","object":<expr>,"method":"is","arguments":[{"kind":"Literal","value":{"type":"string","value":"Integer"}}]}
  - Lowering maps this to MIR::TypeOp(Check/ Cast) with the target type resolved by name.

## Declaration `attrs.runes`

Declaration-bearing nodes may carry:

```json
{
  "attrs": {
    "runes": [
      { "name": "Public", "args": [] },
      { "name": "Ownership", "args": ["owned"] }
    ]
  }
}
```

Carrier rules:

- both Rust parser and `.hako` parser must emit the same declaration-local `attrs.runes` shape
- `attrs.runes` is declaration-local only in v0
- AST JSON v0 is the canonical metadata carrier for declaration metadata
- Program(JSON v0) is a retire target and must not be widened for Rune
- Rust direct MIR JSON mirrors declaration-local attrs on function-bearing nodes
- current `.hako` source-route keep preserves selected-entry attrs via a transitional transport shim; it must not widen Program(JSON v0) root/body attrs

## Example

Source:
```
print("x")
```

Expanded AST JSON v0:
```
{"kind":"Program","statements":[{"kind":"Print","expression":{"kind":"Literal","value":{"type":"string","value":"x"}}}]}
```
