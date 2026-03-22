use super::super::{MirInterpreter, VMError};
use crate::mir::BasicBlock;
use crate::mir::{MirFunction, MirInstruction, ValueId};

impl MirInterpreter {
    fn vm_contract_preflight_enabled() -> bool {
        if !crate::config::env::using_is_dev() {
            return false;
        }
        if !crate::config::env::stageb_dev_verify_enabled() {
            return false;
        }
        if !crate::config::env::joinir_dev::planner_required_enabled() {
            return false;
        }
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled()
    }

    fn preflight_fail_fast_vm_contract_allowlist(&self, func: &MirFunction) -> Result<(), VMError> {
        for (bb_id, block) in &func.blocks {
            let phi_count = block.phi_instructions().count();
            for (inst_idx, sp) in block.iter_spanned_enumerated().skip(phi_count) {
                if let Some(inst_tag) =
                    crate::mir::contracts::backend_core_ops::lowered_away_tag(sp.inst)
                {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[vm/preflight:unsupported_instruction] fn={} bb={:?} inst_idx={} inst={} reason=lowered_away",
                            func.signature.name, bb_id, inst_idx, inst_tag
                        ));
                    }
                    return Err(VMError::InvalidInstruction(format!(
                        "[freeze:contract][vm/preflight:unsupported_instruction] fn={} bb={:?} inst_idx={} inst={} reason=lowered_away",
                        func.signature.name, bb_id, inst_idx, inst_tag
                    )));
                }
                if let Some(reason) =
                    crate::mir::contracts::backend_core_ops::legacy_callsite_reject_code(sp.inst)
                {
                    let inst_tag =
                        crate::mir::contracts::backend_core_ops::instruction_tag(sp.inst);
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[vm/preflight:callsite] fn={} bb={:?} inst_idx={} inst={} reason={}",
                            func.signature.name, bb_id, inst_idx, inst_tag, reason
                        ));
                    }
                    return Err(VMError::InvalidInstruction(format!(
                        "[freeze:contract][vm/preflight:callsite] fn={} bb={:?} inst_idx={} inst={} reason={}",
                        func.signature.name, bb_id, inst_idx, inst_tag, reason
                    )));
                }
                if crate::mir::contracts::backend_core_ops::is_supported_vm_instruction(sp.inst) {
                    continue;
                }
                let inst_tag = crate::mir::contracts::backend_core_ops::instruction_tag(sp.inst);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[vm/preflight:unsupported_instruction] fn={} bb={:?} inst_idx={} inst={}",
                        func.signature.name, bb_id, inst_idx, inst_tag
                    ));
                }
                return Err(VMError::InvalidInstruction(format!(
                    "[freeze:contract][vm/preflight:unsupported_instruction] fn={} bb={:?} inst_idx={} inst={}",
                    func.signature.name, bb_id, inst_idx, inst_tag
                )));
            }

            if let Some(term) = &block.terminator {
                if crate::mir::contracts::backend_core_ops::is_supported_vm_terminator(term) {
                    continue;
                }
                let inst_tag = crate::mir::contracts::backend_core_ops::instruction_tag(term);
                let term_idx = block.instructions.len();
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[vm/preflight:unsupported_terminator] fn={} bb={:?} inst_idx={} inst={}",
                        func.signature.name, bb_id, term_idx, inst_tag
                    ));
                }
                return Err(VMError::InvalidInstruction(format!(
                    "[freeze:contract][vm/preflight:unsupported_terminator] fn={} bb={:?} inst_idx={} inst={}",
                    func.signature.name, bb_id, term_idx, inst_tag
                )));
            }
        }
        Ok(())
    }

    pub(crate) fn lookup_span_for_inst(
        &self,
        block: &BasicBlock,
        inst_index: Option<usize>,
    ) -> Option<crate::ast::Span> {
        let idx = inst_index?;
        if idx < block.instructions.len() {
            block.instruction_with_span(idx).map(|sp| sp.span)
        } else if idx == block.instructions.len() {
            block.terminator_spanned().map(|sp| sp.span)
        } else {
            None
        }
    }

    pub(crate) fn preflight_fail_fast_phi_undefined_if_enabled(
        &mut self,
        func: &MirFunction,
    ) -> Result<(), VMError> {
        // Restrict to strict/dev + planner_required gates; release behavior remains unchanged.
        if !Self::vm_contract_preflight_enabled() {
            return Ok(());
        }

        if self.preflight_checked_fns.contains(&func.signature.name) {
            return Ok(());
        }
        self.preflight_checked_fns
            .insert(func.signature.name.clone());

        self.preflight_fail_fast_vm_contract_allowlist(func)?;

        let mut verifier = crate::mir::verification::MirVerifier::new();
        let Err(errors) = verifier.verify_function(func) else {
            return Ok(());
        };

        for e in errors {
            match e {
                crate::mir::verification_types::VerificationError::UndefinedValue {
                    value,
                    block,
                    instruction_index,
                } => {
                    let Some(bb) = func.blocks.get(&block) else {
                        continue;
                    };
                    let Some((_idx, sp)) = bb
                        .all_spanned_instructions_enumerated()
                        .nth(instruction_index)
                    else {
                        continue;
                    };
                    if !matches!(sp.inst, MirInstruction::Phi { .. }) {
                        continue;
                    }
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[mir/verify:phi_undefined] fn={} bb={:?} inst_idx={} used={:?} inst={:?}",
                            func.signature.name, block, instruction_index, value, sp.inst
                        ));
                    }
                    return Err(VMError::InvalidValue(format!(
                        "[freeze:contract][mir/verify:phi_undefined] fn={} bb={:?} inst_idx={} used={:?}",
                        func.signature.name, block, instruction_index, value
                    )));
                }
                crate::mir::verification_types::VerificationError::MultipleDefinition {
                    value,
                    first_block,
                    second_block,
                } => {
                    // Add concrete evidence (def instructions) to cut diagnosis distance.
                    let mut first_def = String::new();
                    let mut second_def = String::new();
                    let mut first_span = None;
                    let mut second_span = None;
                    if let Some(bb) = func.blocks.get(&first_block) {
                        for (inst_idx, sp) in bb.all_spanned_instructions_enumerated() {
                            if sp.inst.dst_value() == Some(value) {
                                first_def = format!(
                                    " first_inst_idx={} first_inst={:?}",
                                    inst_idx, sp.inst
                                );
                                first_span = Some(sp.span);
                                break;
                            }
                        }
                    }
                    if let Some(bb) = func.blocks.get(&second_block) {
                        for (inst_idx, sp) in bb.all_spanned_instructions_enumerated() {
                            if sp.inst.dst_value() == Some(value) {
                                second_def = format!(
                                    " second_inst_idx={} second_inst={:?}",
                                    inst_idx, sp.inst
                                );
                                second_span = Some(sp.span);
                                break;
                            }
                        }
                    }
                    let value_caller = func
                        .metadata
                        .value_origin_callers
                        .get(&value)
                        .map(|s| s.as_str())
                        .unwrap_or("none");
                    let mut evidence =
                        format!("{}{} value_caller={}", first_def, second_def, value_caller);
                    if let Some(span) = first_span {
                        evidence.push_str(&format!(" first_span={}", span.location_string()));
                    }
                    if let Some(span) = second_span {
                        evidence.push_str(&format!(" second_span={}", span.location_string()));
                    }
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[mir/verify:multiple_definition] fn={} value={:?} first_block={:?} second_block={:?}{}",
                            func.signature.name, value, first_block, second_block, evidence
                        ));
                    }
                    return Err(VMError::InvalidValue(format!(
                        "[freeze:contract][mir/verify:multiple_definition] fn={} value={:?} first_block={:?} second_block={:?}{}",
                        func.signature.name, value, first_block, second_block, evidence
                    )));
                }
                crate::mir::verification_types::VerificationError::DominatorViolation {
                    value,
                    use_block,
                    def_block,
                } => {
                    // Provide concrete evidence (specific use + def instructions) to cut diagnosis distance.
                    // `DominatorViolation` itself doesn't carry inst_idx; we derive it here (dev-only).
                    let mut evidence = String::new();
                    let mut is_phi_input = false;
                    let mut phi_dst = None;
                    let mut inst_found = false;
                    let mut copy_dst = None;
                    // Prefer identifying the exact Phi that consumes (use_block,value) as an incoming.
                    'phi_search: for (bb_id, bb) in &func.blocks {
                        for (inst_idx, sp) in bb.all_spanned_instructions_enumerated() {
                            if let MirInstruction::Phi { dst, inputs, .. } = &sp.inst {
                                if inputs
                                    .iter()
                                    .any(|(pred, v)| *pred == use_block && *v == value)
                                {
                                    is_phi_input = true;
                                    phi_dst = Some(*dst);
                                    evidence = format!(
                                        " kind=phi_input phi_block={:?} phi_inst_idx={} phi_dst={:?} phi_inst={:?}",
                                        bb_id, inst_idx, dst, sp.inst
                                    );
                                    break 'phi_search;
                                }
                            }
                        }
                    }
                    // Otherwise, try to find the first non-Phi instruction that uses the value in use_block.
                    if !is_phi_input {
                        if let Some(bb) = func.blocks.get(&use_block) {
                            for (inst_idx, sp) in bb.all_spanned_instructions_enumerated() {
                                if matches!(sp.inst, MirInstruction::Phi { .. }) {
                                    continue;
                                }
                                if sp.inst.used_values().iter().any(|v| *v == value) {
                                    // If this is a LocalSSA Copy, also show the first instruction that consumes
                                    // the copied dst. This helps identify the "real" use site (e.g., Branch cond).
                                    let mut copy_consumer = String::new();
                                    if let MirInstruction::Copy {
                                        dst: copy_dst_val, ..
                                    } = &sp.inst
                                    {
                                        copy_dst = Some(*copy_dst_val);
                                        for (use2_idx, sp2) in
                                            bb.all_spanned_instructions_enumerated()
                                        {
                                            if use2_idx <= inst_idx {
                                                continue;
                                            }
                                            if sp2
                                                .inst
                                                .used_values()
                                                .iter()
                                                .any(|v| *v == *copy_dst_val)
                                            {
                                                copy_consumer = format!(
                                                    " copy_used_by_idx={} copy_used_by={:?}",
                                                    use2_idx, sp2.inst
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    evidence = format!(
                                        " kind=inst inst_idx={} inst={:?}{}",
                                        inst_idx, sp.inst, copy_consumer
                                    );
                                    inst_found = true;
                                    break;
                                }
                            }
                        }
                        if inst_found {
                            let value_caller = func
                                .metadata
                                .value_origin_callers
                                .get(&value)
                                .map(|s| s.as_str())
                                .unwrap_or("none");
                            evidence.push_str(&format!(" value_caller={}", value_caller));
                            if let Some(dst) = copy_dst {
                                let copy_dst_caller = func
                                    .metadata
                                    .value_origin_callers
                                    .get(&dst)
                                    .map(|s| s.as_str())
                                    .unwrap_or("none");
                                evidence.push_str(&format!(" copy_dst_caller={}", copy_dst_caller));
                            }
                        }
                    }

                    if is_phi_input {
                        let phi_dst_caller = phi_dst
                            .and_then(|dst| func.metadata.value_origin_callers.get(&dst))
                            .map(|s| s.as_str())
                            .unwrap_or("none");
                        let bad_in_caller = func
                            .metadata
                            .value_origin_callers
                            .get(&value)
                            .map(|s| s.as_str())
                            .unwrap_or("none");
                        evidence.push_str(&format!(
                            " phi_dst_caller={} bad_in_caller={}",
                            phi_dst_caller, bad_in_caller
                        ));
                    }

                    // Also try to show where this ValueId was defined (within def_block).
                    if let Some(bb) = func.blocks.get(&def_block) {
                        for (inst_idx, sp) in bb.all_spanned_instructions_enumerated() {
                            if sp.inst.dst_value() == Some(value) {
                                // If the defining instruction is a Compare/BinOp, try to decode literal operands
                                // (when they are Const-defined) to pinpoint which source-level condition triggered.
                                let mut def_operand_consts = String::new();
                                let find_const = |vid: ValueId| -> Option<crate::mir::ConstValue> {
                                    for bb in func.blocks.values() {
                                        for inst in bb.all_instructions() {
                                            if let MirInstruction::Const { dst, value } = inst {
                                                if *dst == vid {
                                                    return Some(value.clone());
                                                }
                                            }
                                        }
                                    }
                                    None
                                };

                                match &sp.inst {
                                    MirInstruction::Compare { lhs, rhs, .. } => {
                                        if let Some(v) = find_const(*lhs) {
                                            def_operand_consts
                                                .push_str(&format!(" def_cmp_lhs_const={:?}", v));
                                        }
                                        if let Some(v) = find_const(*rhs) {
                                            def_operand_consts
                                                .push_str(&format!(" def_cmp_rhs_const={:?}", v));
                                        }
                                    }
                                    MirInstruction::BinOp { lhs, rhs, .. } => {
                                        if let Some(v) = find_const(*lhs) {
                                            def_operand_consts
                                                .push_str(&format!(" def_bin_lhs_const={:?}", v));
                                        }
                                        if let Some(v) = find_const(*rhs) {
                                            def_operand_consts
                                                .push_str(&format!(" def_bin_rhs_const={:?}", v));
                                        }
                                    }
                                    _ => {}
                                }

                                evidence.push_str(&format!(
                                    " def_inst_idx={} def_inst={:?}{}",
                                    inst_idx, sp.inst, def_operand_consts
                                ));
                                break;
                            }
                        }
                    }

                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[mir/verify:dominator_violation] fn={} use_block={:?} def_block={:?} value={:?}{}",
                            func.signature.name, use_block, def_block, value, evidence
                        ));
                    }
                    return Err(VMError::InvalidValue(format!(
                        "[freeze:contract][mir/verify:dominator_violation] fn={} use_block={:?} def_block={:?} value={:?}{}",
                        func.signature.name, use_block, def_block, value, evidence
                    )));
                }
                crate::mir::verification_types::VerificationError::InvalidPhi {
                    phi_value,
                    block,
                    reason,
                } => {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[mir/verify:invalid_phi] fn={} bb={:?} phi={:?} reason={}",
                            func.signature.name, block, phi_value, reason
                        ));
                    }
                    return Err(VMError::InvalidValue(format!(
                        "[freeze:contract][mir/verify:invalid_phi] fn={} bb={:?} phi={:?} reason={}",
                        func.signature.name, block, phi_value, reason
                    )));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId,
    };
    use std::sync::{Mutex, OnceLock};

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (k, v) in vars {
                saved.push((*k, std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, old) in self.saved.drain(..) {
                if let Some(v) = old {
                    std::env::set_var(k, v);
                } else {
                    std::env::remove_var(k);
                }
            }
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn build_unsupported_throw_fixture() -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.main/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(signature, entry);
        let block = func.blocks.get_mut(&entry).expect("entry block must exist");
        let thrown = ValueId::new(1);
        block.add_instruction(MirInstruction::Const {
            dst: thrown,
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Throw {
            exception: thrown,
            effects: EffectMask::PANIC,
        });
        func
    }

    fn build_legacy_call_without_callee_fixture() -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.main/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(signature, entry);
        let block = func.blocks.get_mut(&entry).expect("entry block must exist");
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::new(1),
            callee: None,
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        func
    }

    fn build_closure_call_with_runtime_args_fixture() -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.main/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(signature, entry);
        let block = func.blocks.get_mut(&entry).expect("entry block must exist");
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![],
                me_capture: None,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        func
    }

    #[test]
    fn vm_preflight_rejects_unsupported_throw_under_strict_gate() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "1"),
            ("HAKO_JOINIR_PLANNER_REQUIRED", "1"),
            ("NYASH_USING_PROFILE", "dev"),
            ("NYASH_STAGEB_DEV_VERIFY", "1"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
        ]);

        let mut interp = MirInterpreter::new();
        let func = build_unsupported_throw_fixture();

        let err = interp
            .preflight_fail_fast_phi_undefined_if_enabled(&func)
            .expect_err("strict/dev preflight must reject unsupported Throw");

        let msg = match err {
            VMError::InvalidInstruction(msg) => msg,
            other => panic!("unexpected error kind: {}", other),
        };

        assert!(
            msg.contains("[freeze:contract][vm/preflight:unsupported_terminator]"),
            "missing freeze tag: {}",
            msg
        );
        assert!(
            !msg.contains("reason=lowered_away"),
            "unexpected lowered-away reason: {}",
            msg
        );
        assert!(
            msg.contains("inst=Throw"),
            "missing instruction tag: {}",
            msg
        );
    }

    #[test]
    fn vm_preflight_rejects_legacy_call_without_callee_under_strict_gate() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "1"),
            ("HAKO_JOINIR_PLANNER_REQUIRED", "1"),
            ("NYASH_USING_PROFILE", "dev"),
            ("NYASH_STAGEB_DEV_VERIFY", "1"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
        ]);

        let mut interp = MirInterpreter::new();
        let func = build_legacy_call_without_callee_fixture();

        let err = interp
            .preflight_fail_fast_phi_undefined_if_enabled(&func)
            .expect_err("strict/dev preflight must reject legacy call without callee");

        let msg = match err {
            VMError::InvalidInstruction(msg) => msg,
            other => panic!("unexpected error kind: {}", other),
        };

        assert!(
            msg.contains("[freeze:contract][vm/preflight:callsite]"),
            "missing callsite freeze tag: {}",
            msg
        );
        assert!(
            msg.contains("reason=call-missing-callee"),
            "missing callsite reason: {}",
            msg
        );
        assert!(
            msg.contains("inst=Call"),
            "missing instruction tag: {}",
            msg
        );
    }

    #[test]
    fn vm_preflight_rejects_closure_call_with_runtime_args_under_strict_gate() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "1"),
            ("HAKO_JOINIR_PLANNER_REQUIRED", "1"),
            ("NYASH_USING_PROFILE", "dev"),
            ("NYASH_STAGEB_DEV_VERIFY", "1"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
        ]);

        let mut interp = MirInterpreter::new();
        let func = build_closure_call_with_runtime_args_fixture();

        let err = interp
            .preflight_fail_fast_phi_undefined_if_enabled(&func)
            .expect_err("strict/dev preflight must reject closure call with runtime args");

        let msg = match err {
            VMError::InvalidInstruction(msg) => msg,
            other => panic!("unexpected error kind: {}", other),
        };

        assert!(
            msg.contains("[freeze:contract][vm/preflight:callsite]"),
            "missing callsite freeze tag: {}",
            msg
        );
        assert!(
            msg.contains("reason=call-closure-runtime-args"),
            "missing closure callsite reason: {}",
            msg
        );
        assert!(
            msg.contains("inst=Call"),
            "missing instruction tag: {}",
            msg
        );
    }
}
