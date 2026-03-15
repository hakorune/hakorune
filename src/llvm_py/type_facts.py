"""Type Facts Box - 型情報伝播のSSoT

Phase 97 Refactoring: stringish等のtype tag伝播を一箇所に集約。
"""

from typing import Dict, Any, Optional


def make_box_handle_fact(box_type: str) -> Dict[str, Any]:
    return {"kind": "handle", "box_type": box_type}


def is_box_handle_fact(fact: Any, box_type: str) -> bool:
    return (
        isinstance(fact, dict)
        and fact.get("kind") == "handle"
        and fact.get("box_type") == box_type
    )


def is_stringish_fact(fact: Any) -> bool:
    if isinstance(fact, dict):
        return fact.get("kind") == "string" or is_box_handle_fact(fact, "StringBox")
    return isinstance(fact, str) and fact in ("string", "String", "StringBox")


def is_arrayish_fact(fact: Any) -> bool:
    if isinstance(fact, dict):
        return is_box_handle_fact(fact, "ArrayBox")
    return isinstance(fact, str) and fact in ("array", "Array", "ArrayBox")


class TypeFactsBox:
    """型情報（Type Facts）の管理と伝播Box

    責務:
    - 型tag（stringish等）の登録・取得
    - Copy命令での型伝播
    - PHI命令での型伝播
    - 伝播ルールのSSoT化

    契約:
    - 入力: ValueId、型情報
    - 出力: 伝播後の型情報
    - 不変条件: 一度tagが付いたValueIdは変更不可（monotonic）

    Fail-Fast:
    - 矛盾する型tag → AssertionError

    設計原則:
    - monotonic: 型情報は追加のみ、削除・変更は禁止
    - explicit: 暗黙的な型推論は行わない、明示的なtagのみ
    """

    def __init__(self):
        self._facts: Dict[Any, Dict[str, Any]] = {}

    def _ensure_slot(self, value_id: Any) -> Dict[str, Any]:
        if value_id not in self._facts:
            self._facts[value_id] = {}
        return self._facts[value_id]

    def mark_string(self, value_id: Any, reason: str = "explicit"):
        """ValueIdをstringishとしてマーク

        Args:
            value_id: ValueId
            reason: マーク理由（デバッグ用）
        """
        facts = self._ensure_slot(value_id)
        # monotonic check
        if "stringish" in facts:
            assert facts["stringish"], \
                f"[TypeFactsBox] Cannot change stringish tag for {value_id}"

        facts["stringish"] = True
        facts["reason"] = reason

    def propagate_copy(self, dst: Any, src: Any):
        """Copy命令での型伝播

        契約: dst = copy src → dst inherits src's type facts

        Args:
            dst: コピー先ValueId
            src: コピー元ValueId
        """
        if src in self._facts:
            src_facts = self._facts[src].copy()
            src_facts["reason"] = f"copy from {src}"
            self._facts[dst] = src_facts

    def _common_incoming_stringish_reason(self, incoming_ids: list) -> Optional[str]:
        if not incoming_ids:
            return None
        if all(self.is_stringish(vid) for vid in incoming_ids):
            return f"phi from {incoming_ids}"
        return None

    def propagate_phi(self, phi_id: Any, incoming_ids: list):
        """PHI命令での型伝播

        契約: phi = PHI [v1, v2, ...] → phi inherits common type facts

        Args:
            phi_id: PHI結果ValueId
            incoming_ids: PHI入力ValueId list
        """
        reason = self._common_incoming_stringish_reason(incoming_ids)
        if reason is not None:
            self.mark_string(phi_id, reason=reason)

    def is_stringish(self, value_id: Any) -> bool:
        """ValueIdがstringishか判定

        Args:
            value_id: ValueId

        Returns:
            stringishならTrue
        """
        return self._facts.get(value_id, {}).get("stringish", False)

    def get_facts(self, value_id: Any) -> dict:
        """ValueIdの型情報を取得

        Args:
            value_id: ValueId

        Returns:
            型情報dict（存在しない場合は空dict）
        """
        return self._facts.get(value_id, {}).copy()
