//! JoinIR JSON シリアライザ (jsonir v0)
//!
//! JoinModule を JSON 形式でシリアライズする。
//! 用途: デバッグ、テスト、将来の selfhost JoinIR ロワーの参照フォーマット。
//!
//! 仕様: docs/private/roadmap2/phases/phase-30-final-joinir-world/joinir_json.md

use std::io::Write;

use super::{
    BinOpKind, CompareOp, ConstValue, JoinFunction, JoinInst, JoinModule, MirLikeInst, UnaryOp,
};

/// JoinModule を JSON としてシリアライズする
///
/// # Example
/// ```ignore
/// let mut output = Vec::new();
/// write_join_module_as_json(&module, &mut output)?;
/// let json_str = String::from_utf8(output)?;
/// ```
pub fn write_join_module_as_json<W: Write>(
    module: &JoinModule,
    out: &mut W,
) -> std::io::Result<()> {
    write!(out, "{{")?;
    write!(out, "\"version\":0")?;

    // entry
    match module.entry {
        Some(entry_id) => write!(out, ",\"entry\":{}", entry_id.0)?,
        None => write!(out, ",\"entry\":null")?,
    }

    // functions
    write!(out, ",\"functions\":[")?;
    let mut first_func = true;
    for func in module.functions.values() {
        if !first_func {
            write!(out, ",")?;
        }
        first_func = false;
        write_function(func, out)?;
    }
    write!(out, "]")?;

    write!(out, "}}")?;
    Ok(())
}

fn write_function<W: Write>(func: &JoinFunction, out: &mut W) -> std::io::Result<()> {
    write!(out, "{{")?;
    write!(out, "\"id\":{}", func.id.0)?;
    write!(out, ",\"name\":\"{}\"", escape_json_string(&func.name))?;

    // params
    write!(out, ",\"params\":[")?;
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(out, ",")?;
        }
        write!(out, "{}", param.0)?;
    }
    write!(out, "]")?;

    // exit_cont
    match func.exit_cont {
        Some(cont_id) => write!(out, ",\"exit_cont\":{}", cont_id.0)?,
        None => write!(out, ",\"exit_cont\":null")?,
    }

    // body
    write!(out, ",\"body\":[")?;
    for (i, inst) in func.body.iter().enumerate() {
        if i > 0 {
            write!(out, ",")?;
        }
        write_inst(inst, out)?;
    }
    write!(out, "]")?;

    write!(out, "}}")?;
    Ok(())
}

