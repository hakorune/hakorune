use super::{AstToJoinIrLowerer, BinOpKind, CompareOp, ConstValue, ExtractCtx, JoinInst, VarId};

impl AstToJoinIrLowerer {
    /// Phase 34-5: expr から「値を計算する JoinIR」と「結果を入れる ValueId」を返す
    ///
    /// ## 設計方針
    ///
    /// - **Int literal**: 新しい dst を割り当てて Const 命令を生成
    /// - **Var 参照**: ctx の var_map から既存 ValueId を引く（追加命令なし）
    /// - **Method 呼び出し**: pattern match のみ実装（JoinIR 出力はダミーでも可）
    ///
    /// ## 戻り値
    ///
    /// - `ValueId`: 結果が入る変数 ID
    /// - `Vec<JoinInst>`: 値を計算するための JoinIR 命令列
    ///
    /// ## Phase 34-5 実装範囲
    ///
    /// - **段階 1**: Int / Var 対応（確実に実装）
    /// - **段階 2**: Method 呼び出し pattern match（ダミー可）
    ///
    /// ## Panics
    ///
    /// - 未対応の expr 形式（Phase 34-5 は tiny テスト専用）
    #[allow(dead_code)] // Phase 34-5.4 で lower_if_return_pattern から呼ばれる
    pub(super) fn extract_value(
        &self,
        expr: &serde_json::Value,
        ctx: &mut ExtractCtx,
    ) -> (VarId, Vec<JoinInst>) {
        let expr_type = expr["type"].as_str().expect("expr must have 'type' field");

        match expr_type {
            // 段階 1: Int literal 対応
            "Int" => {
                let value = expr["value"].as_i64().expect("Int value must be i64");

                let dst = ctx.alloc_var();
                let inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst,
                    value: ConstValue::Integer(value),
                });

                (dst, vec![inst])
            }

