//! JoinKey / JoinRegistry - Join block 管理（Phase 29bq+）
//!
//! # 目的
//! - Join block の再利用が必要な場合のみ使用（副産物）
//! - メイン機能は PlanBuildSession の手順 SSOT
//!
//! # 設計原則
//! - JoinKey は session-local（clone/freshen 跨ぎ共有禁止）
//! - ID は PlanBuildSession が発行
//! - ShortCircuitOuter は JoinKey ではない（on_true/on_false ターゲットで表現）

use std::collections::BTreeMap;
use crate::mir::BasicBlockId;

/// Join block の種別（session-local な id で識別）
///
/// # 注意
/// - これは「再利用」が必要な場合のみ使う副産物
/// - メイン機能は PlanBuildSession の手順 SSOT
///
/// # 設計決定
/// - ShortCircuitOuter は JoinKey ではない
/// - cond lowering の出口ターゲット（on_true/on_false）で表現する
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JoinKey {
    /// if-else の merge block
    IfJoin { id: u32 },
    /// loop header PHI/params の合流点（"SSAの芯"）
    LoopHeader { id: u32 },
    /// loop の after block（break/exit 合流）
    LoopAfter { id: u32 },
}

/// JoinKey → BasicBlockId の registry（PlanBuildSession 内部で使用）
///
/// # 設計原則
/// - 外へ露出させない（session.register_join() / get_join() で間接的に使用）
/// - session-local（clone/freshen と独立）
#[derive(Debug, Clone)]
pub(super) struct JoinRegistry {
    joins: BTreeMap<JoinKey, BasicBlockId>,
}

impl JoinRegistry {
    /// 空の registry を生成
    pub fn new() -> Self {
        Self {
            joins: BTreeMap::new(),
        }
    }

    /// JoinKey で登録済みの block を取得
    pub fn get(&self, key: JoinKey) -> Option<BasicBlockId> {
        self.joins.get(&key).copied()
    }

    /// JoinKey と block を関連付け
    ///
    /// # 注意
    /// - 既に登録済みの場合は上書き
    pub fn register(&mut self, key: JoinKey, bb: BasicBlockId) {
        self.joins.insert(key, bb);
    }
}

impl Default for JoinRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_registry() {
        let mut registry = JoinRegistry::new();
        let key = JoinKey::IfJoin { id: 0 };
        let bb = BasicBlockId(42);

        // 未登録
        assert_eq!(registry.get(key), None);

        // 登録
        registry.register(key, bb);
        assert_eq!(registry.get(key), Some(bb));

        // 上書き
        let bb2 = BasicBlockId(99);
        registry.register(key, bb2);
        assert_eq!(registry.get(key), Some(bb2));
    }

    #[test]
    fn test_loop_keys() {
        let mut registry = JoinRegistry::new();
        let header_key = JoinKey::LoopHeader { id: 0 };
        let after_key = JoinKey::LoopAfter { id: 0 };
        let header_bb = BasicBlockId(10);
        let after_bb = BasicBlockId(20);

        registry.register(header_key, header_bb);
        registry.register(after_key, after_bb);

        assert_eq!(registry.get(header_key), Some(header_bb));
        assert_eq!(registry.get(after_key), Some(after_bb));
    }
}
