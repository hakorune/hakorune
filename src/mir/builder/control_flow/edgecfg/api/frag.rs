/*!
 * Frag - CFG Fragment（Phase 264: 構造化制御の合成単位）
 *
 * 構造化制御（if/loop/catch/cleanup）から CFG への lowering において、
 * 未配線の脱出エッジを持つ CFG 断片を表現する。
 */

use std::collections::BTreeMap;
use crate::mir::basic_block::BasicBlockId;
use super::block_params::BlockParams;
use super::exit_kind::ExitKind;
use super::edge_stub::EdgeStub;
use super::branch_stub::BranchStub;

/// CFG Fragment（構造化制御の合成単位）
///
/// # 責務（Phase 267 P0 更新）
/// - `entry`: 断片の入口ブロック
/// - `exits`: 断片から外へ出る未配線 edge の集合（target = None のみ）
/// - `wires`: 断片内部で解決された配線（target = Some(...) のみ、Jump/Return 専用）
/// - `branches`: 断片内部の Branch 配線（Phase 267 P0 追加、Branch 専用）
///
/// # 設計原則
/// - 各 Frag は「入口1つ、出口複数（種別ごと）」を持つ
/// - 合成則（seq/if_/loop_/cleanup）で複数 Frag を組み合わせる
/// - pattern番号は「形の認識」までに留め、配線層へ逆流させない
///
/// # 不変条件（verify で検証）
/// - entry は有効な BasicBlockId
/// - exits 内の EdgeStub は target = None（未配線、外へ出る exit）
/// - wires 内の EdgeStub は target = Some(...)（配線済み、内部配線、Jump/Return のみ）
/// - branches 内の BranchStub は Branch 専用配線（Phase 267 P0）
/// - EdgeStub.kind と Map のキーが一致
///
/// # BTreeMap の使用理由
/// - Phase 69-3 の教訓: HashMap は非決定的イテレーションを起こす
/// - ExitKind の順序を決定的にすることで、デバッグ出力・テストが安定
#[derive(Debug, Clone)]
pub struct Frag {
    /// 断片の入口ブロック
    pub entry: BasicBlockId,

    /// 断片の block params（join 受け口）
    pub block_params: BTreeMap<BasicBlockId, BlockParams>,

    /// 断片からの未配線脱出エッジ（ExitKind → EdgeStub のリスト）
    ///
    /// Phase 265 P2: target = None のみ（外へ出る exit）
    /// BTreeMap を使用して決定的順序を保証（Phase 69-3 の教訓）
    pub exits: BTreeMap<ExitKind, Vec<EdgeStub>>,

    /// 配線済みの内部配線（Phase 265 P2 追加）
    ///
    /// target = Some(...) のみ（断片内部で解決された配線）
    /// Phase 266 で MIR terminator に落とす
    /// Jump/Return 専用（Branch は branches フィールドへ）
    pub wires: Vec<EdgeStub>,

    /// 配線済みの分岐（Phase 267 P0 追加）
    ///
    /// Branch 専用の配線
    /// wires は Jump/Return 専用のまま維持（分離を保つ）
    pub branches: Vec<BranchStub>,
}

impl Frag {
    /// 新規 Frag を生成（出口なし）
    pub fn new(entry: BasicBlockId) -> Self {
        Self {
            entry,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![],  // Phase 265 P2: 配線済み内部配線
            branches: vec![],  // Phase 267 P0: 配線済み分岐
        }
    }

    /// 単一出口を持つ Frag を生成
    pub fn with_single_exit(entry: BasicBlockId, stub: EdgeStub) -> Self {
        let mut exits = BTreeMap::new();
        exits.insert(stub.kind, vec![stub]);
        Self {
            entry,
            block_params: BTreeMap::new(),
            exits,
            wires: vec![],  // Phase 265 P2: 配線済み内部配線
            branches: vec![],  // Phase 267 P0: 配線済み分岐
        }
    }

    /// 特定 ExitKind の未配線 edge を追加
    #[cfg(test)]
    pub fn add_exit(&mut self, stub: EdgeStub) {
        self.exits.entry(stub.kind).or_insert_with(Vec::new).push(stub);
    }

    /// 特定 ExitKind の未配線 edge を取得
    #[cfg(test)]
    pub fn get_exits(&self, kind: &ExitKind) -> Option<&Vec<EdgeStub>> {
        self.exits.get(kind)
    }

    /// すべての ExitKind を列挙
    #[cfg(test)]
    pub fn exit_kinds(&self) -> impl Iterator<Item = &ExitKind> {
        self.exits.keys()
    }
}

// ============================================================================
// ユニットテスト（Phase 264: 最小3個）
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frag_basic_construction() {
        // 最小のテスト: Frag が ExitKind::Normal を保持できる
        let entry = BasicBlockId::new(0);
        let from = BasicBlockId::new(1);

        let stub = EdgeStub::without_args(from, ExitKind::Normal);
        let frag = Frag::with_single_exit(entry, stub.clone());

        assert_eq!(frag.entry, entry);

        let exits = frag.get_exits(&ExitKind::Normal).unwrap();
        assert_eq!(exits.len(), 1);
        assert_eq!(exits[0].from, from);
        assert_eq!(exits[0].kind, ExitKind::Normal);
    }

    #[test]
    fn test_frag_multiple_exits() {
        // 複数 ExitKind のテスト
        let entry = BasicBlockId::new(0);
        let mut frag = Frag::new(entry);

        frag.add_exit(EdgeStub::without_args(BasicBlockId::new(1), ExitKind::Normal));
        frag.add_exit(EdgeStub::without_args(BasicBlockId::new(2), ExitKind::Return));

        assert_eq!(frag.exits.len(), 2);
        assert!(frag.get_exits(&ExitKind::Normal).is_some());
        assert!(frag.get_exits(&ExitKind::Return).is_some());
        assert!(frag.get_exits(&ExitKind::Unwind).is_none());
    }

    #[test]
    fn test_exit_kind_helpers() {
        use crate::mir::control_form::LoopId;

        assert!(!ExitKind::Normal.is_loop_exit());
        assert!(ExitKind::Break(LoopId(0)).is_loop_exit());
        assert!(ExitKind::Continue(LoopId(0)).is_loop_exit());

        assert!(ExitKind::Return.is_function_exit());
        assert!(ExitKind::Unwind.is_function_exit());
        assert!(!ExitKind::Normal.is_function_exit());
    }
}
