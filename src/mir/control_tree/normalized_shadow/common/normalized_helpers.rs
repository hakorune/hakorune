//! Phase 143.5: Normalized lowering 共通ヘルパー関数
//!
//! 複数のパターン（loop, if）で共有されるヘルパー関数群
//! - alloc_value_id: ValueId割り当て
//! - alloc_env_params: Env parameters 割り当て
//! - build_env_map: BTreeMap構築
//! - collect_env_args: Env arguments 収集
//! - is_bool_true_literal: Loop condition 検証

use crate::mir::ValueId;
use crate::mir::join_ir::lowering::join_value_space::{PARAM_MIN, LOCAL_MIN};
use crate::ast::ASTNode;
use std::collections::BTreeMap;

#[cfg(test)]
use crate::ast::Span;

/// Box-First: Normalized lowering 共通ヘルパー
pub struct NormalizedHelperBox;

impl NormalizedHelperBox {
    /// Allocate next ValueId and increment counter
    pub fn alloc_value_id(next_value_id: &mut u32) -> ValueId {
        let vid = ValueId(*next_value_id);
        *next_value_id += 1;
        vid
    }

    /// Allocate env parameters (one ValueId per field)
    pub fn alloc_env_params(
        fields: &[String],
        next_value_id: &mut u32,
    ) -> Vec<ValueId> {
        fields
            .iter()
            .map(|_| Self::alloc_value_id(next_value_id))
            .collect()
    }

    /// Build env map from fields and parameters
    pub fn build_env_map(
        fields: &[String],
        params: &[ValueId],
    ) -> BTreeMap<String, ValueId> {
        let mut env = BTreeMap::new();
        for (name, vid) in fields.iter().zip(params.iter()) {
            env.insert(name.clone(), *vid);
        }
        env
    }

    /// Collect env arguments from environment
    pub fn collect_env_args(
        fields: &[String],
        env: &BTreeMap<String, ValueId>,
    ) -> Result<Vec<ValueId>, String> {
        let mut args = Vec::with_capacity(fields.len());
        for name in fields {
            let vid = env
                .get(name)
                .copied()
                .ok_or_else(|| format!("Missing env variable: {}", name))?;
            args.push(vid);
        }
        Ok(args)
    }

    /// Check if AST node is Bool(true) literal
    pub fn is_bool_true_literal(ast: &ASTNode) -> bool {
        matches!(
            ast,
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Bool(true),
                ..
            }
        )
    }

    /// Allocate env parameters in Param region (100+)
    ///
    /// Phase 143 fix: env params must be in Param region (100-999) per JoinValueSpace contract.
    ///
    /// Returns (params, next_local) where:
    /// - params: Vec<ValueId> in Param region [100, 100+n)
    /// - next_local: Starting point for local allocations (1000+)
    ///
    /// Call sites should use `next_local` for subsequent alloc_value_id() calls.
    ///
    /// ## Contract
    ///
    /// - PHI Reserved (0-99): Loop header PHI dst
    /// - Param Region (100-999): env params (this function)
    /// - Local Region (1000+): Const, BinOp, condition results
    pub fn alloc_env_params_param_region(fields: &[String]) -> (Vec<ValueId>, u32) {
        let params: Vec<ValueId> = fields
            .iter()
            .enumerate()
            .map(|(i, _)| ValueId(PARAM_MIN + i as u32))
            .collect();
        // Local region starts at LOCAL_MIN (1000)
        (params, LOCAL_MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_value_id() {
        let mut next = 10;
        let vid = NormalizedHelperBox::alloc_value_id(&mut next);
        assert_eq!(vid, ValueId(10));
        assert_eq!(next, 11);
    }

    #[test]
    fn test_alloc_value_id_increments() {
        let mut next = 1;
        let vid1 = NormalizedHelperBox::alloc_value_id(&mut next);
        let vid2 = NormalizedHelperBox::alloc_value_id(&mut next);
        assert_eq!(vid1, ValueId(1));
        assert_eq!(vid2, ValueId(2));
        assert_eq!(next, 3);
    }

    #[test]
    fn test_alloc_env_params() {
        let fields = vec!["a".to_string(), "b".to_string()];
        let mut next = 1;
        let params = NormalizedHelperBox::alloc_env_params(&fields, &mut next);
        assert_eq!(params, vec![ValueId(1), ValueId(2)]);
        assert_eq!(next, 3);
    }

    #[test]
    fn test_build_env_map() {
        let fields = vec!["x".to_string(), "y".to_string()];
        let params = vec![ValueId(10), ValueId(20)];
        let env = NormalizedHelperBox::build_env_map(&fields, &params);
        assert_eq!(env.get("x"), Some(&ValueId(10)));
        assert_eq!(env.get("y"), Some(&ValueId(20)));
    }

    #[test]
    fn test_collect_env_args() {
        let mut env = BTreeMap::new();
        env.insert("a".to_string(), ValueId(1));
        env.insert("b".to_string(), ValueId(2));
        let fields = vec!["a".to_string(), "b".to_string()];
        let args = NormalizedHelperBox::collect_env_args(&fields, &env).unwrap();
        assert_eq!(args, vec![ValueId(1), ValueId(2)]);
    }

    #[test]
    fn test_is_bool_true_literal() {
        let true_lit = ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        assert!(NormalizedHelperBox::is_bool_true_literal(&true_lit));

        let false_lit = ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(false),
            span: Span::unknown(),
        };
        assert!(!NormalizedHelperBox::is_bool_true_literal(&false_lit));
    }

    #[test]
    fn test_alloc_env_params_param_region() {
        let fields = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let (params, next_local) = NormalizedHelperBox::alloc_env_params_param_region(&fields);

        // Params should be in Param region [100, 103)
        assert_eq!(params, vec![ValueId(100), ValueId(101), ValueId(102)]);
        // next_local should be at LOCAL_MIN (1000)
        assert_eq!(next_local, 1000);
    }

    #[test]
    fn test_alloc_env_params_param_region_empty() {
        let fields: Vec<String> = vec![];
        let (params, next_local) = NormalizedHelperBox::alloc_env_params_param_region(&fields);

        assert!(params.is_empty());
        assert_eq!(next_local, 1000);
    }
}
