//! Phase 84-3: PhiTypeResolver — PHI + Copy グラフ型推論箱（ChatGPT Pro設計）
//!
//! # 責務
//!
//! 「PHI + Copy の小さなグラフ」を辿って、安全に型を決められるときだけ
//! MirType を返す。GenericTypeResolver / CopyTypePropagator の責務を
//! 太らせず、PHI 系だけをこの箱に閉じ込める。
//!
//! # 設計原則（箱理論）
//!
//! - **単一責務**: PHI + Copy グラフ追跡と安全条件判定のみ
//! - **探索限定**: Copy / Phi / base 定義 の 3 種類だけ
//! - **安全条件**: 1 種類の型に収束する場合のみ Some を返す
//!
//! # アルゴリズム
//!
//! 1. DFS/BFS で root から探索開始
//! 2. Copy → src へ進む
//! 3. Phi → 各 incoming ValueId へ進む
//! 4. それ以外（Const/Call/NewBox/TypeOp/BinOp...）は base 定義
//! 5. base_types 集合を収集し、1 種類なら返す
//!
//! # 安全装置
//!
//! - visited で同じ ValueId を 2 回以上辿らない（ループ防止）
//! - 探索上限で打ち切り

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// Phase 84-3: PHI + Copy グラフ型推論箱
pub struct PhiTypeResolver<'f> {
    value_types: &'f BTreeMap<ValueId, MirType>,
    definitions: BTreeMap<ValueId, DefKind>,
}

impl<'f> PhiTypeResolver<'f> {
    /// 新しい PhiTypeResolver を作成
    pub fn new(func: &'f MirFunction, value_types: &'f BTreeMap<ValueId, MirType>) -> Self {
        Self {
            value_types,
            definitions: Self::build_definitions(func),
        }
    }

    /// root (ret_val など) の型を、PHI + Copy グラフから安全に推論できれば返す
    ///
    /// # 戻り値
    ///
    /// - `Some(MirType)`: 1 種類の型に収束した場合
    /// - `None`: 空 / 2 種類以上 / Unknown/Void のみ
    pub fn resolve(&self, root: ValueId) -> Option<MirType> {
        let mut visited: BTreeSet<ValueId> = BTreeSet::new();
        let mut stack: Vec<ValueId> = vec![root];
        let mut base_types: Vec<MirType> = Vec::new();

        // 安全装置: 探索上限
        let max_visits = self.value_types.len() + 100;

        while let Some(v) = stack.pop() {
            // 既に訪問済みならスキップ
            if visited.contains(&v) {
                continue;
            }
            visited.insert(v);

            // 安全装置: 探索上限チェック
            if visited.len() > max_visits {
                if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                    crate::runtime::get_global_ring0().log.warn(&format!(
                        "[phi_resolver] warning: exceeded max visits {}, stopping",
                        max_visits
                    ));
                }
                break;
            }

            // v の定義元命令を探す
            match self.find_definition(v) {
                Some(DefKind::Copy { src }) => {
                    // Copy → src へ進む
                    if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                        crate::runtime::get_global_ring0()
                            .log
                            .debug(&format!("[phi_resolver] {:?} -> Copy from {:?}", v, src));
                    }
                    stack.push(*src);
                }
                Some(DefKind::Phi { inputs }) => {
                    // Phi → 各 incoming ValueId へ進む
                    if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[phi_resolver] {:?} -> Phi with {} inputs",
                            v,
                            inputs.len()
                        ));
                    }
                    for (_, incoming) in inputs {
                        stack.push(*incoming);
                    }
                }
                Some(DefKind::Base) | None => {
                    // base 定義または未知 → value_types から型を取得
                    if let Some(ty) = self.value_types.get(&v) {
                        // Unknown / Void は除外
                        if !matches!(ty, MirType::Unknown | MirType::Void) {
                            // 重複を避けて追加（eq で比較）
                            if !base_types.iter().any(|t| t == ty) {
                                if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                                    crate::runtime::get_global_ring0().log.debug(&format!(
                                        "[phi_resolver] {:?} -> Base type {:?}",
                                        v, ty
                                    ));
                                }
                                base_types.push(ty.clone());
                            }
                        }
                    } else if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                        crate::runtime::get_global_ring0()
                            .log
                            .debug(&format!("[phi_resolver] {:?} -> No type in value_types", v));
                    }
                }
            }
        }

        // 安全条件: 1 種類に収束したら返す
        if base_types.len() == 1 {
            let ty = base_types.into_iter().next().unwrap();
            if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[phi_resolver] resolved {:?} -> {:?} (visited {})",
                    root,
                    ty,
                    visited.len()
                ));
            }
            return Some(ty);
        }

        if std::env::var("NYASH_PHI_RESOLVER_DEBUG").is_ok() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[phi_resolver] failed for {:?}: base_types = {:?}",
                root, base_types
            ));
        }

        None
    }

    fn build_definitions(func: &MirFunction) -> BTreeMap<ValueId, DefKind> {
        let mut definitions = BTreeMap::new();
        for bb in func.blocks.values() {
            for inst in &bb.instructions {
                if let Some((dst, kind)) = Self::definition_from_instruction(inst) {
                    definitions.entry(dst).or_insert(kind);
                }
            }
        }
        definitions
    }

    fn definition_from_instruction(inst: &MirInstruction) -> Option<(ValueId, DefKind)> {
        match inst {
            MirInstruction::Copy { dst, src } => Some((*dst, DefKind::Copy { src: *src })),
            MirInstruction::Phi { dst, inputs, .. } => Some((
                *dst,
                DefKind::Phi {
                    inputs: inputs.clone(),
                },
            )),
            MirInstruction::Const { dst, .. }
            | MirInstruction::UnaryOp { dst, .. }
            | MirInstruction::BinOp { dst, .. }
            | MirInstruction::Compare { dst, .. }
            | MirInstruction::TypeOp { dst, .. }
            | MirInstruction::Load { dst, .. }
            | MirInstruction::NewBox { dst, .. } => Some((*dst, DefKind::Base)),
            MirInstruction::Call { dst: Some(dst), .. } => Some((*dst, DefKind::Base)),
            _ => None,
        }
    }

    /// ValueId の定義元命令を探す
    fn find_definition(&self, v: ValueId) -> Option<&DefKind> {
        self.definitions.get(&v)
    }
}

