use super::WasmCodegen;
use super::WasmError;
use crate::backend::wasm::extern_contract::{extern_import_name, supported_extern_calls_csv};
use crate::mir::MirInstruction;
use crate::mir::{BinaryOp, CompareOp, ConstValue, ValueId};

impl WasmCodegen {
    /// Generate WASM instructions for a single MIR instruction
    pub(crate) fn generate_instruction(
        &mut self,
        instruction: &MirInstruction,
    ) -> Result<Vec<String>, WasmError> {
        match instruction {
            // Phase 8.2 PoC1: Basic operations
            MirInstruction::Const { dst, value } => self.generate_const(*dst, value),

            MirInstruction::BinOp { dst, op, lhs, rhs } => {
                self.generate_binop(*dst, *op, *lhs, *rhs)
            }

            MirInstruction::Compare { dst, op, lhs, rhs } => {
                self.generate_compare(*dst, *op, *lhs, *rhs)
            }

            MirInstruction::Return { value } => self.generate_return(value.as_ref()),

            // Phase 8.3 PoC2: Reference operations
            MirInstruction::RefNew { dst, box_val } => {
                // Create a new reference to a Box by copying the Box value
                // This assumes box_val contains a Box pointer already
                Ok(vec![
                    format!("local.get ${}", self.get_local_index(*box_val)?),
                    format!("local.set ${}", self.get_local_index(*dst)?),
                ])
            }

            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } => {
                // Create a new Box using the generic allocator
                match box_type.as_str() {
                    "DataBox" => {
                        // Use specific allocator for known types
                        let mut instructions = vec![
                            "call $alloc_databox".to_string(),
                            format!("local.set ${}", self.get_local_index(*dst)?),
                        ];

                        // Initialize fields with arguments if provided
                        for (i, arg) in args.iter().enumerate() {
                            instructions.extend(vec![
                                format!("local.get ${}", self.get_local_index(*dst)?),
                                format!("i32.const {}", 12 + i * 4), // Field offset
                                "i32.add".to_string(),
                                format!("local.get ${}", self.get_local_index(*arg)?),
                                "i32.store".to_string(),
                            ]);
                        }

                        Ok(instructions)
                    }
                    _ => {
                        // Use generic allocator for unknown types
                        // This is a fallback - in a real implementation, all Box types should be known
                        Ok(vec![
                            "i32.const 8192".to_string(), // Default unknown type ID
                            format!("i32.const {}", args.len()),
                            "call $box_alloc".to_string(),
                            format!("local.set ${}", self.get_local_index(*dst)?),
                        ])
                    }
                }
            }

            // Phase 8.4 PoC3: Extension stubs
            MirInstruction::WeakRef {
                dst,
                op: crate::mir::WeakRefOp::New,
                value: box_val,
            }
            | MirInstruction::FutureNew {
                dst,
                value: box_val,
            } => {
                // Treat as regular reference for now
                Ok(vec![
                    format!("local.get ${}", self.get_local_index(*box_val)?),
                    format!("local.set ${}", self.get_local_index(*dst)?),
                ])
            }

            MirInstruction::Await {
                dst,
                future: weak_ref,
            } => {
                // Always succeed for now
                Ok(vec![
                    format!("local.get ${}", self.get_local_index(*weak_ref)?),
                    format!("local.set ${}", self.get_local_index(*dst)?),
                ])
            }

            MirInstruction::FutureSet { .. } | MirInstruction::Safepoint => {
                // Lower to no code in WAT (canonical no-op).
                Ok(Vec::new())
            }

            // WSM-02a: SSA/local path unblock
            MirInstruction::Copy { dst, src } => Ok(vec![
                format!("local.get ${}", self.get_local_index(*src)?),
                format!("local.set ${}", self.get_local_index(*dst)?),
            ]),

            MirInstruction::ReleaseStrong { .. } | MirInstruction::KeepAlive { .. } => {
                // Current WASM backend does not model runtime RC/GC semantics yet.
                // Keep fail-fast for unsupported mutating ops, but treat release/liveness hints as no-op.
                Ok(Vec::new())
            }

            // Control Flow Instructions (Critical for loops and conditions)
            MirInstruction::Jump { target, .. } => {
                // Unconditional jump to target basic block
                // Use WASM br instruction to break to the target block
                Ok(vec![format!("br $block_{}", target.as_u32())])
            }

            MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                ..
            } => {
                // Conditional branch based on condition value
                // Load condition value and branch accordingly
                Ok(vec![
                    // Load condition value onto stack
                    format!("local.get ${}", self.get_local_index(*condition)?),
                    // If condition is true (non-zero), branch to then_bb
                    format!("br_if $block_{}", then_bb.as_u32()),
                    // Otherwise, fall through to else_bb
                    format!("br $block_{}", else_bb.as_u32()),
                ])
            }

