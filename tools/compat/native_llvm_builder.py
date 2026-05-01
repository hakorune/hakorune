#!/usr/bin/env python3
"""
Native LLVM Builder (bootstrap)

Goal: minimal Python-only emitter that generates LLVM IR text from a tiny
subset of Nyash MIR JSON and compiles it to an object via `llc`.

Supported (MVP):
- schema_version v1 or tolerant shapes
- Single function: ny_main(): i64
- Instructions: const(i64), binop(add/sub/mul/div/mod/&/|/^/<< >>), compare(==)
- ret(value)

Usage:
  python3 tools/compat/native_llvm_builder.py --in in.json --emit obj --out out.o

Notes:
- No external Python packages required. Assumes `llc` is in PATH.
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path


def _normalize_canary(v: dict) -> dict:
    # Coerce schema_version
    sv = v.get("schema_version")
    if isinstance(sv, int) and sv == 1:
        v["schema_version"] = "1.0"
    if isinstance(sv, str) and sv == "1":
        v["schema_version"] = "1.0"
    # Normalize blocks.inst -> instructions
    funs = v.get("functions")
    if isinstance(funs, list):
        for f in funs:
            blks = f.get("blocks")
            if isinstance(blks, list):
                for b in blks:
                    if "inst" in b and "instructions" not in b:
                        b["instructions"] = b.pop("inst")
                    ins = b.get("instructions")
                    if isinstance(ins, list):
                        for insn in ins:
                            if insn.get("op") == "const":
                                if "value" in insn and isinstance(insn["value"], dict) and "type" in insn["value"]:
                                    pass
                                else:
                                    ty = insn.pop("ty", "i64")
                                    val = insn.pop("value", 0)
                                    insn["value"] = {"type": ty, "value": val}
    return v


def build_ir(ny_json: dict) -> str:
    ny = _normalize_canary(ny_json)
    funs = ny.get("functions", [])
    fn = None
    for f in funs:
        if f.get("name") == "ny_main":
            fn = f
            break
    if fn is None:
        raise ValueError("ny_main not found")
    blocks = fn.get("blocks", [])
    if not blocks:
        # trivial
        return (
            "; ModuleID = \"nyash_native\"\n"
            "define i64 @ny_main(){\n  ret i64 0\n}\n"
        )

    # IR pieces
    lines = []
    # Keep IR minimal; let llc choose target triple/datalayout
    lines.append("; ModuleID = \"nyash_native\"")
    lines.append("")
    lines.append("define i64 @ny_main(){")

    # Simple vmap; const map holds immediate ints; ssa map holds emitted names
    const_map = {}
    ssa_map = {}
    is_i1 = set()

    def val_of(vid):
        if vid in ssa_map:
            return f"%{ssa_map[vid]}", (vid in is_i1)
        if vid in const_map:
            return f"i64 {const_map[vid]}", False
        # default zero
        return "i64 0", False

    tmp_idx = 0
    def fresh(name):
        nonlocal tmp_idx
        tmp_idx += 1
        return f"{name}_{tmp_idx}"

    # Emit each block with an explicit label: bb<id>:
    for b in blocks:
        bid = b.get("id")
        lines.append(f"bb{bid}:")
        ins = b.get("instructions", [])
        for insn in ins:
            op = insn.get("op")
            if op == "const":
                dst = insn.get("dst")
                v = insn.get("value", {})
                ty = v.get("type", insn.get("ty", "i64"))
                val = v.get("value", 0)
                if ty != "i64":
                    val = 0
                const_map[dst] = int(val)
            elif op == "binop":
                dst = insn.get("dst")
                opx = (insn.get("operation") or '').lower()
                aliases = {
                    'add': '+', 'plus': '+', 'sub': '-', 'minus': '-', 'mul': '*', 'times': '*',
                    'div': '/', 'mod': '%', 'rem': '%', 'band': '&', 'bitand': '&', 'bor': '|', 'bitor': '|',
                    'bxor': '^', 'xor': '^', 'shl': '<<', 'shr': '>>', 'ashr': '>>'
                }
                sym = aliases.get(opx, opx)
                lhs = insn.get("lhs"); rhs = insn.get("rhs")
                lv, _ = val_of(lhs); rv, _ = val_of(rhs)
                name = fresh("bin")
                if sym == '+':
                    lines.append(f"  %{name} = add i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '-':
                    lines.append(f"  %{name} = sub i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '*':
                    lines.append(f"  %{name} = mul i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '/':
                    lines.append(f"  %{name} = sdiv i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '%':
                    lines.append(f"  %{name} = srem i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '&':
                    lines.append(f"  %{name} = and i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '|':
                    lines.append(f"  %{name} = or i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '^':
                    lines.append(f"  %{name} = xor i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '<<':
                    lines.append(f"  %{name} = shl i64 {lv.split()[-1]}, {rv.split()[-1]}")
                elif sym == '>>':
                    lines.append(f"  %{name} = ashr i64 {lv.split()[-1]}, {rv.split()[-1]}")
                else:
                    lines.append(f"  %{name} = add i64 0, 0")
                ssa_map[dst] = name
            elif op == "compare":
                dst = insn.get("dst")
                opx = insn.get("operation") or insn.get("cmp") or '=='
                lhs = insn.get("lhs"); rhs = insn.get("rhs")
                lv, _ = val_of(lhs); rv, _ = val_of(rhs)
                name = fresh("cmp")
                # Support eq/lt minimal
                if opx in ('==', 'Eq'): pred = 'eq'
                elif opx in ('<', 'Lt'): pred = 'slt'
                else: pred = 'ne'
                lines.append(f"  %{name} = icmp {pred} i64 {lv.split()[-1]}, {rv.split()[-1]}")
                ssa_map[dst] = name
                is_i1.add(dst)
            elif op == "branch":
                # Conditional branch: {cond, then, else}
                cond = insn.get("cond")
                then_id = insn.get("then")
                else_id = insn.get("else")
                cv, ci1 = val_of(cond)
                if ci1:
                    cond_name = cv if cv.startswith('%') else f"%{cv}"
                    lines.append(f"  br i1 {cond_name}, label %bb{then_id}, label %bb{else_id}")
                else:
                    # Build i1 from i64 via icmp ne 0
                    name = fresh("cnd")
                    lines.append(f"  %{name} = icmp ne i64 {cv.split()[-1]}, 0")
                    lines.append(f"  br i1 %{name}, label %bb{then_id}, label %bb{else_id}")
            elif op == "jump":
                target = insn.get("target")
                lines.append(f"  br label %bb{target}")
            elif op == "ret":
                vid = insn.get("value")
                if vid in is_i1:
                    vname = ssa_map.get(vid)
                    z = fresh("zext")
                    lines.append(f"  %{z} = zext i1 %{vname} to i64")
                    lines.append(f"  ret i64 %{z}")
                elif vid in ssa_map:
                    lines.append(f"  ret i64 %{ssa_map[vid]}")
                elif vid in const_map:
                    lines.append(f"  ret i64 {const_map[vid]}")
                else:
                    lines.append("  ret i64 0")
    lines.append("}")
    return "\n".join(lines) + "\n"


def compile_ir_to_obj(ir_text: str, out_obj: Path) -> None:
    tmp = Path("/tmp/native_ir_{}.ll".format(os.getpid()))
    tmp.write_text(ir_text)
    try:
        subprocess.check_call(["llc", "-filetype=obj", "-o", str(out_obj), str(tmp)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    finally:
        try:
            tmp.unlink()
        except Exception:
            pass


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--in", dest="infile", required=True)
    ap.add_argument("--emit", dest="emit", default="obj")
    ap.add_argument("--out", dest="out", required=True)
    args = ap.parse_args()

    with open(args.infile, 'r') as f:
        ny = json.load(f)

    ir = build_ir(ny)
    if os.environ.get('NYASH_LLVM_NATIVE_TRACE') in ('1','true','on','YES','yes','True'):
        print(ir, file=sys.stderr)
    if args.emit == 'll':
        Path(args.out).write_text(ir)
        print(f"[native] ll written: {args.out}")
        return
    if args.emit == 'obj':
        compile_ir_to_obj(ir, Path(args.out))
        print(f"[native] obj written: {args.out}")
        return
    print("error: unsupported emit kind", file=sys.stderr)
    sys.exit(2)


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"[native] error: {e}", file=sys.stderr)
        sys.exit(1)