/// 定義元の種類
#[derive(Clone)]
enum DefKind {
    /// Copy { dst, src } → src へ進む
    Copy { src: ValueId },
    /// Phi { dst, inputs } → 各 incoming へ進む
    Phi {
        inputs: Vec<(crate::mir::BasicBlockId, ValueId)>,
    },
    /// その他の base 定義（Const/Call/NewBox 等）
    Base,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction};

    fn make_test_function() -> MirFunction {
        let sig = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        MirFunction::new(sig, BasicBlockId::new(0))
    }

    #[test]
    fn test_resolve_direct_type() {
        let f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has type Integer directly
        value_types.insert(ValueId(1), MirType::Integer);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        assert_eq!(resolver.resolve(ValueId(1)), Some(MirType::Integer));
    }

    #[test]
    fn test_resolve_through_copy() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has type Integer, Copy v2 <- v1
        value_types.insert(ValueId(1), MirType::Integer);

        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        copy_emitter::emit_copy_into_detached_block(
            &mut bb,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestPhiTypeResolverCopy,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        assert_eq!(resolver.resolve(ValueId(2)), Some(MirType::Integer));
    }

    #[test]
    fn test_resolve_through_phi_uniform() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 and v2 both have type Integer
        value_types.insert(ValueId(1), MirType::Integer);
        value_types.insert(ValueId(2), MirType::Integer);

        // Phi v3 <- [(Block0, v1), (Block1, v2)]
        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        bb.instructions.push(MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![
                (BasicBlockId::new(0), ValueId(1)),
                (BasicBlockId::new(1), ValueId(2)),
            ],
            type_hint: None,
        });
        f.blocks.insert(BasicBlockId::new(0), bb);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        assert_eq!(resolver.resolve(ValueId(3)), Some(MirType::Integer));
    }

    #[test]
    fn test_resolve_through_phi_mixed_types() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has Integer, v2 has String (different types)
        value_types.insert(ValueId(1), MirType::Integer);
        value_types.insert(ValueId(2), MirType::String);

        // Phi v3 <- [(Block0, v1), (Block1, v2)]
        let mut bb = BasicBlock::new(BasicBlockId::new(0));
        bb.instructions.push(MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![
                (BasicBlockId::new(0), ValueId(1)),
                (BasicBlockId::new(1), ValueId(2)),
            ],
            type_hint: None,
        });
        f.blocks.insert(BasicBlockId::new(0), bb);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        // Should return None because types don't match
        assert_eq!(resolver.resolve(ValueId(3)), None);
    }

    #[test]
    fn test_resolve_through_phi_and_copy() {
        let mut f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has type Integer
        value_types.insert(ValueId(1), MirType::Integer);

        let mut bb0 = BasicBlock::new(BasicBlockId::new(0));
        // Copy v2 <- v1
        copy_emitter::emit_copy_into_detached_block(
            &mut bb0,
            ValueId(2),
            ValueId(1),
            CopyEmitReason::TestPhiTypeResolverPhiCopy,
        )
        .unwrap();
        f.blocks.insert(BasicBlockId::new(0), bb0);

        let mut bb1 = BasicBlock::new(BasicBlockId::new(1));
        // Phi v3 <- [(Block0, v2)]
        bb1.instructions.push(MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![(BasicBlockId::new(0), ValueId(2))],
            type_hint: None,
        });
        f.blocks.insert(BasicBlockId::new(1), bb1);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        // Should resolve v3 through Phi -> v2 -> Copy -> v1 -> Integer
        assert_eq!(resolver.resolve(ValueId(3)), Some(MirType::Integer));
    }

    #[test]
    fn test_resolve_unknown_returns_none() {
        let f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has type Unknown
        value_types.insert(ValueId(1), MirType::Unknown);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        // Unknown should be filtered out
        assert_eq!(resolver.resolve(ValueId(1)), None);
    }

    #[test]
    fn test_resolve_void_returns_none() {
        let f = make_test_function();
        let mut value_types = BTreeMap::new();

        // v1 has type Void
        value_types.insert(ValueId(1), MirType::Void);

        let resolver = PhiTypeResolver::new(&f, &value_types);
        // Void should be filtered out
        assert_eq!(resolver.resolve(ValueId(1)), None);
    }
}
