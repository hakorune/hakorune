//! Trim JoinIR Lowering
//!
//! Handles JoinIR-specific lowering for Trim routes.
//! Responsible for:
//! - Trim break condition generation
//! - Carrier binding setup in ConditionEnv

use crate::ast::{ASTNode, Span, UnaryOperator};
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper;
use crate::mir::ValueId;

pub(in crate::mir::builder) struct TrimLowerer;

impl TrimLowerer {
    /// Generate Trim-specific JoinIR break condition
    ///
    /// Replaces: break on (ch == " " || ...)
    /// With: break on !is_carrier
    ///
    /// # Arguments
    ///
    /// * `trim_helper` - The TrimLoopHelper containing pattern info
    ///
    /// # Returns
    ///
    /// AST node representing the negated carrier check
    pub(in crate::mir::builder) fn generate_trim_break_condition(
        trim_helper: &TrimLoopHelper,
    ) -> ASTNode {
        let carrier_var_node = ASTNode::Variable {
            name: trim_helper.carrier_name.clone(),
            span: Span::unknown(),
        };

        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(carrier_var_node),
            span: Span::unknown(),
        }
    }

    /// Setup Trim carrier in ConditionEnv
    ///
    /// Creates binding for bool carrier in JoinIR condition space
    ///
    /// # Arguments
    ///
    /// * `trim_helper` - The TrimLoopHelper containing pattern info
    /// * `get_host_value` - Closure to get host ValueId for carrier name
    /// * `alloc_join_value` - Closure to allocate new JoinIR ValueId
    ///
    /// # Returns
    ///
    /// ConditionBinding for the carrier
    pub(in crate::mir::builder) fn setup_trim_carrier_binding(
        trim_helper: &TrimLoopHelper,
        get_host_value: impl Fn(&str) -> Option<ValueId>,
        alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<ConditionBinding, String> {
        let carrier_name = &trim_helper.carrier_name;

        // Get host ValueId for carrier
        let host_value_id = get_host_value(carrier_name).ok_or_else(|| {
            format!(
                "[TrimLowerer] Carrier '{}' not in variable_map",
                carrier_name
            )
        })?;

        // Allocate JoinIR ValueId
        let joinir_value_id = alloc_join_value();

        Ok(ConditionBinding {
            name: carrier_name.clone(),
            host_value: host_value_id,
            join_value: joinir_value_id,
        })
    }

    /// Add Trim carrier to ConditionEnv
    ///
    /// Inserts the carrier into the environment and returns its binding
    ///
    /// # Arguments
    ///
    /// * `trim_helper` - The TrimLoopHelper containing pattern info
    /// * `get_host_value` - Closure to get host ValueId for carrier name
    /// * `insert_to_env` - Closure to insert binding into env
    /// * `alloc_join_value` - Closure to allocate new JoinIR ValueId
    ///
    /// # Returns
    ///
    /// ConditionBinding for the carrier
    pub(in crate::mir::builder) fn add_to_condition_env(
        trim_helper: &TrimLoopHelper,
        get_host_value: impl Fn(&str) -> Option<ValueId>,
        insert_to_env: impl FnOnce(String, ValueId),
        alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<ConditionBinding, String> {
        let binding =
            Self::setup_trim_carrier_binding(trim_helper, get_host_value, alloc_join_value)?;

        // Insert into env
        insert_to_env(binding.name.clone(), binding.join_value);

        Ok(binding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_helper() -> TrimLoopHelper {
        TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_ws".to_string(),
            whitespace_chars: vec![" ".to_string()],
        }
    }

    #[test]
    fn test_generate_trim_break_condition() {
        let helper = create_test_helper();
        let result = TrimLowerer::generate_trim_break_condition(&helper);

        // Should be UnaryOp::Not
        match result {
            ASTNode::UnaryOp {
                operator, operand, ..
            } => {
                assert_eq!(operator, UnaryOperator::Not);
                // Operand should be Variable with carrier name
                match *operand {
                    ASTNode::Variable { name, .. } => {
                        assert_eq!(name, "is_ws");
                    }
                    _ => panic!("Expected Variable node"),
                }
            }
            _ => panic!("Expected UnaryOp node"),
        }
    }

    #[test]
    fn test_setup_trim_carrier_binding() {
        use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism
        let helper = create_test_helper();
        let mut variable_map = BTreeMap::new();
        variable_map.insert("is_ws".to_string(), ValueId(42));

        let mut counter = 100u32;
        let mut alloc = || {
            let id = ValueId(counter);
            counter += 1;
            id
        };

        let get_value = |name: &str| variable_map.get(name).copied();
        let result = TrimLowerer::setup_trim_carrier_binding(&helper, get_value, &mut alloc);

        assert!(result.is_ok());
        let binding = result.unwrap();
        assert_eq!(binding.name, "is_ws");
        assert_eq!(binding.host_value, ValueId(42));
        assert_eq!(binding.join_value, ValueId(100));
    }

    #[test]
    fn test_setup_trim_carrier_binding_missing_carrier() {
        use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism
        let helper = create_test_helper();
        let variable_map: BTreeMap<String, ValueId> = BTreeMap::new(); // Empty!

        let mut counter = 100u32;
        let mut alloc = || {
            let id = ValueId(counter);
            counter += 1;
            id
        };

        let get_value = |name: &str| variable_map.get(name).copied();
        let result = TrimLowerer::setup_trim_carrier_binding(&helper, get_value, &mut alloc);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not in variable_map"));
    }

    #[test]
    fn test_add_to_condition_env() {
        use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism
        let helper = create_test_helper();
        let mut variable_map = BTreeMap::new();
        variable_map.insert("is_ws".to_string(), ValueId(42));

        let mut env = BTreeMap::new();

        let mut counter = 100u32;
        let mut alloc = || {
            let id = ValueId(counter);
            counter += 1;
            id
        };

        let get_value = |name: &str| variable_map.get(name).copied();
        let insert = |name: String, value: ValueId| {
            env.insert(name, value);
        };

        let result =
            TrimLowerer::add_to_condition_env(&helper, get_value, insert, &mut alloc);

        assert!(result.is_ok());
        let binding = result.unwrap();

        // Check that env was updated
        assert!(env.contains_key("is_ws"));
        assert_eq!(env["is_ws"], ValueId(100));

        // Check binding
        assert_eq!(binding.name, "is_ws");
        assert_eq!(binding.host_value, ValueId(42));
        assert_eq!(binding.join_value, ValueId(100));
    }
}
