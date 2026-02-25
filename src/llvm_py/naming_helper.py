"""
MIR NamingHelper — Python mirror of Rust src/mir/naming.rs

Responsibility:
- Encode/decode static box methods to MIR function names.
- Centralize naming rules (e.g., Main._nop/0) for Builder/PyVM consistency.
- Minimal: handle `main.*` → `Main.*` cases safely.

Non-responsibility:
- Dynamic dispatch or BoxFactory name resolution.
- Entry point selection policy (NYASH_ENTRY).
"""


def encode_static_method(box_name: str, method: str, arity: int) -> str:
    """
    Encode a static box method into a MIR function name: `BoxName.method/arity`.

    Args:
        box_name: Raw box name (e.g., "main", "Main", "Calculator")
        method: Method name (e.g., "main", "_nop", "add")
        arity: Number of parameters (e.g., 0, 1, 3)

    Returns:
        Canonical MIR function name (e.g., "Main.main/1", "Main._nop/0")

    Examples:
        >>> encode_static_method("main", "main", 1)
        'Main.main/1'
        >>> encode_static_method("Main", "_nop", 0)
        'Main._nop/0'
        >>> encode_static_method("Calculator", "add", 2)
        'Calculator.add/2'
    """
    return f"{canonical_box_name(box_name)}.{method}/{arity}"


def canonical_box_name(raw: str) -> str:
    """
    Canonicalize a static box name for MIR-level usage.

    Current rules:
    - "main" → "Main" (minimal correction)
    - Others: return as-is (avoid wide-scope spec changes)

    Args:
        raw: Raw box name (e.g., "main", "Main", "Calculator")

    Returns:
        Canonical box name (e.g., "Main", "Calculator")

    Examples:
        >>> canonical_box_name("main")
        'Main'
        >>> canonical_box_name("Main")
        'Main'
        >>> canonical_box_name("Calculator")
        'Calculator'
    """
    return "Main" if raw == "main" else raw


def normalize_static_global_name(func_name: str) -> str:
    """
    If `func_name` looks like a static box method like `main._nop/0`,
    normalize the box part (`main` → `Main`) and return canonical form.

    Args:
        func_name: MIR function name (e.g., "main._nop/0", "Main.main/1", "print")

    Returns:
        Normalized function name (e.g., "Main._nop/0", "Main.main/1", "print")

    Examples:
        >>> normalize_static_global_name("main._nop/0")
        'Main._nop/0'
        >>> normalize_static_global_name("Main._nop/0")
        'Main._nop/0'
        >>> normalize_static_global_name("print")
        'print'
    """
    if '.' in func_name:
        box_part, rest = func_name.split('.', 1)
        # rest contains "method/arity"
        canon = canonical_box_name(box_part)
        if canon != box_part:
            return f"{canon}.{rest}"
    return func_name
