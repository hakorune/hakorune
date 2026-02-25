"""Print Argument Marshaller Box - print引数の型変換SSoT

Phase 97 Refactoring: printの引数marshal処理を一箇所に集約。
"""

from typing import Any, Dict, Optional
from .arg_resolver import resolve_call_arg


class PrintArgMarshallerBox:
    """print引数のmarshall処理Box

    責務:
    - print引数の型判定（stringish / non-stringish）
    - non-stringishの場合: box.from_i64 → to_i8p_h
    - stringishの場合: そのまま渡す

    契約:
    - 入力: 引数ValueId、型情報（stringish判定）
    - 出力: marshal後のi8*ポインタ
    - 前提条件: ValueIdが解決可能

    Fail-Fast:
    - ValueIdが未定義 → KeyError
    - 型情報が不正 → TypeError

    重要な境界:
    「printはstringish以外を box.from_i64 してから to_i8p_h」
    これはLLVM FFI境界の契約であり、変更時は慎重に。
    """

    @staticmethod
    def marshal(
        arg_id: Any,
        type_info: dict,
        builder,
        resolver,
        module,
        *,
        vmap: Optional[Dict[int, Any]] = None,
        preds: Optional[Dict[int, list]] = None,
        block_end_values: Optional[Dict[int, Dict[int, Any]]] = None,
        bb_map: Optional[Dict[int, Any]] = None,
    ) -> Any:
        """print引数をi8*にmarshal

        Args:
            arg_id: 引数ValueId
            type_info: 型情報（"stringish": bool）
            builder: LLVM builder
            resolver: Value resolver
            module: LLVM module
            vmap: Optional value map (strict resolver local-first)
            preds: Optional block predecessor map
            block_end_values: Optional block_end_values map
            bb_map: Optional basic-block map

        Returns:
            i8*ポインタ（LLVM Value）

        Raises:
            KeyError: ValueIdが未定義
            TypeError: 型情報が不正
        """
        if "stringish" not in type_info:
            raise TypeError("[PrintArgMarshallerBox] type_info must contain 'stringish'")

        is_stringish = type_info["stringish"]

        # Resolve argument value
        local_vmap = vmap if isinstance(vmap, dict) else {}
        local_preds = preds if isinstance(preds, dict) else None
        local_bev = block_end_values if isinstance(block_end_values, dict) else None
        arg_val = resolve_call_arg(
            arg_id,
            builder,
            local_vmap,
            resolver,
            None,
            preds=local_preds,
            block_end_values=local_bev,
            bb_map=bb_map,
            hot_scope="call",
        )

        if arg_val is None:
            raise KeyError(f"[PrintArgMarshallerBox] Cannot resolve ValueId: {arg_id}")

        if is_stringish:
            # stringishはそのまま渡す（既にi8*として扱える）
            # to_i8p_h を経由して変換
            import llvmlite.ir as ir
            i8p = ir.IntType(8).as_pointer()
            to_i8p = None
            for f in module.functions:
                if f.name == "nyash.string.to_i8p_h":
                    to_i8p = f
                    break
            if not to_i8p:
                to_i8p_type = ir.FunctionType(i8p, [ir.IntType(64)])
                to_i8p = ir.Function(module, to_i8p_type, name="nyash.string.to_i8p_h")

            return builder.call(to_i8p, [arg_val])
        else:
            # non-stringish: box.from_i64 → to_i8p_h
            import llvmlite.ir as ir
            i8p = ir.IntType(8).as_pointer()

            # Get or create box.from_i64
            boxer = None
            for f in module.functions:
                if f.name == "nyash.box.from_i64":
                    boxer = f
                    break
            if boxer is None:
                boxer = ir.Function(module, ir.FunctionType(ir.IntType(64), [ir.IntType(64)]), name="nyash.box.from_i64")

            # Get or create to_i8p_h
            to_i8p = None
            for f in module.functions:
                if f.name == "nyash.string.to_i8p_h":
                    to_i8p = f
                    break
            if not to_i8p:
                to_i8p_type = ir.FunctionType(i8p, [ir.IntType(64)])
                to_i8p = ir.Function(module, to_i8p_type, name="nyash.string.to_i8p_h")

            # box.from_i64(arg_val)
            box_val = builder.call(boxer, [arg_val])
            # to_i8p_h(box_val)
            i8p_val = builder.call(to_i8p, [box_val])
            return i8p_val

    @staticmethod
    def is_stringish(type_info: dict) -> bool:
        """型がstringishか判定

        Args:
            type_info: 型情報dict

        Returns:
            stringishならTrue
        """
        return type_info.get("stringish", False)
