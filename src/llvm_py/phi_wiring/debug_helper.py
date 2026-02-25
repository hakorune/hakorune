"""Phase 278 P0: PHI デバッグ環境変数のSSOT

統合された環境変数を一元管理。

Phase 277 P2 統合（8個 → 3個）:
- NYASH_LLVM_DEBUG_PHI: 一般デバッグ（旧変数削除済み）
- NYASH_LLVM_DEBUG_PHI_TRACE: 詳細トレース（旧変数削除済み）
- NYASH_LLVM_PHI_STRICT: 厳格モード（変更なし）

Phase 278 P0: 旧環境変数の後方互換性を削除（fail-fast化）
"""
import os
import sys


def is_phi_debug_enabled():
    """PHI一般デバッグが有効か

    統合対象:
    - NYASH_LLVM_PHI_DEBUG (旧)
    - NYASH_PHI_TYPE_DEBUG (旧)
    - NYASH_PHI_ORDERING_DEBUG (旧)

    Returns:
        bool: デバッグモードが有効
    """
    # 新環境変数
    if os.environ.get('NYASH_LLVM_DEBUG_PHI') == '1':
        return True

    # 旧環境変数の後方互換性（Phase 278で削除）
    legacy_vars = [
        'NYASH_LLVM_PHI_DEBUG',
        'NYASH_PHI_TYPE_DEBUG',
        'NYASH_PHI_ORDERING_DEBUG'
    ]
    for var in legacy_vars:
        if os.environ.get(var) == '1':
            print(f"❌ ERROR: {var} was removed in Phase 278. Use NYASH_LLVM_DEBUG_PHI=1 instead.",
                  file=sys.stderr)
            sys.exit(1)

    return False


def is_phi_trace_enabled():
    """PHI詳細トレースが有効か

    統合対象:
    - NYASH_LLVM_TRACE_PHI (旧)
    - NYASH_LLVM_VMAP_TRACE (旧)

    Returns:
        bool: トレースモードが有効
    """
    # 新環境変数
    if os.environ.get('NYASH_LLVM_DEBUG_PHI_TRACE') == '1':
        return True

    # 旧環境変数の後方互換性（Phase 278で削除）
    legacy_vars = [
        'NYASH_LLVM_TRACE_PHI',
        'NYASH_LLVM_VMAP_TRACE'
    ]
    for var in legacy_vars:
        if os.environ.get(var) == '1':
            print(f"❌ ERROR: {var} was removed in Phase 278. Use NYASH_LLVM_DEBUG_PHI_TRACE=1 instead.",
                  file=sys.stderr)
            sys.exit(1)

    return False


def is_phi_strict_enabled():
    """PHI厳格モードが有効か（既存維持）

    ゼロフォールバック時にエラーを発生させる。

    Returns:
        bool: 厳格モードが有効
    """
    return os.environ.get('NYASH_LLVM_PHI_STRICT') == '1'
