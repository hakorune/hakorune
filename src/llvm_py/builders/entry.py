from typing import Optional
import os
import sys
# NamingBox SSOT: Add parent directory to path for naming_helper import
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))
from naming_helper import encode_static_method

def ensure_ny_main(builder) -> None:
    """Ensure ny_main wrapper exists by delegating to Main.main/1 or main().
    Phase 285LLVM-1.1: Register user box declarations before calling main.
    Modifies builder.module in place; no return value.
    """
    has_ny_main = any(f.name == 'ny_main' for f in builder.module.functions)
    fn_main_box = None
    fn_main_plain = None
    # NamingBox SSOT: Use encode_static_method for name comparison
    main_box_name = encode_static_method("Main", "main", 1)
    for f in builder.module.functions:
        if f.name == main_box_name:
            fn_main_box = f
        elif f.name == 'main':
            fn_main_plain = f
    target_fn = fn_main_box or fn_main_plain
    if target_fn is None or has_ny_main:
        return

    # Hide the target to avoid symbol conflicts
    try:
        target_fn.linkage = 'private'
    except Exception:
        pass
    # i32 ny_main() { return (i32) Main.main(args) | main(); }
    from llvmlite import ir
    ny_main_ty = ir.FunctionType(builder.i64, [])
    ny_main = ir.Function(builder.module, ny_main_ty, name='ny_main')
    entry = ny_main.append_basic_block('entry')
    b = ir.IRBuilder(entry)

    # Phase 285LLVM-1.1: Register user box declarations before calling main
    user_box_decls = getattr(builder, 'user_box_decls', [])
    if user_box_decls:
        _emit_user_box_registration(b, builder.module, user_box_decls)
    def _build_main_args_handle():
        # Build args handle for entry functions that take one parameter.
        i64 = builder.i64
        i8 = builder.i8
        i8p = builder.i8p
        use_argv = os.environ.get('NYASH_EXE_ARGV') == '1' or os.environ.get('HAKO_EXE_ARGV') == '1'
        if use_argv:
            # Prefer runtime provider: nyash.env.argv_get() -> i64 handle
            callee = None
            for f in builder.module.functions:
                if f.name == 'nyash.env.argv_get':
                    callee = f
                    break
            if callee is None:
                callee = ir.Function(builder.module, ir.FunctionType(i64, []), name='nyash.env.argv_get')
            args_handle = b.call(callee, [], name='ny_main_args')
        else:
            # Default empty ArrayBox via nyash.env.box.new_i64x("ArrayBox")
            callee = None
            for f in builder.module.functions:
                if f.name == 'nyash.env.box.new_i64x':
                    callee = f
                    break
            if callee is None:
                callee = ir.Function(builder.module, ir.FunctionType(i64, [i8p, i64, i64, i64, i64, i64]), name='nyash.env.box.new_i64x')
            # Create "ArrayBox\0" global
            sbytes = b"ArrayBox\0"
            arr_ty = ir.ArrayType(i8, len(sbytes))
            g = ir.GlobalVariable(builder.module, arr_ty, name='.ny_main_arraybox')
            g.linkage = 'private'
            g.global_constant = True
            g.initializer = ir.Constant(arr_ty, bytearray(sbytes))
            c0 = ir.Constant(builder.i32, 0)
            ptr = b.gep(g, [c0, c0], inbounds=True)
            zero = ir.Constant(i64, 0)
            args_handle = b.call(callee, [ptr, zero, zero, zero, zero, zero], name='ny_main_args')
        return args_handle

    if fn_main_box is not None:
        args_handle = _build_main_args_handle()
        rv = b.call(fn_main_box, [args_handle], name='call_Main_main_1')
    else:
        # Plain main() fallback
        if len(fn_main_plain.args) == 0:
            rv = b.call(fn_main_plain, [], name='call_user_main')
        elif len(fn_main_plain.args) == 1:
            args_handle = _build_main_args_handle()
            rv = b.call(fn_main_plain, [args_handle], name='call_user_main_1')
        else:
            rv = ir.Constant(builder.i64, 0)
    if hasattr(rv, 'type') and isinstance(rv.type, ir.IntType) and rv.type.width != 32:
        rv64 = b.trunc(rv, builder.i64) if rv.type.width > 64 else b.zext(rv, builder.i64)
        b.ret(rv64)
    elif hasattr(rv, 'type') and isinstance(rv.type, ir.IntType) and rv.type.width == 64:
        b.ret(rv)
    else:
        b.ret(ir.Constant(builder.i64, 0))


def _emit_user_box_registration(b, module, user_box_decls):
    """Emit calls to nyrt_register_user_box_decl() for each user box declaration.

    Phase 285LLVM-1.1: Register user-defined boxes before main execution.

    Args:
        b: IRBuilder instance (positioned at ny_main entry block)
        module: LLVM module
        user_box_decls: List[dict] with format [{"name": "SomeBox", "fields": ["x"]}, ...]
    """
    from llvmlite import ir
    import json

    i32 = ir.IntType(32)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()

    # Declare nyrt_register_user_box_decl if not exists
    reg_func = None
    for f in module.functions:
        if f.name == "nyrt_register_user_box_decl":
            reg_func = f
            break

    if not reg_func:
        # i32 nyrt_register_user_box_decl(i8* name, i8* fields_json)
        reg_func_type = ir.FunctionType(i32, [i8p, i8p])
        reg_func = ir.Function(module, reg_func_type, name="nyrt_register_user_box_decl")

    # Emit registration calls for each box declaration
    for box_decl in user_box_decls:
        name = box_decl.get("name", "")
        fields = box_decl.get("fields", [])

        if not name:
            continue

        # Create global string constant for name
        name_bytes = (name + "\0").encode('utf-8')
        name_arr_ty = ir.ArrayType(i8, len(name_bytes))
        name_global = f".user_box_name_{name}"

        # Check if global already exists
        existing_global = None
        for g in module.global_values:
            if g.name == name_global:
                existing_global = g
                break

        if existing_global is None:
            g_name = ir.GlobalVariable(module, name_arr_ty, name=name_global)
            g_name.linkage = 'private'
            g_name.global_constant = True
            g_name.initializer = ir.Constant(name_arr_ty, bytearray(name_bytes))
        else:
            g_name = existing_global

        # Create global string constant for fields JSON
        fields_json = json.dumps(fields)
        fields_bytes = (fields_json + "\0").encode('utf-8')
        fields_arr_ty = ir.ArrayType(i8, len(fields_bytes))
        fields_global = f".user_box_fields_{name}"

        # Check if global already exists
        existing_fields_global = None
        for g in module.global_values:
            if g.name == fields_global:
                existing_fields_global = g
                break

        if existing_fields_global is None:
            g_fields = ir.GlobalVariable(module, fields_arr_ty, name=fields_global)
            g_fields.linkage = 'private'
            g_fields.global_constant = True
            g_fields.initializer = ir.Constant(fields_arr_ty, bytearray(fields_bytes))
        else:
            g_fields = existing_fields_global

        # Get pointers to strings
        c0 = ir.Constant(ir.IntType(32), 0)
        name_ptr = b.gep(g_name, [c0, c0], inbounds=True)
        fields_ptr = b.gep(g_fields, [c0, c0], inbounds=True)

        # Call nyrt_register_user_box_decl(name, fields_json)
        b.call(reg_func, [name_ptr, fields_ptr])
