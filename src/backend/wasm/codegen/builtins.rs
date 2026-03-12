use super::WasmCodegen;
use super::WasmError;
use crate::mir::ValueId;

impl WasmCodegen {
    /// Generate BoxCall method invocation
    /// Implements critical Box methods: toString, print, equals, clone
    pub(crate) fn generate_box_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        match method {
            "toString" => self.generate_to_string_call(dst, box_val),
            "print" => self.generate_print_call(dst, box_val),
            "equals" => self.generate_equals_call(dst, box_val, args),
            "clone" => self.generate_clone_call(dst, box_val),
            "log" => self.generate_log_call(dst, box_val, args),
            "info" => self.generate_info_call(dst, box_val, args),
            "debug" => self.generate_debug_call(dst, box_val, args),
            "warn" => self.generate_warn_call(dst, box_val, args),
            "error" => self.generate_error_call(dst, box_val, args),
            _ => Err(WasmError::UnsupportedInstruction(format!(
                "Unsupported BoxCall method: {} (supported: toString, print, equals, clone, log, info, debug, warn, error)",
                method
            ))),
        }
    }

    /// Generate toString() method call - Box → String conversion
    fn generate_to_string_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
    ) -> Result<Vec<String>, WasmError> {
        let Some(dst) = dst else {
            return Err(WasmError::CodegenError(
                "toString() requires destination".to_string(),
            ));
        };

        Ok(vec![
            format!(
                ";; toString() implementation for ValueId({})",
                box_val.as_u32()
            ),
            format!("local.get ${}", self.get_local_index(box_val)?),
            "call $box_to_string".to_string(),
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate print() method call - Basic output
    fn generate_print_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
    ) -> Result<Vec<String>, WasmError> {
        let mut instructions = vec![
            format!(
                ";; print() implementation for ValueId({})",
                box_val.as_u32()
            ),
            format!("local.get ${}", self.get_local_index(box_val)?),
            "call $box_print".to_string(),
        ];

        // Store void result if destination is provided
        if let Some(dst) = dst {
            instructions.extend(vec![
                "i32.const 0".to_string(), // Void result
                format!("local.set ${}", self.get_local_index(dst)?),
            ]);
        }

        Ok(instructions)
    }

    /// Generate equals() method call - Box comparison
    fn generate_equals_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        let Some(dst) = dst else {
            return Err(WasmError::CodegenError(
                "equals() requires destination".to_string(),
            ));
        };

        if args.len() != 1 {
            return Err(WasmError::CodegenError(format!(
                "equals() expects 1 argument, got {}",
                args.len()
            )));
        }

        Ok(vec![
            format!(
                ";; equals() implementation for ValueId({}) == ValueId({})",
                box_val.as_u32(),
                args[0].as_u32()
            ),
            format!("local.get ${}", self.get_local_index(box_val)?),
            format!("local.get ${}", self.get_local_index(args[0])?),
            "call $box_equals".to_string(),
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate clone() method call - Box duplication
    fn generate_clone_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
    ) -> Result<Vec<String>, WasmError> {
        let Some(dst) = dst else {
            return Err(WasmError::CodegenError(
                "clone() requires destination".to_string(),
            ));
        };

        Ok(vec![
            format!(
                ";; clone() implementation for ValueId({})",
                box_val.as_u32()
            ),
            format!("local.get ${}", self.get_local_index(box_val)?),
            "call $box_clone".to_string(),
            format!("local.set ${}", self.get_local_index(dst)?),
        ])
    }

    /// Generate log() method call - Console logging (ConsoleBox.log)
    fn generate_log_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        self.generate_console_call("log", "console_log", dst, box_val, args)
    }

    /// Generate info() method call - Console info output (ConsoleBox.info)
    fn generate_info_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        self.generate_console_call("info", "console_info", dst, box_val, args)
    }

    /// Generate debug() method call - Console debug output (ConsoleBox.debug)
    fn generate_debug_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        self.generate_console_call("debug", "console_debug", dst, box_val, args)
    }

    /// Generate warn() method call - Console warning output (ConsoleBox.warn)
    fn generate_warn_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        self.generate_console_call("warn", "console_warn", dst, box_val, args)
    }

    /// Generate error() method call - Console error output (ConsoleBox.error)
    fn generate_error_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        self.generate_console_call("error", "console_error", dst, box_val, args)
    }

    fn generate_console_call(
        &mut self,
        method_name: &str,
        target_import: &str,
        dst: Option<ValueId>,
        box_val: ValueId,
        args: &[ValueId],
    ) -> Result<Vec<String>, WasmError> {
        if args.is_empty() {
            return Err(WasmError::CodegenError(format!(
                "{}() expects at least 1 argument, got {}",
                method_name,
                args.len()
            )));
        }

        let mut instructions = vec![format!(
            ";; {}() implementation for ConsoleBox ValueId({})",
            method_name,
            box_val.as_u32(),
        )];

        // Console imports are ABI-fixed as (ptr,len), so convert argument Box to StringBox first.
        // Accept both call shapes:
        // - [message]
        // - [receiver_like, message]
        // and always treat the last argument as the message payload.
        let arg = args[args.len() - 1];
        // ptr = box_to_string(arg).data_ptr
        instructions.push(format!("local.get ${}", self.get_local_index(arg)?));
        instructions.push("call $box_to_string".to_string());
        instructions.push("i32.const 12".to_string());
        instructions.push("i32.add".to_string());
        instructions.push("i32.load".to_string());
        // len = box_to_string(arg).length
        instructions.push(format!("local.get ${}", self.get_local_index(arg)?));
        instructions.push("call $box_to_string".to_string());
        instructions.push("i32.const 16".to_string());
        instructions.push("i32.add".to_string());
        instructions.push("i32.load".to_string());

        // Call console output function
        instructions.push(format!("call ${}", target_import));

        // Store void result if destination is provided
        if let Some(dst) = dst {
            instructions.extend(vec![
                "i32.const 0".to_string(), // Void result
                format!("local.set ${}", self.get_local_index(dst)?),
            ]);
        }

        Ok(instructions)
    }
}
