"""Type Facts Box - 型情報伝播のSSoT

Phase 97 Refactoring: stringish等のtype tag伝播を一箇所に集約。
"""

from typing import Dict, Set, Any


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

    def mark_string(self, value_id: Any, reason: str = "explicit"):
        """ValueIdをstringishとしてマーク

        Args:
            value_id: ValueId
            reason: マーク理由（デバッグ用）
        """
        if value_id not in self._facts:
            self._facts[value_id] = {}

        # monotonic check
        if "stringish" in self._facts[value_id]:
            assert self._facts[value_id]["stringish"], \
                f"[TypeFactsBox] Cannot change stringish tag for {value_id}"

        self._facts[value_id]["stringish"] = True
        self._facts[value_id]["reason"] = reason

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

    def propagate_phi(self, phi_id: Any, incoming_ids: list):
        """PHI命令での型伝播

        契約: phi = PHI [v1, v2, ...] → phi inherits common type facts

        Args:
            phi_id: PHI結果ValueId
            incoming_ids: PHI入力ValueId list
        """
        # 全入力が同じtype factを持つ場合のみ伝播
        if not incoming_ids:
            return

        # 最初の入力の型情報を基準
        first_facts = self._facts.get(incoming_ids[0], {})

        # 全入力が同じstringish tagを持つか確認
        all_stringish = all(
            self._facts.get(vid, {}).get("stringish", False)
            for vid in incoming_ids
        )

        if all_stringish and "stringish" in first_facts:
            self.mark_string(phi_id, reason=f"phi from {incoming_ids}")

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