fn write_inst<W: Write>(inst: &JoinInst, out: &mut W) -> std::io::Result<()> {
    match inst {
        JoinInst::Call {
            func,
            args,
            k_next,
            dst,
        } => {
            write!(out, "{{\"type\":\"call\"")?;
            write!(out, ",\"func\":{}", func.0)?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            match k_next {
                Some(k) => write!(out, ",\"k_next\":{}", k.0)?,
                None => write!(out, ",\"k_next\":null")?,
            }
            match dst {
                Some(d) => write!(out, ",\"dst\":{}", d.0)?,
                None => write!(out, ",\"dst\":null")?,
            }
            write!(out, "}}")?;
        }
        JoinInst::Jump { cont, args, cond } => {
            write!(out, "{{\"type\":\"jump\"")?;
            write!(out, ",\"cont\":{}", cont.0)?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            match cond {
                Some(c) => write!(out, ",\"cond\":{}", c.0)?,
                None => write!(out, ",\"cond\":null")?,
            }
            write!(out, "}}")?;
        }
        JoinInst::Ret { value } => {
            write!(out, "{{\"type\":\"ret\"")?;
            match value {
                Some(v) => write!(out, ",\"value\":{}", v.0)?,
                None => write!(out, ",\"value\":null")?,
            }
            write!(out, "}}")?;
        }
        // Phase 33: Select instruction JSON serialization
        JoinInst::Select {
            dst,
            cond,
            then_val,
            else_val,
            type_hint,
        } => {
            write!(out, "{{\"type\":\"select\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"cond\":{}", cond.0)?;
            write!(out, ",\"then_val\":{}", then_val.0)?;
            write!(out, ",\"else_val\":{}", else_val.0)?;
            // Phase 63-3: type_hint を JSON 出力（存在する場合のみ）
            if let Some(ref th) = type_hint {
                write!(out, ",\"type_hint\":\"{}\"", format!("{:?}", th))?;
            }
            write!(out, "}}")?;
        }
        // Phase 33-6: IfMerge instruction JSON serialization
        JoinInst::IfMerge {
            cond,
            merges,
            k_next,
        } => {
            write!(out, "{{\"type\":\"if_merge\"")?;
            write!(out, ",\"cond\":{}", cond.0)?;
            write!(out, ",\"merges\":[")?;
            for (i, merge) in merges.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{{")?;
                write!(out, "\"dst\":{}", merge.dst.0)?;
                write!(out, ",\"then_val\":{}", merge.then_val.0)?;
                write!(out, ",\"else_val\":{}", merge.else_val.0)?;
                write!(out, "}}")?;
            }
            write!(out, "]")?;
            match k_next {
                Some(k) => write!(out, ",\"k_next\":{}", k.0)?,
                None => write!(out, ",\"k_next\":null")?,
            }
            write!(out, "}}")?;
        }
        // Phase 34-6: MethodCall instruction JSON serialization
        JoinInst::MethodCall {
            dst,
            receiver,
            method,
            args,
            type_hint, // Phase 65-2-A: 型ヒント追加（JSON には現状出力しない）
        } => {
            write!(out, "{{\"type\":\"method_call\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"receiver\":{}", receiver.0)?;
            write!(out, ",\"method\":\"{}\"", escape_json_string(method))?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            // Phase 65-2-A: TODO: type_hint を JSON に含めるかは Phase 65-3 で検討
            let _ = type_hint; // unused warning 回避
            write!(out, "}}")?;
        }
        // Phase 56: ConditionalMethodCall instruction JSON serialization
        JoinInst::ConditionalMethodCall {
            cond,
            dst,
            receiver,
            method,
            args,
        } => {
            write!(out, "{{\"type\":\"conditional_method_call\"")?;
            write!(out, ",\"cond\":{}", cond.0)?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"receiver\":{}", receiver.0)?;
            write!(out, ",\"method\":\"{}\"", escape_json_string(method))?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            write!(out, "}}")?;
        }
        // Phase 41-4: NestedIfMerge instruction JSON serialization
        JoinInst::NestedIfMerge {
            conds,
            merges,
            k_next,
        } => {
            write!(out, "{{\"type\":\"nested_if_merge\"")?;
            // conds array
            write!(out, ",\"conds\":[")?;
            for (i, cond) in conds.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", cond.0)?;
            }
            write!(out, "]")?;
            // merges array
            write!(out, ",\"merges\":[")?;
            for (i, merge) in merges.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{{")?;
                write!(out, "\"dst\":{}", merge.dst.0)?;
                write!(out, ",\"then_val\":{}", merge.then_val.0)?;
                write!(out, ",\"else_val\":{}", merge.else_val.0)?;
                write!(out, "}}")?;
            }
            write!(out, "]")?;
            // k_next
            match k_next {
                Some(k) => write!(out, ",\"k_next\":{}", k.0)?,
                None => write!(out, ",\"k_next\":null")?,
            }
            write!(out, "}}")?;
        }
        // Phase 51: FieldAccess instruction JSON serialization
        JoinInst::FieldAccess { dst, object, field } => {
            write!(out, "{{\"type\":\"field_access\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"object\":{}", object.0)?;
            write!(out, ",\"field\":\"{}\"", escape_json_string(field))?;
            write!(out, "}}")?;
        }
        // Phase 51: NewBox instruction JSON serialization
        JoinInst::NewBox {
            dst,
            box_name,
            args,
            type_hint, // Phase 65-2-B: 型ヒント追加（JSON には現状出力しない）
        } => {
            write!(out, "{{\"type\":\"new_box\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"box_name\":\"{}\"", escape_json_string(box_name))?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            // Phase 65-2-B: TODO: type_hint を JSON に含めるかは Phase 65-3 で検討
            let _ = type_hint; // unused warning 回避
            write!(out, "}}")?;
        }
        JoinInst::Compute(mir_like) => {
            write!(out, "{{\"type\":\"compute\",\"op\":")?;
            write_mir_like_inst(mir_like, out)?;
            write!(out, "}}")?;
        }
    }
    Ok(())
}