            // Phase 9.7: External Function Calls (canonical Call + Callee::Extern)
            MirInstruction::Call {
                dst,
                callee: Some(crate::mir::Callee::Extern(extern_name)),
                args,
                ..
            } => {
                // Generate call to external function import
                let call_target = extern_import_name(extern_name).ok_or_else(|| {
                    WasmError::UnsupportedInstruction(format!(
                        "Unsupported extern call: {} (supported: {})",
                        extern_name,
                        supported_extern_calls_csv()
                    ))
                })?;

                let mut instructions = Vec::new();

                // Load all arguments onto stack in order
                for arg in args {
                    instructions.push(format!("local.get ${}", self.get_local_index(*arg)?));
                }

                // Call the external function
                instructions.push(format!("call ${}", call_target));

                // Store result if destination is provided
                if let Some(dst) = dst {
                    // For void functions, we still need to provide a dummy value
                    instructions.push("i32.const 0".to_string()); // Void result
                    instructions.push(format!("local.set ${}", self.get_local_index(*dst)?));
                }

                Ok(instructions)
            }

            // Global function call codegen (canonical Call + Callee::Global)
            MirInstruction::Call {
                dst,
                callee: Some(crate::mir::Callee::Global(func_name)),
                args,
                ..
            } => {
                let expected_params =
                    self.get_function_param_count(func_name).ok_or_else(|| {
                        WasmError::UnsupportedInstruction(format!(
                            "Unsupported global call: {} (supported: {})",
                            func_name,
                            self.supported_global_calls_csv()
                        ))
                    })?;

                if args.len() > expected_params {
                    return Err(WasmError::UnsupportedInstruction(format!(
                        "Global call arity mismatch: {} expects <= {} args, got {}",
                        func_name,
                        expected_params,
                        args.len()
                    )));
                }

                let mut instructions = Vec::new();
                for arg in args {
                    instructions.push(format!("local.get ${}", self.get_local_index(*arg)?));
                }
                for _ in args.len()..expected_params {
                    instructions.push("i32.const 0".to_string());
                }
                instructions.push(format!("call ${}", func_name));

                let has_return = self.function_has_return_value(func_name)?;
                match (dst, has_return) {
                    (Some(dst), true) => {
                        instructions.push(format!("local.set ${}", self.get_local_index(*dst)?));
                    }
                    (Some(dst), false) => {
                        instructions.push("i32.const 0".to_string());
                        instructions.push(format!("local.set ${}", self.get_local_index(*dst)?));
                    }
                    (None, true) => {
                        instructions.push("drop".to_string());
                    }
                    (None, false) => {}
                }

                Ok(instructions)
            }

            // Method call codegen (canonical Call + Callee::Method with receiver)
            MirInstruction::Call {
                dst,
                callee:
                    Some(crate::mir::Callee::Method {
                        receiver: Some(box_val),
                        method,
                        ..
                    }),
                args,
                ..
            } => self.generate_box_call(*dst, *box_val, method, args),

