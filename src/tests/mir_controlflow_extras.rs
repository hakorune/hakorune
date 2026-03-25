#[cfg(test)]
mod tests {
    use crate::backend::VM;
    use crate::mir::function::MirFunction;
    use crate::mir::types::ConstValue;
    use crate::mir::BasicBlockId;
    use crate::mir::{MirInstruction, ValueId};
    use crate::parser::NyashParser;

    fn run(code: &str) -> String {
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");
        let mut vm = VM::new();
        let out = vm.execute_module(&result.module).expect("vm exec");
        out.to_string_box().value
    }

    #[test]
    fn phi_merge_then_only_assignment() {
        let code = r#"
        local x = 5
        if 1 < 2 { x = 7 } else { }
        return x
        "#;
        assert_eq!(run(code), "7");
    }

    #[test]
    fn phi_merge_else_only_assignment() {
        let code = r#"
        local y = 5
        if 2 < 1 { y = 7 } else { }
        return y
        "#;
        assert_eq!(run(code), "5");
    }

    #[test]
    fn shortcircuit_and_skips_rhs_side_effect() {
        let code = r#"
        local x = 0
        ((x = x + 1) < 0) && ((x = x + 1) < 0)
        return x
        "#;
        // LHS false ⇒ RHS not evaluated ⇒ x == 1
        assert_eq!(run(code), "1");
    }

    #[test]
    fn shortcircuit_or_skips_rhs_side_effect() {
        let code = r#"
        local x = 0
        ((x = x + 1) >= 0) || ((x = x + 1) < 0)
        return x
        "#;
        // LHS true ⇒ RHS not evaluated ⇒ x == 1
        assert_eq!(run(code), "1");
    }

    #[test]
    fn nested_loops_break_continue_mixed() {
        let code = r#"
        local i = 0
        local s = 0
        loop(i < 3) {
          local j = 0
          loop(j < 4) {
            j = j + 1
            if j == 1 { continue }
            if j == 3 { break }
            s = s + 1
          }
          i = i + 1
        }
        return s
        "#;
        // For each i: j=1 continue (skip s), j=2 => s++, then j=3 break ⇒ s increments once per outer iter ⇒ 3
        assert_eq!(run(code), "3");
    }

    /// Helper: check if a ValueId is defined as Bool const in the given block
    fn is_bool_const_in_block(func: &MirFunction, bb: BasicBlockId, val: ValueId) -> bool {
        if let Some(block) = func.get_block(bb) {
            for inst in &block.instructions {
                if let MirInstruction::Const { dst, value } = inst {
                    if *dst == val {
                        return matches!(value, ConstValue::Bool(_));
                    }
                }
            }
        }
        false
    }

    /// Phase 29bq+ Option 2: Verify inner join PHI is eliminated
    /// Old structure had a 2-input PHI where both inputs were Bool consts (rhs_join).
    /// New 3-exit structure should have no such PHI.
    #[test]
    fn shortcircuit_no_inner_join_phi() {
        let code = r#"
        local x = 0
        local r = (x == 1) && (x == 2)
        return r
        "#;
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");

        // Find main function
        let main_fn = result.module.get_function("main").expect("main function");

        // Check: no 2-input PHI where both inputs are Bool consts
        // (旧実装の rhs_join PHI はこのパターンに該当して落ちる)
        for block in main_fn.blocks.iter() {
            for inst in &block.instructions {
                if let MirInstruction::Phi { inputs, .. } = inst {
                    if inputs.len() == 2 {
                        let (pred0, val0) = inputs[0];
                        let (pred1, val1) = inputs[1];
                        let both_bool_const = is_bool_const_in_block(main_fn, pred0, val0)
                            && is_bool_const_in_block(main_fn, pred1, val1);
                        assert!(
                            !both_bool_const,
                            "Found 2-input PHI with both Bool consts (inner join still exists)"
                        );
                    }
                }
            }
        }
    }
}
