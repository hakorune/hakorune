use super::*;

// VM dispatch trace macro (used across handlers)
macro_rules! trace_dispatch {
    ($method:expr, $handler:expr) => {
        if $method == "length" && std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[vm-trace] length dispatch handler={}", $handler));
        }
    };
}

mod arithmetic;
mod boxcall_dispatch;
mod boxcall_prelude;
mod boxes;
mod boxes_array;
mod boxes_buffer;
mod boxes_file;
mod boxes_instance;
mod boxes_map;
mod boxes_object_fields;
mod boxes_path;
mod boxes_plugin;
mod boxes_string;
mod boxes_void_guards;
mod calls;
mod extern_provider;
mod externals;
mod lifecycle;
mod memory;
mod misc;
mod string_fastpath;
mod string_method_helpers;
mod sum_bridge;
mod sum_ops;
mod temp_dispatch;
mod type_ops;
mod weak; // Phase 285A0: WeakRef handlers

#[cfg(test)]
mod async_contract_tests;

impl MirInterpreter {
    pub(super) fn execute_instruction(&mut self, inst: &MirInstruction) -> Result<(), VMError> {
        match inst {
            MirInstruction::Const { dst, value } => self.handle_const(*dst, value)?,
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } => self.handle_new_box(*dst, box_type, args)?,
            MirInstruction::BinOp { dst, op, lhs, rhs } => {
                self.handle_binop(*dst, *op, *lhs, *rhs)?
            }
            MirInstruction::UnaryOp { dst, op, operand } => {
                self.handle_unary_op(*dst, *op, *operand)?
            }
            MirInstruction::Compare { dst, op, lhs, rhs } => {
                self.handle_compare(*dst, *op, *lhs, *rhs)?
            }
            MirInstruction::TypeOp { dst, op, value, ty } => {
                self.handle_type_op(*dst, *op, *value, ty)?
            }
            MirInstruction::Copy { dst, src } => self.handle_copy(*dst, *src)?,
            MirInstruction::Load { dst, ptr } => self.handle_load(*dst, *ptr)?,
            MirInstruction::Store { ptr, value } => self.handle_store(*ptr, *value)?,
            MirInstruction::FieldGet {
                dst, base, field, ..
            } => {
                let field_id = ValueId::new(u32::MAX - 2);
                self.regs.insert(field_id, VMValue::String(field.clone()));
                let handled = boxes_object_fields::try_handle_object_fields(
                    self,
                    Some(*dst),
                    *base,
                    "getField",
                    &[field_id],
                )?;
                self.regs.remove(&field_id);
                if !handled {
                    return Err(self.err_invalid(format!(
                        "MIR interp: field.get unsupported for {}.{}",
                        base, field
                    )));
                }
            }
            MirInstruction::FieldSet {
                base, field, value, ..
            } => {
                let field_id = ValueId::new(u32::MAX - 2);
                self.regs.insert(field_id, VMValue::String(field.clone()));
                let handled = boxes_object_fields::try_handle_object_fields(
                    self,
                    None,
                    *base,
                    "setField",
                    &[field_id, *value],
                )?;
                self.regs.remove(&field_id);
                if !handled {
                    return Err(self.err_invalid(format!(
                        "MIR interp: field.set unsupported for {}.{}",
                        base, field
                    )));
                }
            }
            MirInstruction::VariantMake {
                dst,
                enum_name,
                variant,
                tag,
                payload,
                payload_type,
            } => self.handle_variant_make(
                *dst,
                enum_name,
                variant,
                *tag,
                *payload,
                payload_type.as_ref(),
            )?,
            MirInstruction::VariantTag {
                dst,
                value,
                enum_name,
            } => self.handle_variant_tag(*dst, *value, enum_name)?,
            MirInstruction::VariantProject {
                dst,
                value,
                enum_name,
                variant,
                tag,
                payload_type,
            } => self.handle_variant_project(
                *dst,
                *value,
                enum_name,
                variant,
                *tag,
                payload_type.as_ref(),
            )?,
            MirInstruction::Call {
                dst,
                func,
                callee,
                args,
                ..
            } => self.handle_call(*dst, *func, callee.as_ref(), args)?,
            MirInstruction::Debug { message, value } => {
                self.handle_debug(message, *value)?;
            }
            // Phase 256 P1.5: Select instruction (ternary conditional)
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => {
                let cond_val = self.reg_load(*cond)?;
                let is_true = cond_val.as_bool()?;
                let selected_val = if is_true {
                    self.reg_load(*then_val)?
                } else {
                    self.reg_load(*else_val)?
                };
                self.write_reg(*dst, selected_val);
            }
            // Phase 285A0: WeakRef handlers (delegated to weak.rs)
            MirInstruction::WeakRef { dst, op, value } => match op {
                WeakRefOp::New => self.handle_weak_new(*dst, *value)?,
                WeakRefOp::Load => self.handle_weak_load(*dst, *value)?,
            },
            MirInstruction::RefNew { dst, box_val } => {
                self.handle_ref_new(*dst, *box_val)?;
            }
            MirInstruction::Barrier { .. } | MirInstruction::Safepoint => {}
            MirInstruction::FutureNew { dst, value } => {
                let fut = crate::boxes::future::FutureBox::new();
                let v = self.load_as_box(*value)?;
                fut.set_result(v);
                self.write_result(Some(*dst), VMValue::Future(fut));
            }
            MirInstruction::FutureSet { future, value } => {
                let f = self.reg_load(*future)?;
                let v = self.load_as_box(*value)?;
                match f {
                    VMValue::Future(fut) => {
                        fut.set_result(v);
                    }
                    _ => {
                        return Err(VMError::TypeError(
                            "FutureSet expects Future in `future` operand".into(),
                        ));
                    }
                }
            }
            MirInstruction::Await { dst, future } => {
                let f = self.reg_load(*future)?;
                match f {
                    VMValue::Future(fut) => match fut.wait_and_get() {
                        Ok(v) => {
                            self.write_result(Some(*dst), VMValue::from_nyash_box(v));
                        }
                        Err(error) => {
                            return Err(VMError::TaskFailed(error.to_string_box().value));
                        }
                    },
                    _ => {
                        return Err(VMError::TypeError(
                            "Await expects Future in `future` operand".into(),
                        ));
                    }
                }
            }
            // Phase 287: Lifecycle management
            MirInstruction::KeepAlive { .. } => {
                // No-op: KeepAlive only affects DCE/liveness analysis
            }
            MirInstruction::ReleaseStrong { values } => {
                self.release_strong_refs(values);
            }
            other => {
                return Err(self.err_invalid(format!(
                    "MIR interp: unimplemented instruction: {:?}",
                    other
                )))
            }
        }
        Ok(())
    }
}