fn write_mir_like_inst<W: Write>(inst: &MirLikeInst, out: &mut W) -> std::io::Result<()> {
    match inst {
        MirLikeInst::Const { dst, value } => {
            write!(out, "{{\"kind\":\"const\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            match value {
                ConstValue::Integer(n) => {
                    write!(out, ",\"value_type\":\"integer\"")?;
                    write!(out, ",\"value\":{}", n)?;
                }
                ConstValue::Bool(b) => {
                    write!(out, ",\"value_type\":\"bool\"")?;
                    write!(out, ",\"value\":{}", b)?;
                }
                ConstValue::String(s) => {
                    write!(out, ",\"value_type\":\"string\"")?;
                    write!(out, ",\"value\":\"{}\"", escape_json_string(s))?;
                }
                ConstValue::Null => {
                    write!(out, ",\"value_type\":\"null\"")?;
                    write!(out, ",\"value\":null")?;
                }
            }
            write!(out, "}}")?;
        }
        MirLikeInst::BinOp { dst, op, lhs, rhs } => {
            write!(out, "{{\"kind\":\"binop\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"op\":\"{}\"", binop_to_str(*op))?;
            write!(out, ",\"lhs\":{}", lhs.0)?;
            write!(out, ",\"rhs\":{}", rhs.0)?;
            write!(out, "}}")?;
        }
        MirLikeInst::Compare { dst, op, lhs, rhs } => {
            write!(out, "{{\"kind\":\"compare\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"op\":\"{}\"", compare_to_str(*op))?;
            write!(out, ",\"lhs\":{}", lhs.0)?;
            write!(out, ",\"rhs\":{}", rhs.0)?;
            write!(out, "}}")?;
        }
        MirLikeInst::BoxCall {
            dst,
            box_name,
            method,
            args,
        } => {
            write!(out, "{{\"kind\":\"boxcall\"")?;
            match dst {
                Some(d) => write!(out, ",\"dst\":{}", d.0)?,
                None => write!(out, ",\"dst\":null")?,
            }
            write!(out, ",\"box\":\"{}\"", escape_json_string(box_name))?;
            write!(out, ",\"method\":\"{}\"", escape_json_string(method))?;
            write!(out, ",\"args\":[")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", arg.0)?;
            }
            write!(out, "]")?;
            write!(out, "}}")?;
        }
        // Phase 56: UnaryOp
        MirLikeInst::UnaryOp { dst, op, operand } => {
            write!(out, "{{\"kind\":\"unaryop\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"op\":\"{}\"", unaryop_to_str(*op))?;
            write!(out, ",\"operand\":{}", operand.0)?;
            write!(out, "}}")?;
        }
        // Phase 188: Print
        MirLikeInst::Print { value } => {
            write!(out, "{{\"kind\":\"print\"")?;
            write!(out, ",\"value\":{}", value.0)?;
            write!(out, "}}")?;
        }
        // Phase 188-Impl-3: Select
        MirLikeInst::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            write!(out, "{{\"kind\":\"select\"")?;
            write!(out, ",\"dst\":{}", dst.0)?;
            write!(out, ",\"cond\":{}", cond.0)?;
            write!(out, ",\"then_val\":{}", then_val.0)?;
            write!(out, ",\"else_val\":{}", else_val.0)?;
            write!(out, "}}")?;
        }
    }
    Ok(())
}

fn binop_to_str(op: BinOpKind) -> &'static str {
    match op {
        BinOpKind::Add => "add",
        BinOpKind::Sub => "sub",
        BinOpKind::Mul => "mul",
        BinOpKind::Div => "div",
        BinOpKind::Mod => "mod", // Phase 188-Impl-3
        BinOpKind::Or => "or",
        BinOpKind::And => "and",
    }
}

fn compare_to_str(op: CompareOp) -> &'static str {
    match op {
        CompareOp::Lt => "lt",
        CompareOp::Le => "le",
        CompareOp::Gt => "gt",
        CompareOp::Ge => "ge",
        CompareOp::Eq => "eq",
        CompareOp::Ne => "ne",
    }
}

// Phase 56: UnaryOp to string
fn unaryop_to_str(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Not => "not",
        UnaryOp::Neg => "neg",
    }
}

/// JSON 文字列のエスケープ
fn escape_json_string(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => escaped.push(c),
        }
    }
    escaped
}

