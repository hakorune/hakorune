from typing import Dict, Any, List


def scan_call_arities(funcs: List[Dict[str, Any]]):
    ar: Dict[str, int] = {}
    for f in funcs or []:
        const_names: Dict[int, str] = {}
        for bb in (f.get("blocks") or []):
            for ins in (bb.get("instructions") or []):
                try:
                    op = ins.get("op")
                    if op == "const":
                        dst = ins.get("dst")
                        val = ins.get("value") or {}
                        name = None
                        if isinstance(val, dict):
                            v = val.get("value")
                            t = val.get("type")
                            if isinstance(v, str) and (
                                t == "string"
                                or (isinstance(t, dict) and t.get("box_type") == "StringBox")
                            ):
                                name = v
                        if isinstance(dst, int) and isinstance(name, str):
                            const_names[int(dst)] = name
                    elif op == "call":
                        func_id = ins.get("func")
                        if isinstance(func_id, int) and func_id in const_names:
                            nm = const_names[func_id]
                            argc = len(ins.get("args") or [])
                            prev = ar.get(nm, 0)
                            if argc > prev:
                                ar[nm] = argc
                except Exception:
                    continue
    return ar
