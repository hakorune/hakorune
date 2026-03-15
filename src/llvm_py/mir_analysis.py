from typing import Dict, Any, List


def _const_string_name(ins: Dict[str, Any]):
    val = ins.get("value") or {}
    if not isinstance(val, dict):
        return None
    value = val.get("value")
    ty = val.get("type")
    if not isinstance(value, str):
        return None
    if ty == "string":
        return value
    if isinstance(ty, dict) and ty.get("box_type") == "StringBox":
        return value
    return None


def _collect_const_string_names(blocks: List[Dict[str, Any]]) -> Dict[int, str]:
    const_names: Dict[int, str] = {}
    for bb in blocks or []:
        for ins in (bb.get("instructions") or []):
            try:
                if ins.get("op") != "const":
                    continue
                dst = ins.get("dst")
                name = _const_string_name(ins)
                if isinstance(dst, int) and isinstance(name, str):
                    const_names[int(dst)] = name
            except Exception:
                continue
    return const_names


def _record_call_arity(
    arities: Dict[str, int],
    const_names: Dict[int, str],
    ins: Dict[str, Any],
) -> None:
    if ins.get("op") != "call":
        return
    func_id = ins.get("func")
    if not isinstance(func_id, int) or func_id not in const_names:
        return
    name = const_names[func_id]
    argc = len(ins.get("args") or [])
    prev = arities.get(name, 0)
    if name not in arities or argc > prev:
        arities[name] = argc


def scan_call_arities(funcs: List[Dict[str, Any]]):
    ar: Dict[str, int] = {}
    for f in funcs or []:
        blocks = f.get("blocks") or []
        const_names = _collect_const_string_names(blocks)
        for bb in blocks:
            for ins in (bb.get("instructions") or []):
                try:
                    _record_call_arity(ar, const_names, ins)
                except Exception:
                    continue
    return ar
