"""Call Routing Policy Box - Call種別判定のSSoT

Phase 97 Refactoring: static method / instance method / plugin invoke の
ルーティング判定を一箇所に集約。
"""

from enum import Enum
from typing import Optional, NamedTuple


class CallKind(Enum):
    """Call の種別"""
    STATIC_METHOD = "static_method"      # Box.method() 形式
    INSTANCE_METHOD = "instance_method"  # box.method() 形式
    PLUGIN_INVOKE = "plugin_invoke"      # Plugin経由の呼び出し


class RouteDecision(NamedTuple):
    """ルーティング判定結果"""
    kind: CallKind
    is_direct_call: bool  # static method直呼びか
    reason: str           # 判定理由（デバッグ用）


class CallRoutePolicyBox:
    """Call ルーティングのPolicy Box

    責務:
    - Callee文字列から Call種別を判定
    - static method直呼びの判定
    - 判定理由の明示

    契約:
    - 入力: callee文字列（例: "StringBox.concat", "box.method", "PluginBox.invoke"）
    - 出力: RouteDecision（kind, is_direct_call, reason）
    - 前提条件: callee文字列が非空

    Fail-Fast:
    - callee が空文字列 → ValueError
    - 不明なcallee形式 → ValueError（"unknown callee format"）
    """

    @staticmethod
    def decide(callee: str, ctx: Optional[dict] = None) -> RouteDecision:
        """Call種別を判定

        Args:
            callee: Callee文字列（例: "StringBox.concat"）
            ctx: 追加コンテキスト（将来拡張用）

        Returns:
            RouteDecision（kind, is_direct_call, reason）

        Raises:
            ValueError: callee が空文字列または不明な形式
        """
        if not callee:
            raise ValueError("[CallRoutePolicyBox] callee must not be empty")

        ctx = ctx or {}

        # static method判定（Box.method形式）
        if "." in callee and callee[0].isupper():
            # 例: "StringBox.concat", "IntegerBox.create"
            is_direct = CallRoutePolicyBox._is_direct_static_call(callee, ctx)
            reason = f"static method: {callee}, direct={is_direct}"
            return RouteDecision(
                kind=CallKind.STATIC_METHOD,
                is_direct_call=is_direct,
                reason=reason
            )

        # instance method判定（box.method形式）
        if "." in callee and not callee[0].isupper():
            # 例: "receiver.substring", "obj.get"
            reason = f"instance method: {callee}"
            return RouteDecision(
                kind=CallKind.INSTANCE_METHOD,
                is_direct_call=False,
                reason=reason
            )

        # plugin invoke判定
        if "Plugin" in callee or ctx.get("is_plugin", False):
            reason = f"plugin invoke: {callee}"
            return RouteDecision(
                kind=CallKind.PLUGIN_INVOKE,
                is_direct_call=False,
                reason=reason
            )

        # 不明な形式
        raise ValueError(f"[CallRoutePolicyBox] unknown callee format: {callee}")

    @staticmethod
    def _is_direct_static_call(callee: str, ctx: dict) -> bool:
        """static method が直呼び可能か判定

        Phase 97: builtin Box（StringBox, IntegerBox等）は直呼び可能。
        Plugin Boxは非直呼び。

        Args:
            callee: static method名（例: "StringBox.concat"）
            ctx: コンテキスト（builtin_boxes等）

        Returns:
            直呼び可能ならTrue
        """
        builtin_boxes = ctx.get("builtin_boxes", [
            "StringBox", "IntegerBox", "BoolBox", "ArrayBox", "MapBox"
        ])

        box_name = callee.split(".")[0]
        is_builtin = box_name in builtin_boxes

        return is_builtin


# デバッグ用ヘルパー
def log_route_decision(decision: RouteDecision, verbose: bool = False):
    """ルーティング判定をログ出力

    Args:
        decision: RouteDecision
        verbose: 詳細ログ有効化
    """
    if not verbose:
        return

    print(f"[phase97/call-route] {decision.reason} (kind={decision.kind.value}, direct={decision.is_direct_call})")