            // Phase 34-8: Bool literal 対応
            "Bool" => {
                let value = expr["value"].as_bool().expect("Bool value must be boolean");

                let dst = ctx.alloc_var();
                let inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst,
                    value: ConstValue::Bool(value),
                });

                (dst, vec![inst])
            }

            // Phase 34-ATOI: String literal 対応
            "String" => {
                let value = expr["value"]
                    .as_str()
                    .expect("String literal must have 'value'")
                    .to_string();

                let dst = ctx.alloc_var();
                let inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst,
                    value: ConstValue::String(value),
                });

                (dst, vec![inst])
            }

            // 段階 1: Var 参照対応
            "Var" => {
                let var_name = expr["name"].as_str().expect("Var must have 'name' field");

                let var_id = ctx
                    .get_var(var_name)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", var_name));

                // Var 参照は追加命令なし（既存の ValueId を返すだけ）
                (var_id, vec![])
            }

            // Phase 34-6: Method 呼び出し構造の完全実装
            "Method" | "MethodCall" => {
                // receiver.method(args...) の構造を抽出
                let receiver_expr = &expr["receiver"];
                let method_name = expr["method"]
                    .as_str()
                    .expect("Method must have 'method' field");
                let args_array = expr["args"]
                    .as_array()
                    .expect("Method must have 'args' array");

                // receiver を extract_value で処理
                let (receiver_var, receiver_insts) = self.extract_value(receiver_expr, ctx);

                // args を extract_value で処理
                let mut arg_vars = Vec::new();
                let mut arg_insts = Vec::new();
                for arg_expr in args_array {
                    let (arg_var, arg_inst) = self.extract_value(arg_expr, ctx);
                    arg_vars.push(arg_var);
                    arg_insts.extend(arg_inst);
                }

                // MethodCall 命令を生成
                let dst = ctx.alloc_var();
                let method_call_inst = JoinInst::MethodCall {
                    dst,
                    receiver: receiver_var,
                    method: method_name.to_string(),
                    args: arg_vars,
                    type_hint: None, // Phase 65-2-A: 汎用経路では None（Phase 65-3 で型推論追加予定）
                };

                // すべての命令を結合（receiver → args → MethodCall の順）
                let mut insts = receiver_insts;
                insts.extend(arg_insts);
                insts.push(method_call_inst);

                (dst, insts)
            }

            // Phase 34-7.4a: Binary 演算対応（i + 1 など）
            "Binary" => {
                let op_str = expr["op"].as_str().expect("Binary must have 'op' field");
                let lhs_expr = &expr["lhs"];
                let rhs_expr = &expr["rhs"];

                // op 文字列を BinOpKind に変換
                let op = match op_str {
                    "+" => BinOpKind::Add,
                    "-" => BinOpKind::Sub,
                    "*" => BinOpKind::Mul,
                    "/" => BinOpKind::Div,
                    "&&" => BinOpKind::And,
                    "||" => BinOpKind::Or,
                    _ => panic!("Unsupported binary op: {}", op_str),
                };

                // lhs と rhs を再帰的に extract_value
                let (lhs_var, lhs_insts) = self.extract_value(lhs_expr, ctx);
                let (rhs_var, rhs_insts) = self.extract_value(rhs_expr, ctx);

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // BinOp 命令を生成
                let binop_inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::BinOp {
                    dst,
                    op,
                    lhs: lhs_var,
                    rhs: rhs_var,
                });

                // すべての命令を結合（lhs → rhs → BinOp の順）
                let mut insts = lhs_insts;
                insts.extend(rhs_insts);
                insts.push(binop_inst);

                (dst, insts)
            }

            // Phase 34-7.4a: Compare 演算対応（i < n など）
            "Compare" => {
                let op_str = expr["op"].as_str().expect("Compare must have 'op' field");
                let lhs_expr = &expr["lhs"];
                let rhs_expr = &expr["rhs"];

                // op 文字列を CompareOp に変換
                let op = match op_str {
                    "<" => CompareOp::Lt,
                    "<=" => CompareOp::Le,
                    ">" => CompareOp::Gt,
                    ">=" => CompareOp::Ge,
                    "==" => CompareOp::Eq,
                    "!=" => CompareOp::Ne,
                    _ => panic!("Unsupported compare op: {}", op_str),
                };

                // lhs と rhs を再帰的に extract_value
                let (lhs_var, lhs_insts) = self.extract_value(lhs_expr, ctx);
                let (rhs_var, rhs_insts) = self.extract_value(rhs_expr, ctx);

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // Compare 命令を生成
                let compare_inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Compare {
                    dst,
                    op,
                    lhs: lhs_var,
                    rhs: rhs_var,
                });

                // すべての命令を結合（lhs → rhs → Compare の順）
                let mut insts = lhs_insts;
                insts.extend(rhs_insts);
                insts.push(compare_inst);

                (dst, insts)
            }

            // Phase 51: フィールドアクセス対応（me.tokens 等）
            "Field" => {
                let object_expr = &expr["object"];
                let field_name = expr["field"]
                    .as_str()
                    .expect("Field must have 'field' string");

                // object を再帰的に extract_value
                let (object_var, object_insts) = self.extract_value(object_expr, ctx);

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // FieldAccess 命令を生成
                let field_inst = JoinInst::FieldAccess {
                    dst,
                    object: object_var,
                    field: field_name.to_string(),
                };

                let mut insts = object_insts;
                insts.push(field_inst);

                (dst, insts)
            }

            // Phase 51: NewBox 対応（new ArrayBox() 等）
            "NewBox" => {
                let box_name = expr["box_name"]
                    .as_str()
                    .expect("NewBox must have 'box_name' string");
                let empty_args = vec![];
                let args_array = expr["args"].as_array().unwrap_or(&empty_args);

                // args を再帰的に extract_value
                let mut arg_vars = Vec::new();
                let mut arg_insts = Vec::new();
                for arg_expr in args_array {
                    let (arg_var, arg_inst) = self.extract_value(arg_expr, ctx);
                    arg_vars.push(arg_var);
                    arg_insts.extend(arg_inst);
                }

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // Phase 65-2-B: Box 名から型ヒントを推論
                let type_hint =
                    crate::mir::join_ir::lowering::type_inference::infer_box_type(box_name);

                // NewBox 命令を生成
                let newbox_inst = JoinInst::NewBox {
                    dst,
                    box_name: box_name.to_string(),
                    args: arg_vars,
                    type_hint, // Phase 65-2-B: P3-B Box コンストラクタ型ヒント
                };

                let mut insts = arg_insts;
                insts.push(newbox_inst);

                (dst, insts)
            }

            // Phase 56: Unary 対応（not 等）
            "Unary" => {
                let op = expr["op"].as_str().expect("Unary must have 'op' field");
                let operand_expr = &expr["operand"];

                // operand を再帰的に extract_value
                let (operand_var, operand_insts) = self.extract_value(operand_expr, ctx);

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // UnaryOp 命令を生成
                let unary_op = match op {
                    "not" => crate::mir::join_ir::UnaryOp::Not,
                    "-" => crate::mir::join_ir::UnaryOp::Neg,
                    _ => panic!("Unsupported unary op: {}", op),
                };

                let unary_inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::UnaryOp {
                    dst,
                    op: unary_op,
                    operand: operand_var,
                });

                let mut insts = operand_insts;
                insts.push(unary_inst);

                (dst, insts)
            }

            // Refactor-A: Null literal 対応（実ループ基盤整備）
            "Null" => {
                let dst = ctx.alloc_var();
                let inst = JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                    dst,
                    value: ConstValue::Null,
                });

                (dst, vec![inst])
            }

            // Phase 56: Call（関数呼び出し、pred(v) 等）
            // 変数参照の関数呼び出しは MethodCall で代替（receiver=func_var, method="__call__"）
            "Call" => {
                let func_name = expr["func"]
                    .as_str()
                    .or_else(|| expr["name"].as_str())
                    .expect("Call must have 'func' or 'name' field");
                let empty_args = vec![];
                let args_array = expr["args"]
                    .as_array()
                    .or_else(|| expr["arguments"].as_array())
                    .unwrap_or(&empty_args);

                // func_name を変数として取得（pred, callback 等）
                let func_var = ctx
                    .get_var(func_name)
                    .unwrap_or_else(|| panic!("Undefined function variable: {}", func_name));

                // args を extract_value で処理
                let mut arg_vars = Vec::new();
                let mut arg_insts = Vec::new();
                for arg_expr in args_array {
                    let (arg_var, arg_inst) = self.extract_value(arg_expr, ctx);
                    arg_vars.push(arg_var);
                    arg_insts.extend(arg_inst);
                }

                // 結果変数を割り当て
                let dst = ctx.alloc_var();

                // MethodCall で代替（func_var.__call__(args)）
                let call_inst = JoinInst::MethodCall {
                    dst,
                    receiver: func_var,
                    method: "__call__".to_string(),
                    args: arg_vars,
                    type_hint: None, // Phase 65-2-A: __call__ は汎用的なため型推論不可
                };

                let mut insts = arg_insts;
                insts.push(call_inst);

                (dst, insts)
            }

            _ => panic!("Unsupported expr type: {}", expr_type),
        }
    }
}