            // Unsupported instructions
            _ => Err(WasmError::UnsupportedInstruction(format!(
                "Instruction not yet supported: {:?}",
                instruction
            ))),
        }
    }

    /// Generate constant loading
    fn generate_const(
        &mut self,
        dst: ValueId,
        value: &ConstValue,
    ) -> Result<Vec<String>, WasmError> {
        let const_instruction = match value {
            ConstValue::Integer(n) => format!("i32.const {}", n),
            ConstValue::Bool(b) => format!("i32.const {}", if *b { 1 } else { 0 }),
            ConstValue::Null | ConstValue::Void => "i32.const 0".to_string(),
            ConstValue::String(s) => {
                // Register the string literal and get its offset
                let data_offset = self.register_string_literal(s);
                let string_len = s.len() as u32;

                // Generate code to allocate a StringBox and return its pointer
                // This is more complex and will need StringBox allocation
                return self.generate_string_box_const(dst, data_offset, string_len);
            }
            _ => {
                return Err(WasmError::UnsupportedInstruction(format!(
                    "Unsupported constant type: {:?}",
                    value
                )))
            }
        };

        Ok(vec![
            const_instruction,
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate binary operation
    fn generate_binop(
        &self,
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<Vec<String>, WasmError> {
        let wasm_op = match op {
            BinaryOp::Add => "i32.add",
            BinaryOp::Sub => "i32.sub",
            BinaryOp::Mul => "i32.mul",
            BinaryOp::Div => "i32.div_s",
            BinaryOp::And => "i32.and",
            BinaryOp::Or => "i32.or",
            _ => {
                return Err(WasmError::UnsupportedInstruction(format!(
                    "Unsupported binary operation: {:?}",
                    op
                )))
            }
        };

        Ok(vec![
            format!("local.get ${}", self.get_local_index(lhs)?),
            format!("local.get ${}", self.get_local_index(rhs)?),
            wasm_op.to_string(),
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate comparison operation
    fn generate_compare(
        &self,
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<Vec<String>, WasmError> {
        let wasm_op = match op {
            CompareOp::Eq => "i32.eq",
            CompareOp::Ne => "i32.ne",
            CompareOp::Lt => "i32.lt_s",
            CompareOp::Le => "i32.le_s",
            CompareOp::Gt => "i32.gt_s",
            CompareOp::Ge => "i32.ge_s",
        };

        Ok(vec![
            format!("local.get ${}", self.get_local_index(lhs)?),
            format!("local.get ${}", self.get_local_index(rhs)?),
            wasm_op.to_string(),
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate return instruction
    fn generate_return(&self, value: Option<&ValueId>) -> Result<Vec<String>, WasmError> {
        if let Some(value_id) = value {
            Ok(vec![
                format!("local.get ${}", self.get_local_index(*value_id)?),
                "return".to_string(),
            ])
        } else {
            Ok(vec!["return".to_string()])
        }
    }

    /// Generate StringBox allocation for a string constant
    fn generate_string_box_const(
        &self,
        dst: ValueId,
        data_offset: u32,
        string_len: u32,
    ) -> Result<Vec<String>, WasmError> {
        // Allocate a StringBox using the StringBox allocator
        // StringBox layout: [type_id:0x1001][ref_count:1][field_count:2][data_ptr:offset][length:len]
        Ok(vec![
            // Call StringBox allocator function
            "call $alloc_stringbox".to_string(),
            // Store the result (StringBox pointer) in local variable
            format!("local.set ${}", self.get_local_index(dst)?),
            // Initialize StringBox fields
            // Get StringBox pointer back
            format!("local.get ${}", self.get_local_index(dst)?),
            // Set data_ptr field (offset 12 from StringBox pointer)
            "i32.const 12".to_string(),
            "i32.add".to_string(),
            format!("i32.const {}", data_offset),
            "i32.store".to_string(),
            // Get StringBox pointer again
            format!("local.get ${}", self.get_local_index(dst)?),
            // Set length field (offset 16 from StringBox pointer)
            "i32.const 16".to_string(),
            "i32.add".to_string(),
            format!("i32.const {}", string_len),
            "i32.store".to_string(),
        ])
    }
}
