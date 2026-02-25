/// CarrierInitBuilder: Centralized CarrierInit value generation
///
/// # Purpose
/// Provides single source of truth for generating MIR values from CarrierInit enum.
/// Eliminates scattered match patterns across header PHI, exit line, boundary injection.
///
/// # Phase 86 Context
/// - Consolidates ~100 lines of duplicated CarrierInit matching logic
/// - Used by: loop_header_phi_builder, exit_line/meta_collector, boundary_builder
/// - Ensures consistent const generation and debug output
///
/// # Design Principles
/// - **SSOT**: Single function for all CarrierInit → ValueId generation
/// - **Testability**: Pure function, easy to unit test
/// - **Consistency**: Uniform debug output format
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInit;
use crate::mir::value_id::ValueId;

/// Generate a ValueId for the given CarrierInit policy
///
/// # Arguments
/// * `builder` - MIR builder for emitting const instructions
/// * `init` - Initialization policy (FromHost, BoolConst, LoopLocalZero)
/// * `host_id` - Host variable's ValueId (used for FromHost)
/// * `name` - Carrier variable name (for debug output)
/// * `debug` - Enable debug output
///
/// # Returns
/// * `ValueId` - Either host_id (FromHost) or newly emitted const
///
/// # Examples
/// ```
/// // FromHost: Returns host_id directly
/// let value = init_value(&mut builder, &CarrierInit::FromHost, host_id, "counter", false);
/// // value == host_id
///
/// // BoolConst: Emits new const instruction
/// let value = init_value(&mut builder, &CarrierInit::BoolConst(true), host_id, "flag", true);
/// // Emits: %N = Const { dst: ValueId(N), value: Bool(true) }
/// // Debug: "[carrier_init_builder] 'flag': BoolConst(true) -> ValueId(N)"
///
/// // LoopLocalZero: Emits Integer(0) const
/// let value = init_value(&mut builder, &CarrierInit::LoopLocalZero, host_id, "digit", false);
/// // Emits: %N = Const { dst: ValueId(N), value: Integer(0) }
/// ```
pub fn init_value(
    builder: &mut MirBuilder,
    init: &CarrierInit,
    host_id: ValueId,
    name: &str,
    debug: bool,
) -> Result<ValueId, String> {
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    match init {
        CarrierInit::FromHost => {
            // Use host variable's ValueId directly (no const emission needed)
            if debug {
                trace.stderr_if(
                    &format!(
                        "[carrier_init_builder] '{}': FromHost -> ValueId({})",
                        name, host_id.0
                    ),
                    true,
                );
            }
            Ok(host_id)
        }
        CarrierInit::BoolConst(val) => {
            // Generate explicit bool constant (used for ConditionOnly carriers)
            let const_id = crate::mir::builder::emission::constant::emit_bool(builder, *val)?;
            if debug {
                trace.stderr_if(
                    &format!(
                        "[carrier_init_builder] '{}': BoolConst({}) -> ValueId({})",
                        name, val, const_id.0
                    ),
                    true,
                );
            }
            Ok(const_id)
        }
        CarrierInit::LoopLocalZero => {
            // Generate Integer(0) const for loop-local derived carriers (no host slot)
            let const_id = crate::mir::builder::emission::constant::emit_integer(builder, 0)?;
            if debug {
                trace.stderr_if(
                    &format!(
                        "[carrier_init_builder] '{}': LoopLocalZero -> ValueId({})",
                        name, const_id.0
                    ),
                    true,
                );
            }
            Ok(const_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test FromHost returns host_id directly without emitting instructions
    #[test]
    fn test_from_host_returns_host_id() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_from_host_returns_host_id".to_string());
        let host_id = ValueId(42);

        let result =
            init_value(&mut builder, &CarrierInit::FromHost, host_id, "test", false).unwrap();

        assert_eq!(result, host_id, "FromHost should return host_id directly");
        // No instruction should be emitted for FromHost
    }

    /// Test BoolConst(true) emits new ValueId (not host_id)
    #[test]
    fn test_bool_const_true_emits_new_value() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_bool_const_true_emits_new_value".to_string());
        let host_id = ValueId(999); // Dummy host_id (not used for BoolConst)

        let result = init_value(
            &mut builder,
            &CarrierInit::BoolConst(true),
            host_id,
            "test",
            false,
        )
        .unwrap();

        assert_ne!(result, host_id, "BoolConst should emit new ValueId");
    }

    /// Test BoolConst(false) emits new ValueId
    #[test]
    fn test_bool_const_false_emits_new_value() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_bool_const_false_emits_new_value".to_string());
        let host_id = ValueId(999);

        let result = init_value(
            &mut builder,
            &CarrierInit::BoolConst(false),
            host_id,
            "flag",
            false,
        )
        .unwrap();

        assert_ne!(result, host_id, "BoolConst should emit new ValueId");
    }

    /// Test LoopLocalZero emits Integer(0) const
    #[test]
    fn test_loop_local_zero_emits_new_value() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_loop_local_zero_emits_new_value".to_string());
        let host_id = ValueId(999);

        let result = init_value(
            &mut builder,
            &CarrierInit::LoopLocalZero,
            host_id,
            "digit",
            false,
        )
        .unwrap();

        assert_ne!(result, host_id, "LoopLocalZero should emit new ValueId");
    }

    /// Test multiple calls produce different ValueIds
    #[test]
    fn test_multiple_calls_produce_unique_values() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_multiple_calls_produce_unique_values".to_string());
        let host_id = ValueId(100);

        let result1 = init_value(
            &mut builder,
            &CarrierInit::BoolConst(true),
            host_id,
            "flag1",
            false,
        )
        .unwrap();
        let result2 = init_value(
            &mut builder,
            &CarrierInit::BoolConst(false),
            host_id,
            "flag2",
            false,
        )
        .unwrap();
        let result3 = init_value(
            &mut builder,
            &CarrierInit::LoopLocalZero,
            host_id,
            "counter",
            false,
        )
        .unwrap();

        assert_ne!(
            result1, result2,
            "Different BoolConst calls should produce different ValueIds"
        );
        assert_ne!(
            result2, result3,
            "BoolConst and LoopLocalZero should produce different ValueIds"
        );
        assert_ne!(result1, result3, "All ValueIds should be unique");
    }

    /// Test debug output doesn't crash (no assertion, just execution)
    #[test]
    fn test_debug_output_from_host() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_debug_output_from_host".to_string());
        let host_id = ValueId(42);

        let _ = init_value(
            &mut builder,
            &CarrierInit::FromHost,
            host_id,
            "debug_test",
            true,
        )
        .unwrap();
        // Expected stderr output: "[carrier_init_builder] 'debug_test': FromHost -> ValueId(42)"
        // (This test just verifies it doesn't crash)
    }

    /// Test debug output for BoolConst doesn't crash
    #[test]
    fn test_debug_output_bool_const() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_debug_output_bool_const".to_string());
        let host_id = ValueId(999);

        let _result = init_value(
            &mut builder,
            &CarrierInit::BoolConst(true),
            host_id,
            "debug_bool",
            true,
        )
        .unwrap();
        // Expected stderr output: "[carrier_init_builder] 'debug_bool': BoolConst(true) -> ValueId(N)"
        // (This test just verifies it doesn't crash)
    }

    /// Test debug output for LoopLocalZero doesn't crash
    #[test]
    fn test_debug_output_loop_local_zero() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_debug_output_loop_local_zero".to_string());
        let host_id = ValueId(999);

        let _result = init_value(
            &mut builder,
            &CarrierInit::LoopLocalZero,
            host_id,
            "debug_zero",
            true,
        )
        .unwrap();
        // Expected stderr output: "[carrier_init_builder] 'debug_zero': LoopLocalZero -> ValueId(N)"
        // (This test just verifies it doesn't crash)
    }
}
