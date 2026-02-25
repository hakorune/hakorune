"""Phase 275 P0: PHI型取得のSSOT

PHI dst_type 取得ロジックを統一管理。
- 優先度1: MIR JSON instruction の dst_type (最新)
- 優先度2: resolver.value_types (型推論後)
"""

def get_phi_dst_type(builder, dst_vid, inst=None):
    """PHI destination type を取得

    Args:
        builder: LLVM builder instance
        dst_vid: Destination ValueId (int)
        inst: MIR JSON instruction dict (optional, priority source)

    Returns:
        str | None: 'f64', 'i64', 'void', etc. (MIR type string)

    Example:
        >>> dst_type = get_phi_dst_type(builder, 36, inst)
        >>> if dst_type == 'f64':
        >>>     phi_type = ir.DoubleType()
    """
    # Priority 1: instruction JSON (最新、直接指定)
    if inst is not None:
        dst_type = inst.get("dst_type")
        if dst_type is not None:
            return dst_type

    # Priority 2: resolver.value_types (型推論結果)
    try:
        if hasattr(builder, 'resolver') and hasattr(builder.resolver, 'value_types'):
            vt = builder.resolver.value_types.get(int(dst_vid))
            # f64 の場合
            if vt == 'f64' or (isinstance(vt, dict) and vt.get('type') == 'f64'):
                return 'f64'
            # i64 の場合（default）
            if vt == 'i64' or vt == 'Integer':
                return 'i64'
            # void の場合
            if vt == 'void' or vt == 'Void':
                return 'void'
            # Box型の場合
            if isinstance(vt, dict) and vt.get('kind') == 'handle':
                return 'handle'
    except Exception:
        pass

    return None


def dst_type_to_llvm_type(dst_type, builder):
    """MIR dst_type を LLVM IR type に変換

    Args:
        dst_type: str | None (from get_phi_dst_type)
        builder: LLVM builder (for builder.i64 access)

    Returns:
        ir.Type: LLVM IR type (DoubleType, IntType, etc.)

    Example:
        >>> dst_type = get_phi_dst_type(builder, 36, inst)
        >>> llvm_type = dst_type_to_llvm_type(dst_type, builder)
        >>> phi = b.phi(llvm_type, name=f"phi_{dst_vid}")
    """
    import llvmlite.ir as ir

    if dst_type == 'f64' or dst_type == 'double':
        return ir.DoubleType()
    # デフォルトは i64
    return builder.i64
