"""PHI Snapshot Policy Box - PHI値のSSA有効性契約

Phase 97 Refactoring: PHI値のsnapshot処理とSSA有効性の契約をSSOT化。
"""

from typing import Any, Optional


class PhiSnapshotPolicyBox:
    """PHI Snapshot Policy Box

    責務:
    - PHI値のSSA有効性判定
    - Snapshot上のPHI参照ポリシー
    - PHI miss判定の統一

    契約（重要）:
    「PHIはSSA値として他blockでも有効」
    - PHI値はdefining blockのみでなく、dominate先でも有効
    - snapshot上のPHIを「miss」扱いしてはならない
    - PHI値は一度定義されたら変更されない（SSA不変条件）

    Fail-Fast:
    - PHI値を「未定義」扱い → AssertionError
    - snapshot miss時にPHI値を無視 → AssertionError

    この契約の破綻により過去に以下の問題が発生：
    - PHI値が他blockで「未定義」扱いされる
    - snapshot miss時にPHI値が消失
    - SSA不変条件の破綻
    """

    @staticmethod
    def is_phi_valid_at(phi_id: Any, block_id: Any, dominator_info: dict) -> bool:
        """PHI値が指定blockで有効か判定

        契約: PHI値は defining block および dominate先で有効

        Args:
            phi_id: PHI ValueId
            block_id: 参照block
            dominator_info: dominator情報

        Returns:
            有効ならTrue
        """
        phi_block = dominator_info.get_defining_block(phi_id)

        # PHI値のdefining blockまたはdominate先なら有効
        if block_id == phi_block:
            return True

        if dominator_info.dominates(phi_block, block_id):
            return True

        return False

    @staticmethod
    def resolve_phi_at_snapshot(phi_id: Any, snapshot: dict,
                                resolver: Any) -> Optional[Any]:
        """Snapshot上でPHI値を解決

        契約: snapshot miss時もPHI値を返す（miss扱いしない）

        Args:
            phi_id: PHI ValueId
            snapshot: block終端のsnapshot
            resolver: Value resolver

        Returns:
            PHI値（snapshot missでもPHI定義値を返す）
        """
        # まずsnapshotを確認
        if phi_id in snapshot:
            return snapshot[phi_id]

        # snapshot miss時: PHI定義値を返す（miss扱いしない）
        if resolver and hasattr(resolver, 'get_phi_definition'):
            return resolver.get_phi_definition(phi_id)

        # PHI値が取得できない場合は契約違反
        raise AssertionError(
            f"[PhiSnapshotPolicyBox] Cannot resolve PHI value: {phi_id}"
        )

    @staticmethod
    def is_phi(value_id: Any, resolver: Any) -> bool:
        """ValueIdがPHI値か判定

        Args:
            value_id: ValueId
            resolver: Value resolver

        Returns:
            PHI値ならTrue
        """
        if resolver and hasattr(resolver, 'is_phi'):
            return resolver.is_phi(value_id)

        # Fallback: PHI判定ができない場合はFalse
        return False