/// JoinModule を JSON 文字列として返す（テスト用ヘルパー）
pub fn join_module_to_json_string(module: &JoinModule) -> String {
    let mut output = Vec::new();
    write_join_module_as_json(module, &mut output).expect("JSON serialization failed");
    String::from_utf8(output).expect("Invalid UTF-8 in JSON output")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::{JoinContId, JoinFuncId, MergePair};
    use crate::mir::ValueId;

    #[test]
    fn test_empty_module() {
        let module = JoinModule::new();
        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"version\":0"));
        assert!(json.contains("\"entry\":null"));
        assert!(json.contains("\"functions\":[]"));
    }

    #[test]
    fn test_simple_function() {
        let mut module = JoinModule::new();
        let mut func =
            JoinFunction::new(JoinFuncId::new(0), "test".to_string(), vec![ValueId(100)]);
        func.body.push(JoinInst::Ret {
            value: Some(ValueId(100)),
        });
        module.add_function(func);
        module.entry = Some(JoinFuncId::new(0));

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"entry\":0"));
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"params\":[100]"));
        assert!(json.contains("\"type\":\"ret\""));
        assert!(json.contains("\"value\":100"));
    }

    #[test]
    fn test_const_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(42),
        }));
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"kind\":\"const\""));
        assert!(json.contains("\"value_type\":\"integer\""));
        assert!(json.contains("\"value\":42"));
    }

    #[test]
    fn test_binop_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: ValueId(3),
            op: BinOpKind::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        }));
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"kind\":\"binop\""));
        assert!(json.contains("\"op\":\"add\""));
        assert!(json.contains("\"lhs\":1"));
        assert!(json.contains("\"rhs\":2"));
    }

    #[test]
    fn test_call_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Call {
            func: JoinFuncId::new(1),
            args: vec![ValueId(100), ValueId(101)],
            k_next: Some(JoinContId::new(5)),
            dst: Some(ValueId(200)),
        });
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"type\":\"call\""));
        assert!(json.contains("\"func\":1"));
        assert!(json.contains("\"args\":[100,101]"));
        assert!(json.contains("\"k_next\":5"));
        assert!(json.contains("\"dst\":200"));
    }

    #[test]
    fn test_jump_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Jump {
            cont: JoinContId::new(3),
            args: vec![ValueId(10)],
            cond: Some(ValueId(5)),
        });
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"type\":\"jump\""));
        assert!(json.contains("\"cont\":3"));
        assert!(json.contains("\"cond\":5"));
    }

    #[test]
    fn test_boxcall_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ValueId(10)),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![ValueId(1)],
        }));
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\"kind\":\"boxcall\""));
        assert!(json.contains("\"box\":\"StringBox\""));
        assert!(json.contains("\"method\":\"length\""));
    }

    #[test]
    fn test_string_escaping() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(1),
            value: ConstValue::String("hello\nworld\"test".to_string()),
        }));
        module.add_function(func);

        let json = join_module_to_json_string(&module);
        assert!(json.contains("\\n"));
        assert!(json.contains("\\\""));
    }

    // Phase 33-6: IfMerge instruction JSON serialization test
    #[test]
    fn test_if_merge_instruction() {
        let mut module = JoinModule::new();
        let mut func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);

        // Add IfMerge instruction with 2 merge pairs
        func.body.push(JoinInst::IfMerge {
            cond: ValueId(1),
            merges: vec![
                MergePair {
                    dst: ValueId(10),
                    then_val: ValueId(20),
                    else_val: ValueId(30),
                    type_hint: None, // Phase 63-3
                },
                MergePair {
                    dst: ValueId(11),
                    then_val: ValueId(21),
                    else_val: ValueId(31),
                    type_hint: None, // Phase 63-3
                },
            ],
            k_next: None,
        });
        module.add_function(func);

        let json = join_module_to_json_string(&module);

        // Verify JSON structure
        assert!(json.contains("\"type\":\"if_merge\""));
        assert!(json.contains("\"cond\":1"));
        assert!(json.contains("\"merges\":["));
        assert!(json.contains("\"dst\":10"));
        assert!(json.contains("\"then_val\":20"));
        assert!(json.contains("\"else_val\":30"));
        assert!(json.contains("\"dst\":11"));
        assert!(json.contains("\"then_val\":21"));
        assert!(json.contains("\"else_val\":31"));
        assert!(json.contains("\"k_next\":null"));
    }
}
